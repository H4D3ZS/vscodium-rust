pub mod actions;

use crate::ui::icons::IconName;
use crate::HadesAppView;
use gpui_kit::gpui::*;
use std::sync::Arc;

pub type CommandFn =
    Arc<dyn Fn(&mut HadesAppView, &mut Window, &mut Context<HadesAppView>) + Send + Sync>;

#[derive(Clone)]
pub struct Command {
    pub id: &'static str,
    pub title: &'static str,
    pub category: &'static str,
    pub shortcut: Option<&'static str>,
    pub icon: IconName,
    pub handler: CommandFn,
}

#[derive(Clone)]
pub struct CommandRegistry {
    commands: Vec<Command>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            commands: Vec::new(),
        };
        actions::register_all_commands(&mut registry);
        registry
    }

    pub fn register(&mut self, command: Command) {
        self.commands.push(command);
    }

    pub fn all(&self) -> &[Command] {
        &self.commands
    }

    pub fn find(&self, id: &str) -> Option<&Command> {
        self.commands.iter().find(|c| c.id == id)
    }

    pub fn search(&self, query: &str) -> Vec<Command> {
        if query.is_empty() {
            return self.commands.clone();
        }
        let q = query.to_lowercase();
        self.commands
            .iter()
            .filter(|c| {
                c.title.to_lowercase().contains(&q)
                    || c.category.to_lowercase().contains(&q)
                    || c.id.to_lowercase().contains(&q)
            })
            .cloned()
            .collect()
    }

    pub fn execute(
        &self,
        id: &str,
        view: &mut HadesAppView,
        window: &mut Window,
        cx: &mut Context<HadesAppView>,
    ) -> bool {
        if let Some(cmd) = self.find(id) {
            let handler = cmd.handler.clone();
            handler(view, window, cx);
            true
        } else {
            false
        }
    }
}
