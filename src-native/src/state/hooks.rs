#[derive(Clone, Debug)]
pub struct AgentHook {
    pub id: String,
    pub pattern: String,
    pub prompt: String,
    pub enabled: bool,
}
