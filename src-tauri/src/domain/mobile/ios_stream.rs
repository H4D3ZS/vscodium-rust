//! Pure-Rust iOS simulator video pipeline (macOS).
//! Capture pipe → Bytes broadcast → Axum MJPEG multipart. Zero disk, zero per-frame IPC.

#[cfg(target_os = "macos")]
mod mac {
    use axum::{
        body::Body,
        extract::State,
        http::{header, HeaderValue},
        response::{IntoResponse, Response},
        routing::get,
        Json, Router,
    };
    use bytes::{Bytes, BytesMut};
    use futures::Stream;
    use serde::Serialize;
    use std::convert::Infallible;
    use std::pin::Pin;
    use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
    use std::sync::{Arc, Mutex};
    use tokio::net::TcpListener;
    use tokio::sync::{broadcast, RwLock};

    const BOUNDARY: &[u8] = b"hadesframe";

    #[derive(Clone, Default, Serialize)]
    pub struct StreamStatus {
        pub stream_url: String,
        pub width: u32,
        pub height: u32,
        pub device_name: String,
        pub device_udid: String,
        pub frame_count: u64,
        pub profile: String,
        pub running: bool,
        pub paused: bool,
    }

    struct Hub {
        tx: broadcast::Sender<Bytes>,
        status: RwLock<StreamStatus>,
        frame_count: AtomicU64,
        running: AtomicBool,
        paused: AtomicBool,
    }

    static HUB: Mutex<Option<Arc<Hub>>> = Mutex::new(None);
    static PORT: Mutex<Option<u16>> = Mutex::new(None);
    static SERVER_TASK: Mutex<Option<tauri::async_runtime::JoinHandle<()>>> = Mutex::new(None);

    fn format_mjpeg_chunk(jpeg: &Bytes) -> Bytes {
        let mut buf = BytesMut::with_capacity(jpeg.len() + 72);
        buf.extend_from_slice(b"--");
        buf.extend_from_slice(BOUNDARY);
        buf.extend_from_slice(b"\r\nContent-Type: image/jpeg\r\nContent-Length: ");
        buf.extend_from_slice(jpeg.len().to_string().as_bytes());
        buf.extend_from_slice(b"\r\n\r\n");
        buf.extend_from_slice(jpeg);
        buf.extend_from_slice(b"\r\n");
        buf.freeze()
    }

    fn mjpeg_stream(
        rx: broadcast::Receiver<Bytes>,
    ) -> Pin<Box<dyn Stream<Item = Result<Bytes, Infallible>> + Send>> {
        Box::pin(futures::stream::unfold(rx, |mut rx| async move {
            loop {
                match rx.recv().await {
                    Ok(jpeg) => return Some((Ok(format_mjpeg_chunk(&jpeg)), rx)),
                    Err(broadcast::error::RecvError::Lagged(_)) => continue,
                    Err(broadcast::error::RecvError::Closed) => return None,
                }
            }
        }))
    }

    async fn mjpeg_handler(State(hub): State<Arc<Hub>>) -> Response {
        let rx = hub.tx.subscribe();
        let body = Body::from_stream(mjpeg_stream(rx));
        let mut resp = Response::new(body);
        if let Ok(v) = HeaderValue::from_str(&format!(
            "multipart/x-mixed-replace; boundary={}",
            std::str::from_utf8(BOUNDARY).unwrap_or("hadesframe")
        )) {
            resp.headers_mut().insert(header::CONTENT_TYPE, v);
        }
        resp.headers_mut().insert(
            header::CACHE_CONTROL,
            HeaderValue::from_static("no-cache, no-store, must-revalidate"),
        );
        resp.headers_mut().insert(
            header::CONNECTION,
            HeaderValue::from_static("close"),
        );
        resp
    }

    async fn status_handler(State(hub): State<Arc<Hub>>) -> impl IntoResponse {
        Json(current_status(&hub).await)
    }

    async fn current_status(hub: &Hub) -> StreamStatus {
        let mut s = hub.status.read().await.clone();
        s.frame_count = hub.frame_count.load(Ordering::Relaxed);
        s.running = hub.running.load(Ordering::Relaxed);
        s.paused = hub.paused.load(Ordering::Relaxed);
        s
    }

