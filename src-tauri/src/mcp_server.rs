use axum::{routing::post, Router, Json};
use serde_json::{json, Value};
use std::sync::Arc;

use crate::ai_tools::AiTools;

pub struct McpServer {
    ai_tools: Arc<AiTools>,
}

impl McpServer {
    pub fn new(ai_tools: Arc<AiTools>) -> Self {
        Self { ai_tools }
    }

    pub async fn start(self: Arc<Self>, port: u16) {
        let app = Router::new()
            .route("/mcp/v1/call", post(move |body: Json<Value>| {
                let server = self.clone();
                async move {
                    let result = server.handle_call(body.0).await;
                    Json(result)
                }
            }));

        let addr = format!("127.0.0.1:{}", port);
        println!("[MCP-SERVER] Listening on {}", addr);
        
        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }

    async fn handle_call(&self, request: Value) -> Value {
        let method = request.get("method").and_then(|v| v.as_str()).unwrap_or("");
        let params = request.get("params").cloned().unwrap_or(json!({}));

        match method {
            "list_tools" => {
                let tools = self.ai_tools.list_tools();
                json!({ "result": tools })
            }
            "call_tool" => {
                let name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let args = params.get("arguments").cloned().unwrap_or(json!({}));
                match self.ai_tools.call_tool(name, args).await {
                    Ok(res) => json!({ "result": res }),
                    Err(e) => json!({ "error": e.to_string() }),
                }
            }
            _ => json!({ "error": "Method not found" }),
        }
    }
}
