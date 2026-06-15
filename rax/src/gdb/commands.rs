//! GDB RSP command handlers.

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

use tracing::debug;

/// x86_64 target description XML for GDB/IDA Pro.
const TARGET_XML: &str = r#"<?xml version="1.0"?>
<!DOCTYPE target SYSTEM "gdb-target.dtd">
<target version="1.0">
  <architecture>i386:x86-64</architecture>
  <feature name="org.gnu.gdb.i386.core">
    <reg name="rax" bitsize="64" type="int64"/>
    <reg name="rbx" bitsize="64" type="int64"/>
    <reg name="rcx" bitsize="64" type="int64"/>
    <reg name="rdx" bitsize="64" type="int64"/>
    <reg name="rsi" bitsize="64" type="int64"/>
    <reg name="rdi" bitsize="64" type="int64"/>
    <reg name="rbp" bitsize="64" type="data_ptr"/>
    <reg name="rsp" bitsize="64" type="data_ptr"/>
    <reg name="r8" bitsize="64" type="int64"/>
    <reg name="r9" bitsize="64" type="int64"/>
    <reg name="r10" bitsize="64" type="int64"/>
    <reg name="r11" bitsize="64" type="int64"/>
    <reg name="r12" bitsize="64" type="int64"/>
    <reg name="r13" bitsize="64" type="int64"/>
    <reg name="r14" bitsize="64" type="int64"/>
    <reg name="r15" bitsize="64" type="int64"/>
    <reg name="rip" bitsize="64" type="code_ptr"/>
    <reg name="eflags" bitsize="32" type="i386_eflags"/>
    <reg name="cs" bitsize="32" type="int32"/>
    <reg name="ss" bitsize="32" type="int32"/>
    <reg name="ds" bitsize="32" type="int32"/>
    <reg name="es" bitsize="32" type="int32"/>
    <reg name="fs" bitsize="32" type="int32"/>
    <reg name="gs" bitsize="32" type="int32"/>
  </feature>
</target>"#;

use super::protocol::{
    decode_hex, parse_addr, parse_len, send_empty, send_error, send_ok, send_packet,
    send_stop_reply,
};
use super::{GdbChannels, GdbCommand, GdbResponse};

/// Wait for a stop response from VMM while also checking for interrupt (0x03) from debugger.
/// This allows Ctrl+C / pause to work during execution.
fn wait_for_stop_or_interrupt(
    stream: &mut TcpStream,
    channels: &GdbChannels,
) -> std::io::Result<GdbResponse> {
    // Set stream to non-blocking so we can poll for interrupts
    stream.set_nonblocking(true)?;

    let mut buf = [0u8; 64];
    loop {
        // Check for response from VMM (non-blocking)
        match channels.resp_rx.try_recv() {
            Ok(resp) => {
                // Got a response, restore blocking mode and return
                stream.set_nonblocking(false)?;
                return Ok(resp);
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => {
                // No response yet, check for interrupt from debugger
            }
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                stream.set_nonblocking(false)?;
                return Ok(GdbResponse::Error(1));
            }
        }

        // Check for incoming data from debugger (0x03 interrupt)
        match stream.read(&mut buf) {
            Ok(0) => {
                // Connection closed
                stream.set_nonblocking(false)?;
                return Ok(GdbResponse::Error(1));
            }
            Ok(n) => {
                // Check if any byte is 0x03 (interrupt)
                if buf[..n].contains(&0x03) {
                    debug!("Received interrupt (0x03) during execution");
                    // Send interrupt command to VMM
                    channels.cmd_tx.send(GdbCommand::Interrupt).ok();
                    // Now wait for the stop response (blocking is fine now)
                    stream.set_nonblocking(false)?;
                    match channels.resp_rx.recv() {
                        Ok(resp) => return Ok(resp),
                        Err(_) => return Ok(GdbResponse::Error(1)),
                    }
                }
                // Otherwise ignore (could be ACKs or other data we don't care about)
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // No data available, sleep briefly to avoid busy-spinning
                std::thread::sleep(Duration::from_millis(1));
            }
            Err(e) => {
                stream.set_nonblocking(false)?;
                return Err(e);
            }
        }
    }
}

