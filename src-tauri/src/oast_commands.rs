//! Tauri IPC for the OAST (out-of-band) collaborator server.

use crate::oast::{self, Interaction, OastPayload, OastStatus};

#[tauri::command]
pub async fn oast_start(port: Option<u16>, public_host: Option<String>) -> Result<u16, String> {
    oast::start(port.unwrap_or(8889), public_host).await
}

#[tauri::command]
pub fn oast_stop() -> OastStatus {
    oast::stop();
    oast::status()
}

#[tauri::command]
pub fn oast_status() -> OastStatus {
    oast::status()
}

#[tauri::command]
pub fn oast_register() -> OastPayload {
    oast::register()
}

#[tauri::command]
pub fn oast_poll(token: Option<String>) -> Vec<Interaction> {
    oast::poll(token.as_deref())
}

#[tauri::command]
pub fn oast_clear() {
    oast::clear();
}

#[tauri::command]
pub fn oast_set_public_host(host: String) -> OastStatus {
    oast::set_public_host(host);
    oast::status()
}
