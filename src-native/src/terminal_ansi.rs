// ANSI Escape Code Parser for Native PTY Terminal Rendering.
//
// Parses SGR (Select Graphic Rendition) escape sequences from terminal output
// and maps them to GPUI Rgba colors for proper terminal display.
//
// Supports:
// - Standard 8 colors (30-37 fg, 40-47 bg)
// - Bright/high-intensity colors (90-97 fg, 100-107 bg)
// - 256-color mode (38;5;N / 48;5;N)
// - 24-bit true color (38;2;R;G;B / 48;2;R;G;B)
// - Bold, dim, italic, underline, strikethrough attributes
// - Reset (0)

use gpui_kit::gpui::{rgb, Rgba};

/// Terminal text style attributes extracted from ANSI escape codes.
#[derive(Clone, Debug, Default)]
pub struct AnsiStyle {
    pub fg: Option<Rgba>,
    pub bg: Option<Rgba>,
    pub bold: bool,
    pub dim: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
}

/// A segment of terminal text with its associated ANSI style.
#[derive(Clone, Debug)]
pub struct StyledSegment {
    pub text: String,
    pub style: AnsiStyle,
}

/// Standard ANSI 8-color palette (used for codes 30-37 / 40-47).
const ANSI_COLORS: [u32; 8] = [
    0x1a1a1a, // 0: Black
    0xe06c75, // 1: Red
    0x98c379, // 2: Green
    0xe5c07b, // 3: Yellow
    0x61afef, // 4: Blue
    0xc678dd, // 5: Magenta
    0x56b6c2, // 6: Cyan
    0xabb2bf, // 7: White
];

/// Bright ANSI 8-color palette (used for codes 90-97 / 100-107).
const ANSI_BRIGHT_COLORS: [u32; 8] = [
    0x5c6370, // 0: Bright Black (Gray)
    0xf44747, // 1: Bright Red
    0x4ec9b0, // 2: Bright Green
    0xfbbf24, // 3: Bright Yellow
    0x60a5fa, // 4: Bright Blue
    0xc084fc, // 5: Bright Magenta
    0x22d3ee, // 6: Bright Cyan
    0xffffff, // 7: Bright White
];

/// Convert a 256-color code to Rgba.
fn color_256(code: u8) -> Rgba {
    match code {
        0..=7 => rgb(ANSI_COLORS[code as usize]),
        8..=15 => rgb(ANSI_BRIGHT_COLORS[(code - 8) as usize]),
        16..=231 => {
            // 6x6x6 color cube
            let idx = code - 16;
            let r = (idx / 36) * 51;
            let g = ((idx % 36) / 6) * 51;
            let b = (idx % 6) * 51;
            rgb(((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
        }
        232..=255 => {
            // Grayscale ramp (24 shades: 8 to 238)
            let gray = 8 + (code - 232) * 10;
            rgb(((gray as u32) << 16) | ((gray as u32) << 8) | (gray as u32))
        }
    }
}

/// Parse a single SGR parameter sequence and apply it to the current style.
fn apply_sgr(style: &mut AnsiStyle, params: &[u8]) {
    let mut i = 0;
    while i < params.len() {
        match params[i] {
            0 => *style = AnsiStyle::default(),
            1 => style.bold = true,
            2 => style.dim = true,
            3 => style.italic = true,
            4 => style.underline = true,
            9 => style.strikethrough = true,
            22 => {
                style.bold = false;
                style.dim = false;
            }
            23 => style.italic = false,
            24 => style.underline = false,
            29 => style.strikethrough = false,
            // Standard foreground colors
            30..=37 => style.fg = Some(rgb(ANSI_COLORS[(params[i] - 30) as usize])),
            39 => style.fg = None, // Default foreground
            // Standard background colors
            40..=47 => style.bg = Some(rgb(ANSI_COLORS[(params[i] - 40) as usize])),
            49 => style.bg = None, // Default background
            // Bright foreground colors
            90..=97 => style.fg = Some(rgb(ANSI_BRIGHT_COLORS[(params[i] - 90) as usize])),
            // Bright background colors
            100..=107 => style.bg = Some(rgb(ANSI_BRIGHT_COLORS[(params[i] - 100) as usize])),
            // Extended color modes
            38 => {
                // Foreground extended
                if i + 1 < params.len() {
                    match params[i + 1] {
                        5 if i + 2 < params.len() => {
                            style.fg = Some(color_256(params[i + 2]));
                            i += 2;
                        }
                        2 if i + 4 < params.len() => {
                            let r = params[i + 2] as u32;
                            let g = params[i + 3] as u32;
                            let b = params[i + 4] as u32;
                            style.fg = Some(rgb((r << 16) | (g << 8) | b));
                            i += 4;
                        }
                        _ => {
                            i += 1;
                        }
                    }
                }
            }
            48 => {
                // Background extended
                if i + 1 < params.len() {
                    match params[i + 1] {
                        5 if i + 2 < params.len() => {
                            style.bg = Some(color_256(params[i + 2]));
                            i += 2;
                        }
                        2 if i + 4 < params.len() => {
                            let r = params[i + 2] as u32;
                            let g = params[i + 3] as u32;
                            let b = params[i + 4] as u32;
                            style.bg = Some(rgb((r << 16) | (g << 8) | b));
                            i += 4;
                        }
                        _ => {
                            i += 1;
                        }
                    }
                }
            }
            _ => {}
        }
        i += 1;
    }
}

/// Parse a line of terminal output, extracting ANSI escape sequences and splitting
/// into styled text segments suitable for GPUI rendering.
pub fn parse_ansi_line(line: &str) -> Vec<StyledSegment> {
    let mut segments = Vec::new();
    let mut current_style = AnsiStyle::default();
    let mut text_buf = String::new();
    let mut chars = line.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            // Start of escape sequence
            if chars.peek() == Some(&'[') {
                chars.next(); // consume '['
                              // Collect parameter bytes until we hit the final byte (letter)
                let mut param_str = String::new();
                while let Some(&next) = chars.peek() {
                    if next.is_ascii_alphabetic() || next == 'm' {
                        break;
                    }
                    param_str.push(chars.next().unwrap());
                }
                let final_byte = chars.next(); // consume the final byte

                if final_byte == Some('m') {
                    // SGR sequence — flush current text and apply style changes
                    if !text_buf.is_empty() {
                        segments.push(StyledSegment {
                            text: std::mem::take(&mut text_buf),
                            style: current_style.clone(),
                        });
                    }

                    let params: Vec<u8> = if param_str.is_empty() {
                        vec![0] // ESC[m is equivalent to ESC[0m (reset)
                    } else {
                        param_str
                            .split(';')
                            .filter_map(|s| s.parse::<u8>().ok())
                            .collect()
                    };

                    apply_sgr(&mut current_style, &params);
                }
                // Other escape sequences (cursor movement, etc.) are silently consumed
            }
        } else if ch == '\r' {
            // Carriage return — skip (terminal renders handle this differently)
            continue;
        } else {
            text_buf.push(ch);
        }
    }

    // Flush remaining text
    if !text_buf.is_empty() {
        segments.push(StyledSegment {
            text: text_buf,
            style: current_style,
        });
    }

    // If no segments at all, return a single empty segment for blank lines
    if segments.is_empty() {
        segments.push(StyledSegment {
            text: String::new(),
            style: AnsiStyle::default(),
        });
    }

    segments
}

/// Strip all ANSI escape sequences from a line, returning only the visible text.
#[allow(dead_code)]
pub fn strip_ansi(line: &str) -> String {
    let segments = parse_ansi_line(line);
    segments.into_iter().map(|s| s.text).collect()
}
