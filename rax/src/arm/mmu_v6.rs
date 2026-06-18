//! ARMv6/ARMv7 short-descriptor MMU walk (VMSAv6).
//!
//! Translates 32-bit virtual addresses through the two-level short-descriptor
//! page tables used by 32-bit ARM Linux: first-level sections/supersections
//! and coarse second-level small/large pages, with domain (DACR) and AP
//! permission checks. Subpages (the pre-v6 AP0..AP3 split) are not modelled —
//! ARMv6K kernels with CONFIG_CPU_V6K use the v6 format (XN bit, AP[2:0]).
//!
//! The walk reads physical memory through a caller-supplied closure so it can
//! be used from any memory bridge.

/// Result of a successful translation.
#[derive(Clone, Copy, Debug)]
pub struct V6Translation {
    pub pa: u32,
}

/// Fault status (DFSR/IFSR FS encoding, short-descriptor format) + level.
#[derive(Clone, Copy, Debug)]
pub struct V6Fault {
    /// FS[3:0] fault status code (FS[4]=0 for these).
    pub fsr: u32,
    /// Domain number for domain faults (reported in DFSR[7:4]).
    pub domain: u32,
}

pub const FS_TRANSLATION_SECTION: u32 = 0b0101;
pub const FS_TRANSLATION_PAGE: u32 = 0b0111;
pub const FS_DOMAIN_SECTION: u32 = 0b1001;
pub const FS_DOMAIN_PAGE: u32 = 0b1011;
pub const FS_PERMISSION_SECTION: u32 = 0b1101;
pub const FS_PERMISSION_PAGE: u32 = 0b1111;

/// MMU configuration snapshot taken from CP15.
#[derive(Clone, Copy, Debug, Default)]
pub struct V6MmuConfig {
    pub enabled: bool,
    pub ttbr0: u32,
    pub ttbr1: u32,
    /// TTBCR.N: number of high bits steering to TTBR1 (0 = always TTBR0).
    pub ttbcr_n: u32,
    pub dacr: u32,
    /// SCTLR.AFE (access flag enable) — ARMv6K Linux leaves this 0.
    pub afe: bool,
}

/// Access type for permission checks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum V6Access {
    Read,
    Write,
    Execute,
}

#[inline]
fn ap_permits(ap2: u32, ap10: u32, privileged: bool, access: V6Access) -> bool {
    // v6 AP[2:0] model (SCTLR.AFE=0). AP2 = read-only flag.
    let write = access == V6Access::Write;
    match (ap2, ap10) {
        (0, 0b00) => false,                       // no access
        (0, 0b01) => privileged,                  // privileged RW
        (0, 0b10) => privileged || !write,        // priv RW, user RO
        (0, 0b11) => true,                        // full RW
        (1, 0b00) => false,                       // reserved
        (1, 0b01) => privileged && !write,        // privileged RO
        (1, 0b10) | (1, 0b11) => !write,          // read-only
        _ => false,
    }
}

