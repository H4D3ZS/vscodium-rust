//! Bridge to ioscpy daemon on jailbroken iPhones.
//!
//! When the device-side `ioscpyd` daemon is running (installed from the
//! Sileo/Zebra repo), we can mirror and control it natively — H.264 hardware
//! video, native touch injection, clipboard sync — without WDA or signing.
//! This module implements a minimal client of the ioscpy wire protocol
//! (PROTOCOL_VERSION 4) for detection, streaming, and input.
//!
//! For non-jailbroken devices we fall back to the go-ios + WDA path.

use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

use serde::{Deserialize, Serialize};

// ─── Protocol constants (from ioscpy protocol/frame.md) ──────────────────────

const MAGIC: u32 = 0x4943_5059; // "ICPY"
const PROTOCOL_VERSION: u16 = 4;
const HEADER_SIZE: usize = 32;
const MAX_PAYLOAD: u32 = 16 * 1024 * 1024;

pub const DEFAULT_PORT: u16 = 27183;

// Channels
const CHANNEL_CONTROL: u64 = 0;
const CHANNEL_VIDEO: u64 = 1;

// Codec selector for START_STREAM
pub const VIDEO_CODEC_MJPEG: u8 = 0;
pub const VIDEO_CODEC_H264: u8 = 1;

// Video frame flags
pub const VIDEO_FLAG_H264: u32 = 0x1;
pub const VIDEO_FLAG_KEYFRAME: u32 = 0x2;
pub const VIDEO_FLAG_CONFIG: u32 = 0x4;

// ─── Message types ───────────────────────────────────────────────────────────

#[allow(dead_code)]
#[repr(u16)]
enum MsgType {
    Hello = 1,
    HelloAck = 2,
    CapabilitiesRequest = 3,
    CapabilitiesResponse = 4,
    Authenticate = 5,
    StartStream = 10,
    StopStream = 11,
    VideoFrame = 12,
    RequestKeyframe = 13,
    InputTouch = 20,
    InputKey = 21,
    InputText = 22,
    ClipboardSet = 31,
    SystemAction = 50,
    KeyboardMode = 51,
    Ping = 60,
    Pong = 61,
    Error = 70,
}

// ─── Handshake payloads (JSON) ───────────────────────────────────────────────

#[derive(Serialize)]
struct Hello {
    role: &'static str,
    host_version: String,
    protocol_version: u16,
    nonce: String,
}

