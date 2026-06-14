//! System register XML parsing.
//!
//! This module parses ARM system register XML files to extract register
//! definitions with their fields.

use quick_xml::events::Event;
use quick_xml::Reader;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Registers that should be treated as 32-bit even if declared as 64-bit.
const REGS_32: &[&str] = &[
    "AFSR0_EL1",
    "AFSR0_EL2",
    "AFSR0_EL3",
    "AFSR1_EL1",
    "AFSR1_EL2",
    "AFSR1_EL3",
    "AIDR_EL1",
    "CCSIDR2_EL1",
    "CCSIDR_EL1",
    "CNTFRQ_EL0",
    "CNTHCTL_EL2",
    "CNTHPS_CTL_EL2",
    "CNTHPS_TVAL_EL2",
    "CNTHP_CTL_EL2",
    "CNTHP_TVAL_EL2",
    "CNTHVS_CTL_EL2",
    "CNTHVS_TVAL_EL2",
    "CNTHV_CTL_EL2",
    "CNTHV_TVAL_EL2",
    "CNTKCTL_EL1",
    "CNTPS_CTL_EL1",
    "CNTPS_TVAL_EL1",
    "CNTP_CTL_EL0",
    "CNTP_TVAL_EL0",
    "CNTV_CTL_EL0",
    "CNTV_TVAL_EL0",
    "CONTEXTIDR_EL1",
    "CONTEXTIDR_EL2",
    "CPACR_EL1",
    "CPTR_EL2",
    "CPTR_EL3",
    "CSSELR_EL1",
    "CTR_EL0",
    "CURRENTEL",
    "DACR32_EL2",
    "DAIF",
    "DBGAUTHSTATUS_EL1",
    "DBGBCRN_EL1",
    "DBGCLAIMCLR_EL1",
    "DBGCLAIMSET_EL1",
    "DBGDTRRX_EL0",
    "DBGDTRTX_EL0",
    "DBGPRCR_EL1",
    "DBGVCR32_EL2",
    "DBGWCRN_EL1",
    "DCZID_EL0",
    "DIT",
    "DSPSR_EL0",
    "ESR_EL0",
    "ESR_EL1",
    "ESR_EL2",
    "ESR_EL3",
    "FPCR",
    "FPEXC32_EL2",
    "FPSR",
    "HACR_EL2",
    "HSTR_EL2",
    "ICC_AP0RN_EL1",
    "ICC_AP1RN_EL1",
    "ICC_BPR0_EL1",
    "ICC_BPR1_EL1",
    "ICC_CTLR_EL1",
    "ICC_CTLR_EL3",
    "ICC_DIR_EL1",
    "ICC_EOIR0_EL1",
    "ICC_EOIR1_EL1",
    "ICC_HPPIR0_EL1",
    "ICC_HPPIR1_EL1",
    "ICC_IAR0_EL1",
    "ICC_IAR1_EL1",
    "ICC_IGRPEN0_EL1",
    "ICC_IGRPEN1_EL1",
    "ICC_IGRPEN1_EL3",
    "ICC_PMR_EL1",
    "ICC_RPR_EL1",
    "ICC_SRE_EL1",
    "ICC_SRE_EL2",
    "ICC_SRE_EL3",
    "ICH_AP0RN_EL2",
    "ICH_AP1RN_EL2",
    "ICH_EISR_EL2",
    "ICH_ELRSR_EL2",
    "ICH_HCR_EL2",
    "ICH_MISR_EL2",
    "ICH_VMCR_EL2",
    "ICH_VTR_EL2",
    "ICV_AP0RN_EL1",
    "ICV_AP1RN_EL1",
    "ICV_BPR0_EL1",
    "ICV_BPR1_EL1",
    "ICV_CTLR_EL1",
    "ICV_DIR_EL1",
    "ICV_EOIR0_EL1",
    "ICV_EOIR1_EL1",
    "ICV_HPPIR0_EL1",
    "ICV_HPPIR1_EL1",
    "ICV_IAR0_EL1",
    "ICV_IAR1_EL1",
    "ICV_IGRPEN0_EL1",
    "ICV_IGRPEN1_EL1",
    "ICV_PMR_EL1",
    "ICV_RPR_EL1",
    "ID_AFR0_EL1",
    "ID_DFR0_EL1",
    "ID_ISAR0_EL1",
    "ID_ISAR1_EL1",
    "ID_ISAR2_EL1",
    "ID_ISAR3_EL1",
    "ID_ISAR4_EL1",
    "ID_ISAR5_EL1",
    "ID_ISAR6_EL1",
    "ID_MMFR0_EL1",
    "ID_MMFR1_EL1",
    "ID_MMFR2_EL1",
    "ID_MMFR3_EL1",
    "ID_MMFR4_EL1",
    "ID_PFR0_EL1",
    "ID_PFR1_EL1",
    "ID_PFR2_EL1",
    "IFSR32_EL2",
    "ISR_EL1",
    "MDCCINT_EL1",
    "MDCCSR_EL0",
    "MDCR_EL2",
    "MDCR_EL3",
    "MDSCR_EL1",
    "MIDR_EL1",
    "MVFR0_EL1",
    "MVFR1_EL1",
    "MVFR2_EL1",
    "NZCV",
    "OSDLR_EL1",
    "OSDTRRX_EL1",
    "OSDTRTX_EL1",
    "OSECCR_EL1",
    "OSLAR_EL1",
    "OSLSR_EL1",
    "PAN",
    "PMCCFILTR_EL0",
    "PMCNTENCLR_EL0",
    "PMCNTENSET_EL0",
    "PMCR_EL0",
    "PMEVCNTRN_EL0",
    "PMEVTYPERN_EL0",
    "PMINTENCLR_EL1",
    "PMINTENSET_EL1",
    "PMOVSCLR_EL0",
    "PMOVSSET_EL0",
    "PMSELR_EL0",
    "PMSWINC_EL0",
    "PMUSERENR_EL0",
    "PMXEVCNTR_EL0",
    "PMXEVTYPER_EL0",
    "REVIDR_EL1",
    "RGSR_EL1",
    "RMR_EL1",
    "RMR_EL2",
    "RMR_EL3",
    "RMUID_EL0",
    "SCR_EL3",
    "SDER32_EL2",
    "SDER32_EL3",
    "SPSEL",
    "SPSR_abt",
    "SPSR_EL0",
    "SPSR_EL1",
    "SPSR_EL2",
    "SPSR_EL3",
    "SPSR_fiq",
    "SPSR_irq",
    "SPSR_und",
    "SSBS",
    "TCR_EL3",
    "UAO",
    "VPIDR_EL2",
    "VSTCR_EL2",
    "VTCR_EL2",
];