/// Walk the short-descriptor tables. `read32` reads a 32-bit word of
/// PHYSICAL memory (table walk access).
pub fn translate_v6<F>(
    cfg: &V6MmuConfig,
    va: u32,
    privileged: bool,
    access: V6Access,
    mut read32: F,
) -> Result<V6Translation, V6Fault>
where
    F: FnMut(u32) -> Option<u32>,
{
    if !cfg.enabled {
        return Ok(V6Translation { pa: va });
    }

    // TTBR select: with TTBCR.N > 0, VAs with any of the top N bits set use
    // TTBR1, others (and N==0) use TTBR0. The TTBR0 table shrinks to
    // 16KB >> N.
    let (ttbr, n) = if cfg.ttbcr_n != 0 && (va >> (32 - cfg.ttbcr_n)) != 0 {
        (cfg.ttbr1, 0)
    } else {
        (cfg.ttbr0, cfg.ttbcr_n)
    };

    let table_base = ttbr & !(0x3FFFu32 >> n);
    let l1_index = (va << n) >> (n + 20);
    let l1_addr = table_base | (l1_index << 2);
    let l1 = read32(l1_addr).ok_or(V6Fault {
        fsr: FS_TRANSLATION_SECTION,
        domain: 0,
    })?;

    match l1 & 0x3 {
        0b00 => Err(V6Fault {
            fsr: FS_TRANSLATION_SECTION,
            domain: 0,
        }),
        0b10 => {
            // Section (1MB) or supersection (16MB).
            let domain = (l1 >> 5) & 0xF;
            match (cfg.dacr >> (domain * 2)) & 0x3 {
                0b00 | 0b10 => {
                    return Err(V6Fault {
                        fsr: FS_DOMAIN_SECTION,
                        domain,
                    });
                }
                0b11 => {} // manager: no permission check
                _ => {
                    // client: check AP
                    let ap2 = (l1 >> 15) & 1;
                    let ap10 = (l1 >> 10) & 0x3;
                    let xn = (l1 >> 4) & 1 == 1;
                    if (access == V6Access::Execute && xn)
                        || !ap_permits(ap2, ap10, privileged, access)
                    {
                        return Err(V6Fault {
                            fsr: FS_PERMISSION_SECTION,
                            domain,
                        });
                    }
                }
            }
            let pa = if (l1 >> 18) & 1 == 1 {
                // Supersection: 16MB, base in bits [31:24].
                (l1 & 0xFF00_0000) | (va & 0x00FF_FFFF)
            } else {
                (l1 & 0xFFF0_0000) | (va & 0x000F_FFFF)
            };
            Ok(V6Translation { pa })
        }
        _ => {
            // Coarse page table descriptor (01); 11 is reserved and treated
            // as a translation fault by real cores without PXN.
            if l1 & 0x3 == 0b11 {
                return Err(V6Fault {
                    fsr: FS_TRANSLATION_SECTION,
                    domain: 0,
                });
            }
            let domain = (l1 >> 5) & 0xF;
            let dac = (cfg.dacr >> (domain * 2)) & 0x3;
            if dac == 0b00 || dac == 0b10 {
                return Err(V6Fault {
                    fsr: FS_DOMAIN_PAGE,
                    domain,
                });
            }
            let l2_base = l1 & 0xFFFF_FC00;
            let l2_index = (va >> 12) & 0xFF;
            let l2_addr = l2_base | (l2_index << 2);
            let l2 = read32(l2_addr).ok_or(V6Fault {
                fsr: FS_TRANSLATION_PAGE,
                domain,
            })?;
            let (pa, xn, ap2, ap10) = match l2 & 0x3 {
                0b00 => {
                    return Err(V6Fault {
                        fsr: FS_TRANSLATION_PAGE,
                        domain,
                    });
                }
                0b01 => {
                    // Large page (64KB). XN in bit 15.
                    (
                        (l2 & 0xFFFF_0000) | (va & 0xFFFF),
                        (l2 >> 15) & 1 == 1,
                        (l2 >> 9) & 1,
                        (l2 >> 4) & 0x3,
                    )
                }
                _ => {
                    // Small page (4KB). XN in bit 0 (descriptor type 1x).
                    (
                        (l2 & 0xFFFF_F000) | (va & 0xFFF),
                        l2 & 1 == 1,
                        (l2 >> 9) & 1,
                        (l2 >> 4) & 0x3,
                    )
                }
            };
            if dac != 0b11 {
                if (access == V6Access::Execute && xn)
                    || !ap_permits(ap2, ap10, privileged, access)
                {
                    return Err(V6Fault {
                        fsr: FS_PERMISSION_PAGE,
                        domain,
                    });
                }
            }
            Ok(V6Translation { pa })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn mem(entries: &[(u32, u32)]) -> HashMap<u32, u32> {
        entries.iter().copied().collect()
    }

    #[test]
    fn disabled_is_identity() {
        let cfg = V6MmuConfig::default();
        let t = translate_v6(&cfg, 0x1234_5678, true, V6Access::Read, |_| None).unwrap();
        assert_eq!(t.pa, 0x1234_5678);
    }

    #[test]
    fn section_translation() {
        let cfg = V6MmuConfig {
            enabled: true,
            ttbr0: 0x5000_4000,
            dacr: 0x1, // domain 0 client
            ..Default::default()
        };
        // VA 0xC0000000 -> L1 index 0xC00 -> section to PA 0x50000000,
        // AP=01 (priv RW), domain 0.
        let l1e = 0x5000_0000u32 | (0b01 << 10) | 0b10;
        let m = mem(&[(0x5000_4000 + 0xC00 * 4, l1e)]);
        let t = translate_v6(&cfg, 0xC002_3456, true, V6Access::Read, |a| {
            m.get(&a).copied()
        })
        .unwrap();
        assert_eq!(t.pa, 0x5002_3456);
        // User access must fault (AP=01).
        let e = translate_v6(&cfg, 0xC002_3456, false, V6Access::Read, |a| {
            m.get(&a).copied()
        })
        .unwrap_err();
        assert_eq!(e.fsr, FS_PERMISSION_SECTION);
    }

    #[test]
    fn small_page_translation() {
        let cfg = V6MmuConfig {
            enabled: true,
            ttbr0: 0x5000_4000,
            dacr: 0x1,
            ..Default::default()
        };
        // VA 0x00010000: L1 index 0 -> coarse table at 0x50008000;
        // L2 index 0x10 -> small page at 0x50100000, AP=11 (full).
        let l1e = 0x5000_8000u32 | 0b01;
        let l2e = 0x5010_0000u32 | (0b11 << 4) | 0b10;
        let m = mem(&[(0x5000_4000, l1e), (0x5000_8000 + 0x10 * 4, l2e)]);
        let t = translate_v6(&cfg, 0x0001_0234, false, V6Access::Write, |a| {
            m.get(&a).copied()
        })
        .unwrap();
        assert_eq!(t.pa, 0x5010_0234);
        // Unmapped L2 entry -> page translation fault.
        let e = translate_v6(&cfg, 0x0002_0000, false, V6Access::Read, |a| {
            m.get(&a).copied()
        })
        .unwrap_err();
        assert_eq!(e.fsr, FS_TRANSLATION_PAGE);
    }
}
