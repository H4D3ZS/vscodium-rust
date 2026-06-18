// Disassemble a range of a raw binary using the RAX AArch32 decoder.
// args: <file> <base_hex> <start_vaddr_hex> <count> <arm|thumb>
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let data = std::fs::read(&a[1]).unwrap();
    let base = u32::from_str_radix(a[2].trim_start_matches("0x"), 16).unwrap();
    let start = u32::from_str_radix(a[3].trim_start_matches("0x"), 16).unwrap();
    let count: usize = a[4].parse().unwrap();
    let thumb = a.get(5).map(|s| s == "thumb").unwrap_or(true);
    let st = if thumb {
        rax::arm::ExecutionState::Thumb
    } else {
        rax::arm::ExecutionState::Aarch32
    };
    let mut d = rax::arm::decoder::Decoder::new(st);
    d.set_state(st);
    let mut va = start;
    for _ in 0..count {
        let off = (va - base) as usize;
        if off + 4 > data.len() {
            break;
        }
        let b = &data[off..off + 4];
        match d.decode(b) {
            Ok(i) => {
                let len = if thumb && (u16::from_le_bytes([b[0], b[1]]) >> 11) < 0x1D {
                    2
                } else {
                    4
                };
                let raw = if len == 2 {
                    format!("{:04x}", u16::from_le_bytes([b[0], b[1]]))
                } else {
                    format!("{:08x}", u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
                };
                println!("{:#010x}: {:<8} {:?} {:?}", va, raw, i.mnemonic, i.operands);
                va += len;
            }
            Err(_) => {
                println!("{:#010x}: ??", va);
                va += 2;
            }
        }
    }
}
