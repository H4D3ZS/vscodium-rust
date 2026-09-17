use crate::panels::traits::{AuxiliaryTab, BottomPanelTab, WorkbenchPanel};
use std::sync::Arc;

pub struct PanelRegistry {
    pub primary_panels: Vec<Arc<dyn WorkbenchPanel>>,
    pub bottom_tabs: Vec<Arc<dyn BottomPanelTab>>,
    pub auxiliary_tabs: Vec<Arc<dyn AuxiliaryTab>>,
}

impl PanelRegistry {
    pub fn new() -> Self {
        Self {
            primary_panels: Vec::new(),
            bottom_tabs: Vec::new(),
            auxiliary_tabs: Vec::new(),
        }
    }

    pub fn register_primary<P: WorkbenchPanel>(&mut self, panel: P) {
        self.primary_panels.push(Arc::new(panel));
    }

    pub fn register_bottom<B: BottomPanelTab>(&mut self, tab: B) {
        self.bottom_tabs.push(Arc::new(tab));
    }

    pub fn register_auxiliary<A: AuxiliaryTab>(&mut self, tab: A) {
        self.auxiliary_tabs.push(Arc::new(tab));
    }

    pub fn find_primary(&self, id: &str) -> Option<Arc<dyn WorkbenchPanel>> {
        self.primary_panels.iter().find(|p| p.id() == id).cloned()
    }

    pub fn find_bottom(&self, id: &str) -> Option<Arc<dyn BottomPanelTab>> {
        self.bottom_tabs.iter().find(|p| p.id() == id).cloned()
    }

    pub fn find_auxiliary(&self, id: &str) -> Option<Arc<dyn AuxiliaryTab>> {
        self.auxiliary_tabs.iter().find(|p| p.id() == id).cloned()
    }
}
