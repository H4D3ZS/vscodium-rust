#![cfg(feature = "x86_64-suite")]

// Aggregated test modules for x86_64 instruction suites.
// Auto-generated - includes all test files

// Common utilities
#[path = "x86_64/common/mod.rs"]
mod common;

// Arithmetic
#[path = "x86_64/arithmetic/aaa_aas.rs"]
mod x86_64_arithmetic_aaa_aas;
#[path = "x86_64/arithmetic/aam_aad.rs"]
mod x86_64_arithmetic_aam_aad;
#[path = "x86_64/arithmetic/adc_extended.rs"]
mod x86_64_arithmetic_adc_extended;
#[path = "x86_64/arithmetic/adcx_adox.rs"]
mod x86_64_arithmetic_adcx_adox;
#[path = "x86_64/arithmetic/add_extended.rs"]
mod x86_64_arithmetic_add_extended;
#[path = "x86_64/arithmetic/bcd/aaa.rs"]
mod x86_64_arithmetic_bcd_aaa;
#[path = "x86_64/arithmetic/bcd/aad.rs"]
mod x86_64_arithmetic_bcd_aad;
#[path = "x86_64/arithmetic/bcd/aam.rs"]
mod x86_64_arithmetic_bcd_aam;
#[path = "x86_64/arithmetic/bcd/aas.rs"]
mod x86_64_arithmetic_bcd_aas;
#[path = "x86_64/arithmetic/bcd/daa.rs"]
mod x86_64_arithmetic_bcd_daa;
#[path = "x86_64/arithmetic/bcd/das.rs"]
mod x86_64_arithmetic_bcd_das;
#[path = "x86_64/arithmetic/cmp_extended.rs"]
mod x86_64_arithmetic_cmp_extended;
#[path = "x86_64/arithmetic/comparison/cmp.rs"]
mod x86_64_arithmetic_comparison_cmp;
#[path = "x86_64/arithmetic/comprehensive_arithmetic.rs"]
mod x86_64_arithmetic_comprehensive_arithmetic;
#[path = "x86_64/arithmetic/daa_das.rs"]
mod x86_64_arithmetic_daa_das;
#[path = "x86_64/arithmetic/div.rs"]
mod x86_64_arithmetic_div;
#[path = "x86_64/arithmetic/idiv.rs"]
mod x86_64_arithmetic_idiv;
#[path = "x86_64/arithmetic/imul.rs"]
mod x86_64_arithmetic_imul;
#[path = "x86_64/arithmetic/inc_dec.rs"]
mod x86_64_arithmetic_inc_dec;
#[path = "x86_64/arithmetic/integer_addition_carry/adc.rs"]
mod x86_64_arithmetic_integer_addition_carry_adc;
#[path = "x86_64/arithmetic/integer_addition_carry/add.rs"]
mod x86_64_arithmetic_integer_addition_carry_add;
#[path = "x86_64/arithmetic/integer_division/div.rs"]
mod x86_64_arithmetic_integer_division_div;
#[path = "x86_64/arithmetic/integer_division/idiv.rs"]
mod x86_64_arithmetic_integer_division_idiv;
#[path = "x86_64/arithmetic/integer_multiplication/imul.rs"]
mod x86_64_arithmetic_integer_multiplication_imul;
#[path = "x86_64/arithmetic/integer_multiplication/mul.rs"]
mod x86_64_arithmetic_integer_multiplication_mul;
#[path = "x86_64/arithmetic/integer_subtraction_base/sub.rs"]
mod x86_64_arithmetic_integer_subtraction_base_sub;
#[path = "x86_64/arithmetic/integer_subtraction/dec.rs"]
mod x86_64_arithmetic_integer_subtraction_dec;
#[path = "x86_64/arithmetic/integer_subtraction/inc.rs"]
mod x86_64_arithmetic_integer_subtraction_inc;
#[path = "x86_64/arithmetic/integer_subtraction/neg.rs"]
mod x86_64_arithmetic_integer_subtraction_neg;
#[path = "x86_64/arithmetic/integer_subtraction/sbb.rs"]
mod x86_64_arithmetic_integer_subtraction_sbb;
#[path = "x86_64/arithmetic/mul.rs"]
mod x86_64_arithmetic_mul;
#[path = "x86_64/arithmetic/neg.rs"]
mod x86_64_arithmetic_neg;
#[path = "x86_64/arithmetic/sbb_extended.rs"]
mod x86_64_arithmetic_sbb_extended;
#[path = "x86_64/arithmetic/sub_extended.rs"]
mod x86_64_arithmetic_sub_extended;

// Bcd
#[path = "x86_64/bcd/aam_aad.rs"]
mod x86_64_bcd_aam_aad;
#[path = "x86_64/bcd/daa_das.rs"]
mod x86_64_bcd_daa_das;

// Bit
#[path = "x86_64/bit/bt.rs"]
mod x86_64_bit_bt;
#[path = "x86_64/bit/btc.rs"]
mod x86_64_bit_btc;
#[path = "x86_64/bit/btr.rs"]
mod x86_64_bit_btr;
#[path = "x86_64/bit/bts.rs"]
mod x86_64_bit_bts;

// Bmi
#[path = "x86_64/bmi/andn.rs"]
mod x86_64_bmi_andn;
#[path = "x86_64/bmi/bextr.rs"]
mod x86_64_bmi_bextr;
#[path = "x86_64/bmi/blsi.rs"]
mod x86_64_bmi_blsi;
#[path = "x86_64/bmi/blsmsk.rs"]
mod x86_64_bmi_blsmsk;
#[path = "x86_64/bmi/blsr.rs"]
mod x86_64_bmi_blsr;
#[path = "x86_64/bmi/bmi2_extended.rs"]
mod x86_64_bmi_bmi2_extended;
#[path = "x86_64/bmi/bzhi_extended.rs"]
mod x86_64_bmi_bzhi_extended;
#[path = "x86_64/bmi/lzcnt.rs"]
mod x86_64_bmi_lzcnt;
#[path = "x86_64/bmi/mulx.rs"]
mod x86_64_bmi_mulx;
#[path = "x86_64/bmi/pdep.rs"]
mod x86_64_bmi_pdep;
#[path = "x86_64/bmi/pext.rs"]
mod x86_64_bmi_pext;
#[path = "x86_64/bmi/popcnt.rs"]
mod x86_64_bmi_popcnt;
#[path = "x86_64/bmi/rorx.rs"]
mod x86_64_bmi_rorx;
#[path = "x86_64/bmi/sarx_shlx_shrx.rs"]
mod x86_64_bmi_sarx_shlx_shrx;
#[path = "x86_64/bmi/sarx_shlx_shrx_extended.rs"]
mod x86_64_bmi_sarx_shlx_shrx_extended;
#[path = "x86_64/bmi/tbm_blcfill.rs"]
mod x86_64_bmi_tbm_blcfill;
#[path = "x86_64/bmi/tbm_blci.rs"]
mod x86_64_bmi_tbm_blci;
#[path = "x86_64/bmi/tbm_blcs_blsfill_blsic_t1mskc_tzmsk.rs"]
mod x86_64_bmi_tbm_blcs_blsfill_blsic_t1mskc_tzmsk;
#[path = "x86_64/bmi/tzcnt.rs"]
mod x86_64_bmi_tzcnt;

// Control Flow
#[path = "x86_64/control_flow/bound_extended.rs"]
mod x86_64_control_flow_bound_extended;
#[path = "x86_64/control_flow/call_ret/call.rs"]
mod x86_64_control_flow_call_ret_call;
#[path = "x86_64/control_flow/call_ret/ret.rs"]
mod x86_64_control_flow_call_ret_ret;
#[path = "x86_64/control_flow/call_return/call.rs"]
mod x86_64_control_flow_call_return_call;
#[path = "x86_64/control_flow/call_return/ret.rs"]
mod x86_64_control_flow_call_return_ret;
#[path = "x86_64/control_flow/conditional_jump/ja.rs"]
mod x86_64_control_flow_conditional_jump_ja;
#[path = "x86_64/control_flow/conditional_jump/jae.rs"]
mod x86_64_control_flow_conditional_jump_jae;
#[path = "x86_64/control_flow/conditional_jump/jb.rs"]
mod x86_64_control_flow_conditional_jump_jb;
#[path = "x86_64/control_flow/conditional_jump/jbe.rs"]
mod x86_64_control_flow_conditional_jump_jbe;
#[path = "x86_64/control_flow/conditional_jump/je.rs"]
mod x86_64_control_flow_conditional_jump_je;
#[path = "x86_64/control_flow/conditional_jump/jg.rs"]
mod x86_64_control_flow_conditional_jump_jg;
#[path = "x86_64/control_flow/conditional_jump/jge.rs"]
mod x86_64_control_flow_conditional_jump_jge;
#[path = "x86_64/control_flow/conditional_jump/jl.rs"]
mod x86_64_control_flow_conditional_jump_jl;
#[path = "x86_64/control_flow/conditional_jump/jle.rs"]
mod x86_64_control_flow_conditional_jump_jle;
#[path = "x86_64/control_flow/conditional_jump/jne.rs"]
mod x86_64_control_flow_conditional_jump_jne;
#[path = "x86_64/control_flow/conditional_jump/jno.rs"]
mod x86_64_control_flow_conditional_jump_jno;
#[path = "x86_64/control_flow/conditional_jump/jnp.rs"]
mod x86_64_control_flow_conditional_jump_jnp;
#[path = "x86_64/control_flow/conditional_jump/jns.rs"]
mod x86_64_control_flow_conditional_jump_jns;
#[path = "x86_64/control_flow/conditional_jump/jo.rs"]
mod x86_64_control_flow_conditional_jump_jo;
#[path = "x86_64/control_flow/conditional_jump/jp.rs"]
mod x86_64_control_flow_conditional_jump_jp;
#[path = "x86_64/control_flow/conditional_jump/js.rs"]
mod x86_64_control_flow_conditional_jump_js;
#[path = "x86_64/control_flow/far_call.rs"]
mod x86_64_control_flow_far_call;
#[path = "x86_64/control_flow/far_jmp.rs"]
mod x86_64_control_flow_far_jmp;
#[path = "x86_64/control_flow/far_ret.rs"]
mod x86_64_control_flow_far_ret;
#[path = "x86_64/control_flow/int_into_int3.rs"]
mod x86_64_control_flow_int_into_int3;
#[path = "x86_64/control_flow/iret_iretd_iretq.rs"]
mod x86_64_control_flow_iret_iretd_iretq;
#[path = "x86_64/control_flow/jcc_all.rs"]
mod x86_64_control_flow_jcc_all;
#[path = "x86_64/control_flow/jecxz_jrcxz.rs"]
mod x86_64_control_flow_jecxz_jrcxz;
#[path = "x86_64/control_flow/jump/jmp.rs"]
mod x86_64_control_flow_jump_jmp;
#[path = "x86_64/control_flow/loop.rs"]
mod x86_64_control_flow_loop;
#[path = "x86_64/control_flow/loop/loop.rs"]
mod x86_64_control_flow_loop_loop;
#[path = "x86_64/control_flow/loop/loope.rs"]
mod x86_64_control_flow_loop_loope;
#[path = "x86_64/control_flow/loop/loopne.rs"]
mod x86_64_control_flow_loop_loopne;
#[path = "x86_64/control_flow/syscall_sysret.rs"]
mod x86_64_control_flow_syscall_sysret;
#[path = "x86_64/control_flow/sysenter_sysexit.rs"]
mod x86_64_control_flow_sysenter_sysexit;
#[path = "x86_64/control_flow/unconditional_jump/jmp.rs"]
mod x86_64_control_flow_unconditional_jump_jmp;

// Conversion
#[path = "x86_64/conversion/cbw_cwde_cdqe.rs"]
mod x86_64_conversion_cbw_cwde_cdqe;
#[path = "x86_64/conversion/cvtpi2pd_cvtpd2pi.rs"]
mod x86_64_conversion_cvtpi2pd_cvtpd2pi;
#[path = "x86_64/conversion/cvtpi2ps_cvtps2pi.rs"]
mod x86_64_conversion_cvtpi2ps_cvtps2pi;
#[path = "x86_64/conversion/cvtss2si_cvtsd2si_extended.rs"]
mod x86_64_conversion_cvtss2si_cvtsd2si_extended;
#[path = "x86_64/conversion/cvttps2pi_cvttpd2pi.rs"]
mod x86_64_conversion_cvttps2pi_cvttpd2pi;
#[path = "x86_64/conversion/cwd_cdq_cqo.rs"]
mod x86_64_conversion_cwd_cdq_cqo;
#[path = "x86_64/conversion/movsx.rs"]
mod x86_64_conversion_movsx;
#[path = "x86_64/conversion/movsxd.rs"]
mod x86_64_conversion_movsxd;
#[path = "x86_64/conversion/movzx.rs"]
mod x86_64_conversion_movzx;

