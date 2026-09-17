use crate::app_state::HadesNativeState;
use crate::ui::icons::IconName;
use crate::HadesAppView;
use gpui_kit::gpui::*;

/// Polymorphic contract for Primary Sidebar Panels (Explorer, Search, SCM, Debug, Extensions, Mobile, Apex, Settings).
pub trait WorkbenchPanel: 'static {
    fn id(&self) -> &'static str;
    fn title(&self) -> &'static str;
    fn icon(&self) -> IconName;
    fn badge(&self, _state: &HadesNativeState) -> Option<String> {
        None
    }
    fn render(&self, state: &HadesNativeState, cx: &mut Context<HadesAppView>) -> AnyElement;
}

/// Polymorphic contract for Bottom Drawer Panels (Terminal, Problems, Output, Debug Console).
pub trait BottomPanelTab: 'static {
    fn id(&self) -> &'static str;
    fn title(&self) -> &'static str;
    fn badge(&self, _state: &HadesNativeState) -> Option<String> {
        None
    }
    fn render(&self, state: &HadesNativeState, cx: &mut Context<HadesAppView>) -> AnyElement;
}

/// Polymorphic contract for Right Auxiliary Sidebar Panels (Composer/Chat, Specs, AIM, Rules).
pub trait AuxiliaryTab: 'static {
    fn id(&self) -> &'static str;
    fn title(&self) -> &'static str;
    fn icon(&self) -> IconName;
    fn badge(&self, _state: &HadesNativeState) -> Option<String> {
        None
    }
    fn render(&self, state: &HadesNativeState, cx: &mut Context<HadesAppView>) -> AnyElement;
}
