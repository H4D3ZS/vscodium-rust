use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityTab {
    Explorer,
    AiChat,
    Search,
    SourceControl,
    Debug,
    Extensions,
    Terminal,
    Canvases,
    ApexSecurity,
    IosMirror,
    PyTorchStudio,
    VisionGrounding,
    ExternalBrowser,
    Settings,
}

impl ActivityTab {
    pub fn id(&self) -> &'static str {
        match self {
            ActivityTab::Explorer => "explorer",
            ActivityTab::AiChat => "aichat",
            ActivityTab::Search => "search",
            ActivityTab::SourceControl => "scm",
            ActivityTab::Debug => "debug",
            ActivityTab::Extensions => "extensions",
            ActivityTab::Terminal => "terminal",
            ActivityTab::Canvases => "canvases",
            ActivityTab::ApexSecurity => "apex",
            ActivityTab::IosMirror => "mobile",
            ActivityTab::PyTorchStudio => "pytorch",
            ActivityTab::VisionGrounding => "vision",
            ActivityTab::ExternalBrowser => "browser",
            ActivityTab::Settings => "settings",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RightSidebarTab {
    AgentChat,
    AgentStudio,
    Specs,
    Rules,
    Kortex,
    Mobile,
    History,
    VectorSearch,
}

impl RightSidebarTab {
    pub fn id(&self) -> &'static str {
        match self {
            RightSidebarTab::AgentChat => "chat",
            RightSidebarTab::AgentStudio => "studio",
            RightSidebarTab::Specs => "specs",
            RightSidebarTab::Rules => "rules",
            RightSidebarTab::Kortex => "kortex",
            RightSidebarTab::Mobile => "mobile",
            RightSidebarTab::History => "history",
            RightSidebarTab::VectorSearch => "vector_search",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmulatorSubTab {
    Android,
    IPhone,
    Device,
    Toolchain,
    Gradle,
}

impl Default for EmulatorSubTab {
    fn default() -> Self {
        EmulatorSubTab::Device
    }
}

impl RightSidebarTab {
    pub fn title(&self) -> &'static str {
        match self {
            RightSidebarTab::AgentChat => "Chat",
            RightSidebarTab::AgentStudio => "Studio",
            RightSidebarTab::Specs => "Specs",
            RightSidebarTab::Rules => "Rules",
            RightSidebarTab::Kortex => "Kortex",
            RightSidebarTab::Mobile => "Mobile",
            RightSidebarTab::History => "History",
            RightSidebarTab::VectorSearch => "Vector Search",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BottomPanelTab {
    Terminal,
    Output,
    DebugConsole,
    Logcat,
    Problems,
    Ports,
    Jobs,
}

impl BottomPanelTab {
    pub fn id(&self) -> &'static str {
        match self {
            BottomPanelTab::Terminal => "terminal",
            BottomPanelTab::Output => "output",
            BottomPanelTab::DebugConsole => "debug_console",
            BottomPanelTab::Logcat => "logcat",
            BottomPanelTab::Problems => "problems",
            BottomPanelTab::Ports => "ports",
            BottomPanelTab::Jobs => "jobs",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FocusedPanel {
    Editor,
    Composer,
    Terminal,
    Explorer,
}

#[derive(Clone, Debug)]
pub struct LayoutStore {
    pub sidebar_open: bool,
    pub sidebar_width: f32,
    pub right_sidebar_open: bool,
    pub right_sidebar_width: f32,
    pub iphone_preview_open: bool,
    pub iphone_preview_width: f32,
    pub bottom_panel_open: bool,
    pub bottom_panel_height: f32,
    pub active_activity_tab: ActivityTab,
    pub active_right_tab: RightSidebarTab,
    pub active_bottom_tab: BottomPanelTab,
    pub focused_panel: FocusedPanel,
}

impl Default for LayoutStore {
    fn default() -> Self {
        Self {
            sidebar_open: true,
            sidebar_width: 260.0,
            right_sidebar_open: true,
            right_sidebar_width: 380.0,
            iphone_preview_open: false,
            iphone_preview_width: 390.0,
            bottom_panel_open: false,
            bottom_panel_height: 220.0,
            active_activity_tab: ActivityTab::Explorer,
            active_right_tab: RightSidebarTab::AgentChat,
            active_bottom_tab: BottomPanelTab::Terminal,
            focused_panel: FocusedPanel::Editor,
        }
    }
}
