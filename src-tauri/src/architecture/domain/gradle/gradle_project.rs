use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradleModule {
    pub path: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradleTask {
    pub path: String,
    pub name: String,
    pub group: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradleProject {
    pub root: String,
    pub is_android: bool,
    pub uses_kotlin: bool,
    pub wrapper_present: bool,
    pub modules: Vec<GradleModule>,
    pub tasks: Vec<GradleTask>,
}