// Crypto
#[path = "x86_64/crypto/aes_keylocker.rs"]
mod x86_64_crypto_aes_keylocker;
#[path = "x86_64/crypto/aesdec.rs"]
mod x86_64_crypto_aesdec;
#[path = "x86_64/crypto/aesdeclast.rs"]
mod x86_64_crypto_aesdeclast;
#[path = "x86_64/crypto/aesenc.rs"]
mod x86_64_crypto_aesenc;
#[path = "x86_64/crypto/aesenclast.rs"]
mod x86_64_crypto_aesenclast;
#[path = "x86_64/crypto/aesimc.rs"]
mod x86_64_crypto_aesimc;
#[path = "x86_64/crypto/aeskeygenassist.rs"]
mod x86_64_crypto_aeskeygenassist;
#[path = "x86_64/crypto/galois_field.rs"]
mod x86_64_crypto_galois_field;
#[path = "x86_64/crypto/gf2p8.rs"]
mod x86_64_crypto_gf2p8;
#[path = "x86_64/crypto/pclmulqdq.rs"]
mod x86_64_crypto_pclmulqdq;
#[path = "x86_64/crypto/sha1msg1.rs"]
mod x86_64_crypto_sha1msg1;
#[path = "x86_64/crypto/sha1msg2.rs"]
mod x86_64_crypto_sha1msg2;
#[path = "x86_64/crypto/sha1nexte.rs"]
mod x86_64_crypto_sha1nexte;
#[path = "x86_64/crypto/sha1rnds4.rs"]
mod x86_64_crypto_sha1rnds4;
#[path = "x86_64/crypto/sha256msg1.rs"]
mod x86_64_crypto_sha256msg1;
#[path = "x86_64/crypto/sha256msg2.rs"]
mod x86_64_crypto_sha256msg2;
#[path = "x86_64/crypto/sha256rnds2.rs"]
mod x86_64_crypto_sha256rnds2;

// Data Movement
#[path = "x86_64/data_movement/basic_move/mov.rs"]
mod x86_64_data_movement_basic_move_mov;
#[path = "x86_64/data_movement/compare_exchange/cmpxchg.rs"]
mod x86_64_data_movement_compare_exchange_cmpxchg;
#[path = "x86_64/data_movement/conditional_move/cmova.rs"]
mod x86_64_data_movement_conditional_move_cmova;
#[path = "x86_64/data_movement/conditional_move/cmovae.rs"]
mod x86_64_data_movement_conditional_move_cmovae;
#[path = "x86_64/data_movement/conditional_move/cmovb.rs"]
mod x86_64_data_movement_conditional_move_cmovb;
#[path = "x86_64/data_movement/conditional_move/cmovbe.rs"]
mod x86_64_data_movement_conditional_move_cmovbe;
#[path = "x86_64/data_movement/conditional_move/cmove.rs"]
mod x86_64_data_movement_conditional_move_cmove;
#[path = "x86_64/data_movement/conditional_move/cmovg.rs"]
mod x86_64_data_movement_conditional_move_cmovg;
#[path = "x86_64/data_movement/conditional_move/cmovl.rs"]
mod x86_64_data_movement_conditional_move_cmovl;
#[path = "x86_64/data_movement/conditional_move/cmovne.rs"]
mod x86_64_data_movement_conditional_move_cmovne;
#[path = "x86_64/data_movement/conditional_move/cmovs.rs"]
mod x86_64_data_movement_conditional_move_cmovs;
#[path = "x86_64/data_movement/exchange_add/xadd.rs"]
mod x86_64_data_movement_exchange_add_xadd;
#[path = "x86_64/data_movement/exchange/xchg.rs"]
mod x86_64_data_movement_exchange_xchg;
#[path = "x86_64/data_movement/extend_move/movsx.rs"]
mod x86_64_data_movement_extend_move_movsx;
#[path = "x86_64/data_movement/extend_move/movzx.rs"]
mod x86_64_data_movement_extend_move_movzx;
#[path = "x86_64/data_movement/lea/lea.rs"]
mod x86_64_data_movement_lea_lea;

// Data Transfer
#[path = "x86_64/data_transfer/bswap.rs"]
mod x86_64_data_transfer_bswap;
#[path = "x86_64/data_transfer/cdqe_cqo_extended.rs"]
mod x86_64_data_transfer_cdqe_cqo_extended;
#[path = "x86_64/data_transfer/cmov.rs"]
mod x86_64_data_transfer_cmov;
#[path = "x86_64/data_transfer/lahf_sahf_extended.rs"]
mod x86_64_data_transfer_lahf_sahf_extended;
#[path = "x86_64/data_transfer/lea.rs"]
mod x86_64_data_transfer_lea;
#[path = "x86_64/data_transfer/mov_extended.rs"]
mod x86_64_data_transfer_mov_extended;
#[path = "x86_64/data_transfer/movbe.rs"]
mod x86_64_data_transfer_movbe;
#[path = "x86_64/data_transfer/movdir64b.rs"]
mod x86_64_data_transfer_movdir64b;
#[path = "x86_64/data_transfer/movdiri.rs"]
mod x86_64_data_transfer_movdiri;
#[path = "x86_64/data_transfer/movsx_extended.rs"]
mod x86_64_data_transfer_movsx_extended;
#[path = "x86_64/data_transfer/movzx_extended.rs"]
mod x86_64_data_transfer_movzx_extended;
#[path = "x86_64/data_transfer/pop_extended.rs"]
mod x86_64_data_transfer_pop_extended;
#[path = "x86_64/data_transfer/push_extended.rs"]
mod x86_64_data_transfer_push_extended;
#[path = "x86_64/data_transfer/pushad_popad.rs"]
mod x86_64_data_transfer_pushad_popad;
#[path = "x86_64/data_transfer/setcc.rs"]
mod x86_64_data_transfer_setcc;
#[path = "x86_64/data_transfer/xchg.rs"]
mod x86_64_data_transfer_xchg;

// Flags
#[path = "x86_64/flags/clc_stc_cmc.rs"]
mod x86_64_flags_clc_stc_cmc;
#[path = "x86_64/flags/cld_std.rs"]
mod x86_64_flags_cld_std;
#[path = "x86_64/flags/lahf_sahf.rs"]
mod x86_64_flags_lahf_sahf;
#[path = "x86_64/flags/pushf_popf.rs"]
mod x86_64_flags_pushf_popf;

// Fpu
#[path = "x86_64/fpu/arithmetic_variants.rs"]
mod x86_64_fpu_arithmetic_variants;
#[path = "x86_64/fpu/comparison_control.rs"]
mod x86_64_fpu_comparison_control;
#[path = "x86_64/fpu/f2xm1.rs"]
mod x86_64_fpu_f2xm1;
#[path = "x86_64/fpu/fabs.rs"]
mod x86_64_fpu_fabs;
#[path = "x86_64/fpu/fadd.rs"]
mod x86_64_fpu_fadd;
#[path = "x86_64/fpu/faddp_fsubp_fmulp_fdivp.rs"]
mod x86_64_fpu_faddp_fsubp_fmulp_fdivp;
#[path = "x86_64/fpu/fbld_fbstp.rs"]
mod x86_64_fpu_fbld_fbstp;
#[path = "x86_64/fpu/fchs.rs"]
mod x86_64_fpu_fchs;
#[path = "x86_64/fpu/fclex_fnclex.rs"]
mod x86_64_fpu_fclex_fnclex;
#[path = "x86_64/fpu/fcmovcc.rs"]
mod x86_64_fpu_fcmovcc;
#[path = "x86_64/fpu/fcom.rs"]
mod x86_64_fpu_fcom;
#[path = "x86_64/fpu/fcomi_fcomip.rs"]
mod x86_64_fpu_fcomi_fcomip;
#[path = "x86_64/fpu/fcompp.rs"]
mod x86_64_fpu_fcompp;
#[path = "x86_64/fpu/fcos.rs"]
mod x86_64_fpu_fcos;
#[path = "x86_64/fpu/fdiv.rs"]
mod x86_64_fpu_fdiv;
#[path = "x86_64/fpu/ffree.rs"]
mod x86_64_fpu_ffree;
#[path = "x86_64/fpu/fiadd.rs"]
mod x86_64_fpu_fiadd;
#[path = "x86_64/fpu/ficom_ficomp.rs"]
mod x86_64_fpu_ficom_ficomp;
#[path = "x86_64/fpu/fidiv.rs"]
mod x86_64_fpu_fidiv;
#[path = "x86_64/fpu/fidivr.rs"]
mod x86_64_fpu_fidivr;
#[path = "x86_64/fpu/fild.rs"]
mod x86_64_fpu_fild;
#[path = "x86_64/fpu/fimul.rs"]
mod x86_64_fpu_fimul;
#[path = "x86_64/fpu/fincstp_fdecstp.rs"]
mod x86_64_fpu_fincstp_fdecstp;
#[path = "x86_64/fpu/finit_fninit.rs"]
mod x86_64_fpu_finit_fninit;
#[path = "x86_64/fpu/fist_fistp.rs"]
mod x86_64_fpu_fist_fistp;
#[path = "x86_64/fpu/fisttp.rs"]
mod x86_64_fpu_fisttp;
#[path = "x86_64/fpu/fisub.rs"]
mod x86_64_fpu_fisub;
#[path = "x86_64/fpu/fisubr.rs"]
mod x86_64_fpu_fisubr;
#[path = "x86_64/fpu/fld.rs"]
mod x86_64_fpu_fld;
#[path = "x86_64/fpu/fld_constants.rs"]
mod x86_64_fpu_fld_constants;
#[path = "x86_64/fpu/fldcw_fstcw.rs"]
mod x86_64_fpu_fldcw_fstcw;
#[path = "x86_64/fpu/fldenv_fstenv.rs"]
mod x86_64_fpu_fldenv_fstenv;
#[path = "x86_64/fpu/fmul.rs"]
mod x86_64_fpu_fmul;
#[path = "x86_64/fpu/fninit_extended.rs"]
mod x86_64_fpu_fninit_extended;
#[path = "x86_64/fpu/fnop.rs"]
mod x86_64_fpu_fnop;
#[path = "x86_64/fpu/fnsave_fnop.rs"]
mod x86_64_fpu_fnsave_fnop;
#[path = "x86_64/fpu/fpatan.rs"]
mod x86_64_fpu_fpatan;
#[path = "x86_64/fpu/fprem.rs"]
mod x86_64_fpu_fprem;
#[path = "x86_64/fpu/fprem1.rs"]
mod x86_64_fpu_fprem1;
#[path = "x86_64/fpu/fptan.rs"]
mod x86_64_fpu_fptan;
#[path = "x86_64/fpu/frndint.rs"]
mod x86_64_fpu_frndint;
#[path = "x86_64/fpu/frndint_extended.rs"]
mod x86_64_fpu_frndint_extended;
#[path = "x86_64/fpu/fsave_frstor.rs"]
mod x86_64_fpu_fsave_frstor;
#[path = "x86_64/fpu/fscale.rs"]
mod x86_64_fpu_fscale;
#[path = "x86_64/fpu/fsin_fcos.rs"]
mod x86_64_fpu_fsin_fcos;
#[path = "x86_64/fpu/fsincos.rs"]
mod x86_64_fpu_fsincos;
#[path = "x86_64/fpu/fsqrt.rs"]
mod x86_64_fpu_fsqrt;
#[path = "x86_64/fpu/fst_fstp.rs"]
mod x86_64_fpu_fst_fstp;
#[path = "x86_64/fpu/fstenv_fnstenv.rs"]
mod x86_64_fpu_fstenv_fnstenv;
#[path = "x86_64/fpu/fstsw_fnstsw.rs"]
mod x86_64_fpu_fstsw_fnstsw;
#[path = "x86_64/fpu/fsub.rs"]
mod x86_64_fpu_fsub;
#[path = "x86_64/fpu/ftst.rs"]
mod x86_64_fpu_ftst;
#[path = "x86_64/fpu/fucom_fucomp_fucompp.rs"]
mod x86_64_fpu_fucom_fucomp_fucompp;
#[path = "x86_64/fpu/fucomi_fucomip.rs"]
mod x86_64_fpu_fucomi_fucomip;
#[path = "x86_64/fpu/fxam.rs"]
mod x86_64_fpu_fxam;
#[path = "x86_64/fpu/fxch.rs"]
mod x86_64_fpu_fxch;
#[path = "x86_64/fpu/fxsave64_fxrstor64.rs"]
mod x86_64_fpu_fxsave64_fxrstor64;
#[path = "x86_64/fpu/fxsave_fxrstor.rs"]
mod x86_64_fpu_fxsave_fxrstor;
#[path = "x86_64/fpu/fxtract.rs"]
mod x86_64_fpu_fxtract;
#[path = "x86_64/fpu/fyl2x.rs"]
mod x86_64_fpu_fyl2x;
#[path = "x86_64/fpu/fyl2xp1.rs"]
mod x86_64_fpu_fyl2xp1;

// Io
#[path = "x86_64/io/in.rs"]
mod x86_64_io_in;
#[path = "x86_64/io/in_out.rs"]
mod x86_64_io_in_out;
#[path = "x86_64/io/ins.rs"]
mod x86_64_io_ins;
#[path = "x86_64/io/ins_outs.rs"]
mod x86_64_io_ins_outs;
#[path = "x86_64/io/out.rs"]
mod x86_64_io_out;
#[path = "x86_64/io/outs.rs"]
mod x86_64_io_outs;
#[path = "x86_64/io/pit.rs"]
mod x86_64_io_pit;
#[path = "x86_64/io/serial.rs"]
mod x86_64_io_serial;

