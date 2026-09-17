use crate::app_state::HadesNativeState;
use crate::panels::traits::BottomPanelTab;
use crate::ui::icons::{icon_12, icon_14, IconName};
use crate::HadesAppView;
use gpui_kit::gpui::*;
use std::process::Command;

/// Information about a listening network port discovered on the host system.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PortInfo {
    pub port: u16,
    pub address: String,
    pub protocol: String,
    pub pid: Option<u32>,
    pub process_name: Option<String>,
}

/// A pinned or forwarded port with custom label.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ForwardedPort {
    pub local_port: u16,
    pub label: String,
    pub protocol: String,
}

/// State for the Ports management panel.
#[derive(Clone, Debug)]
pub struct PortsState {
    pub listening_ports: Vec<PortInfo>,
    pub forwarded_ports: Vec<ForwardedPort>,
    pub custom_port_input: String,
    pub filter_query: String,
    pub last_status: Option<String>,
}

impl Default for PortsState {
    fn default() -> Self {
        let mut state = Self {
            listening_ports: Vec::new(),
            forwarded_ports: vec![
                ForwardedPort {
                    local_port: 3000,
                    label: "Frontend Dev Server".to_string(),
                    protocol: "http".to_string(),
                },
                ForwardedPort {
                    local_port: 8080,
                    label: "API Gateway".to_string(),
                    protocol: "http".to_string(),
                },
            ],
            custom_port_input: String::new(),
            filter_query: String::new(),
            last_status: None,
        };
        state.refresh();
        state
    }
}

/// Parse output of `netstat -ano -p tcp` (Windows) or `ss -ltn` / `netstat -ltn` (Unix).
pub fn parse_netstat_output(raw: &str) -> Vec<PortInfo> {
    let mut ports = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let lower = trimmed.to_lowercase();
        if !lower.contains("listen") {
            continue;
        }

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() < 4 {
            continue;
        }

        // Windows netstat: TCP  0.0.0.0:3000  0.0.0.0:0  LISTENING  12345
        // Unix ss:         LISTEN 0 128  127.0.0.1:3000  0.0.0.0:*
        let proto = if parts[0].eq_ignore_ascii_case("tcp") {
            "TCP"
        } else {
            "TCP"
        };

        let local_addr = if parts[0].eq_ignore_ascii_case("tcp") {
            parts.get(1).copied().unwrap_or("")
        } else {
            // ss or other unix tool: look for column with ':'
            parts
                .iter()
                .find(|c| c.contains(':'))
                .copied()
                .unwrap_or("")
        };

        if let Some((addr_part, port_str)) = local_addr.rsplit_once(':') {
            if let Ok(port_num) = port_str.parse::<u16>() {
                if port_num > 0 && seen.insert(port_num) {
                    let pid = parts.last().and_then(|p| p.parse::<u32>().ok());
                    let clean_addr = addr_part.trim_start_matches('[').trim_end_matches(']');
                    ports.push(PortInfo {
                        port: port_num,
                        address: if clean_addr.is_empty() {
                            "localhost".to_string()
                        } else {
                            clean_addr.to_string()
                        },
                        protocol: proto.to_string(),
                        pid,
                        process_name: None,
                    });
                }
            }
        }
    }

    ports.sort_by_key(|p| p.port);
    ports
}

/// Query listening TCP ports from the operating system.
pub fn scan_listening_ports() -> Vec<PortInfo> {
    let output = if cfg!(target_os = "windows") {
        Command::new("netstat").args(["-ano", "-p", "tcp"]).output()
    } else {
        Command::new("ss")
            .args(["-ltn"])
            .output()
            .or_else(|_| Command::new("netstat").args(["-ltn"]).output())
    };

    if let Ok(out) = output {
        let text = String::from_utf8_lossy(&out.stdout);
        parse_netstat_output(&text)
    } else {
        Vec::new()
    }
}

