//! Trace-sidecar support for the vphone600ap iOS boot path.
//!
//! This module intentionally does not claim to be a full Apple SoC machine.
//! It validates and mirrors the C++ Acheron/STYX boot layout, emits a stable
//! JSONL trace schema, and turns the existing WHP progress log into concrete
//! stall findings that can drive the next device-model fix.

use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::error::{Error, Result};

pub const RAM_BASE: u64 = 0x8000_0000;
pub const RAM_SIZE: u64 = 4 * 1024 * 1024 * 1024;
pub const KERNEL_LOAD_BASE: u64 = 0x8100_0000;
pub const KERNEL_ENTRY: u64 = 0x8227_0000;
pub const DTB_BASE: u64 = 0x9000_0000;
pub const BOOT_ARGS_BASE: u64 = RAM_BASE + 0x800000;
pub const SP_BASE: u64 = RAM_BASE + 0x700000;
pub const TPIDR_EL1_BASE: u64 = RAM_BASE + 0xB00000;
pub const VBAR_EL1_BASE: u64 = RAM_BASE;
pub const PSTATE_EL1H_DAIF: u64 = 0x3C5;
pub const SCTLR_EL1_RESET: u64 = 0x00C5_0830;
pub const FRAMEBUFFER_BASE: u64 = RAM_BASE + RAM_SIZE - 0x1000000;
pub const FRAMEBUFFER_WIDTH: u32 = 1290;
pub const FRAMEBUFFER_HEIGHT: u32 = 2796;
pub const FRAMEBUFFER_DEPTH: u32 = 32;

const FDT_MAGIC: u32 = 0xD00D_FEED;
const MH_MAGIC_64: u32 = 0xFEED_FACF;
const LC_SEGMENT_64: u32 = 0x19;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BootLayout {
    pub ram_base: u64,
    pub ram_size: u64,
    pub kernel_load_base: u64,
    pub kernel_entry: u64,
    pub dtb_base: u64,
    pub boot_args_base: u64,
    pub sp: u64,
    pub pstate: u64,
    pub sctlr_el1: u64,
    pub vbar_el1: u64,
    pub tpidr_el1: u64,
    pub framebuffer_base: u64,
    pub framebuffer_width: u32,
    pub framebuffer_height: u32,
    pub framebuffer_depth: u32,
}