/// Handle a GDB RSP command.
/// Returns Ok(true) to continue, Ok(false) to disconnect.
pub fn handle_command(
    packet: &str,
    stream: &mut TcpStream,
    channels: &GdbChannels,
    breakpoints: &mut HashMap<u64, u8>,
) -> std::io::Result<bool> {
    // Get first character (command type)
    let cmd = packet.chars().next().unwrap_or(' ');
    let args = if packet.len() > 1 { &packet[1..] } else { "" };

    match cmd {
        // Query halt reason
        '?' => {
            send_stop_reply(stream, 5)?; // SIGTRAP
        }

        // Read registers
        'g' => {
            channels.cmd_tx.send(GdbCommand::ReadRegisters).ok();
            match channels.resp_rx.recv_timeout(Duration::from_secs(5)) {
                Ok(GdbResponse::Registers(hex)) => {
                    send_packet(stream, &hex)?;
                }
                Ok(GdbResponse::Error(code)) => {
                    send_error(stream, code)?;
                }
                _ => {
                    send_error(stream, 1)?;
                }
            }
        }

        // Write registers
        'G' => {
            let data = match decode_hex(args) {
                Some(d) => d,
                None => {
                    send_error(stream, 1)?;
                    return Ok(true);
                }
            };
            channels.cmd_tx.send(GdbCommand::WriteRegisters(data)).ok();
            match channels.resp_rx.recv_timeout(Duration::from_secs(5)) {
                Ok(GdbResponse::Ok) => send_ok(stream)?,
                _ => send_error(stream, 1)?,
            }
        }

        // Read single register (p<reg_num>)
        'p' => {
            let reg_num = match usize::from_str_radix(args, 16) {
                Ok(n) => n,
                Err(_) => {
                    send_error(stream, 1)?;
                    return Ok(true);
                }
            };
            // We need to get all registers and extract the one requested
            channels.cmd_tx.send(GdbCommand::ReadRegisters).ok();
            match channels.resp_rx.recv_timeout(Duration::from_secs(5)) {
                Ok(GdbResponse::Registers(hex)) => {
                    // Extract the specific register from the full register set
                    let reg_data = extract_register(&hex, reg_num);
                    send_packet(stream, &reg_data)?;
                }
                _ => {
                    send_error(stream, 1)?;
                }
            }
        }

        // Write single register (P<reg_num>=<value>)
        'P' => {
            let eq_pos = match args.find('=') {
                Some(p) => p,
                None => {
                    send_error(stream, 1)?;
                    return Ok(true);
                }
            };
            let _reg_num = match usize::from_str_radix(&args[..eq_pos], 16) {
                Ok(n) => n,
                Err(_) => {
                    send_error(stream, 1)?;
                    return Ok(true);
                }
            };
            // For now, return error - single register write not fully implemented
            // Would need to read all, modify one, write all back
            send_error(stream, 1)?;
        }

        // Read memory
        'm' => {
            let parts: Vec<&str> = args.split(',').collect();
            if parts.len() != 2 {
                send_error(stream, 1)?;
                return Ok(true);
            }
            let addr = match parse_addr(parts[0]) {
                Some(a) => a,
                None => {
                    send_error(stream, 1)?;
                    return Ok(true);
                }
            };
            let len = match parse_len(parts[1]) {
                Some(l) => l,
                None => {
                    send_error(stream, 1)?;
                    return Ok(true);
                }
            };

            channels
                .cmd_tx
                .send(GdbCommand::ReadMemory { addr, len })
                .ok();
            match channels.resp_rx.recv_timeout(Duration::from_secs(5)) {
                Ok(GdbResponse::Memory(hex)) => {
                    send_packet(stream, &hex)?;
                }
                Ok(GdbResponse::Error(code)) => {
                    send_error(stream, code)?;
                }
                _ => {
                    send_error(stream, 1)?;
                }
            }
        }

        // Write memory
        'M' => {
            let colon_pos = match args.find(':') {
                Some(p) => p,
                None => {
                    send_error(stream, 1)?;
                    return Ok(true);
                }
            };
            let header = &args[..colon_pos];
            let hex_data = &args[colon_pos + 1..];

            let parts: Vec<&str> = header.split(',').collect();
            if parts.len() != 2 {
                send_error(stream, 1)?;
                return Ok(true);
            }

            let addr = match parse_addr(parts[0]) {
                Some(a) => a,
                None => {
                    send_error(stream, 1)?;
                    return Ok(true);
                }
            };

            let data = match decode_hex(hex_data) {
                Some(d) => d,
                None => {
                    send_error(stream, 1)?;
                    return Ok(true);
                }
            };

            channels
                .cmd_tx
                .send(GdbCommand::WriteMemory { addr, data })
                .ok();
            match channels.resp_rx.recv_timeout(Duration::from_secs(5)) {
                Ok(GdbResponse::Ok) => send_ok(stream)?,
                _ => send_error(stream, 1)?,
            }
        }

        // Continue execution
        'c' => {
            // Optional address argument (ignored for now)
            channels.cmd_tx.send(GdbCommand::Continue).ok();
            // Wait for stop, also checking for interrupt (0x03) from debugger
            match wait_for_stop_or_interrupt(stream, channels)? {
                GdbResponse::StopReply(sig) => {
                    send_stop_reply(stream, sig)?;
                }
                GdbResponse::Stopped => {
                    send_stop_reply(stream, 5)?; // SIGTRAP
                }
                _ => {
                    send_stop_reply(stream, 5)?;
                }
            }
        }

        // Single step
        's' => {
            // Optional address argument (ignored for now)
            channels.cmd_tx.send(GdbCommand::Step).ok();
            // Wait for stop
            match channels.resp_rx.recv() {
                Ok(GdbResponse::StopReply(sig)) => {
                    send_stop_reply(stream, sig)?;
                }
                Ok(GdbResponse::Stopped) => {
                    send_stop_reply(stream, 5)?; // SIGTRAP
                }
                _ => {
                    send_stop_reply(stream, 5)?;
                }
            }
        }

        // Set/remove breakpoint
        'Z' | 'z' => {
            let parts: Vec<&str> = args.split(',').collect();
            if parts.len() < 2 {
                send_error(stream, 1)?;
                return Ok(true);
            }

            let bp_type = parts[0];
            let addr = match parse_addr(parts[1]) {
                Some(a) => a,
                None => {
                    send_error(stream, 1)?;
                    return Ok(true);
                }
            };

            // Only support software breakpoints (type 0)
            if bp_type != "0" {
                send_empty(stream)?; // Unsupported
                return Ok(true);
            }

            if cmd == 'Z' {
                channels
                    .cmd_tx
                    .send(GdbCommand::SetBreakpoint { addr })
                    .ok();
            } else {
                channels
                    .cmd_tx
                    .send(GdbCommand::RemoveBreakpoint { addr })
                    .ok();
            }

            match channels.resp_rx.recv_timeout(Duration::from_secs(5)) {
                Ok(GdbResponse::Ok) => send_ok(stream)?,
                _ => send_error(stream, 1)?,
            }
        }

        // Detach
        'D' => {
            channels.cmd_tx.send(GdbCommand::Detach).ok();
            send_ok(stream)?;
            return Ok(false);
        }

        // Kill
        'k' => {
            channels.cmd_tx.send(GdbCommand::Kill).ok();
            return Ok(false);
        }

        // Query commands
        'q' => {
            handle_query(args, stream)?;
        }

        // Set thread (ignored - single threaded)
        'H' => {
            send_ok(stream)?;
        }

        // vCont (extended continue/step)
        'v' => {
            if args.starts_with("Cont?") {
                // Report supported vCont actions (with thread support)
                send_packet(stream, "vCont;c;C;s;S;t")?;
            } else if args.starts_with("Cont;") {
                // Parse vCont actions: vCont;action[:thread];action[:thread]...
                // Examples: vCont;s:1, vCont;c, vCont;s:-1, vCont;c:1;s:2
                let actions = &args[5..];

                // Find first step or continue action (ignore thread IDs for single-threaded)
                let mut do_step = false;
                let mut do_continue = false;

                for part in actions.split(';') {
                    // Get action char (before any ':thread' suffix)
                    let action_char = part.chars().next().unwrap_or(' ');
                    match action_char {
                        's' | 'S' => {
                            do_step = true;
                            break;
                        }
                        'c' | 'C' => {
                            do_continue = true;
                        }
                        't' => { /* stop/halt - ignore for now */ }
                        _ => {}
                    }
                }

                if do_step {
                    channels.cmd_tx.send(GdbCommand::Step).ok();
                    match channels.resp_rx.recv() {
                        Ok(GdbResponse::StopReply(sig)) => send_stop_reply(stream, sig)?,
                        _ => send_stop_reply(stream, 5)?,
                    }
                } else if do_continue {
                    channels.cmd_tx.send(GdbCommand::Continue).ok();
                    // Wait for stop, also checking for interrupt (0x03) from debugger
                    match wait_for_stop_or_interrupt(stream, channels)? {
                        GdbResponse::StopReply(sig) => send_stop_reply(stream, sig)?,
                        _ => send_stop_reply(stream, 5)?,
                    }
                } else {
                    send_empty(stream)?;
                }
            } else if args.starts_with("Stopped") {
                // vStopped - report no more stop events
                send_ok(stream)?;
            } else {
                send_empty(stream)?;
            }
        }

        // Unknown command
        _ => {
            debug!(cmd = %cmd, "Unknown GDB command");
            send_empty(stream)?;
        }
    }

    Ok(true)
}