/// A system register definition.
#[derive(Debug, Clone)]
pub struct SystemRegister {
    /// Register name (e.g., "SCTLR_EL1")
    pub name: String,
    /// Long descriptive name
    pub long_name: String,
    /// Register width in bits (32 or 64)
    pub width: u32,
    /// Register fields (name -> list of (msb, lsb) slices)
    pub fields: HashMap<String, Vec<(u32, u32)>>,
    /// Array bounds if this is an array register (start, end)
    pub array_bounds: Option<(u32, u32)>,
}

/// Read system registers from a directory.
pub fn read_registers(dir: &Path) -> Result<Vec<SystemRegister>, String> {
    let mut registers: HashMap<String, SystemRegister> = HashMap::new();

    let entries = fs::read_dir(dir).map_err(|e| format!("Failed to read directory: {}", e))?;

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "xml") {
            if let Ok(regs) = read_register_file(&path) {
                for reg in regs {
                    // Merge fields if register already exists
                    if let Some(existing) = registers.get_mut(&reg.name) {
                        for (name, slices) in reg.fields {
                            existing.fields.entry(name).or_insert(slices);
                        }
                    } else {
                        registers.insert(reg.name.clone(), reg);
                    }
                }
            }
        }
    }

    Ok(registers.into_values().collect())
}

