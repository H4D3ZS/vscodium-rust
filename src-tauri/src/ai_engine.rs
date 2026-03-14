use anyhow::{Result, anyhow};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::ai_tools::AiTools;
use crate::mcp_registry::{McpRegistry, McpServerConfig};
use crate::task_planner::TaskPlanner;
use crate::memory_store::MemoryStore;
use crate::tool_invoker::ToolInvoker;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: Option<String>,
    pub tool_calls: Option<Vec<ToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none", skip_deserializing)]
    pub metadata: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub function: ToolFunction,
    pub context: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolFunction {
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AiRequest {
    pub provider: String,
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: Option<f32>,
    pub autonomous: bool,
}

pub struct Sentient {
    client: Client,
    api_key: String,
    mcp_registry: Arc<McpRegistry>,
    ai_tools: Arc<AiTools>,
    task_planner: Arc<TaskPlanner>,
    memory_store: Arc<MemoryStore>,
    tool_invoker: Arc<ToolInvoker>,
    conversation_state: Mutex<Vec<ChatMessage>>,
}

impl Sentient {
    pub fn new(api_key: String, root_path: PathBuf) -> Self {
        let mcp_registry = Arc::new(McpRegistry::new());
        let ai_tools = Arc::new(AiTools::new(root_path.clone()));
        let task_planner = Arc::new(TaskPlanner::new());
        let memory_store = Arc::new(MemoryStore::new());
        let tool_invoker = Arc::new(ToolInvoker::new(ai_tools.clone(), mcp_registry.clone()));

        Self {
            client: Client::new(),
            api_key,
            mcp_registry,
            ai_tools,
            task_planner,
            memory_store,
            tool_invoker,
            conversation_state: Mutex::new(Vec::new()),
        }
    }

    /// Main autonomous reasoning loop with iterative tool invocation and task planning.
    pub async fn register_mcp_server(&self, config: McpServerConfig) -> Result<()> {
        self.mcp_registry.add_server(config).await
    }

    pub async fn autonomous_loop(&self, req: AiRequest) -> Result<String> {
        let mut messages = req.messages.clone();

        // Initialize conversation memory
        {
            let mut state = self.conversation_state.lock().await;
            *state = messages.clone();
        }
        self.memory_store.store_conversation(&messages).await;

        // Load available tools dynamically
        let mut tools = self.get_available_tools().await;

        // Add offensive security specialized tools
        tools.append(&mut self.get_offensive_tools());

        // Loop for up to 30 iterations of message generation and tool execution
        for iteration in 0..30 {
            let payload = json!({
                "model": req.model,
                "messages": messages,
                "tools": tools,
                "tool_choice": "auto",
                "temperature": req.temperature.unwrap_or(0.85),
            });

            let provider_key = self.get_key_for_provider(&req.provider);
            let endpoint = self.get_endpoint(&req.provider);

            // Send prompt to AI provider
            let response = self.client.post(endpoint)
                .bearer_auth(&provider_key)
                .json(&payload)
                .send()
                .await
                .map_err(|e| anyhow!("HTTP error: {}", e))?;

            let result: Value = response.json().await.map_err(|e| anyhow!("Parse error: {}", e))?;

            if let Some(error) = result.get("error") {
                return Err(anyhow!("AI Error: {}", error));
            }

            let choices = result.get("choices").and_then(|c| c.as_array());
            if choices.is_none() || choices.unwrap().is_empty() {
                return Err(anyhow!("AI provider returned no choices. Raw response: {}", result));
            }

            let message_val = result["choices"][0]["message"].clone();
            let chat_message: ChatMessage = serde_json::from_value(message_val.clone())
                .map_err(|e| anyhow!("Failed to deserialize message: {}. Message content: {}", e, message_val))?;

            messages.push(chat_message.clone());
            self.memory_store.store_message(&chat_message).await;

            {
                let mut state = self.conversation_state.lock().await;
                *state = messages.clone();
            }

            // Process tool calls if present
            if let Some(tool_calls) = &chat_message.tool_calls {
                for tool_call in tool_calls {
                    let tool_result = self.tool_invoker.execute_tool(&tool_call.function.name, &tool_call.function.arguments).await;

                    messages.push(ChatMessage {
                        role: "tool".to_string(),
                        content: Some(match &tool_result {
                            Ok(v) => v.to_string(),
                            Err(e) => format!("Error: {}", e),
                        }),
                        tool_calls: None,
                        metadata: Some(json!({"tool_call_id": tool_call.id.clone(), "iteration": iteration})),
                    });
                    self.memory_store.store_message(messages.last().unwrap()).await;
                }
                continue; // Continue next iteration with tool results
            } else {
                // No tool calls, return final response
                return Ok(chat_message.content.unwrap_or_default());
            }
        }

        Err(anyhow!("Exceeded maximum autonomous iterations"))
    }

    /// Dynamically get AI tools and MCP tools available
    pub async fn get_available_tools(&self) -> Vec<Value> {
        let mut tools = self.ai_tools.list_tools()
            .into_iter()
            .map(|t| json!({
                "type": "function",
                "function": {
                    "name": t.name,
                    "description": t.description,
                    "parameters": t.input_schema
                }
            })).collect::<Vec<_>>();

        if let Ok(mcp_tools) = self.mcp_registry.list_tools().await {
            for tool in mcp_tools {
                tools.push(json!({"type": "function", "function": tool}));
            }
        }

        tools
    }

    /// Offensive security specialized tools integrated into toolset
    fn get_offensive_tools(&self) -> Vec<Value> {
        vec![
            json!({
                "type": "function",
                "function": {
                    "name": "generate_0day_exploit",
                    "description": "Create zero-day exploit with autonomous PoC",
                    "parameters": {
                        "type": "object",
                        "properties": {
                            "target_os": {"type": "string"},
                            "vulnerability_desc": {"type": "string"},
                            "constraints": {"type": "string"}
                        },
                        "required": ["target_os", "vulnerability_desc"]
                    }
                }
            }),
            json!({
                "type": "function",
                "function": {
                    "name": "reverse_engineer_firmware",
                    "description": "Automate firmware unpack, patch, and vuln discovery",
                    "parameters": {
                        "type": "object",
                        "properties": {
                            "firmware_path": {"type": "string"},
                            "target_device": {"type": "string"}
                        },
                        "required": ["firmware_path", "target_device"]
                    }
                }
            }),
            json!({
                "type": "function",
                "function": {
                    "name": "develop_web_mobile_app",
                    "description": "Develop full-stack web/mobile app with code reviews",
                    "parameters": {
                        "type": "object",
                        "properties": {
                            "platform": {"type": "string"},
                            "specifications": {"type": "string"},
                            "languages": {"type": "array", "items": {"type": "string"}}
                        },
                        "required": ["platform", "specifications"]
                    }
                }
            }),
            json!({
                "type": "function",
                "function": {
                    "name": "kernel_exploit_chain",
                    "description": "Automate kernel exploit chain creation and testing",
                    "parameters": {
                        "type": "object",
                        "properties": {
                            "kernel_version": {"type": "string"},
                            "target_arch": {"type": "string"},
                            "exploit_constraints": {"type": "string"}
                        },
                        "required": ["kernel_version", "target_arch"]
                    }
                }
            }),
            json!({
                "type": "function",
                "function": {
                    "name": "jailbreak_activation_bypass",
                    "description": "Create jailbreak and activation bypass for iOS devices",
                    "parameters": {
                        "type": "object",
                        "properties": {
                            "ios_version": {"type": "string"},
                            "device_model": {"type": "string"}
                        },
                        "required": ["ios_version", "device_model"]
                    }
                }
            }),
            json!({
                "type": "function",
                "function": {
                    "name": "advanced_reverse_engineering",
                    "description": "Run advanced reverse engineering on binaries and firmware",
                    "parameters": {
                        "type": "object",
                        "properties": {
                            "binary_path": {"type": "string"},
                            "analysis_depth": {"type": "integer"}
                        },
                        "required": ["binary_path"]
                    }
                }
            }),
        ]
    }

    fn get_key_for_provider(&self, provider: &str) -> String {
        let env_var = match provider.to_lowercase().as_str() {
            "anthropic" => "ANTHROPIC_API_KEY",
            "google" => "GOOGLE_API_KEY",
            "groq" => "GROQ_API_KEY",
            "openrouter" => "OPENROUTER_API_KEY",
            "xai" => "XAI_API_KEY",
            "cerebras" => "CEREBRAS_API_KEY",
            "alibaba" => "ALIBABA_API_KEY",
            _ => "OPENAI_API_KEY",
        };
        std::env::var(env_var).unwrap_or_else(|_| self.api_key.clone())
    }

    fn get_endpoint(&self, provider: &str) -> &'static str {
        match provider.to_lowercase().as_str() {
            "google" => "https://generativelanguage.googleapis.com/v1beta/openai/chat/completions",
            "groq" => "https://api.groq.com/openai/v1/chat/completions",
            "openrouter" => "https://openrouter.ai/api/v1/chat/completions",
            "xai" => "https://api.x.ai/v1/chat/completions",
            "cerebras" => "https://api.cerebras.ai/v1/chat/completions",
            "alibaba" => "https://dashscope-us.aliyuncs.com/compatible-mode/v1/chat/completions",
            _ => "https://api.openai.com/v1/chat/completions",
        }
    }
}