/// Handle query commands (q...).
fn handle_query<W: Write>(args: &str, stream: &mut W) -> std::io::Result<()> {
    if args.starts_with("Supported") {
        // Report supported features including target description
        send_packet(
            stream,
            "PacketSize=4096;swbreak+;vContSupported+;qXfer:features:read+",
        )?;
    } else if args.starts_with("Xfer:features:read:target.xml:") {
        // Target description XML for x86_64
        // Parse offset and length from "target.xml:offset,length"
        let params = &args["Xfer:features:read:target.xml:".len()..];
        let parts: Vec<&str> = params.split(',').collect();
        let offset = parts
            .get(0)
            .and_then(|s| usize::from_str_radix(s, 16).ok())
            .unwrap_or(0);
        let _len = parts
            .get(1)
            .and_then(|s| usize::from_str_radix(s, 16).ok())
            .unwrap_or(4096);

        let xml = TARGET_XML;
        if offset >= xml.len() {
            send_packet(stream, "l")?; // End of data
        } else {
            let remaining = &xml[offset..];
            // 'l' prefix means last chunk, 'm' means more data follows
            if remaining.len() <= 4096 {
                send_packet(stream, &format!("l{}", remaining))?;
            } else {
                send_packet(stream, &format!("m{}", &remaining[..4096]))?;
            }
        }
    } else if args.starts_with("Attached") {
        // We attached to an existing process
        send_packet(stream, "1")?;
    } else if args.starts_with("TStatus") {
        // Tracepoint status - not supported
        send_empty(stream)?;
    } else if args.starts_with("fThreadInfo") {
        // First thread info query
        send_packet(stream, "m1")?; // Thread 1
    } else if args.starts_with("sThreadInfo") {
        // Subsequent thread info query
        send_packet(stream, "l")?; // End of list
    } else if args.starts_with("C") {
        // Current thread
        send_packet(stream, "QC1")?; // Thread 1
    } else if args.starts_with("Offsets") {
        // Section offsets - not applicable
        send_empty(stream)?;
    } else if args.starts_with("Symbol") {
        // Symbol lookup - not supported
        send_ok(stream)?;
    } else if args.starts_with("Rcmd") {
        // Remote command - not supported
        send_ok(stream)?;
    } else if args.starts_with("L") {
        // Thread list (older format)
        send_packet(stream, "qM001000000000000001")?; // One thread
    } else if args.starts_with("P") {
        // Get thread info - not supported
        send_empty(stream)?;
    } else if args.starts_with("ThreadExtraInfo") {
        // Thread extra info
        // Return hex-encoded "CPU 0"
        let info = "CPU 0";
        let hex: String = info.bytes().map(|b| format!("{:02x}", b)).collect();
        send_packet(stream, &hex)?;
    } else {
        debug!(query = %args, "Unknown query");
        send_empty(stream)?;
    }

    Ok(())
}

/// Extract a single register from the full register hex string.
/// Returns "xxxxxxxx..." for 64-bit regs (0-16), "xxxx" for 32-bit (17-23)
fn extract_register(hex: &str, reg_num: usize) -> String {
    // Register layout in hex string:
    // 0-15: 64-bit GPRs (16 hex chars each) = 0-255
    // 16: rip (16 hex chars) = 256-271
    // 17: eflags (8 hex chars) = 272-279
    // 18-23: segment regs (8 hex chars each) = 280-327

    if reg_num <= 16 {
        // 64-bit register
        let start = reg_num * 16;
        let end = start + 16;
        if end <= hex.len() {
            return hex[start..end].to_string();
        }
    } else if reg_num <= 23 {
        // 32-bit register (eflags or segment)
        let base = 17 * 16; // After all 64-bit regs
        let idx = reg_num - 17;
        let start = base + idx * 8;
        let end = start + 8;
        if end <= hex.len() {
            return hex[start..end].to_string();
        }
    }

    // Return zeros if out of range
    if reg_num <= 16 {
        "0000000000000000".to_string()
    } else {
        "00000000".to_string()
    }
}