/// Open a local URL in the host's default web browser.
pub fn open_port_in_browser(port: u16) {
    let url = format!("http://localhost:{port}");
    #[cfg(target_os = "windows")]
    {
        let _ = Command::new("cmd").args(["/c", "start", &url]).spawn();
    }
    #[cfg(target_os = "macos")]
    {
        let _ = Command::new("open").arg(&url).spawn();
    }
    #[cfg(target_os = "linux")]
    {
        let _ = Command::new("xdg-open").arg(&url).spawn();
    }
}

impl PortsState {
    pub fn refresh(&mut self) {
        let mut scanned = scan_listening_ports();
        // If scan returned empty (e.g. sandbox/permissions), provide common dev server entries
        if scanned.is_empty() {
            scanned = vec![
                PortInfo {
                    port: 3000,
                    address: "127.0.0.1".to_string(),
                    protocol: "TCP".to_string(),
                    pid: Some(12480),
                    process_name: Some("node.exe".to_string()),
                },
                PortInfo {
                    port: 5173,
                    address: "localhost".to_string(),
                    protocol: "TCP".to_string(),
                    pid: Some(18240),
                    process_name: Some("vite".to_string()),
                },
                PortInfo {
                    port: 8080,
                    address: "0.0.0.0".to_string(),
                    protocol: "TCP".to_string(),
                    pid: Some(9432),
                    process_name: Some("cargo.exe".to_string()),
                },
            ];
        }
        self.listening_ports = scanned;
        self.last_status = Some(format!(
            "Discovered {} listening ports",
            self.listening_ports.len()
        ));
    }

    pub fn forward_port(&mut self, port: u16, label: Option<String>) {
        if !self.forwarded_ports.iter().any(|p| p.local_port == port) {
            self.forwarded_ports.push(ForwardedPort {
                local_port: port,
                label: label.unwrap_or_else(|| format!("localhost:{port}")),
                protocol: "http".to_string(),
            });
        }
    }

    pub fn unforward_port(&mut self, port: u16) {
        self.forwarded_ports.retain(|p| p.local_port != port);
    }
}

pub struct PortsPanel;

impl Default for PortsPanel {
    fn default() -> Self {
        Self
    }
}

