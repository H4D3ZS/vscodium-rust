#[derive(Clone, Debug)]
pub struct SpecsTask {
    pub id: String,
    pub title: String,
    pub status: String,     // "todo", "in_progress", "done"
    pub complexity: String, // "Easy", "Medium", "Hard"
    pub estimated_hours: u32,
    pub details: String,
}

#[derive(Clone, Debug)]
pub struct SpecsProject {
    pub id: usize,
    pub name: String,
    pub specs: String,
    pub tasks: Vec<SpecsTask>,
}
