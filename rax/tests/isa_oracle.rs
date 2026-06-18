use rax::isa_oracle::{
    ArmState, OracleIsa, OracleOptions, OracleSeed, RiscVIsaProfile, decode_to_json,
    decode_to_json_with_seed, parse_hex_bytes,
};
use rax::riscv::Xlen;

#[test]
fn parses_hex_bytes_with_prefixes_and_separators() {
    let bytes = parse_hex_bytes("0x90, 48-b8").unwrap();
    assert_eq!(bytes, vec![0x90, 0x48, 0xb8]);
}

#[test]
fn decodes_hexagon_packet() {
    let word = 0x5400c000u32.to_le_bytes();
    let mut opts = OracleOptions::default();
    opts.isa = OracleIsa::Hexagon;

    let value = decode_to_json(&word, &opts).unwrap();
    assert_eq!(value["isa"], "hexagon");
    assert_eq!(value["packet_flags"]["end_seen"], true);
    assert_eq!(value["decoded_ops"][0]["opcode"], "J2_trap0");
}

#[test]
fn decodes_riscv_instruction() {
    let mut opts = OracleOptions::default();
    opts.isa = OracleIsa::RiscV;
    opts.riscv_xlen = Xlen::Rv64;
    opts.riscv_isa = RiscVIsaProfile::Rv64Gc;

    let value = decode_to_json(&[0x93, 0x00, 0x10, 0x00], &opts).unwrap();
    assert_eq!(value["isa"], "riscv");
    assert_eq!(value["decoded_ops"][0]["op"], "Addi");
}

#[test]
fn decodes_arm_aarch64_instruction() {
    let mut opts = OracleOptions::default();
    opts.isa = OracleIsa::Arm;
    opts.arm_state = ArmState::Aarch64;

    let value = decode_to_json(&[0x20, 0x00, 0x80, 0xd2], &opts).unwrap();
    assert_eq!(value["isa"], "arm");
    assert_eq!(value["decoded_ops"][0]["mnemonic"], "movz");
}

#[test]
fn decodes_arm_aarch64_non_temporal_pairs() {
    let mut opts = OracleOptions::default();
    opts.isa = OracleIsa::Arm;
    opts.arm_state = ArmState::Aarch64;

    let cases = [
        (0x2840_0820u32, "ldnp"),
        (0xa840_0820u32, "ldnp"),
        (0x2800_0820u32, "stnp"),
        (0xa800_0820u32, "stnp"),
        (0x6940_0820u32, "ldpsw"),
        (0x6840_0820u32, "unknown"),
    ];

    for (raw, mnemonic) in cases {
        let value = decode_to_json(&raw.to_le_bytes(), &opts).unwrap();
        assert_eq!(value["decoded_ops"][0]["mnemonic"], mnemonic);
    }
}

#[test]
fn decodes_arm_aarch64_bti_hints() {
    let mut opts = OracleOptions::default();
    opts.isa = OracleIsa::Arm;
    opts.arm_state = ArmState::Aarch64;

    let bti = [
        0xd503_241fu32,
        0xd503_245fu32,
        0xd503_249fu32,
        0xd503_24dfu32,
    ];
    for raw in bti {
        let value = decode_to_json(&raw.to_le_bytes(), &opts).unwrap();
        assert_eq!(value["decoded_ops"][0]["mnemonic"], "bti");
    }

    for raw in [0xd503_231fu32, 0xd503_235fu32] {
        let value = decode_to_json(&raw.to_le_bytes(), &opts).unwrap();
        assert_eq!(value["decoded_ops"][0]["mnemonic"], "hint");
    }
}

#[test]
fn decodes_arm_aarch64_prfm_literal() {
    let mut opts = OracleOptions::default();
    opts.isa = OracleIsa::Arm;
    opts.arm_state = ArmState::Aarch64;

    let cases = [
        (0xd800_0000u32, "Prfop(PLDL1KEEP)", "Label(0)"),
        (0xd800_0075u32, "Prfop(PSTL3STRM)", "Label(12)"),
        (0xd8ff_ffffu32, "Prfop(Raw(31))", "Label(-4)"),
    ];

    for (raw, prfop, label) in cases {
        let value = decode_to_json(&raw.to_le_bytes(), &opts).unwrap();
        let op = &value["decoded_ops"][0];
        assert_eq!(op["mnemonic"], "prfm");
        assert_eq!(op["operands"][0], prfop);
        assert_eq!(op["operands"][1], label);
    }
}