// Logic And Bit Manipulation
#[path = "x86_64/logic_and_bit_manipulation/basic_logic/and.rs"]
mod x86_64_logic_and_bit_manipulation_basic_logic_and;
#[path = "x86_64/logic_and_bit_manipulation/basic_logic/not.rs"]
mod x86_64_logic_and_bit_manipulation_basic_logic_not;
#[path = "x86_64/logic_and_bit_manipulation/basic_logic/or.rs"]
mod x86_64_logic_and_bit_manipulation_basic_logic_or;
#[path = "x86_64/logic_and_bit_manipulation/basic_logic/test.rs"]
mod x86_64_logic_and_bit_manipulation_basic_logic_test;
#[path = "x86_64/logic_and_bit_manipulation/basic_logic_xor/xor.rs"]
mod x86_64_logic_and_bit_manipulation_basic_logic_xor_xor;
#[path = "x86_64/logic_and_bit_manipulation/bit_counting_swap/bswap.rs"]
mod x86_64_logic_and_bit_manipulation_bit_counting_swap_bswap;
#[path = "x86_64/logic_and_bit_manipulation/bit_counting_swap/lzcnt.rs"]
mod x86_64_logic_and_bit_manipulation_bit_counting_swap_lzcnt;
#[path = "x86_64/logic_and_bit_manipulation/bit_counting_swap/tzcnt.rs"]
mod x86_64_logic_and_bit_manipulation_bit_counting_swap_tzcnt;
#[path = "x86_64/logic_and_bit_manipulation/bit_scanning/bsf.rs"]
mod x86_64_logic_and_bit_manipulation_bit_scanning_bsf;
#[path = "x86_64/logic_and_bit_manipulation/bit_scanning/bsr.rs"]
mod x86_64_logic_and_bit_manipulation_bit_scanning_bsr;
#[path = "x86_64/logic_and_bit_manipulation/bit_testing/bt.rs"]
mod x86_64_logic_and_bit_manipulation_bit_testing_bt;
#[path = "x86_64/logic_and_bit_manipulation/bit_testing/btc.rs"]
mod x86_64_logic_and_bit_manipulation_bit_testing_btc;
#[path = "x86_64/logic_and_bit_manipulation/bit_testing/btr.rs"]
mod x86_64_logic_and_bit_manipulation_bit_testing_btr;
#[path = "x86_64/logic_and_bit_manipulation/bit_testing/bts.rs"]
mod x86_64_logic_and_bit_manipulation_bit_testing_bts;
#[path = "x86_64/logic_and_bit_manipulation/bmi1/bextr.rs"]
mod x86_64_logic_and_bit_manipulation_bmi1_bextr;
#[path = "x86_64/logic_and_bit_manipulation/bmi1/blsi.rs"]
mod x86_64_logic_and_bit_manipulation_bmi1_blsi;
#[path = "x86_64/logic_and_bit_manipulation/bmi1/blsmsk.rs"]
mod x86_64_logic_and_bit_manipulation_bmi1_blsmsk;
#[path = "x86_64/logic_and_bit_manipulation/bmi1/blsr.rs"]
mod x86_64_logic_and_bit_manipulation_bmi1_blsr;
#[path = "x86_64/logic_and_bit_manipulation/bmi2/bzhi.rs"]
mod x86_64_logic_and_bit_manipulation_bmi2_bzhi;
#[path = "x86_64/logic_and_bit_manipulation/bmi2/pdep.rs"]
mod x86_64_logic_and_bit_manipulation_bmi2_pdep;
#[path = "x86_64/logic_and_bit_manipulation/bmi2/pext.rs"]
mod x86_64_logic_and_bit_manipulation_bmi2_pext;
#[path = "x86_64/logic_and_bit_manipulation/rotates_advanced/rorx.rs"]
mod x86_64_logic_and_bit_manipulation_rotates_advanced_rorx;
#[path = "x86_64/logic_and_bit_manipulation/rotates_basic/rcl.rs"]
mod x86_64_logic_and_bit_manipulation_rotates_basic_rcl;
#[path = "x86_64/logic_and_bit_manipulation/rotates_basic/rcr.rs"]
mod x86_64_logic_and_bit_manipulation_rotates_basic_rcr;
#[path = "x86_64/logic_and_bit_manipulation/rotates_basic/rol.rs"]
mod x86_64_logic_and_bit_manipulation_rotates_basic_rol;
#[path = "x86_64/logic_and_bit_manipulation/rotates_basic/ror.rs"]
mod x86_64_logic_and_bit_manipulation_rotates_basic_ror;
#[path = "x86_64/logic_and_bit_manipulation/shifts_arithmetic/sar.rs"]
mod x86_64_logic_and_bit_manipulation_shifts_arithmetic_sar;
#[path = "x86_64/logic_and_bit_manipulation/shifts_double_precision/shld.rs"]
mod x86_64_logic_and_bit_manipulation_shifts_double_precision_shld;
#[path = "x86_64/logic_and_bit_manipulation/shifts_double_precision/shrd.rs"]
mod x86_64_logic_and_bit_manipulation_shifts_double_precision_shrd;
#[path = "x86_64/logic_and_bit_manipulation/shifts_logical/shl.rs"]
mod x86_64_logic_and_bit_manipulation_shifts_logical_shl;
#[path = "x86_64/logic_and_bit_manipulation/shifts_logical/shr.rs"]
mod x86_64_logic_and_bit_manipulation_shifts_logical_shr;
#[path = "x86_64/logic_and_bit_manipulation/shifts_variable/sarx.rs"]
mod x86_64_logic_and_bit_manipulation_shifts_variable_sarx;
#[path = "x86_64/logic_and_bit_manipulation/shifts_variable/shlx.rs"]
mod x86_64_logic_and_bit_manipulation_shifts_variable_shlx;
#[path = "x86_64/logic_and_bit_manipulation/shifts_variable/shrx.rs"]
mod x86_64_logic_and_bit_manipulation_shifts_variable_shrx;

// Logical
#[path = "x86_64/logical/and.rs"]
mod x86_64_logical_and;
#[path = "x86_64/logical/not.rs"]
mod x86_64_logical_not;
#[path = "x86_64/logical/or.rs"]
mod x86_64_logical_or;
#[path = "x86_64/logical/sar.rs"]
mod x86_64_logical_sar;
#[path = "x86_64/logical/shl_sal.rs"]
mod x86_64_logical_shl_sal;
#[path = "x86_64/logical/shr.rs"]
mod x86_64_logical_shr;
#[path = "x86_64/logical/test.rs"]
mod x86_64_logical_test;
#[path = "x86_64/logical/xor.rs"]
mod x86_64_logical_xor;

// Memory
#[path = "x86_64/memory/bound.rs"]
mod x86_64_memory_bound;
#[path = "x86_64/memory/enter_leave.rs"]
mod x86_64_memory_enter_leave;
#[path = "x86_64/memory/mpx.rs"]
mod x86_64_memory_mpx;

// Misc
#[path = "x86_64/misc/cldemote.rs"]
mod x86_64_misc_cldemote;
#[path = "x86_64/misc/clflush.rs"]
mod x86_64_misc_clflush;
#[path = "x86_64/misc/clflush_extended.rs"]
mod x86_64_misc_clflush_extended;
#[path = "x86_64/misc/clwb.rs"]
mod x86_64_misc_clwb;
#[path = "x86_64/misc/cpuid_extended.rs"]
mod x86_64_misc_cpuid_extended;
#[path = "x86_64/misc/crc32.rs"]
mod x86_64_misc_crc32;
#[path = "x86_64/misc/endbr32_endbr64.rs"]
mod x86_64_misc_endbr32_endbr64;
#[path = "x86_64/misc/hlt.rs"]
mod x86_64_misc_hlt;
#[path = "x86_64/misc/lahf_sahf_extended.rs"]
mod x86_64_misc_lahf_sahf_extended;
#[path = "x86_64/misc/legacy_instructions.rs"]
mod x86_64_misc_legacy_instructions;
#[path = "x86_64/misc/lock.rs"]
mod x86_64_misc_lock;
#[path = "x86_64/misc/monitor_mwait.rs"]
mod x86_64_misc_monitor_mwait;
#[path = "x86_64/misc/movbe_extended.rs"]
mod x86_64_misc_movbe_extended;
#[path = "x86_64/misc/movdir64b_extended.rs"]
mod x86_64_misc_movdir64b_extended;
#[path = "x86_64/misc/movdiri_extended.rs"]
mod x86_64_misc_movdiri_extended;
#[path = "x86_64/misc/nop.rs"]
mod x86_64_misc_nop;
#[path = "x86_64/misc/nop_variants.rs"]
mod x86_64_misc_nop_variants;
#[path = "x86_64/misc/pause.rs"]
mod x86_64_misc_pause;
#[path = "x86_64/misc/prefetch.rs"]
mod x86_64_misc_prefetch;
#[path = "x86_64/misc/prefetchw_prefetchwt1.rs"]
mod x86_64_misc_prefetchw_prefetchwt1;
#[path = "x86_64/misc/rdrand_extended.rs"]
mod x86_64_misc_rdrand_extended;
#[path = "x86_64/misc/rdseed_extended.rs"]
mod x86_64_misc_rdseed_extended;
#[path = "x86_64/misc/tpause_umonitor_umwait.rs"]
mod x86_64_misc_tpause_umonitor_umwait;
#[path = "x86_64/misc/ud.rs"]
mod x86_64_misc_ud;
#[path = "x86_64/misc/wait_fwait.rs"]
mod x86_64_misc_wait_fwait;
#[path = "x86_64/misc/xgetbv_xsetbv.rs"]
mod x86_64_misc_xgetbv_xsetbv;
#[path = "x86_64/misc/xlat.rs"]
mod x86_64_misc_xlat;
#[path = "x86_64/misc/xsave_xrstor.rs"]
mod x86_64_misc_xsave_xrstor;

// Rotate
#[path = "x86_64/rotate/rcl.rs"]
mod x86_64_rotate_rcl;
#[path = "x86_64/rotate/rcr.rs"]
mod x86_64_rotate_rcr;
#[path = "x86_64/rotate/rol.rs"]
mod x86_64_rotate_rol;
#[path = "x86_64/rotate/rol_ror_extended.rs"]
mod x86_64_rotate_rol_ror_extended;
#[path = "x86_64/rotate/ror.rs"]
mod x86_64_rotate_ror;
#[path = "x86_64/rotate/shld.rs"]
mod x86_64_rotate_shld;
#[path = "x86_64/rotate/shld_shrd_extended.rs"]
mod x86_64_rotate_shld_shrd_extended;
#[path = "x86_64/rotate/shrd.rs"]
mod x86_64_rotate_shrd;

// Segment
#[path = "x86_64/segment/mov_segment.rs"]
mod x86_64_segment_mov_segment;
#[path = "x86_64/segment/push_pop_segment.rs"]
mod x86_64_segment_push_pop_segment;