/// Read registers from a single XML file.
fn read_register_file(path: &Path) -> Result<Vec<SystemRegister>, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;

    let mut reader = Reader::from_str(&content);
    reader.config_mut().trim_text(true);

    let mut registers = Vec::new();

    // State tracking
    let mut in_register = false;
    let mut is_register = false;
    let mut reg_name = String::new();
    let mut reg_long_name = String::new();
    let mut reg_length: u32 = 0;
    let mut array_bounds: Option<(u32, u32)> = None;
    let mut fields: HashMap<String, Vec<(u32, u32)>> = HashMap::new();
    let mut sliced_fields: HashMap<String, Vec<(u32, u32, (u32, u32))>> = HashMap::new();

    // Field state
    let mut in_field = false;
    let mut field_name = String::new();
    let mut field_msb: u32 = 0;
    let mut field_lsb: u32 = 0;

    // Element content collection
    let mut collecting_element: Option<String> = None;
    let mut element_content = String::new();

    // Track fields element to get max length
    let mut field_lengths: Vec<u32> = Vec::new();

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) => {
                match e.name().as_ref() {
                    b"register" => {
                        in_register = true;
                        is_register = false;
                        reg_name.clear();
                        reg_long_name.clear();
                        reg_length = 0;
                        array_bounds = None;
                        fields.clear();
                        sliced_fields.clear();
                        field_lengths.clear();

                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"is_register" {
                                is_register = String::from_utf8_lossy(&attr.value) == "True";
                            }
                        }
                    }
                    b"reg_short_name" if in_register => {
                        collecting_element = Some("reg_short_name".to_string());
                        element_content.clear();
                    }
                    b"reg_long_name" if in_register => {
                        collecting_element = Some("reg_long_name".to_string());
                        element_content.clear();
                    }
                    b"reg_array_start" if in_register => {
                        collecting_element = Some("reg_array_start".to_string());
                        element_content.clear();
                    }
                    b"reg_array_end" if in_register => {
                        collecting_element = Some("reg_array_end".to_string());
                        element_content.clear();
                    }
                    b"fields" if in_register => {
                        // Get length attribute
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"length" {
                                if let Ok(len) = String::from_utf8_lossy(&attr.value).parse::<u32>()
                                {
                                    field_lengths.push(len);
                                }
                            }
                        }
                    }
                    b"field" if in_register => {
                        in_field = true;
                        field_name.clear();
                        field_msb = 0;
                        field_lsb = 0;
                    }
                    b"field_name" if in_field => {
                        collecting_element = Some("field_name".to_string());
                        element_content.clear();
                    }
                    b"field_msb" if in_field => {
                        collecting_element = Some("field_msb".to_string());
                        element_content.clear();
                    }
                    b"field_lsb" if in_field => {
                        collecting_element = Some("field_lsb".to_string());
                        element_content.clear();
                    }
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) => {
                match e.name().as_ref() {
                    b"register" => {
                        if in_register && is_register && is_valid_identifier(&reg_name) {
                            // Skip LSR register (conflicts with LSR function)
                            if reg_name == "LSR" {
                                in_register = false;
                                continue;
                            }

                            // Get max length
                            reg_length = *field_lengths.iter().max().unwrap_or(&32);

                            // Apply 32-bit workaround
                            if REGS_32.contains(&reg_name.as_str()) {
                                reg_length = 32;
                            }

                            // Merge sliced fields
                            for (name, slices) in sliced_fields.drain() {
                                let mut sorted_slices = slices;
                                sorted_slices.sort_by(|a, b| b.2 .0.cmp(&a.2 .0));
                                let merged: Vec<(u32, u32)> = sorted_slices
                                    .iter()
                                    .map(|(msb, lsb, _)| (*msb, *lsb))
                                    .collect();
                                fields.insert(name, merged);
                            }

                            registers.push(SystemRegister {
                                name: reg_name.clone(),
                                long_name: reg_long_name.clone(),
                                width: reg_length,
                                fields: fields.clone(),
                                array_bounds,
                            });
                        }
                        in_register = false;
                    }
                    b"field" => {
                        if in_field && !field_name.is_empty() {
                            // Check for sliced field notation like FOO[7:0]
                            let re_slice1 = Regex::new(r"^(\w+)\[(\d+)\]$").unwrap();
                            let re_slice2 = Regex::new(r"^(\w+)\[(\d+):(\d+)\]$").unwrap();

                            if let Some(cap) = re_slice1.captures(&field_name) {
                                let name = cap[1].to_string();
                                let bit: u32 = cap[2].parse().unwrap_or(0);
                                sliced_fields.entry(name).or_default().push((
                                    field_msb,
                                    field_lsb,
                                    (bit, bit),
                                ));
                            } else if let Some(cap) = re_slice2.captures(&field_name) {
                                let name = cap[1].to_string();
                                let hi: u32 = cap[2].parse().unwrap_or(0);
                                let lo: u32 = cap[3].parse().unwrap_or(0);
                                sliced_fields.entry(name).or_default().push((
                                    field_msb,
                                    field_lsb,
                                    (hi, lo),
                                ));
                            } else if is_valid_identifier(&field_name) && field_name != "UNKNOWN" {
                                fields.insert(field_name.clone(), vec![(field_msb, field_lsb)]);
                            }
                        }
                        in_field = false;
                    }
                    b"reg_short_name" => {
                        if collecting_element.as_deref() == Some("reg_short_name") {
                            reg_name = element_content.trim().replace("<n>", "");
                            collecting_element = None;
                        }
                    }
                    b"reg_long_name" => {
                        if collecting_element.as_deref() == Some("reg_long_name") {
                            reg_long_name = element_content.trim().to_string();
                            collecting_element = None;
                        }
                    }
                    b"reg_array_start" => {
                        if collecting_element.as_deref() == Some("reg_array_start") {
                            if let Ok(start) = element_content.trim().parse::<u32>() {
                                array_bounds =
                                    Some((start, array_bounds.map(|b| b.1).unwrap_or(0)));
                            }
                            collecting_element = None;
                        }
                    }
                    b"reg_array_end" => {
                        if collecting_element.as_deref() == Some("reg_array_end") {
                            if let Ok(end) = element_content.trim().parse::<u32>() {
                                array_bounds = Some((array_bounds.map(|b| b.0).unwrap_or(0), end));
                            }
                            collecting_element = None;
                        }
                    }
                    b"field_name" => {
                        if collecting_element.as_deref() == Some("field_name") {
                            field_name = element_content.trim().to_string();
                            collecting_element = None;
                        }
                    }
                    b"field_msb" => {
                        if collecting_element.as_deref() == Some("field_msb") {
                            field_msb = element_content.trim().parse().unwrap_or(0);
                            collecting_element = None;
                        }
                    }
                    b"field_lsb" => {
                        if collecting_element.as_deref() == Some("field_lsb") {
                            field_lsb = element_content.trim().parse().unwrap_or(0);
                            collecting_element = None;
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::Text(e)) => {
                if collecting_element.is_some() {
                    if let Ok(text) = e.unescape() {
                        element_content.push_str(&text);
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(format!("XML parse error: {}", e)),
            _ => {}
        }
    }

    Ok(registers)
}

/// Check if a string is a valid ASL identifier.
fn is_valid_identifier(s: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z_]\w*$").unwrap();
    re.is_match(s)
}

/// Generate ASL code for system register definitions.
pub fn generate_registers_asl(registers: &[SystemRegister], notice: &str) -> String {
    let mut output = String::new();

    output.push_str(&"/".repeat(72));
    output.push('\n');
    for line in notice.lines() {
        output.push_str(line);
        output.push('\n');
    }
    output.push_str(&"/".repeat(72));
    output.push_str("\n\n");

    for reg in registers {
        // Comment with long name
        output.push_str(&format!("// {}\n", reg.long_name));

        // Check if implementation defined
        let prefix = if reg.long_name == "IMPLEMENTATION DEFINED registers" {
            "// "
        } else {
            ""
        };

        // Build field list
        let field_strs: Vec<String> = reg
            .fields
            .iter()
            .map(|(name, slices)| {
                let slice_str: String = slices
                    .iter()
                    .map(|(msb, lsb)| format!("{}:{}", msb, lsb))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{} {}", slice_str, name)
            })
            .collect();

        let type_str = format!("__register {} {{ {} }}", reg.width, field_strs.join(", "));

        // Handle array registers
        let decl = if let Some((start, end)) = reg.array_bounds {
            format!("array [{}..{}] of {} {};", start, end, type_str, reg.name)
        } else {
            format!("{} {};", type_str, reg.name)
        };

        output.push_str(prefix);
        output.push_str(&decl);
        output.push_str("\n\n");
    }

    output
}
