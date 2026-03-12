use crate::ai_tools::AiTools;
use crate::mcp_registry::McpRegistry;
use anyhow::Result;
use serde_json::Value;
use std::sync::Arc;

pub struct ToolInvoker {
    ai_tools: Arc<AiTools>,
    mcp_registry: Arc<McpRegistry>,
}

impl ToolInvoker {
    pub fn new(ai_tools: Arc<AiTools>, mcp_registry: Arc<McpRegistry>) -> Self {
        Self { ai_tools, mcp_registry }
    }

    pub async fn execute_tool(&self, name: &str, args: &str) -> Result<Value> {
        let arguments: Value = serde_json::from_str(args)?;

        if let Ok(result) = self.ai_tools.call_tool(name, arguments.clone()) {
            return Ok(result);
        }

        self.mcp_registry.call_tool(name, arguments).await
    }
}