    pub async fn ensure_server() -> Result<String, String> {
        if let Some(port) = *PORT.lock().unwrap() {
            return Ok(format!("http://127.0.0.1:{port}/stream.mjpg"));
        }

        let (tx, _) = broadcast::channel::<Bytes>(4);
        let hub = Arc::new(Hub {
            tx,
            status: RwLock::new(StreamStatus::default()),
            frame_count: AtomicU64::new(0),
            running: AtomicBool::new(false),
            paused: AtomicBool::new(false),
        });
        *HUB.lock().unwrap() = Some(hub.clone());

        let app = Router::new()
            .route("/stream.mjpg", get(mjpeg_handler))
            .route("/status.json", get(status_handler))
            .with_state(hub.clone());

        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .map_err(|e| format!("stream bind: {e}"))?;
        let port = listener.local_addr().map_err(|e| e.to_string())?.port();
        *PORT.lock().unwrap() = Some(port);

        let url = format!("http://127.0.0.1:{port}/stream.mjpg");
        if let Ok(mut st) = hub.status.try_write() {
            st.stream_url = url.clone();
        }

        let handle = tauri::async_runtime::spawn(async move {
            let _ = axum::serve(listener, app).await;
        });
        *SERVER_TASK.lock().unwrap() = Some(handle);
        Ok(url)
    }

    pub fn stream_url() -> Option<String> {
        PORT.lock()
            .unwrap()
            .map(|p| format!("http://127.0.0.1:{p}/stream.mjpg"))
    }

    pub async fn stream_status() -> StreamStatus {
        let hub = HUB.lock().unwrap().clone();
        match hub {
            Some(h) => current_status(&h).await,
            None => StreamStatus::default(),
        }
    }

    pub fn set_meta(width: u32, height: u32, device_name: &str, device_udid: &str) {
        if let Some(hub) = HUB.lock().unwrap().as_ref() {
            if let Ok(mut st) = hub.status.try_write() {
                st.width = width;
                st.height = height;
                st.device_name = device_name.to_string();
                st.device_udid = device_udid.to_string();
            }
        }
    }

    pub fn set_profile(label: &str) {
        if let Some(hub) = HUB.lock().unwrap().as_ref() {
            if let Ok(mut st) = hub.status.try_write() {
                st.profile = label.to_string();
            }
        }
    }

    pub fn set_running(running: bool) {
        if let Some(hub) = HUB.lock().unwrap().as_ref() {
            hub.running.store(running, Ordering::Relaxed);
        }
    }

    pub fn set_paused(paused: bool) {
        if let Some(hub) = HUB.lock().unwrap().as_ref() {
            hub.paused.store(paused, Ordering::Relaxed);
        }
    }

    pub fn publish_frame(jpeg: Bytes) {
        if let Some(hub) = HUB.lock().unwrap().as_ref() {
            if hub.paused.load(Ordering::Relaxed) {
                return;
            }
            if hub.tx.send(jpeg).is_ok() {
                hub.frame_count.fetch_add(1, Ordering::Relaxed);
            }
        }
    }

    pub fn stop_server() {
        set_running(false);
        set_paused(false);
        *HUB.lock().unwrap() = None;
        *PORT.lock().unwrap() = None;
        if let Some(h) = SERVER_TASK.lock().unwrap().take() {
            h.abort();
        }
    }
}

#[cfg(target_os = "macos")]
pub use mac::*;

#[cfg(not(target_os = "macos"))]
#[derive(Clone, Default, serde::Serialize)]
pub struct StreamStatus {
    pub stream_url: String,
    pub width: u32,
    pub height: u32,
    pub device_name: String,
    pub device_udid: String,
    pub frame_count: u64,
    pub profile: String,
    pub running: bool,
    pub paused: bool,
}

#[cfg(not(target_os = "macos"))]
pub async fn ensure_server() -> Result<String, String> {
    Err("iOS stream requires macOS".into())
}

#[cfg(not(target_os = "macos"))]
pub fn stream_url() -> Option<String> {
    None
}

#[cfg(not(target_os = "macos"))]
pub async fn stream_status() -> StreamStatus {
    StreamStatus::default()
}

#[cfg(not(target_os = "macos"))]
pub fn set_meta(_: u32, _: u32, _: &str, _: &str) {}

#[cfg(not(target_os = "macos"))]
pub fn set_profile(_: &str) {}

#[cfg(not(target_os = "macos"))]
pub fn set_running(_: bool) {}

#[cfg(not(target_os = "macos"))]
pub fn set_paused(_: bool) {}

#[cfg(not(target_os = "macos"))]
pub fn publish_frame(_: bytes::Bytes) {}

#[cfg(not(target_os = "macos"))]
pub fn stop_server() {}
