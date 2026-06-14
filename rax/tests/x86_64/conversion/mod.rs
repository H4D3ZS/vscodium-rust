// Data conversion instructions test module
//
// This module contains comprehensive tests for x86_64 data conversion instructions:
// - MOVSX: Move with Sign Extension (byte/word to word/dword/qword)
// - MOVSXD: Move with Sign-Extension (dword to qword, 35+ tests)
// - MOVZX: Move with Zero Extension
// - CBW/CWDE/CDQE: Sign-extension accumulator operations
// - CWD/CDQ/CQO: Convert Word/Doubleword/Quadword to Doubleword/Quadword/Octaword
// - CVTPI2PS/CVTPS2PI: Packed MMX integer <-> single precision FP conversions
// - CVTPI2PD/CVTPD2PI: Packed MMX integer <-> double precision FP conversions
// - CVTTPS2PI/CVTTPD2PI: Truncating packed FP to integer conversions
// - CVTSS2SI/CVTSD2SI: Scalar FP to signed integer conversions (extended tests)

mod cbw_cwde_cdqe;
mod cwd_cdq_cqo;
mod cvtpi2ps_cvtps2pi;
mod cvtpi2pd_cvtpd2pi;
mod cvttps2pi_cvttpd2pi;
mod cvtss2si_cvtsd2si_extended;
mod movsx;
mod movsxd;
mod movzx;