// Simd
#[path = "x86_64/simd/avx512/vcompress_vexpand.rs"]
mod simd_avx512_compress_expand;
#[path = "x86_64/simd/avx2/vbroadcasti128.rs"]
mod x86_64_simd_avx2_vbroadcasti128;
#[path = "x86_64/simd/avx2/vextracti128.rs"]
mod x86_64_simd_avx2_vextracti128;
#[path = "x86_64/simd/avx2/vgatherdps_vgatherdpd.rs"]
mod x86_64_simd_avx2_vgatherdps_vgatherdpd;
#[path = "x86_64/simd/avx2/vgatherqps_vgatherqpd.rs"]
mod x86_64_simd_avx2_vgatherqps_vgatherqpd;
#[path = "x86_64/simd/avx2/vinserti128.rs"]
mod x86_64_simd_avx2_vinserti128;
#[path = "x86_64/simd/avx2/vmpsadbw.rs"]
mod x86_64_simd_avx2_vmpsadbw;
#[path = "x86_64/simd/avx2/vpabsb_vpabsw_vpabsd.rs"]
mod x86_64_simd_avx2_vpabsb_vpabsw_vpabsd;
#[path = "x86_64/simd/avx2/vpacksswb_vpackssdw.rs"]
mod x86_64_simd_avx2_vpacksswb_vpackssdw;
#[path = "x86_64/simd/avx2/vpackuswb_vpackusdw.rs"]
mod x86_64_simd_avx2_vpackuswb_vpackusdw;
#[path = "x86_64/simd/avx2/vpaddb_vpaddw_vpaddd_vpaddq.rs"]
mod x86_64_simd_avx2_vpaddb_vpaddw_vpaddd_vpaddq;
#[path = "x86_64/simd/avx2/vpaddsb.rs"]
mod x86_64_simd_avx2_vpaddsb;
#[path = "x86_64/simd/avx2/vpaddsw.rs"]
mod x86_64_simd_avx2_vpaddsw;
#[path = "x86_64/simd/avx2/vpaddusb.rs"]
mod x86_64_simd_avx2_vpaddusb;
#[path = "x86_64/simd/avx2/vpaddusw.rs"]
mod x86_64_simd_avx2_vpaddusw;
#[path = "x86_64/simd/avx2/vpalignr.rs"]
mod x86_64_simd_avx2_vpalignr;
#[path = "x86_64/simd/avx2/vpand_vpor_vpxor.rs"]
mod x86_64_simd_avx2_vpand_vpor_vpxor;
#[path = "x86_64/simd/avx2/vpandn.rs"]
mod x86_64_simd_avx2_vpandn;
#[path = "x86_64/simd/avx2/vpavgb_vpavgw.rs"]
mod x86_64_simd_avx2_vpavgb_vpavgw;
#[path = "x86_64/simd/avx2/vpblendd.rs"]
mod x86_64_simd_avx2_vpblendd;
#[path = "x86_64/simd/avx2/vpblendvb.rs"]
mod x86_64_simd_avx2_vpblendvb;
#[path = "x86_64/simd/avx2/vpblendw.rs"]
mod x86_64_simd_avx2_vpblendw;
#[path = "x86_64/simd/avx2/vpbroadcastb_vpbroadcastw.rs"]
mod x86_64_simd_avx2_vpbroadcastb_vpbroadcastw;
#[path = "x86_64/simd/avx2/vpbroadcastd_vpbroadcastq.rs"]
mod x86_64_simd_avx2_vpbroadcastd_vpbroadcastq;
#[path = "x86_64/simd/avx2/vpcmpeqb_vpcmpeqw_vpcmpeqd_vpcmpeqq.rs"]
mod x86_64_simd_avx2_vpcmpeqb_vpcmpeqw_vpcmpeqd_vpcmpeqq;
#[path = "x86_64/simd/avx2/vpcmpgtb_vpcmpgtw_vpcmpgtd_vpcmpgtq.rs"]
mod x86_64_simd_avx2_vpcmpgtb_vpcmpgtw_vpcmpgtd_vpcmpgtq;
#[path = "x86_64/simd/avx2/vperm2i128.rs"]
mod x86_64_simd_avx2_vperm2i128;
#[path = "x86_64/simd/avx2/vpermd_vpermq.rs"]
mod x86_64_simd_avx2_vpermd_vpermq;
#[path = "x86_64/simd/avx2/vpermpd.rs"]
mod x86_64_simd_avx2_vpermpd;
#[path = "x86_64/simd/avx2/vpermps.rs"]
mod x86_64_simd_avx2_vpermps;
#[path = "x86_64/simd/avx2/vpgatherdd_vpgatherdq.rs"]
mod x86_64_simd_avx2_vpgatherdd_vpgatherdq;
#[path = "x86_64/simd/avx2/vpgatherqd_vpgatherqq.rs"]
mod x86_64_simd_avx2_vpgatherqd_vpgatherqq;
#[path = "x86_64/simd/avx2/vphaddsw_vphsubsw.rs"]
mod x86_64_simd_avx2_vphaddsw_vphsubsw;
#[path = "x86_64/simd/avx2/vphaddw_vphaddd.rs"]
mod x86_64_simd_avx2_vphaddw_vphaddd;
#[path = "x86_64/simd/avx2/vphminposuw.rs"]
mod x86_64_simd_avx2_vphminposuw;
#[path = "x86_64/simd/avx2/vphsubw_vphsubd.rs"]
mod x86_64_simd_avx2_vphsubw_vphsubd;
#[path = "x86_64/simd/avx2/vpmaddubsw.rs"]
mod x86_64_simd_avx2_vpmaddubsw;
#[path = "x86_64/simd/avx2/vpmaddwd.rs"]
mod x86_64_simd_avx2_vpmaddwd;
#[path = "x86_64/simd/avx2/vpmaskmovd_vpmaskmovq.rs"]
mod x86_64_simd_avx2_vpmaskmovd_vpmaskmovq;
#[path = "x86_64/simd/avx2/vpmaxsb_vpmaxsw_vpmaxsd.rs"]
mod x86_64_simd_avx2_vpmaxsb_vpmaxsw_vpmaxsd;
#[path = "x86_64/simd/avx2/vpmaxub_vpmaxuw_vpmaxud.rs"]
mod x86_64_simd_avx2_vpmaxub_vpmaxuw_vpmaxud;
#[path = "x86_64/simd/avx2/vpminsb_vpminsw_vpminsd.rs"]
mod x86_64_simd_avx2_vpminsb_vpminsw_vpminsd;
#[path = "x86_64/simd/avx2/vpminub_vpminuw_vpminud.rs"]
mod x86_64_simd_avx2_vpminub_vpminuw_vpminud;
#[path = "x86_64/simd/avx2/vpmovmskb.rs"]
mod x86_64_simd_avx2_vpmovmskb;
#[path = "x86_64/simd/avx2/vpmovsx_variants.rs"]
mod x86_64_simd_avx2_vpmovsx_variants;
#[path = "x86_64/simd/avx2/vpmovsxbw_vpmovsxbd_vpmovsxbq.rs"]
mod x86_64_simd_avx2_vpmovsxbw_vpmovsxbd_vpmovsxbq;
#[path = "x86_64/simd/avx2/vpmovzxbw_vpmovzxbd_vpmovzxbq.rs"]
mod x86_64_simd_avx2_vpmovzxbw_vpmovzxbd_vpmovzxbq;
#[path = "x86_64/simd/avx2/vpmovzxwd_vpmovzxwq_vpmovzxdq.rs"]
mod x86_64_simd_avx2_vpmovzxwd_vpmovzxwq_vpmovzxdq;
#[path = "x86_64/simd/avx2/vpmuldq.rs"]
mod x86_64_simd_avx2_vpmuldq;
#[path = "x86_64/simd/avx2/vpmulhrsw.rs"]
mod x86_64_simd_avx2_vpmulhrsw;
#[path = "x86_64/simd/avx2/vpmulhw_vpmulhuw.rs"]
mod x86_64_simd_avx2_vpmulhw_vpmulhuw;
#[path = "x86_64/simd/avx2/vpmullw_vpmulld.rs"]
mod x86_64_simd_avx2_vpmullw_vpmulld;
#[path = "x86_64/simd/avx2/vpmuludq.rs"]
mod x86_64_simd_avx2_vpmuludq;
#[path = "x86_64/simd/avx2/vpsadbw.rs"]
mod x86_64_simd_avx2_vpsadbw;
#[path = "x86_64/simd/avx2/vpshufb.rs"]
mod x86_64_simd_avx2_vpshufb;
#[path = "x86_64/simd/avx2/vpshufd.rs"]
mod x86_64_simd_avx2_vpshufd;
#[path = "x86_64/simd/avx2/vpshufhw.rs"]
mod x86_64_simd_avx2_vpshufhw;
#[path = "x86_64/simd/avx2/vpshuflw.rs"]
mod x86_64_simd_avx2_vpshuflw;
#[path = "x86_64/simd/avx2/vpsignb_vpsignw_vpsignd.rs"]
mod x86_64_simd_avx2_vpsignb_vpsignw_vpsignd;
#[path = "x86_64/simd/avx2/vpslldq.rs"]
mod x86_64_simd_avx2_vpslldq;
#[path = "x86_64/simd/avx2/vpsllvd_vpsllvq.rs"]
mod x86_64_simd_avx2_vpsllvd_vpsllvq;
#[path = "x86_64/simd/avx2/vpsllw_vpslld_vpsllq.rs"]
mod x86_64_simd_avx2_vpsllw_vpslld_vpsllq;
#[path = "x86_64/simd/avx2/vpsravd.rs"]
mod x86_64_simd_avx2_vpsravd;
#[path = "x86_64/simd/avx2/vpsraw_vpsrad.rs"]
mod x86_64_simd_avx2_vpsraw_vpsrad;
#[path = "x86_64/simd/avx2/vpsrldq.rs"]
mod x86_64_simd_avx2_vpsrldq;
#[path = "x86_64/simd/avx2/vpsrlvd_vpsrlvq.rs"]
mod x86_64_simd_avx2_vpsrlvd_vpsrlvq;
#[path = "x86_64/simd/avx2/vpsrlw_vpsrld_vpsrlq.rs"]
mod x86_64_simd_avx2_vpsrlw_vpsrld_vpsrlq;
#[path = "x86_64/simd/avx2/vpsubb_vpsubw_vpsubd_vpsubq.rs"]
mod x86_64_simd_avx2_vpsubb_vpsubw_vpsubd_vpsubq;
#[path = "x86_64/simd/avx2/vpsubsb.rs"]
mod x86_64_simd_avx2_vpsubsb;
#[path = "x86_64/simd/avx2/vpsubsw.rs"]
mod x86_64_simd_avx2_vpsubsw;
#[path = "x86_64/simd/avx2/vpsubusb.rs"]
mod x86_64_simd_avx2_vpsubusb;
#[path = "x86_64/simd/avx2/vpsubusw.rs"]
mod x86_64_simd_avx2_vpsubusw;
#[path = "x86_64/simd/avx2/vptest.rs"]
mod x86_64_simd_avx2_vptest;
#[path = "x86_64/simd/avx2/vpunpckhbw_vpunpckhwd_vpunpckhdq_vpunpckhqdq.rs"]
mod x86_64_simd_avx2_vpunpckhbw_vpunpckhwd_vpunpckhdq_vpunpckhqdq;
#[path = "x86_64/simd/avx2/vpunpcklbw_vpunpcklwd_vpunpckldq_vpunpcklqdq.rs"]
mod x86_64_simd_avx2_vpunpcklbw_vpunpcklwd_vpunpckldq_vpunpcklqdq;
#[path = "x86_64/simd/avx512_extended.rs"]
mod x86_64_simd_avx512_extended;
#[path = "x86_64/simd/avx512/kadd_mask.rs"]
mod x86_64_simd_avx512_kadd_mask;
#[path = "x86_64/simd/avx512/kand_kor_kxor.rs"]
mod x86_64_simd_avx512_kand_kor_kxor;
#[path = "x86_64/simd/avx512/kandn_knot_mask.rs"]
mod x86_64_simd_avx512_kandn_knot_mask;
#[path = "x86_64/simd/avx512/kmov.rs"]
mod x86_64_simd_avx512_kmov;
#[path = "x86_64/simd/avx512/ktest_kunpck_kshift.rs"]
mod x86_64_simd_avx512_ktest_kunpck_kshift;
#[path = "x86_64/simd/avx512_mask_ops.rs"]
mod x86_64_simd_avx512_mask_ops;
#[path = "x86_64/simd/avx512/vaddph_vsubph_vmulph_vdivph.rs"]
mod x86_64_simd_avx512_vaddph_vsubph_vmulph_vdivph;
#[path = "x86_64/simd/avx512/vaddps_zmm.rs"]
mod x86_64_simd_avx512_vaddps_zmm;
#[path = "x86_64/simd/avx512/valign_vprol_vpror_vpternlog.rs"]
mod x86_64_simd_avx512_valign_vprol_vpror_vpternlog;
#[path = "x86_64/simd/avx512/vdbpsadbw_vplzcnt_vpshld.rs"]
mod x86_64_simd_avx512_vdbpsadbw_vplzcnt_vpshld;
#[path = "x86_64/simd/avx512/vdivps_zmm.rs"]
mod x86_64_simd_avx512_vdivps_zmm;
#[path = "x86_64/simd/avx512/vmovaps_zmm.rs"]
mod x86_64_simd_avx512_vmovaps_zmm;
#[path = "x86_64/simd/avx512/vmovups_zmm.rs"]
mod x86_64_simd_avx512_vmovups_zmm;
#[path = "x86_64/simd/avx512/vmulps_zmm.rs"]
mod x86_64_simd_avx512_vmulps_zmm;
#[path = "x86_64/simd/avx512/vsubps_zmm.rs"]
mod x86_64_simd_avx512_vsubps_zmm;
#[path = "x86_64/simd/avx/vaddps_vaddpd.rs"]
mod x86_64_simd_avx_vaddps_vaddpd;
#[path = "x86_64/simd/avx/vaddss_vaddsd.rs"]
mod x86_64_simd_avx_vaddss_vaddsd;
#[path = "x86_64/simd/avx/vaddsubps_vaddsubpd.rs"]
mod x86_64_simd_avx_vaddsubps_vaddsubpd;
#[path = "x86_64/simd/avx/vandnps_vandnpd.rs"]
mod x86_64_simd_avx_vandnps_vandnpd;
#[path = "x86_64/simd/avx/vandps_vandpd.rs"]
mod x86_64_simd_avx_vandps_vandpd;
#[path = "x86_64/simd/avx/vblendps_vblendpd.rs"]
mod x86_64_simd_avx_vblendps_vblendpd;
#[path = "x86_64/simd/avx/vblendvpd.rs"]
mod x86_64_simd_avx_vblendvpd;
#[path = "x86_64/simd/avx/vblendvps.rs"]
mod x86_64_simd_avx_vblendvps;
#[path = "x86_64/simd/avx/vbroadcastss_vbroadcastsd.rs"]
mod x86_64_simd_avx_vbroadcastss_vbroadcastsd;
#[path = "x86_64/simd/avx/vcmpps_vcmppd.rs"]
mod x86_64_simd_avx_vcmpps_vcmppd;
#[path = "x86_64/simd/avx/vcomisd.rs"]
mod x86_64_simd_avx_vcomisd;
#[path = "x86_64/simd/avx/vcomiss.rs"]
mod x86_64_simd_avx_vcomiss;
#[path = "x86_64/simd/avx/vcvtdq2pd_vcvtpd2dq.rs"]
mod x86_64_simd_avx_vcvtdq2pd_vcvtpd2dq;
#[path = "x86_64/simd/avx/vcvtdq2ps_vcvtps2dq.rs"]
mod x86_64_simd_avx_vcvtdq2ps_vcvtps2dq;
#[path = "x86_64/simd/avx/vcvtps2pd_vcvtpd2ps.rs"]
mod x86_64_simd_avx_vcvtps2pd_vcvtpd2ps;
#[path = "x86_64/simd/avx/vcvtsi2ss_vcvtsi2sd.rs"]
mod x86_64_simd_avx_vcvtsi2ss_vcvtsi2sd;
#[path = "x86_64/simd/avx/vcvtss2sd_vcvtsd2ss.rs"]
mod x86_64_simd_avx_vcvtss2sd_vcvtsd2ss;
#[path = "x86_64/simd/avx/vcvtss2si_vcvtsd2si.rs"]
mod x86_64_simd_avx_vcvtss2si_vcvtsd2si;
#[path = "x86_64/simd/avx/vcvttps2dq_vcvttpd2dq.rs"]
mod x86_64_simd_avx_vcvttps2dq_vcvttpd2dq;
#[path = "x86_64/simd/avx/vcvttss2si_vcvttsd2si.rs"]
mod x86_64_simd_avx_vcvttss2si_vcvttsd2si;
#[path = "x86_64/simd/avx/vdivps_vdivpd.rs"]
mod x86_64_simd_avx_vdivps_vdivpd;
#[path = "x86_64/simd/avx/vdivss_vdivsd.rs"]
mod x86_64_simd_avx_vdivss_vdivsd;
#[path = "x86_64/simd/avx/vdppd.rs"]
mod x86_64_simd_avx_vdppd;
#[path = "x86_64/simd/avx/vdpps.rs"]
mod x86_64_simd_avx_vdpps;
#[path = "x86_64/simd/avx/vextractf128.rs"]
mod x86_64_simd_avx_vextractf128;
#[path = "x86_64/simd/avx/vextractf128_vinsertf128.rs"]
mod x86_64_simd_avx_vextractf128_vinsertf128;
#[path = "x86_64/simd/avx/vfmadd132pd.rs"]
mod x86_64_simd_avx_vfmadd132pd;
#[path = "x86_64/simd/avx/vfmadd132ps.rs"]
mod x86_64_simd_avx_vfmadd132ps;
#[path = "x86_64/simd/avx/vfmadd213pd.rs"]
mod x86_64_simd_avx_vfmadd213pd;
#[path = "x86_64/simd/avx/vfmadd213ps.rs"]
mod x86_64_simd_avx_vfmadd213ps;
#[path = "x86_64/simd/avx/vfmadd231pd.rs"]
mod x86_64_simd_avx_vfmadd231pd;
#[path = "x86_64/simd/avx/vfmadd231ps.rs"]
mod x86_64_simd_avx_vfmadd231ps;
#[path = "x86_64/simd/avx/vfmsub132pd.rs"]
mod x86_64_simd_avx_vfmsub132pd;
#[path = "x86_64/simd/avx/vfmsub132ps.rs"]
mod x86_64_simd_avx_vfmsub132ps;
#[path = "x86_64/simd/avx/vfmsub213pd.rs"]
mod x86_64_simd_avx_vfmsub213pd;
#[path = "x86_64/simd/avx/vfmsub213ps.rs"]
mod x86_64_simd_avx_vfmsub213ps;
#[path = "x86_64/simd/avx/vfmsub231pd.rs"]
mod x86_64_simd_avx_vfmsub231pd;
#[path = "x86_64/simd/avx/vfmsub231ps.rs"]
mod x86_64_simd_avx_vfmsub231ps;
#[path = "x86_64/simd/avx/vfnmadd132pd.rs"]
mod x86_64_simd_avx_vfnmadd132pd;
#[path = "x86_64/simd/avx/vfnmadd132ps.rs"]
mod x86_64_simd_avx_vfnmadd132ps;
#[path = "x86_64/simd/avx/vfnmadd213pd.rs"]
mod x86_64_simd_avx_vfnmadd213pd;
#[path = "x86_64/simd/avx/vfnmadd213ps.rs"]
mod x86_64_simd_avx_vfnmadd213ps;
#[path = "x86_64/simd/avx/vfnmadd231pd.rs"]
mod x86_64_simd_avx_vfnmadd231pd;
#[path = "x86_64/simd/avx/vfnmadd231ps.rs"]
mod x86_64_simd_avx_vfnmadd231ps;
#[path = "x86_64/simd/avx/vfnmsub132pd.rs"]
mod x86_64_simd_avx_vfnmsub132pd;
#[path = "x86_64/simd/avx/vfnmsub132ps.rs"]
mod x86_64_simd_avx_vfnmsub132ps;
#[path = "x86_64/simd/avx/vfnmsub213pd.rs"]
mod x86_64_simd_avx_vfnmsub213pd;
#[path = "x86_64/simd/avx/vfnmsub213ps.rs"]
mod x86_64_simd_avx_vfnmsub213ps;
#[path = "x86_64/simd/avx/vfnmsub231pd.rs"]
mod x86_64_simd_avx_vfnmsub231pd;
#[path = "x86_64/simd/avx/vfnmsub231ps.rs"]
mod x86_64_simd_avx_vfnmsub231ps;
#[path = "x86_64/simd/avx/vhaddps_vhaddpd.rs"]
mod x86_64_simd_avx_vhaddps_vhaddpd;
#[path = "x86_64/simd/avx/vhsubps_vhsubpd.rs"]
mod x86_64_simd_avx_vhsubps_vhsubpd;
#[path = "x86_64/simd/avx/vinsertf128.rs"]
mod x86_64_simd_avx_vinsertf128;
#[path = "x86_64/simd/avx/vldmxcsr_vstmxcsr.rs"]
mod x86_64_simd_avx_vldmxcsr_vstmxcsr;
#[path = "x86_64/simd/avx/vmaskmovps_vmaskmovpd.rs"]
mod x86_64_simd_avx_vmaskmovps_vmaskmovpd;
#[path = "x86_64/simd/avx/vmaxps_vmaxpd.rs"]
mod x86_64_simd_avx_vmaxps_vmaxpd;
#[path = "x86_64/simd/avx/vminps_vminpd.rs"]
mod x86_64_simd_avx_vminps_vminpd;
#[path = "x86_64/simd/avx/vmovaps_vmovapd.rs"]
mod x86_64_simd_avx_vmovaps_vmovapd;
#[path = "x86_64/simd/avx/vmovddup.rs"]
mod x86_64_simd_avx_vmovddup;
#[path = "x86_64/simd/avx/vmovdqa_vmovdqu.rs"]
mod x86_64_simd_avx_vmovdqa_vmovdqu;
#[path = "x86_64/simd/avx/vmovhlps.rs"]
mod x86_64_simd_avx_vmovhlps;
#[path = "x86_64/simd/avx/vmovhpd.rs"]
mod x86_64_simd_avx_vmovhpd;
#[path = "x86_64/simd/avx/vmovhps.rs"]
mod x86_64_simd_avx_vmovhps;
#[path = "x86_64/simd/avx/vmovlhps.rs"]
mod x86_64_simd_avx_vmovlhps;
#[path = "x86_64/simd/avx/vmovlpd.rs"]
mod x86_64_simd_avx_vmovlpd;
#[path = "x86_64/simd/avx/vmovlps.rs"]
mod x86_64_simd_avx_vmovlps;
#[path = "x86_64/simd/avx/vmovmskps_vmovmskpd.rs"]
mod x86_64_simd_avx_vmovmskps_vmovmskpd;
#[path = "x86_64/simd/avx/vmovntdq.rs"]
mod x86_64_simd_avx_vmovntdq;
#[path = "x86_64/simd/avx/vmovntdqa.rs"]
mod x86_64_simd_avx_vmovntdqa;
#[path = "x86_64/simd/avx/vmovntpd.rs"]
mod x86_64_simd_avx_vmovntpd;
#[path = "x86_64/simd/avx/vmovntps.rs"]
mod x86_64_simd_avx_vmovntps;
#[path = "x86_64/simd/avx/vmovsd.rs"]
mod x86_64_simd_avx_vmovsd;
#[path = "x86_64/simd/avx/vmovshdup.rs"]
mod x86_64_simd_avx_vmovshdup;
#[path = "x86_64/simd/avx/vmovsldup.rs"]
mod x86_64_simd_avx_vmovsldup;
#[path = "x86_64/simd/avx/vmovss.rs"]
mod x86_64_simd_avx_vmovss;
#[path = "x86_64/simd/avx/vmovups_vmovupd.rs"]
mod x86_64_simd_avx_vmovups_vmovupd;
#[path = "x86_64/simd/avx/vmulps_vmulpd.rs"]
mod x86_64_simd_avx_vmulps_vmulpd;
#[path = "x86_64/simd/avx/vmulss_vmulsd.rs"]
mod x86_64_simd_avx_vmulss_vmulsd;
#[path = "x86_64/simd/avx/vorps_vorpd.rs"]
mod x86_64_simd_avx_vorps_vorpd;
#[path = "x86_64/simd/avx/vperm2f128.rs"]
mod x86_64_simd_avx_vperm2f128;
#[path = "x86_64/simd/avx/vpermilpd.rs"]
mod x86_64_simd_avx_vpermilpd;
#[path = "x86_64/simd/avx/vpermilps.rs"]
mod x86_64_simd_avx_vpermilps;
#[path = "x86_64/simd/avx/vrcpps.rs"]
mod x86_64_simd_avx_vrcpps;
#[path = "x86_64/simd/avx/vroundpd.rs"]
mod x86_64_simd_avx_vroundpd;
#[path = "x86_64/simd/avx/vroundps.rs"]
mod x86_64_simd_avx_vroundps;
#[path = "x86_64/simd/avx/vroundsd.rs"]
mod x86_64_simd_avx_vroundsd;
#[path = "x86_64/simd/avx/vroundss.rs"]
mod x86_64_simd_avx_vroundss;
#[path = "x86_64/simd/avx/vrsqrtps.rs"]
mod x86_64_simd_avx_vrsqrtps;
#[path = "x86_64/simd/avx/vshufps_vshufpd.rs"]
mod x86_64_simd_avx_vshufps_vshufpd;
#[path = "x86_64/simd/avx/vsqrtps_vsqrtpd.rs"]
mod x86_64_simd_avx_vsqrtps_vsqrtpd;
#[path = "x86_64/simd/avx/vsubps_vsubpd.rs"]
mod x86_64_simd_avx_vsubps_vsubpd;
#[path = "x86_64/simd/avx/vsubss_vsubsd.rs"]
mod x86_64_simd_avx_vsubss_vsubsd;
#[path = "x86_64/simd/avx/vtestps_vtestpd.rs"]
mod x86_64_simd_avx_vtestps_vtestpd;
#[path = "x86_64/simd/avx/vucomisd.rs"]
mod x86_64_simd_avx_vucomisd;
#[path = "x86_64/simd/avx/vucomiss.rs"]
mod x86_64_simd_avx_vucomiss;
#[path = "x86_64/simd/avx/vunpckhps_vunpckhpd.rs"]
mod x86_64_simd_avx_vunpckhps_vunpckhpd;
#[path = "x86_64/simd/avx/vunpcklps_vunpcklpd.rs"]
mod x86_64_simd_avx_vunpcklps_vunpcklpd;
#[path = "x86_64/simd/avx/vxorps_vxorpd.rs"]
mod x86_64_simd_avx_vxorps_vxorpd;
#[path = "x86_64/simd/avx/vzeroupper_vzeroall.rs"]
mod x86_64_simd_avx_vzeroupper_vzeroall;
#[path = "x86_64/simd/fma/vfmadd132pd_vfmadd213pd_vfmadd231pd.rs"]
mod x86_64_simd_fma_vfmadd132pd_vfmadd213pd_vfmadd231pd;
#[path = "x86_64/simd/fma/vfmadd132ps_vfmadd213ps_vfmadd231ps.rs"]
mod x86_64_simd_fma_vfmadd132ps_vfmadd213ps_vfmadd231ps;
#[path = "x86_64/simd/fma/vfmadd132sd_vfmadd213sd_vfmadd231sd.rs"]
mod x86_64_simd_fma_vfmadd132sd_vfmadd213sd_vfmadd231sd;
#[path = "x86_64/simd/fma/vfmadd132ss_vfmadd213ss_vfmadd231ss.rs"]
mod x86_64_simd_fma_vfmadd132ss_vfmadd213ss_vfmadd231ss;
#[path = "x86_64/simd/fma/vfmaddsub_vfmsubadd.rs"]
mod x86_64_simd_fma_vfmaddsub_vfmsubadd;
#[path = "x86_64/simd/fma/vfmsub_variants.rs"]
mod x86_64_simd_fma_vfmsub_variants;
#[path = "x86_64/simd/fma/vfnmadd_variants.rs"]
mod x86_64_simd_fma_vfnmadd_variants;
#[path = "x86_64/simd/fma/vfnmsub_variants.rs"]
mod x86_64_simd_fma_vfnmsub_variants;
#[path = "x86_64/simd/mmx/emms.rs"]
mod x86_64_simd_mmx_emms;
#[path = "x86_64/simd/mmx/movq.rs"]
mod x86_64_simd_mmx_movq;
#[path = "x86_64/simd/mmx/packsswb_packssdw_mmx.rs"]
mod x86_64_simd_mmx_packsswb_packssdw_mmx;
#[path = "x86_64/simd/mmx/packuswb_mmx.rs"]
mod x86_64_simd_mmx_packuswb_mmx;
#[path = "x86_64/simd/mmx/paddb_paddw_paddd.rs"]
mod x86_64_simd_mmx_paddb_paddw_paddd;
#[path = "x86_64/simd/mmx/paddsb_paddsw_mmx.rs"]
mod x86_64_simd_mmx_paddsb_paddsw_mmx;
#[path = "x86_64/simd/mmx/paddusb_paddusw_mmx.rs"]
mod x86_64_simd_mmx_paddusb_paddusw_mmx;
#[path = "x86_64/simd/mmx/pand_por_pxor.rs"]
mod x86_64_simd_mmx_pand_por_pxor;
#[path = "x86_64/simd/mmx/pandn_mmx.rs"]
mod x86_64_simd_mmx_pandn_mmx;
#[path = "x86_64/simd/mmx/pavgb_pavgw_mmx.rs"]
mod x86_64_simd_mmx_pavgb_pavgw_mmx;
#[path = "x86_64/simd/mmx/pcmpeqb_pcmpeqw_pcmpeqd.rs"]
mod x86_64_simd_mmx_pcmpeqb_pcmpeqw_pcmpeqd;
#[path = "x86_64/simd/mmx/pcmpgtb_pcmpgtw_pcmpgtd_mmx.rs"]
mod x86_64_simd_mmx_pcmpgtb_pcmpgtw_pcmpgtd_mmx;
#[path = "x86_64/simd/mmx/pextrw_mmx.rs"]
mod x86_64_simd_mmx_pextrw_mmx;
#[path = "x86_64/simd/mmx/pinsrw_mmx.rs"]
mod x86_64_simd_mmx_pinsrw_mmx;
#[path = "x86_64/simd/mmx/pmaddwd_mmx.rs"]
mod x86_64_simd_mmx_pmaddwd_mmx;
#[path = "x86_64/simd/mmx/pmaxsw_mmx.rs"]
mod x86_64_simd_mmx_pmaxsw_mmx;
#[path = "x86_64/simd/mmx/pmaxub_mmx.rs"]
mod x86_64_simd_mmx_pmaxub_mmx;
#[path = "x86_64/simd/mmx/pminsw_mmx.rs"]
mod x86_64_simd_mmx_pminsw_mmx;
#[path = "x86_64/simd/mmx/pminub_mmx.rs"]
mod x86_64_simd_mmx_pminub_mmx;
#[path = "x86_64/simd/mmx/pmulhuw_mmx.rs"]
mod x86_64_simd_mmx_pmulhuw_mmx;
#[path = "x86_64/simd/mmx/pmulhw.rs"]
mod x86_64_simd_mmx_pmulhw;
#[path = "x86_64/simd/mmx/pmullw.rs"]
mod x86_64_simd_mmx_pmullw;
#[path = "x86_64/simd/mmx/psadbw_mmx.rs"]
mod x86_64_simd_mmx_psadbw_mmx;
#[path = "x86_64/simd/mmx/pshufw.rs"]
mod x86_64_simd_mmx_pshufw;
#[path = "x86_64/simd/mmx/psllw_pslld_psllq_mmx.rs"]
mod x86_64_simd_mmx_psllw_pslld_psllq_mmx;
#[path = "x86_64/simd/mmx/psraw_psrad_mmx.rs"]
mod x86_64_simd_mmx_psraw_psrad_mmx;
#[path = "x86_64/simd/mmx/psrlw_psrld_psrlq_mmx.rs"]
mod x86_64_simd_mmx_psrlw_psrld_psrlq_mmx;
#[path = "x86_64/simd/mmx/psubb_psubw_psubd.rs"]
mod x86_64_simd_mmx_psubb_psubw_psubd;
#[path = "x86_64/simd/mmx/psubsb_psubsw_mmx.rs"]
mod x86_64_simd_mmx_psubsb_psubsw_mmx;
#[path = "x86_64/simd/mmx/psubusb_psubusw_mmx.rs"]
mod x86_64_simd_mmx_psubusb_psubusw_mmx;
#[path = "x86_64/simd/mmx/punpckhbw_punpckhwd.rs"]
mod x86_64_simd_mmx_punpckhbw_punpckhwd;
#[path = "x86_64/simd/mmx/punpcklbw_punpcklwd.rs"]
mod x86_64_simd_mmx_punpcklbw_punpcklwd;
#[path = "x86_64/simd/packing_ops.rs"]
mod x86_64_simd_packing_ops;
#[path = "x86_64/simd/sse/addps_addpd.rs"]
mod x86_64_simd_sse_addps_addpd;
#[path = "x86_64/simd/sse/addss_addsd.rs"]
mod x86_64_simd_sse_addss_addsd;
#[path = "x86_64/simd/sse/addsubps_addsubpd.rs"]
mod x86_64_simd_sse_addsubps_addsubpd;
#[path = "x86_64/simd/sse/aesdec_aesdeclast.rs"]
mod x86_64_simd_sse_aesdec_aesdeclast;
#[path = "x86_64/simd/sse/aesenc_aesenclast.rs"]
mod x86_64_simd_sse_aesenc_aesenclast;
#[path = "x86_64/simd/sse/aesimc_aeskeygenassist.rs"]
mod x86_64_simd_sse_aesimc_aeskeygenassist;
#[path = "x86_64/simd/sse/andnps_andnpd.rs"]
mod x86_64_simd_sse_andnps_andnpd;
#[path = "x86_64/simd/sse/andps_andpd.rs"]
mod x86_64_simd_sse_andps_andpd;
#[path = "x86_64/simd/sse/blendps_blendpd.rs"]
mod x86_64_simd_sse_blendps_blendpd;
#[path = "x86_64/simd/sse/blendvps_blendvpd.rs"]
mod x86_64_simd_sse_blendvps_blendvpd;
#[path = "x86_64/simd/sse/clflushopt.rs"]
mod x86_64_simd_sse_clflushopt;
#[path = "x86_64/simd/sse/cmppd.rs"]
mod x86_64_simd_sse_cmppd;
#[path = "x86_64/simd/sse/cmpps.rs"]
mod x86_64_simd_sse_cmpps;
#[path = "x86_64/simd/sse/cmpsd.rs"]
mod x86_64_simd_sse_cmpsd;
#[path = "x86_64/simd/sse/cmpss.rs"]
mod x86_64_simd_sse_cmpss;
#[path = "x86_64/simd/sse/comiss_comisd.rs"]
mod x86_64_simd_sse_comiss_comisd;
#[path = "x86_64/simd/sse/crc32.rs"]
mod x86_64_simd_sse_crc32;
#[path = "x86_64/simd/sse/cvtdq2pd_cvtpd2dq.rs"]
mod x86_64_simd_sse_cvtdq2pd_cvtpd2dq;
#[path = "x86_64/simd/sse/cvtdq2ps_cvtps2dq.rs"]
mod x86_64_simd_sse_cvtdq2ps_cvtps2dq;
#[path = "x86_64/simd/sse/cvtpd2ps.rs"]
mod x86_64_simd_sse_cvtpd2ps;
#[path = "x86_64/simd/sse/cvtpi2pd_cvtpd2pi.rs"]
mod x86_64_simd_sse_cvtpi2pd_cvtpd2pi;
#[path = "x86_64/simd/sse/cvtpi2ps_cvtps2pi.rs"]
mod x86_64_simd_sse_cvtpi2ps_cvtps2pi;
#[path = "x86_64/simd/sse/cvtps2pd.rs"]
mod x86_64_simd_sse_cvtps2pd;
#[path = "x86_64/simd/sse/cvtsd2si.rs"]
mod x86_64_simd_sse_cvtsd2si;
#[path = "x86_64/simd/sse/cvtsd2ss.rs"]
mod x86_64_simd_sse_cvtsd2ss;
#[path = "x86_64/simd/sse/cvtsi2sd.rs"]
mod x86_64_simd_sse_cvtsi2sd;
#[path = "x86_64/simd/sse/cvtsi2ss.rs"]
mod x86_64_simd_sse_cvtsi2ss;
#[path = "x86_64/simd/sse/cvtss2sd.rs"]
mod x86_64_simd_sse_cvtss2sd;
#[path = "x86_64/simd/sse/cvtss2si.rs"]
mod x86_64_simd_sse_cvtss2si;
#[path = "x86_64/simd/sse/cvttps2dq_cvttpd2dq.rs"]
mod x86_64_simd_sse_cvttps2dq_cvttpd2dq;
#[path = "x86_64/simd/sse/cvttps2pi_cvttpd2pi.rs"]
mod x86_64_simd_sse_cvttps2pi_cvttpd2pi;
#[path = "x86_64/simd/sse/cvttsd2si_cvttss2si.rs"]
mod x86_64_simd_sse_cvttsd2si_cvttss2si;
#[path = "x86_64/simd/sse/divps_divpd.rs"]
mod x86_64_simd_sse_divps_divpd;
#[path = "x86_64/simd/sse/divss_divsd.rs"]
mod x86_64_simd_sse_divss_divsd;
#[path = "x86_64/simd/sse/dppd.rs"]
mod x86_64_simd_sse_dppd;
#[path = "x86_64/simd/sse/dpps.rs"]
mod x86_64_simd_sse_dpps;
#[path = "x86_64/simd/sse/extractps.rs"]
mod x86_64_simd_sse_extractps;
#[path = "x86_64/simd/sse/fisttp_sse.rs"]
mod x86_64_simd_sse_fisttp_sse;
#[path = "x86_64/simd/sse/haddps_haddpd.rs"]
mod x86_64_simd_sse_haddps_haddpd;
#[path = "x86_64/simd/sse/hsubps_hsubpd.rs"]
mod x86_64_simd_sse_hsubps_hsubpd;
#[path = "x86_64/simd/sse/insertps.rs"]
mod x86_64_simd_sse_insertps;
#[path = "x86_64/simd/sse/lddqu.rs"]
mod x86_64_simd_sse_lddqu;
#[path = "x86_64/simd/sse/ldmxcsr_stmxcsr.rs"]
mod x86_64_simd_sse_ldmxcsr_stmxcsr;
#[path = "x86_64/simd/sse/lfence_mfence_sfence.rs"]
mod x86_64_simd_sse_lfence_mfence_sfence;
#[path = "x86_64/simd/sse/maskmovdqu.rs"]
mod x86_64_simd_sse_maskmovdqu;
#[path = "x86_64/simd/sse/maskmovq_emms.rs"]
mod x86_64_simd_sse_maskmovq_emms;
#[path = "x86_64/simd/sse/maxps_maxpd.rs"]
mod x86_64_simd_sse_maxps_maxpd;
#[path = "x86_64/simd/sse/maxss_maxsd.rs"]
mod x86_64_simd_sse_maxss_maxsd;
#[path = "x86_64/simd/sse/minps_minpd.rs"]
mod x86_64_simd_sse_minps_minpd;
#[path = "x86_64/simd/sse/minss_minsd.rs"]
mod x86_64_simd_sse_minss_minsd;
#[path = "x86_64/simd/sse/monitor_mwait_extended.rs"]
mod x86_64_simd_sse_monitor_mwait_extended;
#[path = "x86_64/simd/sse/movapd.rs"]
mod x86_64_simd_sse_movapd;
#[path = "x86_64/simd/sse/movaps.rs"]
mod x86_64_simd_sse_movaps;
#[path = "x86_64/simd/sse/movd_movq.rs"]
mod x86_64_simd_sse_movd_movq;
#[path = "x86_64/simd/sse/movddup.rs"]
mod x86_64_simd_sse_movddup;
#[path = "x86_64/simd/sse/movddup_extended.rs"]
mod x86_64_simd_sse_movddup_extended;
#[path = "x86_64/simd/sse/movdqa.rs"]
mod x86_64_simd_sse_movdqa;
#[path = "x86_64/simd/sse/movdqu.rs"]
mod x86_64_simd_sse_movdqu;
#[path = "x86_64/simd/sse/movhlps_movlhps.rs"]
mod x86_64_simd_sse_movhlps_movlhps;
#[path = "x86_64/simd/sse/movhps_movlps_movhpd_movlpd.rs"]
mod x86_64_simd_sse_movhps_movlps_movhpd_movlpd;
#[path = "x86_64/simd/sse/movmskps_movmskpd.rs"]
mod x86_64_simd_sse_movmskps_movmskpd;
#[path = "x86_64/simd/sse/movntdq.rs"]
mod x86_64_simd_sse_movntdq;
#[path = "x86_64/simd/sse/movntdqa.rs"]
mod x86_64_simd_sse_movntdqa;
#[path = "x86_64/simd/sse/movnti.rs"]
mod x86_64_simd_sse_movnti;
#[path = "x86_64/simd/sse/movntps_movntpd.rs"]
mod x86_64_simd_sse_movntps_movntpd;
#[path = "x86_64/simd/sse/movntq.rs"]
mod x86_64_simd_sse_movntq;
#[path = "x86_64/simd/sse/movntss_movntsd.rs"]
mod x86_64_simd_sse_movntss_movntsd;
#[path = "x86_64/simd/sse/movq_movq2dq_movdq2q.rs"]
mod x86_64_simd_sse_movq_movq2dq_movdq2q;
#[path = "x86_64/simd/sse/movshdup_movsldup.rs"]
mod x86_64_simd_sse_movshdup_movsldup;
#[path = "x86_64/simd/sse/movsldup_movshdup_extended.rs"]
mod x86_64_simd_sse_movsldup_movshdup_extended;
#[path = "x86_64/simd/sse/movss_movsd_scalar.rs"]
mod x86_64_simd_sse_movss_movsd_scalar;
#[path = "x86_64/simd/sse/movupd.rs"]
mod x86_64_simd_sse_movupd;
#[path = "x86_64/simd/sse/movups.rs"]
mod x86_64_simd_sse_movups;
#[path = "x86_64/simd/sse/mpsadbw.rs"]
mod x86_64_simd_sse_mpsadbw;
#[path = "x86_64/simd/sse/mpsadbw_extended.rs"]
mod x86_64_simd_sse_mpsadbw_extended;
#[path = "x86_64/simd/sse/mulps_mulpd.rs"]
mod x86_64_simd_sse_mulps_mulpd;
#[path = "x86_64/simd/sse/mulss_mulsd.rs"]
mod x86_64_simd_sse_mulss_mulsd;
#[path = "x86_64/simd/sse/orps_orpd.rs"]
mod x86_64_simd_sse_orps_orpd;
#[path = "x86_64/simd/sse/pabsb_pabsw_pabsd.rs"]
mod x86_64_simd_sse_pabsb_pabsw_pabsd;
#[path = "x86_64/simd/sse/packsswb_packssdw.rs"]
mod x86_64_simd_sse_packsswb_packssdw;
#[path = "x86_64/simd/sse/packusdw.rs"]
mod x86_64_simd_sse_packusdw;
#[path = "x86_64/simd/sse/packuswb_packusdw.rs"]
mod x86_64_simd_sse_packuswb_packusdw;
#[path = "x86_64/simd/sse/paddb_paddw_paddd_paddq.rs"]
mod x86_64_simd_sse_paddb_paddw_paddd_paddq;
#[path = "x86_64/simd/sse/paddsb_paddsw.rs"]
mod x86_64_simd_sse_paddsb_paddsw;
#[path = "x86_64/simd/sse/paddusb_paddusw.rs"]
mod x86_64_simd_sse_paddusb_paddusw;
#[path = "x86_64/simd/sse/palignr.rs"]
mod x86_64_simd_sse_palignr;
#[path = "x86_64/simd/sse/pand_por_pxor_pandn.rs"]
mod x86_64_simd_sse_pand_por_pxor_pandn;
#[path = "x86_64/simd/sse/pause.rs"]
mod x86_64_simd_sse_pause;
#[path = "x86_64/simd/sse/pavgb_pavgw.rs"]
mod x86_64_simd_sse_pavgb_pavgw;
#[path = "x86_64/simd/sse/pblendvb.rs"]
mod x86_64_simd_sse_pblendvb;
#[path = "x86_64/simd/sse/pblendw.rs"]
mod x86_64_simd_sse_pblendw;
#[path = "x86_64/simd/sse/pclmulqdq.rs"]
mod x86_64_simd_sse_pclmulqdq;
#[path = "x86_64/simd/sse/pclmulqdq_extended.rs"]
mod x86_64_simd_sse_pclmulqdq_extended;
#[path = "x86_64/simd/sse/pcmpeqb_pcmpeqw_pcmpeqd.rs"]
mod x86_64_simd_sse_pcmpeqb_pcmpeqw_pcmpeqd;
#[path = "x86_64/simd/sse/pcmpeqq.rs"]
mod x86_64_simd_sse_pcmpeqq;
#[path = "x86_64/simd/sse/pcmpestri.rs"]
mod x86_64_simd_sse_pcmpestri;
#[path = "x86_64/simd/sse/pcmpestrm.rs"]
mod x86_64_simd_sse_pcmpestrm;
#[path = "x86_64/simd/sse/pcmpgtb_pcmpgtw_pcmpgtd.rs"]
mod x86_64_simd_sse_pcmpgtb_pcmpgtw_pcmpgtd;
#[path = "x86_64/simd/sse/pcmpgtq.rs"]
mod x86_64_simd_sse_pcmpgtq;
#[path = "x86_64/simd/sse/pcmpistri.rs"]
mod x86_64_simd_sse_pcmpistri;
#[path = "x86_64/simd/sse/pcmpistrm.rs"]
mod x86_64_simd_sse_pcmpistrm;
#[path = "x86_64/simd/sse/pextrb_pextrd_pextrq.rs"]
mod x86_64_simd_sse_pextrb_pextrd_pextrq;
#[path = "x86_64/simd/sse/pextrw.rs"]
mod x86_64_simd_sse_pextrw;
#[path = "x86_64/simd/sse/phaddsw_phsubsw.rs"]
mod x86_64_simd_sse_phaddsw_phsubsw;
#[path = "x86_64/simd/sse/phaddw_phaddd.rs"]
mod x86_64_simd_sse_phaddw_phaddd;
#[path = "x86_64/simd/sse/phminposuw.rs"]
mod x86_64_simd_sse_phminposuw;
#[path = "x86_64/simd/sse/phminposuw_extended.rs"]
mod x86_64_simd_sse_phminposuw_extended;
#[path = "x86_64/simd/sse/phsubw_phsubd.rs"]
mod x86_64_simd_sse_phsubw_phsubd;
#[path = "x86_64/simd/sse/pinsrb_pinsrd_pinsrq.rs"]
mod x86_64_simd_sse_pinsrb_pinsrd_pinsrq;
#[path = "x86_64/simd/sse/pinsrw.rs"]
mod x86_64_simd_sse_pinsrw;
#[path = "x86_64/simd/sse/pmaddubsw.rs"]
mod x86_64_simd_sse_pmaddubsw;
#[path = "x86_64/simd/sse/pmaddubsw_extended.rs"]
mod x86_64_simd_sse_pmaddubsw_extended;
#[path = "x86_64/simd/sse/pmaddwd.rs"]
mod x86_64_simd_sse_pmaddwd;
#[path = "x86_64/simd/sse/pmaxsb_pmaxsd.rs"]
mod x86_64_simd_sse_pmaxsb_pmaxsd;
#[path = "x86_64/simd/sse/pmaxsb_pmaxsw_pmaxsd.rs"]
mod x86_64_simd_sse_pmaxsb_pmaxsw_pmaxsd;
#[path = "x86_64/simd/sse/pmaxub_pmaxuw_extended.rs"]
mod x86_64_simd_sse_pmaxub_pmaxuw_extended;
#[path = "x86_64/simd/sse/pmaxub_pmaxuw_pmaxud.rs"]
mod x86_64_simd_sse_pmaxub_pmaxuw_pmaxud;
#[path = "x86_64/simd/sse/pmaxuw_pmaxud.rs"]
mod x86_64_simd_sse_pmaxuw_pmaxud;
#[path = "x86_64/simd/sse/pminsb_pminsd.rs"]
mod x86_64_simd_sse_pminsb_pminsd;
#[path = "x86_64/simd/sse/pminsb_pminsw_pminsd.rs"]
mod x86_64_simd_sse_pminsb_pminsw_pminsd;
#[path = "x86_64/simd/sse/pminub_pminuw_extended.rs"]
mod x86_64_simd_sse_pminub_pminuw_extended;
#[path = "x86_64/simd/sse/pminub_pminuw_pminud.rs"]
mod x86_64_simd_sse_pminub_pminuw_pminud;
#[path = "x86_64/simd/sse/pminuw_pminud.rs"]
mod x86_64_simd_sse_pminuw_pminud;
#[path = "x86_64/simd/sse/pmovmskb.rs"]
mod x86_64_simd_sse_pmovmskb;
#[path = "x86_64/simd/sse/pmovsxbw_pmovsxbd_pmovsxbq.rs"]
mod x86_64_simd_sse_pmovsxbw_pmovsxbd_pmovsxbq;
#[path = "x86_64/simd/sse/pmovsxwd_pmovsxwq_pmovsxdq.rs"]
mod x86_64_simd_sse_pmovsxwd_pmovsxwq_pmovsxdq;
#[path = "x86_64/simd/sse/pmovzxbw_pmovzxbd_pmovzxbq.rs"]
mod x86_64_simd_sse_pmovzxbw_pmovzxbd_pmovzxbq;
#[path = "x86_64/simd/sse/pmovzxwd_pmovzxwq_pmovzxdq.rs"]
mod x86_64_simd_sse_pmovzxwd_pmovzxwq_pmovzxdq;
#[path = "x86_64/simd/sse/pmuldq.rs"]
mod x86_64_simd_sse_pmuldq;
#[path = "x86_64/simd/sse/pmulhrsw.rs"]
mod x86_64_simd_sse_pmulhrsw;
#[path = "x86_64/simd/sse/pmulhuw.rs"]
mod x86_64_simd_sse_pmulhuw;
#[path = "x86_64/simd/sse/pmulhw.rs"]
mod x86_64_simd_sse_pmulhw;
#[path = "x86_64/simd/sse/pmulld.rs"]
mod x86_64_simd_sse_pmulld;
#[path = "x86_64/simd/sse/pmullq.rs"]
mod x86_64_simd_sse_pmullq;
#[path = "x86_64/simd/sse/pmullw.rs"]
mod x86_64_simd_sse_pmullw;
#[path = "x86_64/simd/sse/pmuludq.rs"]
mod x86_64_simd_sse_pmuludq;
#[path = "x86_64/simd/sse/prefetchnta_prefetcht0_prefetcht1_prefetcht2.rs"]
mod x86_64_simd_sse_prefetchnta_prefetcht0_prefetcht1_prefetcht2;
#[path = "x86_64/simd/sse/psadbw.rs"]
mod x86_64_simd_sse_psadbw;
#[path = "x86_64/simd/sse/pshufb.rs"]
mod x86_64_simd_sse_pshufb;
#[path = "x86_64/simd/sse/pshufd.rs"]
mod x86_64_simd_sse_pshufd;
#[path = "x86_64/simd/sse/pshufhw.rs"]
mod x86_64_simd_sse_pshufhw;
#[path = "x86_64/simd/sse/pshuflw.rs"]
mod x86_64_simd_sse_pshuflw;
#[path = "x86_64/simd/sse/pshufw.rs"]
mod x86_64_simd_sse_pshufw;
#[path = "x86_64/simd/sse/psignb_psignw_psignd.rs"]
mod x86_64_simd_sse_psignb_psignw_psignd;
#[path = "x86_64/simd/sse/pslldq_psrldq.rs"]
mod x86_64_simd_sse_pslldq_psrldq;
#[path = "x86_64/simd/sse/psllw_pslld_psllq.rs"]
mod x86_64_simd_sse_psllw_pslld_psllq;
#[path = "x86_64/simd/sse/psraw_psrad.rs"]
mod x86_64_simd_sse_psraw_psrad;
#[path = "x86_64/simd/sse/psrlw_psrld_psrlq.rs"]
mod x86_64_simd_sse_psrlw_psrld_psrlq;
#[path = "x86_64/simd/sse/psubb_psubw_psubd_psubq.rs"]
mod x86_64_simd_sse_psubb_psubw_psubd_psubq;
#[path = "x86_64/simd/sse/psubsb_psubsw.rs"]
mod x86_64_simd_sse_psubsb_psubsw;
#[path = "x86_64/simd/sse/psubusb_psubusw.rs"]
mod x86_64_simd_sse_psubusb_psubusw;
#[path = "x86_64/simd/sse/ptest.rs"]
mod x86_64_simd_sse_ptest;
#[path = "x86_64/simd/sse/punpckhbw_punpckhwd_punpckhdq_punpckhqdq.rs"]
mod x86_64_simd_sse_punpckhbw_punpckhwd_punpckhdq_punpckhqdq;
#[path = "x86_64/simd/sse/punpcklbw_punpcklwd_punpckldq_punpcklqdq.rs"]
mod x86_64_simd_sse_punpcklbw_punpcklwd_punpckldq_punpcklqdq;
#[path = "x86_64/simd/sse/rcpps.rs"]
mod x86_64_simd_sse_rcpps;
#[path = "x86_64/simd/sse/rcpss.rs"]
mod x86_64_simd_sse_rcpss;
#[path = "x86_64/simd/sse/roundps_roundpd.rs"]
mod x86_64_simd_sse_roundps_roundpd;
#[path = "x86_64/simd/sse/roundss_roundsd.rs"]
mod x86_64_simd_sse_roundss_roundsd;
#[path = "x86_64/simd/sse/rsqrtps.rs"]
mod x86_64_simd_sse_rsqrtps;
#[path = "x86_64/simd/sse/rsqrtss.rs"]
mod x86_64_simd_sse_rsqrtss;
#[path = "x86_64/simd/sse/shufpd.rs"]
mod x86_64_simd_sse_shufpd;
#[path = "x86_64/simd/sse/shufps.rs"]
mod x86_64_simd_sse_shufps;
#[path = "x86_64/simd/sse/sqrtps_sqrtpd.rs"]
mod x86_64_simd_sse_sqrtps_sqrtpd;
#[path = "x86_64/simd/sse/sqrtss_sqrtsd.rs"]
mod x86_64_simd_sse_sqrtss_sqrtsd;
#[path = "x86_64/simd/sse/subps_subpd.rs"]
mod x86_64_simd_sse_subps_subpd;
#[path = "x86_64/simd/sse/subss_subsd.rs"]
mod x86_64_simd_sse_subss_subsd;
#[path = "x86_64/simd/sse/ucomiss_ucomisd.rs"]
mod x86_64_simd_sse_ucomiss_ucomisd;
#[path = "x86_64/simd/sse/unpckhpd.rs"]
mod x86_64_simd_sse_unpckhpd;
#[path = "x86_64/simd/sse/unpckhps.rs"]
mod x86_64_simd_sse_unpckhps;
#[path = "x86_64/simd/sse/unpcklpd.rs"]
mod x86_64_simd_sse_unpcklpd;
#[path = "x86_64/simd/sse/unpcklps.rs"]
mod x86_64_simd_sse_unpcklps;
#[path = "x86_64/simd/sse/xorps_xorpd.rs"]
mod x86_64_simd_sse_xorps_xorpd;

