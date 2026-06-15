//! GDB Remote Serial Protocol packet parsing and encoding.
//!
//! Packet format: $<data>#<checksum>
//! Checksum is 2 hex digits = sum of data bytes mod 256

use std::io::Write;
use tracing::trace;

/// Parse a GDB RSP packet from a buffer.
/// Returns (packet_data, bytes_consumed) if a complete packet is found.
pub fn parse_packet(buf: &[u8]) -> Option<(String, usize)> {
    // Find start of packet
    let start = buf.iter().position(|&b| b == b'$')?;

    // Find end of packet (checksum)
    let hash_pos = buf[start..].iter().position(|&b| b == b'#')?;
    let hash_pos = start + hash_pos;

    // Need 2 more bytes for checksum
    if buf.len() < hash_pos + 3 {
        return None;
    }

    let data = &buf[start + 1..hash_pos];
    let checksum_str = std::str::from_utf8(&buf[hash_pos + 1..hash_pos + 3]).ok()?;
    let expected_checksum = u8::from_str_radix(checksum_str, 16).ok()?;

    // Verify checksum
    let actual_checksum = data.iter().fold(0u8, |acc, &b| acc.wrapping_add(b));
    if actual_checksum != expected_checksum {
        return None;
    }

    let packet = String::from_utf8_lossy(data).to_string();
    Some((packet, hash_pos + 3))
}

/// Calculate checksum for packet data.
fn checksum(data: &[u8]) -> u8 {
    data.iter().fold(0u8, |acc, &b| acc.wrapping_add(b))
}

/// Send a GDB RSP packet.
pub fn send_packet<W: Write>(writer: &mut W, data: &str) -> std::io::Result<()> {
    let sum = checksum(data.as_bytes());
    let display_data = if data.len() > 100 {
        format!("{}...[{} chars total]", &data[..100], data.len())
    } else {
        data.to_string()
    };
    trace!(data = %display_data, checksum = format!("{:02x}", sum), "GDB TX");
    write!(writer, "${}#{:02x}", data, sum)?;
    writer.flush()
}

/// Send a stop reply packet.
pub fn send_stop_reply<W: Write>(writer: &mut W, signal: u8) -> std::io::Result<()> {
    send_packet(writer, &format!("S{:02x}", signal))
}

/// Send an OK response.
pub fn send_ok<W: Write>(writer: &mut W) -> std::io::Result<()> {
    send_packet(writer, "OK")
}

/// Send an error response.
pub fn send_error<W: Write>(writer: &mut W, code: u8) -> std::io::Result<()> {
    send_packet(writer, &format!("E{:02x}", code))
}

/// Send an empty response (unsupported command).
pub fn send_empty<W: Write>(writer: &mut W) -> std::io::Result<()> {
    send_packet(writer, "")
}

/// Encode bytes as hex string.
pub fn encode_hex(data: &[u8]) -> String {
    data.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Decode hex string to bytes.
pub fn decode_hex(hex: &str) -> Option<Vec<u8>> {
    if hex.len() % 2 != 0 {
        return None;
    }

    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).ok())
        .collect()
}

/// Parse address from hex string.
pub fn parse_addr(hex: &str) -> Option<u64> {
    u64::from_str_radix(hex, 16).ok()
}

/// Parse length from hex string.
pub fn parse_len(hex: &str) -> Option<usize> {
    usize::from_str_radix(hex, 16).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_packet() {
        let buf = b"$g#67";
        let (packet, consumed) = parse_packet(buf).unwrap();
        assert_eq!(packet, "g");
        assert_eq!(consumed, 5);
    }

    #[test]
    fn test_parse_packet_with_data() {
        let buf = b"$m1000,10#a5";
        let (packet, consumed) = parse_packet(buf).unwrap();
        assert_eq!(packet, "m1000,10");
        assert_eq!(consumed, 12);
    }

    #[test]
    fn test_checksum() {
        assert_eq!(checksum(b"g"), 0x67);
        assert_eq!(checksum(b"OK"), b'O'.wrapping_add(b'K'));
    }

    #[test]
    fn test_encode_decode_hex() {
        let data = vec![0x12, 0x34, 0xab, 0xcd];
        let hex = encode_hex(&data);
        assert_eq!(hex, "1234abcd");
        assert_eq!(decode_hex(&hex), Some(data));
    }

    #[test]
    fn test_parse_addr() {
        assert_eq!(parse_addr("1000"), Some(0x1000));
        assert_eq!(parse_addr("ffffffff"), Some(0xffffffff));
    }
}