#[test]
fn decodes_arm_aarch64_prfm_memory_forms() {
    let mut opts = OracleOptions::default();
    opts.isa = OracleIsa::Arm;
    opts.arm_state = ArmState::Aarch64;

    let cases = [
        (0xf980_0020u32, "Prfop(PLDL1KEEP)", "offset: Imm(0)"),
        (0xf980_0c35u32, "Prfop(PSTL3STRM)", "offset: Imm(24)"),
        (0xf89f_8022u32, "Prfop(PLDL2KEEP)", "offset: Imm(-8)"),
        (0xf8a2_5824u32, "Prfop(PLDL3KEEP)", "extend_type: UXTW"),
        (0xf8a2_e832u32, "Prfop(PSTL2KEEP)", "extend_type: SXTX"),
    ];

    for (raw, prfop, mem_fragment) in cases {
        let value = decode_to_json(&raw.to_le_bytes(), &opts).unwrap();
        let op = &value["decoded_ops"][0];
        assert_eq!(op["mnemonic"], "prfm");
        assert_eq!(op["operands"][0], prfop);
        let mem = op["operands"][1].as_str().unwrap();
        assert!(mem.contains(mem_fragment), "{mem}");
    }

    for raw in [
        0xf880_8420u32, // post-index prefetch pattern is undefined
        0xf880_8c20u32, // pre-index prefetch pattern is undefined
        0xf8a2_0820u32, // register-offset sub-word extend is undefined
        0xf9c0_0020u32, // size=11/opc=11 is undefined
    ] {
        let value = decode_to_json(&raw.to_le_bytes(), &opts).unwrap();
        assert_eq!(value["decoded_ops"][0]["mnemonic"], "unknown");
    }
}

#[test]
fn decodes_arm_aarch64_ldst_register_offset_forms() {
    let mut opts = OracleOptions::default();
    opts.isa = OracleIsa::Arm;
    opts.arm_state = ArmState::Aarch64;

    let cases = [
        (0xf862_5820u32, "ldr", "extend_type: UXTW", "shift: 3"),
        (0xb822_c820u32, "str", "extend_type: SXTW", "shift: 0"),
        (0x38a2_e820u32, "ldrsb", "extend_type: SXTX", "shift: 0"),
        (0x78e2_5820u32, "ldrsh", "extend_type: UXTW", "shift: 1"),
        (0xf840_8820u32, "ldr", "offset: Imm(8)", "mode: Offset"),
    ];

    for (raw, mnemonic, mem_fragment, shift_fragment) in cases {
        let value = decode_to_json(&raw.to_le_bytes(), &opts).unwrap();
        let op = &value["decoded_ops"][0];
        assert_eq!(op["mnemonic"], mnemonic);
        let mem = op["operands"][1].as_str().unwrap();
        assert!(mem.contains(mem_fragment), "{mem}");
        assert!(mem.contains(shift_fragment), "{mem}");
    }

    let value = decode_to_json(&0xf862_0820u32.to_le_bytes(), &opts).unwrap();
    assert_eq!(value["decoded_ops"][0]["mnemonic"], "unknown");
}

#[test]
fn decodes_x86_with_smir_lift() {
    let mut opts = OracleOptions::default();
    opts.isa = OracleIsa::X86_64;

    let value = decode_to_json(&[0x90], &opts).unwrap();
    assert_eq!(value["isa"], "x86_64");
    assert_eq!(value["smir"]["available"], true);
}

#[test]
fn emits_structured_smir_ops() {
    let mut opts = OracleOptions::default();
    opts.isa = OracleIsa::X86_64;

    let value = decode_to_json(&[0xb8, 0x34, 0x12, 0x00, 0x00], &opts).unwrap();
    let op = &value["smir"]["ops"][0];

    assert_eq!(op["opcode"], "mov");
    assert_eq!(op["kind"]["opcode"], "mov");
    assert_eq!(op["kind"]["dst"]["kind"], "arch");
    assert_eq!(op["kind"]["dst"]["name"], "rax");
    assert_eq!(op["kind"]["src"]["kind"], "imm");
    assert_eq!(op["kind"]["src"]["value"], 0x1234);
    assert!(op.get("debug").is_none());
}

#[test]
fn reports_seeded_side_effects() {
    let mut opts = OracleOptions::default();
    opts.isa = OracleIsa::X86_64;

    let seed = OracleSeed {
        regs: vec![("rax".to_string(), 0)],
        memory: vec![],
        memory_size: None,
    };
    let value =
        decode_to_json_with_seed(&[0xb8, 0x34, 0x12, 0x00, 0x00], &opts, Some(&seed)).unwrap();

    assert_eq!(value["side_effects"]["available"], true);
    assert_eq!(
        value["side_effects"]["changed_regs"]["rax"]["after"],
        "0x1234"
    );
}