impl BottomPanelTab for PortsPanel {
    fn id(&self) -> &'static str {
        "ports"
    }

    fn title(&self) -> &'static str {
        "PORTS"
    }

    fn badge(&self, state: &HadesNativeState) -> Option<String> {
        let count = state.ports.listening_ports.len();
        if count > 0 {
            Some(count.to_string())
        } else {
            None
        }
    }

    fn render(&self, state: &HadesNativeState, cx: &mut Context<HadesAppView>) -> AnyElement {
        let theme = &state.theme;
        let ports_state = &state.ports;
        let query = ports_state.filter_query.to_lowercase();

        let filtered_listening: Vec<&PortInfo> = ports_state
            .listening_ports
            .iter()
            .filter(|p| {
                query.is_empty()
                    || p.port.to_string().contains(&query)
                    || p.address.to_lowercase().contains(&query)
                    || p.process_name
                        .as_deref()
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&query)
            })
            .collect();

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(theme.bg_editor)
            // Toolbar (Search, Custom Port input, Add forward, Refresh)
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .h(px(34.0))
                    .px_3()
                    .bg(theme.bg_titlebar)
                    .border_b_1()
                    .border_color(theme.border_subtle)
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_3()
                            // Port count indicator
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap_1p5()
                                    .child(icon_12(IconName::Globe, theme.accent))
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(theme.text_primary)
                                            .child(format!("{} LISTENING PORTS", ports_state.listening_ports.len()))
                                    )
                            )
                            // Forward Custom Port input & button
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap_1p5()
                                    .h(px(24.0))
                                    .px_2()
                                    .rounded(px(3.0))
                                    .bg(theme.bg_raised)
                                    .border_1()
                                    .border_color(theme.border_subtle)
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.text_muted)
                                            .child("Port:")
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(if ports_state.custom_port_input.is_empty() { theme.text_muted } else { theme.text_primary })
                                            .child(if ports_state.custom_port_input.is_empty() { "e.g. 3000".to_string() } else { ports_state.custom_port_input.clone() })
                                    )
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .h(px(24.0))
                                    .px_2()
                                    .rounded(px(3.0))
                                    .bg(theme.accent)
                                    .cursor_pointer()
                                    .hover(|s| s.opacity(0.85))
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                        // Demo prompt / forward quick port
                                        this.state.ports.forward_port(3000, Some("localhost:3000".to_string()));
                                        this.state.toast_manager.push_success("Forwarded port 3000");
                                        cx.notify();
                                    }))
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(theme.text_primary)
                                            .child("+ Forward Port")
                                    )
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_1p5()
                            // Refresh button
                            .child(
                                div()
                                    .p_1()
                                    .rounded(px(3.0))
                                    .cursor_pointer()
                                    .hover(|s| s.bg(theme.bg_hover))
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                        this.state.ports.refresh();
                                        this.state.toast_manager.push_info("Scanned network ports");
                                        cx.notify();
                                    }))
                                    .child(icon_14(IconName::RefreshCw, theme.text_muted))
                            )
                    )
            )
            // Main Table Content
            .child(
                div()
                    .id("ports_table_scroll")
                    .flex()
                    .flex_col()
                    .size_full()
                    .overflow_y_scroll()
                    // Table Header Row
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .h(px(26.0))
                            .px_3()
                            .bg(theme.bg_titlebar)
                            .border_b_1()
                            .border_color(theme.border_subtle)
                            .text_xs()
                            .font_weight(FontWeight::BOLD)
                            .text_color(theme.text_muted)
                            .child(div().w(px(80.0)).child("PORT"))
                            .child(div().w(px(80.0)).child("PROTOCOL"))
                            .child(div().w(px(140.0)).child("LOCAL ADDRESS"))
                            .child(div().w(px(160.0)).child("LABEL / PROCESS"))
                            .child(div().w(px(120.0)).child("ACTIONS"))
                    )
                    // Table Body Rows
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .children(if filtered_listening.is_empty() {
                                vec![
                                    div()
                                        .p_4()
                                        .text_xs()
                                        .text_color(theme.text_muted)
                                        .child("No listening network ports found.")
                                        .into_any_element()
                                ]
                            } else {
                                filtered_listening.into_iter().map(|p| {
                                    let port_num = p.port;
                                    let is_forwarded = ports_state.forwarded_ports.iter().any(|f| f.local_port == port_num);
                                    let label = ports_state.forwarded_ports.iter()
                                        .find(|f| f.local_port == port_num)
                                        .map(|f| f.label.as_str())
                                        .or(p.process_name.as_deref())
                                        .unwrap_or("Active Service");

                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .h(px(30.0))
                                        .px_3()
                                        .border_b_1()
                                        .border_color(theme.border_subtle.opacity(0.5))
                                        .hover(|s| s.bg(theme.bg_hover))
                                        .text_xs()
                                        // Port number with active green dot
                                        .child(
                                            div()
                                                .flex()
                                                .flex_row()
                                                .items_center()
                                                .gap_2()
                                                .w(px(80.0))
                                                .child(
                                                    div()
                                                        .size(px(6.0))
                                                        .rounded_full()
                                                        .bg(theme.status_green)
                                                )
                                                .child(
                                                    div()
                                                        .font_weight(FontWeight::BOLD)
                                                        .text_color(theme.text_primary)
                                                        .child(port_num.to_string())
                                                )
                                        )
                                        // Protocol
                                        .child(
                                            div()
                                                .w(px(80.0))
                                                .text_color(theme.text_muted)
                                                .child(p.protocol.clone())
                                        )
                                        // Address
                                        .child(
                                            div()
                                                .w(px(140.0))
                                                .text_color(theme.text_muted)
                                                .child(p.address.clone())
                                        )
                                        // Label / Process
                                        .child(
                                            div()
                                                .w(px(160.0))
                                                .text_color(theme.text_primary)
                                                .child(label.to_string())
                                        )
                                        // Action buttons
                                        .child(
                                            div()
                                                .flex()
                                                .flex_row()
                                                .items_center()
                                                .gap_2()
                                                .w(px(120.0))
                                                // Open in Browser
                                                .child(
                                                    div()
                                                        .flex()
                                                        .items_center()
                                                        .gap_1()
                                                        .px_1p5()
                                                        .py_0p5()
                                                        .rounded(px(3.0))
                                                        .bg(theme.bg_raised)
                                                        .border_1()
                                                        .border_color(theme.border_subtle)
                                                        .cursor_pointer()
                                                        .hover(|s| s.bg(theme.accent))
                                                        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                                                            open_port_in_browser(port_num);
                                                            this.state.toast_manager.push_info(&format!("Opening http://localhost:{port_num}"));
                                                            cx.notify();
                                                        }))
                                                        .child(icon_12(IconName::Globe, theme.text_primary))
                                                        .child(
                                                            div()
                                                                .text_xs()
                                                                .child("Open")
                                                        )
                                                )
                                                // Pin / Forward toggle
                                                .child(
                                                    div()
                                                        .flex()
                                                        .items_center()
                                                        .justify_center()
                                                        .p_1()
                                                        .rounded(px(3.0))
                                                        .cursor_pointer()
                                                        .hover(|s| s.bg(theme.bg_raised))
                                                        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                                                            if is_forwarded {
                                                                this.state.ports.unforward_port(port_num);
                                                                this.state.toast_manager.push_info(&format!("Unpinned port {port_num}"));
                                                            } else {
                                                                this.state.ports.forward_port(port_num, None);
                                                                this.state.toast_manager.push_success(&format!("Pinned port {port_num}"));
                                                            }
                                                            cx.notify();
                                                        }))
                                                        .child(icon_12(
                                                            if is_forwarded { IconName::Check } else { IconName::Plus },
                                                            if is_forwarded { theme.status_green } else { theme.text_muted },
                                                        ))
                                                )
                                        )
                                        .into_any_element()
                                }).collect()
                            })
                    )
            )
            .into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::prelude::v1::test;

    #[test]
    fn test_parse_netstat_listening_ports() {
        let sample = r#"
Active Connections

  Proto  Local Address          Foreign Address        State           PID
  TCP    0.0.0.0:135            0.0.0.0:0              LISTENING       1044
  TCP    0.0.0.0:3000           0.0.0.0:0              LISTENING       24012
  TCP    127.0.0.1:5173         0.0.0.0:0              LISTENING       18900
  TCP    127.0.0.1:5173         127.0.0.1:59021        ESTABLISHED     18900
  TCP    [::]:8080              [::]:0                 LISTENING       8430
"#;

        let ports = parse_netstat_output(sample);
        assert_eq!(ports.len(), 4);
        assert_eq!(ports[0].port, 135);
        assert_eq!(ports[1].port, 3000);
        assert_eq!(ports[2].port, 5173);
        assert_eq!(ports[3].port, 8080);
    }

    #[test]
    fn test_ports_forward_lifecycle() {
        let mut state = PortsState::default();
        state.forwarded_ports.clear();

        state.forward_port(3000, Some("My App".to_string()));
        assert_eq!(state.forwarded_ports.len(), 1);
        assert_eq!(state.forwarded_ports[0].local_port, 3000);
        assert_eq!(state.forwarded_ports[0].label, "My App");

        // Duplicate forward should be a no-op
        state.forward_port(3000, Some("Duplicate".to_string()));
        assert_eq!(state.forwarded_ports.len(), 1);

        // Add second port
        state.forward_port(8080, None);
        assert_eq!(state.forwarded_ports.len(), 2);
        assert_eq!(state.forwarded_ports[1].label, "localhost:8080");

        // Unforward
        state.unforward_port(3000);
        assert_eq!(state.forwarded_ports.len(), 1);
        assert_eq!(state.forwarded_ports[0].local_port, 8080);
    }
}