// AVX10.1 Tests
#[path = "x86_64/simd/avx10/bf16.rs"]
mod x86_64_simd_avx10_bf16;
#[path = "x86_64/simd/avx10/bitalg.rs"]
mod x86_64_simd_avx10_bitalg;
#[path = "x86_64/simd/avx10/ifma.rs"]
mod x86_64_simd_avx10_ifma;
#[path = "x86_64/simd/avx10/vbmi.rs"]
mod x86_64_simd_avx10_vbmi;
#[path = "x86_64/simd/avx10/vnni.rs"]
mod x86_64_simd_avx10_vnni;
#[path = "x86_64/simd/avx10/vpopcntdq.rs"]
mod x86_64_simd_avx10_vpopcntdq;
#[path = "x86_64/simd/avx10/ymm_embedded_rounding.rs"]
mod x86_64_simd_avx10_ymm_embedded_rounding;

// AVX10.2 Tests
#[path = "x86_64/simd/avx10/compare_bf16.rs"]
mod x86_64_simd_avx10_compare_bf16;
#[path = "x86_64/simd/avx10/copy_sign.rs"]
mod x86_64_simd_avx10_copy_sign;
#[path = "x86_64/simd/avx10/media_accel.rs"]
mod x86_64_simd_avx10_media_accel;
#[path = "x86_64/simd/avx10/minmax.rs"]
mod x86_64_simd_avx10_minmax;
#[path = "x86_64/simd/avx10/saturation_convert.rs"]
mod x86_64_simd_avx10_saturation_convert;
#[path = "x86_64/simd/avx10/vmpsadbw.rs"]
mod x86_64_simd_avx10_vmpsadbw;

