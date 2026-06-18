use crate::common::TestCase;

// Specialized and vendor-specific system instructions

// ENQCMD/ENQCMDS - Enqueue command

#[test]
fn test_enqcmd_r32_m512() {
    TestCase::from("f2 0f 38 f8 00").check();
}

#[test]
fn test_enqcmd_r64_m512() {
    TestCase::from("f2 48 0f 38 f8 00").check();
}

#[test]
fn test_enqcmd_eax_mem() {
    TestCase::from("f2 0f 38 f8 00").check();
}

#[test]
fn test_enqcmd_rax_mem() {
    TestCase::from("f2 48 0f 38 f8 00").check();
}

#[test]
fn test_enqcmds_r32_m512() {
    TestCase::from("f3 0f 38 f8 00").check();
}

#[test]
fn test_enqcmds_r64_m512() {
    TestCase::from("f3 48 0f 38 f8 00").check();
}

#[test]
fn test_enqcmds_eax_mem() {
    TestCase::from("f3 0f 38 f8 00").check();
}

#[test]
fn test_enqcmds_rax_mem() {
    TestCase::from("f3 48 0f 38 f8 00").check();
}

// HRESET - History reset

#[test]
fn test_hreset_imm8() {
    TestCase::from("f3 0f 3a f0 c0 00").check();
}

#[test]
fn test_hreset_0() {
    TestCase::from("f3 0f 3a f0 c0 00").check();
}

#[test]
fn test_hreset_1() {
    TestCase::from("f3 0f 3a f0 c0 01").check();
}

#[test]
fn test_hreset_ff() {
    TestCase::from("f3 0f 3a f0 c0 ff").check();
}

// SENDUIPI - Send user interprocessor interrupt

#[test]
fn test_senduipi_r64() {
    TestCase::from("f3 0f c7 f0").check();
}

#[test]
fn test_senduipi_rax() {
    TestCase::from("f3 0f c7 f0").check();
}

#[test]
fn test_senduipi_rcx() {
    TestCase::from("f3 0f c7 f1").check();
}

#[test]
fn test_senduipi_rdx() {
    TestCase::from("f3 0f c7 f2").check();
}

#[test]
fn test_senduipi_rbx() {
    TestCase::from("f3 0f c7 f3").check();
}

// SMCTRL - SMM control

#[test]
fn test_smctrl() {
    TestCase::from("0f 01 fa").check();
}

// SWAPGS - Swap GS base register

#[test]
fn test_swapgs() {
    TestCase::from("0f 01 f8").check();
}

// TESTUI - Test user interrupt flag

#[test]
fn test_testui() {
    TestCase::from("f3 0f 01 ed").check();
}

// UIRET - User interrupt return

#[test]
fn test_uiret() {
    TestCase::from("f3 0f 01 ec").check();
}

// CLI - Clear interrupt flag

#[test]
fn test_cli() {
    TestCase::from("fa").check();
}

// INCSSPD/INCSSPQ - Increment shadow stack pointer

#[test]
fn test_incsspd_r32() {
    TestCase::from("f3 0f ae e8").check();
}

#[test]
fn test_incsspd_eax() {
    TestCase::from("f3 0f ae e8").check();
}

#[test]
fn test_incsspd_ecx() {
    TestCase::from("f3 0f ae e9").check();
}

#[test]
fn test_incsspq_r64() {
    TestCase::from("f3 48 0f ae e8").check();
}

#[test]
fn test_incsspq_rax() {
    TestCase::from("f3 48 0f ae e8").check();
}

#[test]
fn test_incsspq_rcx() {
    TestCase::from("f3 48 0f ae e9").check();
}

// RDSSPD/RDSSPQ - Read shadow stack pointer

#[test]
fn test_rdsspd_r32() {
    TestCase::from("f3 0f 1e c8").check();
}

#[test]
fn test_rdsspd_eax() {
    TestCase::from("f3 0f 1e c8").check();
}

#[test]
fn test_rdsspd_ecx() {
    TestCase::from("f3 0f 1e c9").check();
}

#[test]
fn test_rdsspq_r64() {
    TestCase::from("f3 48 0f 1e c8").check();
}

#[test]
fn test_rdsspq_rax() {
    TestCase::from("f3 48 0f 1e c8").check();
}

#[test]
fn test_rdsspq_rcx() {
    TestCase::from("f3 48 0f 1e c9").check();
}

// RDFSBASE/RDGSBASE - Read FS/GS base

#[test]
fn test_rdfsbase_r32() {
    TestCase::from("f3 0f ae c0").check();
}

#[test]
fn test_rdfsbase_r64() {
    TestCase::from("f3 48 0f ae c0").check();
}

#[test]
fn test_rdfsbase_eax() {
    TestCase::from("f3 0f ae c0").check();
}

#[test]
fn test_rdfsbase_rax() {
    TestCase::from("f3 48 0f ae c0").check();
}

#[test]
fn test_rdgsbase_r32() {
    TestCase::from("f3 0f ae c8").check();
}

#[test]
fn test_rdgsbase_r64() {
    TestCase::from("f3 48 0f ae c8").check();
}

#[test]
fn test_rdgsbase_eax() {
    TestCase::from("f3 0f ae c8").check();
}

#[test]
fn test_rdgsbase_rax() {
    TestCase::from("f3 48 0f ae c8").check();
}

// RSM - Resume from System Management Mode

#[test]
fn test_rsm() {
    TestCase::from("0f aa").check();
}