impl Default for BootLayout {
    fn default() -> Self {
        Self {
            ram_base: RAM_BASE,
            ram_size: RAM_SIZE,
            kernel_load_base: KERNEL_LOAD_BASE,
            kernel_entry: KERNEL_ENTRY,
            dtb_base: DTB_BASE,
            boot_args_base: BOOT_ARGS_BASE,
            sp: SP_BASE,
            pstate: PSTATE_EL1H_DAIF,
            sctlr_el1: SCTLR_EL1_RESET,
            vbar_el1: VBAR_EL1_BASE,
            tpidr_el1: TPIDR_EL1_BASE,
            framebuffer_base: FRAMEBUFFER_BASE,
            framebuffer_width: FRAMEBUFFER_WIDTH,
            framebuffer_height: FRAMEBUFFER_HEIGHT,
            framebuffer_depth: FRAMEBUFFER_DEPTH,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Segment64 {
    pub name: String,
    pub vmaddr: u64,
    pub vmsize: u64,
    pub fileoff: u64,
    pub filesize: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct KernelMetadata {
    pub path: PathBuf,
    pub size: u64,
    pub filetype: u32,
    pub ncmds: u32,
    pub original_base: u64,
    pub slide: u64,
    pub entry_file_offset: u64,
    pub entry_point: u64,
    pub first_instruction: Option<u32>,
    pub segments: Vec<Segment64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct DtbMetadata {
    pub path: PathBuf,
    pub file_size: u64,
    pub payload_size: u64,
    pub raw_fdt: bool,
    pub format: String,
    pub payload_offset: u64,
    pub totalsize: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct VphoneImage {
    pub layout: BootLayout,
    pub kernel: KernelMetadata,
    pub dtb: DtbMetadata,
    pub boot_args: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum TraceEvent {
    Metadata {
        layout: BootLayout,
        kernel: KernelMetadata,
        dtb: DtbMetadata,
    },
    Insn {
        pc: u64,
        mnemonic: Option<String>,
        regs_changed: Option<BTreeMap<String, String>>,
    },
    MemRead {
        pc: u64,
        gpa: u64,
        size: u8,
        value: u64,
        mapped: bool,
        device: Option<String>,
    },
    MemWrite {
        pc: u64,
        gpa: u64,
        size: u8,
        value: u64,
        mapped: bool,
        device: Option<String>,
    },
    Sysreg {
        pc: u64,
        name: String,
        op: String,
        value: u64,
    },
    Loop {
        pc: u64,
        count: u64,
        last_mmio: Option<u64>,
        last_sysreg: Option<String>,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PcFinding {
    pub pc: u64,
    pub count: u64,
    pub likely_area: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_mmio: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_mmio_device: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_sysreg: Option<String>,
    pub recommendation: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TraceFindings {
    pub target: String,
    pub progress_log: Option<PathBuf>,
    pub total_pc_samples: u64,
    pub top_spinning_pcs: Vec<PcFinding>,
    pub likely_device_focus: Vec<String>,
    pub next_action: String,
}

pub fn load_vphone_image(kernel_path: &Path, dtb_path: &Path) -> Result<VphoneImage> {
    let kernel = parse_kernel_metadata(kernel_path)?;
    let dtb = parse_dtb_metadata(dtb_path)?;
    let layout = BootLayout::default();
    Ok(VphoneImage {
        boot_args: build_boot_args(&layout, dtb.payload_size as u32, false),
        layout,
        kernel,
        dtb,
    })
}

pub fn parse_kernel_metadata(path: &Path) -> Result<KernelMetadata> {
    let mut data = Vec::new();
    File::open(path)?.read_to_end(&mut data)?;
    if data.len() < 32 {
        return Err(Error::KernelLoad("kernelcache is too small".to_string()));
    }
    let magic = le_u32(&data, 0)?;
    if magic != MH_MAGIC_64 {
        return Err(Error::KernelLoad(format!(
            "{} is not a little-endian Mach-O 64 kernelcache (magic=0x{magic:08x})",
            path.display()
        )));
    }
    let filetype = le_u32(&data, 12)?;
    let ncmds = le_u32(&data, 16)?;
    let sizeofcmds = le_u32(&data, 20)? as usize;
    if 32usize
        .checked_add(sizeofcmds)
        .map(|end| end > data.len())
        .unwrap_or(true)
    {
        return Err(Error::KernelLoad(
            "Mach-O load commands exceed file size".to_string(),
        ));
    }

    let mut off = 32usize;
    let mut segments = Vec::new();
    for _ in 0..ncmds {
        if off + 8 > data.len() {
            return Err(Error::KernelLoad(
                "truncated Mach-O load command".to_string(),
            ));
        }
        let cmd = le_u32(&data, off)?;
        let cmdsize = le_u32(&data, off + 4)? as usize;
        if cmdsize < 8 || off + cmdsize > data.len() {
            return Err(Error::KernelLoad(
                "invalid Mach-O load command size".to_string(),
            ));
        }
        if cmd == LC_SEGMENT_64 && cmdsize >= 72 {
            let raw_name = &data[off + 8..off + 24];
            let name_end = raw_name
                .iter()
                .position(|b| *b == 0)
                .unwrap_or(raw_name.len());
            let name = String::from_utf8_lossy(&raw_name[..name_end]).into_owned();
            segments.push(Segment64 {
                name,
                vmaddr: le_u64(&data, off + 24)?,
                vmsize: le_u64(&data, off + 32)?,
                fileoff: le_u64(&data, off + 40)?,
                filesize: le_u64(&data, off + 48)?,
            });
        }
        off += cmdsize;
    }

    let original_base = segments
        .iter()
        .filter(|s| s.filesize > 0)
        .map(|s| s.vmaddr)
        .min()
        .ok_or_else(|| Error::KernelLoad("kernelcache has no loadable segments".to_string()))?;
    let slide = KERNEL_LOAD_BASE.wrapping_sub(original_base);
    let entry_file_offset = KERNEL_ENTRY
        .checked_sub(KERNEL_LOAD_BASE)
        .ok_or_else(|| Error::KernelLoad("kernel entry precedes load base".to_string()))?;
    let first_instruction = data
        .get(entry_file_offset as usize..entry_file_offset as usize + 4)
        .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]));

    Ok(KernelMetadata {
        path: path.to_path_buf(),
        size: data.len() as u64,
        filetype,
        ncmds,
        original_base,
        slide,
        entry_file_offset,
        entry_point: KERNEL_ENTRY,
        first_instruction,
        segments,
    })
}

pub fn parse_dtb_metadata(path: &Path) -> Result<DtbMetadata> {
    let mut data = Vec::new();
    File::open(path)?.read_to_end(&mut data)?;
    let file_size = data.len() as u64;
    if data.len() < 8 {
        return Err(Error::KernelLoad(format!(
            "{} is too small to be a DeviceTree payload",
            path.display()
        )));
    }

    let mut payload_offset = 0usize;
    let mut payload_size = data.len();
    let mut format = "apple-device-tree".to_string();

    if data.get(6..10) == Some(b"IM4P") {
        let (offset, size) = find_im4p_payload(&data).ok_or_else(|| {
            Error::KernelLoad(format!(
                "{} looks like IM4P but no payload OCTET STRING was found",
                path.display()
            ))
        })?;
        payload_offset = offset;
        payload_size = size;
        format = "im4p-wrapped-apple-device-tree".to_string();
    }

    let payload = data
        .get(payload_offset..payload_offset + payload_size)
        .ok_or_else(|| Error::KernelLoad("DeviceTree payload exceeds file size".to_string()))?;
    let raw_fdt = payload.len() >= 8
        && u32::from_be_bytes(payload[0..4].try_into().unwrap()) == FDT_MAGIC;
    let totalsize = raw_fdt.then(|| u32::from_be_bytes(payload[4..8].try_into().unwrap()));
    if raw_fdt {
        format = if payload_offset == 0 {
            "fdt".to_string()
        } else {
            "im4p-wrapped-fdt".to_string()
        };
    }

    Ok(DtbMetadata {
        path: path.to_path_buf(),
        file_size,
        payload_size: payload_size as u64,
        raw_fdt,
        format,
        payload_offset: payload_offset as u64,
        totalsize,
    })
}

fn find_im4p_payload(data: &[u8]) -> Option<(usize, usize)> {
    let mut offset = 10usize;
    while offset + 2 <= data.len() {
        if data[offset] == 0x04 {
            if let Some((len, len_bytes)) = der_len(&data[offset + 1..]) {
                let payload_offset = offset + 1 + len_bytes;
                if payload_offset + len <= data.len() && len > 0x100 {
                    return Some((payload_offset, len));
                }
            }
        }
        offset += 1;
    }
    None
}

fn der_len(data: &[u8]) -> Option<(usize, usize)> {
    let first = *data.first()?;
    if first & 0x80 == 0 {
        return Some((first as usize, 1));
    }
    let count = (first & 0x7F) as usize;
    if count == 0 || count > 4 || data.len() < 1 + count {
        return None;
    }
    let mut len = 0usize;
    for byte in &data[1..1 + count] {
        len = (len << 8) | (*byte as usize);
    }
    Some((len, 1 + count))
}

pub fn build_boot_args(layout: &BootLayout, dtb_size: u32, ramdisk: bool) -> Vec<u8> {
    let mut ba = vec![0u8; 0x3000];
    write_u16(&mut ba, 0x00, 2);
    write_u16(&mut ba, 0x02, 2);
    write_u64(&mut ba, 0x08, 0xFFFF_FE00_0000_0000);
    write_u64(&mut ba, 0x10, layout.ram_base);
    write_u64(&mut ba, 0x18, layout.ram_size);
    write_u64(&mut ba, 0x20, layout.ram_base + 0x4000000);
    write_u64(&mut ba, 0x28, layout.framebuffer_base);
    write_u64(&mut ba, 0x30, 1);
    write_u32(&mut ba, 0x38, layout.framebuffer_width * 4);
    write_u32(&mut ba, 0x3C, layout.framebuffer_width);
    write_u32(&mut ba, 0x40, layout.framebuffer_height);
    write_u32(&mut ba, 0x44, layout.framebuffer_depth);
    write_u32(&mut ba, 0x68, 0);
    write_u64(&mut ba, 0x70, layout.dtb_base);
    write_u32(&mut ba, 0x78, dtb_size);

    let cmdline = if ramdisk {
        "rd=md0 serial=3 debug=0x14e kextlog=0xfff -v wdt=-1 amfi=0xff cs_enforcement_disable=1"
    } else {
        "serial=3 debug=0x8 kextlog=0xfff -v"
    };
    let cmd = cmdline.as_bytes();
    let len = cmd.len().min(607);
    ba[0x80..0x80 + len].copy_from_slice(&cmd[..len]);
    write_u64(&mut ba, 0x2E0, 0x8);
    write_u64(&mut ba, 0x2E8, layout.ram_size);
    ba
}

pub fn emit_trace_jsonl(
    image: &VphoneImage,
    progress_log: Option<&Path>,
    trace_path: &Path,
    max_insns: u64,
) -> Result<TraceFindings> {
    let findings = analyze_progress_log(progress_log)?;
    let mut writer = BufWriter::new(File::create(trace_path)?);
    write_event(
        &mut writer,
        &TraceEvent::Metadata {
            layout: image.layout.clone(),
            kernel: image.kernel.clone(),
            dtb: image.dtb.clone(),
        },
    )?;
    write_event(
        &mut writer,
        &TraceEvent::Sysreg {
            pc: image.layout.kernel_entry,
            name: "SCTLR_EL1".to_string(),
            op: "seed".to_string(),
            value: image.layout.sctlr_el1,
        },
    )?;
    write_event(
        &mut writer,
        &TraceEvent::Sysreg {
            pc: image.layout.kernel_entry,
            name: "VBAR_EL1".to_string(),
            op: "seed".to_string(),
            value: image.layout.vbar_el1,
        },
    )?;
    write_event(
        &mut writer,
        &TraceEvent::Sysreg {
            pc: image.layout.kernel_entry,
            name: "TPIDR_EL1".to_string(),
            op: "seed".to_string(),
            value: image.layout.tpidr_el1,
        },
    )?;
    write_event(
        &mut writer,
        &TraceEvent::Insn {
            pc: image.layout.kernel_entry,
            mnemonic: image
                .kernel
                .first_instruction
                .map(|i| format!("raw32 0x{i:08x}")),
            regs_changed: Some(BTreeMap::from([
                ("x0".to_string(), hex(image.layout.boot_args_base)),
                ("sp".to_string(), hex(image.layout.sp)),
                ("pc".to_string(), hex(image.layout.kernel_entry)),
            ])),
        },
    )?;

    for finding in findings
        .top_spinning_pcs
        .iter()
        .take(max_insns.min(1024) as usize)
    {
        write_event(
            &mut writer,
            &TraceEvent::Loop {
                pc: finding.pc,
                count: finding.count,
                last_mmio: None,
                last_sysreg: None,
            },
        )?;
    }
    writer.flush()?;
    Ok(findings)
}

pub fn write_findings(path: &Path, findings: &TraceFindings) -> Result<()> {
    let mut writer = BufWriter::new(File::create(path)?);
    serde_json::to_writer_pretty(&mut writer, findings)
        .map_err(|e| Error::InvalidConfig(format!("failed to write findings JSON: {e}")))?;
    writer.write_all(b"\n")?;
    writer.flush()?;
    Ok(())
}

pub fn enrich_findings_from_trace(findings: &mut TraceFindings, trace_path: &Path) -> Result<()> {
    if !trace_path.exists() || findings.top_spinning_pcs.is_empty() {
        return Ok(());
    }

    let mut wanted: BTreeMap<u64, usize> = findings
        .top_spinning_pcs
        .iter()
        .enumerate()
        .map(|(idx, finding)| (finding.pc, idx))
        .collect();
    if wanted.is_empty() {
        return Ok(());
    }

    let mut last_mmio = None;
    let mut last_mmio_device = None;
    let mut last_sysreg = None;
    for line in BufReader::new(File::open(trace_path)?).lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let Ok(event) = serde_json::from_str::<TraceEvent>(&line) else {
            continue;
        };
        match event {
            TraceEvent::MemRead {
                gpa,
                mapped,
                device,
                ..
            }
            | TraceEvent::MemWrite {
                gpa,
                mapped,
                device,
                ..
            } => {
                if !mapped || device.is_some() {
                    last_mmio = Some(gpa);
                    last_mmio_device = device.or_else(|| known_device_for_gpa(gpa).map(str::to_string));
                }
            }
            TraceEvent::Sysreg {
                name, op, value, ..
            } => {
                last_sysreg = Some(format!("{op} {name}=0x{value:x}"));
            }
            TraceEvent::Insn { pc, .. } | TraceEvent::Loop { pc, .. } => {
                if let Some(idx) = wanted.remove(&pc) {
                    let finding = &mut findings.top_spinning_pcs[idx];
                    finding.last_mmio = last_mmio;
                    finding.last_mmio_device = last_mmio_device.clone();
                    finding.last_sysreg = last_sysreg.clone();
                }
                if wanted.is_empty() {
                    break;
                }
            }
            TraceEvent::Metadata { .. } => {}
        }
    }

    if findings
        .top_spinning_pcs
        .iter()
        .any(|f| f.last_mmio.is_some() || f.last_sysreg.is_some())
    {
        findings.likely_device_focus.insert(
            0,
            format!(
                "Runtime trace {} was correlated with hot progress-log PCs",
                trace_path.display()
            ),
        );
    }
    Ok(())
}

pub fn analyze_progress_log(progress_log: Option<&Path>) -> Result<TraceFindings> {
    let Some(path) = progress_log else {
        return Ok(empty_findings(None));
    };
    if !path.exists() {
        return Ok(empty_findings(Some(path.to_path_buf())));
    }

    let file = File::open(path)?;
    let mut counts: BTreeMap<u64, u64> = BTreeMap::new();
    let mut total = 0u64;
    for line in BufReader::new(file).lines() {
        let line = line?;
        if let Some(pc) = extract_pc(&line) {
            *counts.entry(pc).or_insert(0) += 1;
            total += 1;
        }
    }
    let mut pcs: Vec<(u64, u64)> = counts.into_iter().collect();
    pcs.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    let top_spinning_pcs = pcs
        .into_iter()
        .take(32)
        .map(|(pc, count)| PcFinding {
            pc,
            count,
            likely_area: classify_pc(pc).to_string(),
            last_mmio: None,
            last_mmio_device: None,
            last_sysreg: None,
            recommendation: recommendation_for_pc(pc).to_string(),
        })
        .collect();

    Ok(TraceFindings {
        target: "iPhone17,3/vphone600ap iOS 26.1 23B85".to_string(),
        progress_log: Some(path.to_path_buf()),
        total_pc_samples: total,
        top_spinning_pcs,
        likely_device_focus: vec![
            "Correlate hot PCs with rax dynamic MMIO/sysreg trace once CPU stepping is enabled"
                .to_string(),
            "Prioritize AIC/timer/SEP/NVRAM only when a traced poll proves ownership".to_string(),
            "Keep SpringBoard/boot animation milestones out of boot-success criteria".to_string(),
        ],
        next_action: "Run rax-vphone-trace with the current kernel, raw DTB, progress log, then implement the smallest C++ device fix indicated by the hottest traced poll.".to_string(),
    })
}

pub fn known_device_for_gpa(gpa: u64) -> Option<&'static str> {
    const RANGES: &[(u64, u64, &str)] = &[
        (0x2100_00000, 0x1000, "SEP mailbox"),
        (0x2310_00000, 0x1000, "PMU"),
        (0x2320_00000, 0x1000, "display controller"),
        (0x2352_00000, 0x1000, "UART PL011"),
        (0x23B7_00000, 0x1000, "SMC"),
        (0x23E1_00000, 0x1000, "Apple ARM64 timer"),
        (0x28E1_00000, 0x10000, "AIC"),
        (0x4000_00000, 0x200, "VirtIO net"),
        (0x4010_00000, 0x200, "VirtIO block"),
        (0x4020_00000, 0x200, "VirtIO rng"),
        (0x8000_00000, 0x1000000, "framebuffer/VRAM"),
    ];
    RANGES
        .iter()
        .find(|(base, len, _)| gpa >= *base && gpa < *base + *len)
        .map(|(_, _, name)| *name)
}

pub fn runtime_trace_enabled() -> bool {
    runtime_trace_writer().is_some()
}

pub fn record_runtime_event(event: &TraceEvent) {
    let Some(writer) = runtime_trace_writer() else {
        return;
    };
    if let Ok(mut writer) = writer.lock() {
        let _ = write_event(&mut *writer, event);
        let _ = writer.flush();
    }
}

fn runtime_trace_writer() -> Option<&'static Mutex<BufWriter<File>>> {
    static WRITER: OnceLock<Option<Mutex<BufWriter<File>>>> = OnceLock::new();
    WRITER
        .get_or_init(|| {
            let path = std::env::var_os("RAX_TRACE_JSONL")?;
            let file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(path)
                .ok()?;
            Some(Mutex::new(BufWriter::new(file)))
        })
        .as_ref()
}

fn write_event(writer: &mut impl Write, event: &TraceEvent) -> Result<()> {
    serde_json::to_writer(&mut *writer, event)
        .map_err(|e| Error::InvalidConfig(format!("failed to write trace JSONL: {e}")))?;
    writer.write_all(b"\n")?;
    Ok(())
}

fn empty_findings(progress_log: Option<PathBuf>) -> TraceFindings {
    TraceFindings {
        target: "iPhone17,3/vphone600ap iOS 26.1 23B85".to_string(),
        progress_log,
        total_pc_samples: 0,
        top_spinning_pcs: Vec::new(),
        likely_device_focus: vec![
            "No progress log samples were available; trace output contains loader parity events only"
                .to_string(),
        ],
        next_action: "Capture or pass Virtual-iPhone-Emulator/vm/progress.log to rank current spinning PCs.".to_string(),
    }
}

fn extract_pc(line: &str) -> Option<u64> {
    let marker = "PC: 0x";
    let start = line.find(marker)? + marker.len();
    let hex_digits: String = line[start..]
        .chars()
        .take_while(|c| c.is_ascii_hexdigit())
        .collect();
    if hex_digits.is_empty() {
        None
    } else {
        u64::from_str_radix(&hex_digits, 16).ok()
    }
}

fn classify_pc(pc: u64) -> &'static str {
    match pc {
        0x81c5_0000..=0x81c5_ffff => "early XNU platform/device wait loop",
        0x81af_0000..=0x81af_ffff => "early XNU bootstrap/service wait loop",
        0x8222_0000..=0x8223_ffff => "kernel text execution loop near direct-entry path",
        0x8227_0000 => "configured XNU entry point",
        _ => "unclassified kernel PC",
    }
}

fn recommendation_for_pc(pc: u64) -> &'static str {
    match pc {
        0x81c5_0000..=0x81c5_ffff => {
            "Trace the last MMIO/sysreg read before this PC; likely timer/AIC/platform readiness semantics."
        }
        0x81af_0000..=0x81af_ffff => {
            "Correlate with bootstrap object seeds and early service callbacks before adding new stubs."
        }
        0x8222_0000..=0x8223_ffff => {
            "Disassemble the loop and capture the polled register/value; avoid broad NOP gates."
        }
        _ => {
            "Use dynamic rax trace to assign this PC to a concrete poll/device before patching C++."
        }
    }
}

fn le_u32(data: &[u8], off: usize) -> Result<u32> {
    let bytes = data
        .get(off..off + 4)
        .ok_or_else(|| Error::KernelLoad("unexpected EOF reading u32".to_string()))?;
    Ok(u32::from_le_bytes(bytes.try_into().unwrap()))
}

fn le_u64(data: &[u8], off: usize) -> Result<u64> {
    let bytes = data
        .get(off..off + 8)
        .ok_or_else(|| Error::KernelLoad("unexpected EOF reading u64".to_string()))?;
    Ok(u64::from_le_bytes(bytes.try_into().unwrap()))
}

fn write_u16(data: &mut [u8], off: usize, value: u16) {
    data[off..off + 2].copy_from_slice(&value.to_le_bytes());
}

fn write_u32(data: &mut [u8], off: usize, value: u32) {
    data[off..off + 4].copy_from_slice(&value.to_le_bytes());
}

fn write_u64(data: &mut [u8], off: usize, value: u64) {
    data[off..off + 8].copy_from_slice(&value.to_le_bytes());
}

fn hex(value: u64) -> String {
    format!("0x{value:x}")
}

pub fn summary_json(image: &VphoneImage, findings: &TraceFindings) -> serde_json::Value {
    json!({
        "target": findings.target,
        "kernel_entry": hex(image.layout.kernel_entry),
        "boot_args": hex(image.layout.boot_args_base),
        "dtb": hex(image.layout.dtb_base),
        "kernel_size": image.kernel.size,
        "dtb_size": image.dtb.payload_size,
        "pc_samples": findings.total_pc_samples,
        "top_pc": findings.top_spinning_pcs.first().map(|p| hex(p.pc)),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boot_layout_matches_styx_constants() {
        let layout = BootLayout::default();
        assert_eq!(layout.ram_base, 0x8000_0000);
        assert_eq!(layout.kernel_load_base, 0x8100_0000);
        assert_eq!(layout.kernel_entry, 0x8227_0000);
        assert_eq!(layout.dtb_base, 0x9000_0000);
        assert_eq!(layout.boot_args_base, 0x8080_0000);
        assert_eq!(layout.framebuffer_base, 0x17f00_0000);
    }

    #[test]
    fn boot_args_fields_match_windows_hypervisor_layout() {
        let layout = BootLayout::default();
        let args = build_boot_args(&layout, 0x10b00, false);
        assert_eq!(u16::from_le_bytes(args[0x00..0x02].try_into().unwrap()), 2);
        assert_eq!(
            u64::from_le_bytes(args[0x10..0x18].try_into().unwrap()),
            RAM_BASE
        );
        assert_eq!(
            u64::from_le_bytes(args[0x28..0x30].try_into().unwrap()),
            FRAMEBUFFER_BASE
        );
        assert_eq!(
            u32::from_le_bytes(args[0x38..0x3C].try_into().unwrap()),
            FRAMEBUFFER_WIDTH * 4
        );
        assert_eq!(
            u64::from_le_bytes(args[0x70..0x78].try_into().unwrap()),
            DTB_BASE
        );
        assert_eq!(
            u32::from_le_bytes(args[0x78..0x7C].try_into().unwrap()),
            0x10b00
        );
        assert!(
            std::str::from_utf8(&args[0x80..0xC0])
                .unwrap()
                .contains("serial=3")
        );
    }

    #[test]
    fn progress_log_analysis_ranks_hot_pcs() {
        let dir = std::env::temp_dir();
        let path = dir.join(format!(
            "rax-vphone-progress-{}-{}.log",
            std::process::id(),
            "rank"
        ));
        std::fs::write(
            &path,
            "[STYX] 1M exits | PC: 0x81c57074\n[STYX] 2M exits | PC: 0x81c57074\n[STYX] 3M exits | PC: 0x8222ded8\n",
        )
        .unwrap();
        let findings = analyze_progress_log(Some(&path)).unwrap();
        let _ = std::fs::remove_file(&path);
        assert_eq!(findings.total_pc_samples, 3);
        assert_eq!(findings.top_spinning_pcs[0].pc, 0x81c5_7074);
        assert_eq!(findings.top_spinning_pcs[0].count, 2);
        assert!(
            findings.top_spinning_pcs[0]
                .likely_area
                .contains("platform")
        );
    }

    #[test]
    fn known_device_ranges_are_named() {
        assert_eq!(
            known_device_for_gpa(0x23E1_00000),
            Some("Apple ARM64 timer")
        );
        assert_eq!(known_device_for_gpa(0x28E1_00000), Some("AIC"));
        assert_eq!(known_device_for_gpa(0x1234), None);
    }
}