// APX (Advanced Performance Extensions)
#[path = "x86_64/apx/ccmp_ctest.rs"]
mod x86_64_apx_ccmp_ctest;
#[path = "x86_64/apx/combined.rs"]
mod x86_64_apx_combined;
#[path = "x86_64/apx/egpr.rs"]
mod x86_64_apx_egpr;
#[path = "x86_64/apx/ndd.rs"]
mod x86_64_apx_ndd;
#[path = "x86_64/apx/nf.rs"]
mod x86_64_apx_nf;
#[path = "x86_64/apx/push2_pop2.rs"]
mod x86_64_apx_push2_pop2;
#[path = "x86_64/apx/rex2.rs"]
mod x86_64_apx_rex2;
#[path = "x86_64/apx/zu.rs"]
mod x86_64_apx_zu;

// Stack Operations
#[path = "x86_64/stack_operations/enter_extended.rs"]
mod x86_64_stack_operations_enter_extended;
#[path = "x86_64/stack_operations/leave_extended.rs"]
mod x86_64_stack_operations_leave_extended;
#[path = "x86_64/stack_operations/pop_mem.rs"]
mod x86_64_stack_operations_pop_mem;
#[path = "x86_64/stack_operations/pop/pop.rs"]
mod x86_64_stack_operations_pop_pop;
#[path = "x86_64/stack_operations/push_imm.rs"]
mod x86_64_stack_operations_push_imm;
#[path = "x86_64/stack_operations/push_mem.rs"]
mod x86_64_stack_operations_push_mem;
#[path = "x86_64/stack_operations/push/push.rs"]
mod x86_64_stack_operations_push_push;
#[path = "x86_64/stack_operations/pusha_popa.rs"]
mod x86_64_stack_operations_pusha_popa;
#[path = "x86_64/stack_operations/pushf_popf_extended.rs"]
mod x86_64_stack_operations_pushf_popf_extended;
#[path = "x86_64/stack_operations/rsp_operations.rs"]
mod x86_64_stack_operations_rsp_operations;
#[path = "x86_64/stack_operations/stack_alignment.rs"]
mod x86_64_stack_operations_stack_alignment;

