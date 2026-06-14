// Table-driven Hexagon opcode decoding.

include!("opcode_generated.rs");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldVal {
    pub value: u32,
    pub bits: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DecodedOp {
    pub opcode: Opcode,
    pub word: u32,
    pub class: EncClass,
    pub fields: &'static [FieldDesc],
}

impl DecodedOp {
    pub fn field(&self, letter: u8) -> Option<FieldVal> {
        for field in self.fields {
            if field.letter == letter {
                let value = gather_bits(self.word, field.bits);
                return Some(FieldVal {
                    value,
                    bits: field.bits.len() as u8,
                });
            }
        }
        None
    }
}

fn gather_bits(word: u32, positions: &[u8]) -> u32 {
    let mut value = 0u32;
    for &bit in positions {
        value <<= 1;
        value |= (word >> bit) & 1;
    }
    value
}

fn fields_slice(enc: &Encoding) -> &'static [FieldDesc] {
    if enc.fields_len == 0 {
        return &[];
    }
    let start = enc.fields_start as usize;
    let end = start + enc.fields_len as usize;
    &FIELD_POOL[start..end]
}

pub fn opcode_min_version(opcode: Opcode) -> u16 {
    OPCODE_MIN_VERSION[opcode as usize]
}

pub fn opcode_name(opcode: Opcode) -> &'static str {
    OPCODE_NAMES[opcode as usize]
}

pub fn decode_word(word: u32) -> Option<DecodedOp> {
    let iclass = (word >> 28) as usize;
    for enc in ENCODINGS_BY_ICLASS[iclass]
        .iter()
        .chain(ENCODINGS_MISC.iter())
    {
        if (word & enc.mask) == enc.value {
            return Some(DecodedOp {
                opcode: enc.opcode,
                word,
                class: enc.class,
                fields: fields_slice(enc),
            });
        }
    }
    None
}

pub fn decode_sub(word: u16, class: EncClass) -> Option<DecodedOp> {
    let word = word as u32;
    let encs = match class {
        EncClass::SubinsnA => SUBINSN_A,
        EncClass::SubinsnL1 => SUBINSN_L1,
        EncClass::SubinsnL2 => SUBINSN_L2,
        EncClass::SubinsnS1 => SUBINSN_S1,
        EncClass::SubinsnS2 => SUBINSN_S2,
        _ => return None,
    };
    for enc in encs {
        if (word & enc.mask) == enc.value {
            return Some(DecodedOp {
                opcode: enc.opcode,
                word,
                class: enc.class,
                fields: fields_slice(enc),
            });
        }
    }
    None
}