#[derive(Debug, Deserialize)]
pub struct HelloAck {
    pub daemon_version: String,
    pub protocol_version: u16,
    pub session_token: String,
    pub capabilities: Capabilities,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Capabilities {
    pub ios_version: String,
    pub device_model: String,
    pub jailbreak_layout: String,
    #[serde(default)]
    pub jb_prefix: String,
    #[serde(default)]
    pub injection_framework: String,
    #[serde(default)]
    pub stream_backends: Vec<String>,
    #[serde(default)]
    pub input_backends: Vec<String>,
    #[serde(default)]
    pub clipboard: bool,
    #[serde(default)]
    pub keyboard: bool,
}

// ─── Frame I/O ───────────────────────────────────────────────────────────────

#[derive(Debug)]
struct Frame {
    msg_type: u16,
    #[allow(dead_code)]
    flags: u32,
    stream_id: u64,
    #[allow(dead_code)]
    seq: u64,
    payload: Vec<u8>,
}

fn write_frame<W: Write>(
    w: &mut W,
    msg_type: u16,
    stream_id: u64,
    seq: u64,
    payload: &[u8],
) -> Result<(), String> {
    let mut hdr = [0u8; HEADER_SIZE];
    hdr[0..4].copy_from_slice(&MAGIC.to_be_bytes());
    hdr[4..6].copy_from_slice(&PROTOCOL_VERSION.to_be_bytes());
    hdr[6..8].copy_from_slice(&msg_type.to_be_bytes());
    hdr[8..12].copy_from_slice(&0u32.to_be_bytes());
    hdr[12..20].copy_from_slice(&stream_id.to_be_bytes());
    hdr[20..28].copy_from_slice(&seq.to_be_bytes());
    hdr[28..32].copy_from_slice(&(payload.len() as u32).to_be_bytes());
    w.write_all(&hdr).map_err(|e| format!("write header: {e}"))?;
    if !payload.is_empty() {
        w.write_all(payload).map_err(|e| format!("write payload: {e}"))?;
    }
    w.flush().map_err(|e| format!("flush: {e}"))
}

fn read_frame<R: Read>(r: &mut R) -> Result<Frame, String> {
    let mut hdr = [0u8; HEADER_SIZE];
    r.read_exact(&mut hdr).map_err(|e| match e.kind() {
        std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut => {
            "ioscpy daemon did not respond (timeout)".to_string()
        }
        _ => format!("read header: {e}"),
    })?;
    let magic = u32::from_be_bytes(hdr[0..4].try_into().unwrap());
    if magic != MAGIC {
        return Err(format!("bad magic: expected ICPY, got {magic:#010x}"));
    }
    let version = u16::from_be_bytes(hdr[4..6].try_into().unwrap());
    if version != PROTOCOL_VERSION {
        return Err(format!("unsupported protocol version: {version}"));
    }
    let msg_type = u16::from_be_bytes(hdr[6..8].try_into().unwrap());
    let flags = u32::from_be_bytes(hdr[8..12].try_into().unwrap());
    let stream_id = u64::from_be_bytes(hdr[12..20].try_into().unwrap());
    let seq = u64::from_be_bytes(hdr[20..28].try_into().unwrap());
    let length = u32::from_be_bytes(hdr[28..32].try_into().unwrap());
    if length > MAX_PAYLOAD {
        return Err(format!("frame payload too large: {length} bytes"));
    }
    let mut payload = vec![0u8; length as usize];
    if length > 0 {
        r.read_exact(&mut payload).map_err(|e| format!("read payload: {e}"))?;
    }
    Ok(Frame { msg_type, flags, stream_id, seq, payload })
}

// ─── Touch / input encoding ──────────────────────────────────────────────────

#[allow(dead_code)]
#[repr(u8)]
pub enum TouchPhase {
    Down = 0,
    Move = 1,
    Up = 2,
}

pub fn encode_touch(phase: TouchPhase, id: u8, x: f32, y: f32) -> Vec<u8> {
    let mut v = Vec::with_capacity(10);
    v.push(phase as u8);
    v.push(id);
    v.extend_from_slice(&x.to_be_bytes());
    v.extend_from_slice(&y.to_be_bytes());
    v
}

#[allow(dead_code)]
#[repr(u16)]
pub enum SystemAction {
    Home = 1,
    Lock = 2,
    Wake = 3,
    AppSwitcher = 4,
    Back = 8,
}

pub fn encode_system_action(action: SystemAction) -> Vec<u8> {
    (action as u16).to_be_bytes().to_vec()
}

pub fn encode_text(text: &str) -> Vec<u8> {
    text.as_bytes().to_vec()
}

// ─── Video frame parsing ─────────────────────────────────────────────────────

/// Parsed video frame from the daemon.
pub struct VideoFrame {
    pub width: u32,
    pub height: u32,
    pub flags: u32,
    pub data: Vec<u8>,
}

impl VideoFrame {
    pub fn is_h264(&self) -> bool { self.flags & VIDEO_FLAG_H264 != 0 }
    pub fn is_keyframe(&self) -> bool { self.flags & VIDEO_FLAG_KEYFRAME != 0 }
    pub fn is_config(&self) -> bool { self.flags & VIDEO_FLAG_CONFIG != 0 }
    pub fn orientation(&self) -> u8 { (((self.flags & 0x18) >> 3) as u8) + 1 }
}

pub fn parse_video_payload(payload: &[u8]) -> Option<VideoFrame> {
    if payload.len() < 16 { return None; }
    let width = u32::from_be_bytes(payload[0..4].try_into().ok()?);
    let height = u32::from_be_bytes(payload[4..8].try_into().ok()?);
    let flags = u32::from_be_bytes(payload[8..12].try_into().ok()?);
    let length = u32::from_be_bytes(payload[12..16].try_into().ok()?);
    if payload.len() < 16 + length as usize { return None; }
    Some(VideoFrame {
        width,
        height,
        flags,
        data: payload[16..16 + length as usize].to_vec(),
    })
}

// ─── Public API ──────────────────────────────────────────────────────────────

/// Probe the ioscpy daemon via USB forwarding. Returns the HelloAck on success
/// (device is jailbroken and has ioscpy installed), or an error string.
///
/// `forwarded_port` is the local port that iproxy/usbmux forwarded to the
/// device's 27183.
pub fn probe_daemon(forwarded_port: u16) -> Result<HelloAck, String> {
    let addr = format!("127.0.0.1:{forwarded_port}");
    let mut stream = TcpStream::connect(&addr)
        .map_err(|e| format!("connect to ioscpy daemon on {addr}: {e}"))?;
    stream.set_read_timeout(Some(Duration::from_secs(5))).ok();
    stream.set_write_timeout(Some(Duration::from_secs(5))).ok();

    // HELLO
    let hello = Hello {
        role: "host",
        host_version: env!("CARGO_PKG_VERSION").into(),
        protocol_version: PROTOCOL_VERSION,
        nonce: format!("{:016x}", rand_nonce()),
    };
    let hello_json = serde_json::to_string(&hello).map_err(|e| format!("serialize hello: {e}"))?;
    write_frame(&mut stream, MsgType::Hello as u16, CHANNEL_CONTROL, 0, hello_json.as_bytes())?;

    // HELLO_ACK
    let ack_frame = read_frame(&mut stream)?;
    if ack_frame.msg_type != MsgType::HelloAck as u16 {
        return Err(format!(
            "expected HELLO_ACK (2), got message type {}",
            ack_frame.msg_type
        ));
    }
    let ack: HelloAck = serde_json::from_slice(&ack_frame.payload)
        .map_err(|e| format!("parse HELLO_ACK: {e}"))?;
    Ok(ack)
}

/// A live ioscpy session to a jailbroken device.
pub struct IoscpySession {
    stream: TcpStream,
    seq_control: u64,
    seq_video: u64,
}

impl IoscpySession {
    /// Open a session on an already-forwarded port. The daemon must be reachable
    /// at `127.0.0.1:forwarded_port`.
    pub fn connect(forwarded_port: u16) -> Result<(Self, HelloAck), String> {
        let ack = probe_daemon(forwarded_port)?;
        let addr = format!("127.0.0.1:{forwarded_port}");
        let mut stream = TcpStream::connect(&addr)
            .map_err(|e| format!("connect: {e}"))?;
        stream.set_read_timeout(Some(Duration::from_secs(10))).ok();
        stream.set_write_timeout(Some(Duration::from_secs(5))).ok();

        // Re-do the handshake in one shot (probe_daemon already did it, but we
        // need a fresh connection for the session).
        let hello = Hello {
            role: "host",
            host_version: env!("CARGO_PKG_VERSION").into(),
            protocol_version: PROTOCOL_VERSION,
            nonce: format!("{:016x}", rand_nonce()),
        };
        let hello_json = serde_json::to_string(&hello).map_err(|e| format!("serialize: {e}"))?;
        write_frame(&mut stream, MsgType::Hello as u16, CHANNEL_CONTROL, 0, hello_json.as_bytes())?;
        let ack_frame = read_frame(&mut stream)?;
        let session_ack: HelloAck = serde_json::from_slice(&ack_frame.payload)
            .map_err(|e| format!("parse ack: {e}"))?;

        // AUTHENTICATE
        write_frame(
            &mut stream,
            MsgType::Authenticate as u16,
            CHANNEL_CONTROL,
            0,
            session_ack.session_token.as_bytes(),
        )?;

        let session = Self { stream, seq_control: 1, seq_video: 0 };
        Ok((session, session_ack))
    }