// String
#[path = "x86_64/string/cmps.rs"]
mod x86_64_string_cmps;
#[path = "x86_64/string/lods.rs"]
mod x86_64_string_lods;
#[path = "x86_64/string/movs.rs"]
mod x86_64_string_movs;
#[path = "x86_64/string/rep_movs.rs"]
mod x86_64_string_rep_movs;
#[path = "x86_64/string/rep_stos.rs"]
mod x86_64_string_rep_stos;
#[path = "x86_64/string/repe_cmps.rs"]
mod x86_64_string_repe_cmps;
#[path = "x86_64/string/repe_scas.rs"]
mod x86_64_string_repe_scas;
#[path = "x86_64/string/repne_cmps.rs"]
mod x86_64_string_repne_cmps;
#[path = "x86_64/string/repne_scas.rs"]
mod x86_64_string_repne_scas;
#[path = "x86_64/string/scas.rs"]
mod x86_64_string_scas;
#[path = "x86_64/string/stos.rs"]
mod x86_64_string_stos;
#[path = "x86_64/string/string_ops.rs"]
mod x86_64_string_string_ops;

// Sync
#[path = "x86_64/sync/cmpxchg.rs"]
mod x86_64_sync_cmpxchg;
#[path = "x86_64/sync/cmpxchg16b_extended.rs"]
mod x86_64_sync_cmpxchg16b_extended;
#[path = "x86_64/sync/cmpxchg8b_cmpxchg16b.rs"]
mod x86_64_sync_cmpxchg8b_cmpxchg16b;
#[path = "x86_64/sync/cmpxchg8b_extended.rs"]
mod x86_64_sync_cmpxchg8b_extended;
#[path = "x86_64/sync/cmpxchg_extended.rs"]
mod x86_64_sync_cmpxchg_extended;
#[path = "x86_64/sync/lfence_ordering.rs"]
mod x86_64_sync_lfence_ordering;
#[path = "x86_64/sync/lock_prefix.rs"]
mod x86_64_sync_lock_prefix;
#[path = "x86_64/sync/mfence_ordering.rs"]
mod x86_64_sync_mfence_ordering;
#[path = "x86_64/sync/xadd.rs"]
mod x86_64_sync_xadd;
#[path = "x86_64/sync/xadd_extended.rs"]
mod x86_64_sync_xadd_extended;
#[path = "x86_64/sync/xchg_extended.rs"]
mod x86_64_sync_xchg_extended;

// System
#[path = "x86_64/system/amx.rs"]
mod x86_64_system_amx;
#[path = "x86_64/system/arpl.rs"]
mod x86_64_system_arpl;
#[path = "x86_64/system/cache_invalidate.rs"]
mod x86_64_system_cache_invalidate;
#[path = "x86_64/system/cet.rs"]
mod x86_64_system_cet;
#[path = "x86_64/system/clac_stac.rs"]
mod x86_64_system_clac_stac;
#[path = "x86_64/system/clts.rs"]
mod x86_64_system_clts;
#[path = "x86_64/system/cpuid.rs"]
mod x86_64_system_cpuid;
#[path = "x86_64/system/fences.rs"]
mod x86_64_system_fences;
#[path = "x86_64/system/hreset_enqcmd.rs"]
mod x86_64_system_hreset_enqcmd;
#[path = "x86_64/system/invd_wbinvd_invlpg.rs"]
mod x86_64_system_invd_wbinvd_invlpg;
#[path = "x86_64/system/invept_invpcid.rs"]
mod x86_64_system_invept_invpcid;
#[path = "x86_64/system/lar.rs"]
mod x86_64_system_lar;
#[path = "x86_64/system/lgdt_lidt.rs"]
mod x86_64_system_lgdt_lidt;
#[path = "x86_64/system/lldt.rs"]
mod x86_64_system_lldt;
#[path = "x86_64/system/lmsw_smsw.rs"]
mod x86_64_system_lmsw_smsw;
#[path = "x86_64/system/lsl.rs"]
mod x86_64_system_lsl;
#[path = "x86_64/system/ltr.rs"]
mod x86_64_system_ltr;
#[path = "x86_64/system/mmu.rs"]
mod x86_64_system_mmu;
#[path = "x86_64/system/mov_cr.rs"]
mod x86_64_system_mov_cr;
#[path = "x86_64/system/mov_dr.rs"]
mod x86_64_system_mov_dr;
#[path = "x86_64/system/page_fault.rs"]
mod x86_64_system_page_fault;
#[path = "x86_64/system/protection_keys.rs"]
mod x86_64_system_protection_keys;
#[path = "x86_64/system/rdfsbase_wrfsbase.rs"]
mod x86_64_system_rdfsbase_wrfsbase;
#[path = "x86_64/system/rdmsr.rs"]
mod x86_64_system_rdmsr;
#[path = "x86_64/system/rdpid.rs"]
mod x86_64_system_rdpid;
#[path = "x86_64/system/rdpkru_wrpkru.rs"]
mod x86_64_system_rdpkru_wrpkru;
#[path = "x86_64/system/rdpmc.rs"]
mod x86_64_system_rdpmc;
#[path = "x86_64/system/rdrand.rs"]
mod x86_64_system_rdrand;
#[path = "x86_64/system/rdseed.rs"]
mod x86_64_system_rdseed;
#[path = "x86_64/system/rdtsc.rs"]
mod x86_64_system_rdtsc;
#[path = "x86_64/system/rdtscp.rs"]
mod x86_64_system_rdtscp;
#[path = "x86_64/system/serialize.rs"]
mod x86_64_system_serialize;
#[path = "x86_64/system/sgdt_sidt.rs"]
mod x86_64_system_sgdt_sidt;
#[path = "x86_64/system/sgx.rs"]
mod x86_64_system_sgx;
#[path = "x86_64/system/sldt.rs"]
mod x86_64_system_sldt;
#[path = "x86_64/system/specialized.rs"]
mod x86_64_system_specialized;
#[path = "x86_64/system/str.rs"]
mod x86_64_system_str;
#[path = "x86_64/system/swapgs.rs"]
mod x86_64_system_swapgs;
#[path = "x86_64/system/system_management.rs"]
mod x86_64_system_system_management;
#[path = "x86_64/system/tsx.rs"]
mod x86_64_system_tsx;
#[path = "x86_64/system/user_mode_wait.rs"]
mod x86_64_system_user_mode_wait;
#[path = "x86_64/system/verr_verw.rs"]
mod x86_64_system_verr_verw;
#[path = "x86_64/system/virtualization.rs"]
mod x86_64_system_virtualization;
#[path = "x86_64/system/wrmsr.rs"]
mod x86_64_system_wrmsr;
#[path = "x86_64/system/xsave_extended.rs"]
mod x86_64_system_xsave_extended;

// LAPIC integration tests
#[path = "x86_64/lapic_integration.rs"]
mod x86_64_lapic_integration;

// Regression tests
#[path = "x86_64/regressions/lazy_flags_pcmpistri.rs"]
mod x86_64_regressions_lazy_flags_pcmpistri;