    /// Start the video stream. Requests H.264 if the daemon supports it.
    pub fn start_stream(&mut self, caps: &Capabilities) -> Result<(), String> {
        let codec = if caps.stream_backends.iter().any(|b| b == "h264") {
            VIDEO_CODEC_H264
        } else {
            VIDEO_CODEC_MJPEG
        };
        let seq = self.next_seq();
        write_frame(
            &mut self.stream,
            MsgType::StartStream as u16,
            CHANNEL_CONTROL,
            seq,
            &[codec],
        )
    }

    /// Stop the video stream.
    pub fn stop_stream(&mut self) -> Result<(), String> {
        let seq = self.next_seq();
        write_frame(
            &mut self.stream,
            MsgType::StopStream as u16,
            CHANNEL_CONTROL,
            seq,
            &[],
        )
    }

    /// Read the next frame (blocks until available).
    pub fn read_frame(&mut self) -> Result<Frame, String> {
        read_frame(&mut self.stream)
    }

    /// Send a touch event. Coordinates are normalized [0, 1].
    pub fn send_touch(&mut self, phase: TouchPhase, x: f32, y: f32) -> Result<(), String> {
        let payload = encode_touch(phase, 0, x, y);
        let seq = self.next_seq();
        write_frame(
            &mut self.stream,
            MsgType::InputTouch as u16,
            CHANNEL_CONTROL,
            seq,
            &payload,
        )
    }

    /// Send a system action (Home, Back, etc.).
    pub fn send_action(&mut self, action: SystemAction) -> Result<(), String> {
        let payload = encode_system_action(action);
        let seq = self.next_seq();
        write_frame(
            &mut self.stream,
            MsgType::SystemAction as u16,
            CHANNEL_CONTROL,
            seq,
            &payload,
        )
    }

    /// Send text input.
    pub fn send_text(&mut self, text: &str) -> Result<(), String> {
        let payload = encode_text(text);
        let seq = self.next_seq();
        write_frame(
            &mut self.stream,
            MsgType::InputText as u16,
            CHANNEL_CONTROL,
            seq,
            &payload,
        )
    }

    /// Request a keyframe (useful on connect or after a decode gap).
    pub fn request_keyframe(&mut self) -> Result<(), String> {
        let seq = self.next_seq();
        write_frame(
            &mut self.stream,
            MsgType::RequestKeyframe as u16,
            CHANNEL_CONTROL,
            seq,
            &[],
        )
    }

    fn next_seq(&mut self) -> u64 {
        let s = self.seq_control;
        self.seq_control += 1;
        s
    }
}

/// Generate a random nonce for the handshake (simple u64).
fn rand_nonce() -> u64 {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};
    let s = RandomState::new();
    let mut h = s.build_hasher();
    h.write_u64(std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64);
    h.finish()
}
