#BEGIN_HEADER
#
# Copyright (C) 2020 Mahdi Safsafi.
#
# https://github.com/MahdiSafsafi/opcodesDB
#
# See licence file 'LICENCE' for use and distribution rights.
#
#END_HEADER

use strict;
use warnings;

PAGE XEON_CLEVICT0 => {
  title => 'CLEVICT0-Evict L1 line',
};

PAGE XEON_CLEVICT1 => {
  title => 'CLEVICT1-Evict L2 line',
};

PAGE XEON_DELAY => {
  title => 'DELAY-Stall Thread',
};

PAGE XEON_JKNZD => {
  title => 'JKNZD-Jump near if mask is not zero',
};

PAGE XEON_JKZD => {
  title => 'JKZD-Jump near if mask is zero',
};

PAGE XEON_KAND => {
  title => 'KAND-AND Vector Mask',
};

PAGE XEON_KANDN => {
  title => 'KANDN-AND NOT Vector Mask',
};

PAGE XEON_KANDNR => {
  title => 'KANDNR-Reverse AND NOT Vector Mask',
};

PAGE XEON_KCONCATH => {
  title => 'KCONCATH-Pack and Move High Vector Mask',
};

PAGE XEON_KCONCATL => {
  title => 'KCONCATL-Pack and Move Low Vector Mask',
};

PAGE XEON_KEXTRACT => {
  title => 'KEXTRACT-Extract Vector Mask From Register',
};

PAGE XEON_KMERGE2L1H => {
  title => 'KMERGE2L1H-Swap and Merge High Element Portion and Low Portion of Vector Masks',
};

PAGE XEON_KMERGE2L1L => {
  title => 'KMERGE2L1L-Move Low Element Portion into High Portion of Vector Mask',
};

PAGE XEON_KMOV => {
  title => 'KMOV-Move Vector Mask',
};

PAGE XEON_KNOT => {
  title => 'KNOT-Not Vector Mask',
};

PAGE XEON_KOR => {
  title => 'KOR-OR Vector Masks',
};

PAGE XEON_KORTEST => {
  title => 'KORTEST-OR Vector Mask And Set EFLAGS',
};

PAGE XEON_KXNOR => {
  title => 'KXNOR-XNOR Vector Masks',
};

PAGE XEON_KXOR => {
  title => 'KXOR-XOR Vector Masks',
};

PAGE XEON_LZCNT => {
  title => 'LZCNT-Leading Zero Count',
};

PAGE XEON_POPCNT => {
  title => 'POPCNT-Return the Count of Number of Bits Set to 1',
};

PAGE XEON_PREFETCHWT1 => {
  title => 'PREFETCHWT1-Prefetch Vector Data Into Caches with Intent to Write and T1 Hint',
};

PAGE XEON_SPFLT => {
  title => 'SPFLT-Set performance monitor filtering mask',
};

PAGE XEON_TZCNT => {
  title => 'TZCNT-Trailing Zero Count',
};

PAGE XEON_TZCNTI => {
  title => 'TZCNTI-Initialized Trailing Zero Count',
};

PAGE XEON_V4FMADDPS => {
  title => 'V4FMADDPS-Packed Single-Precision Floating-Point Fused Multiply-Add (4-iterations)',
};

PAGE XEON_V4FMADDSS => {
  title => 'V4FMADDSS-Scalar Single-Precision Floating-Point Fused Multiply-Add (4-iterations)',
};

PAGE XEON_V4FNMADDPS => {
  title => 'V4FNMADDPS-Packed Single-Precision Floating-Point Fused Multiply-Add (4-iterations)',
};

PAGE XEON_V4FNMADDSS => {
  title => 'V4FNMADDSS-Scalar Single-Precision Floating-Point Fused Multiply-Add (4-iterations)',
};

PAGE XEON_VADDNPD => {
  title => 'VADDNPD-Add and Negate Float64 Vectors',
};

PAGE XEON_VADDNPS => {
  title => 'VADDNPS-Add and Negate Float32 Vectors',
};

PAGE XEON_VADDPD => {
  title => 'VADDPD-Add Float64 Vectors',
};

PAGE XEON_VADDPS => {
  title => 'VADDPS-Add Float32 Vectors',
};

PAGE XEON_VADDSETSPS => {
  title => 'VADDSETSPS-Add Float32 Vectors and Set Mask to Sign',
};

PAGE XEON_VALIGND => {
  title => 'VALIGND-Align Doubleword Vectors',
};

PAGE XEON_VBLENDMPD => {
  title => 'VBLENDMPD-Blend Float64 Vectors using the Instruction Mask',
};

PAGE XEON_VBLENDMPS => {
  title => 'VBLENDMPS-Blend Float32 Vectors using the Instruction Mask',
};

PAGE XEON_VBROADCASTF32X4 => {
  title => 'VBROADCASTF32X4-Broadcast 4xFloat32 Vector',
};

PAGE XEON_VBROADCASTF64X4 => {
  title => 'VBROADCASTF64X4-Broadcast 4xFloat64 Vector',
};

PAGE XEON_VBROADCASTI32X4 => {
  title => 'VBROADCASTI32X4-Broadcast 4xInt32 Vector',
};

PAGE XEON_VBROADCASTI64X4 => {
  title => 'VBROADCASTI64X4-Broadcast 4xInt64 Vector',
};

PAGE XEON_VBROADCASTSD => {
  title => 'VBROADCASTSD-Broadcast Float64 Vector',
};

PAGE XEON_VBROADCASTSS => {
  title => 'VBROADCASTSS-Broadcast Float32 Vector',
};

PAGE XEON_VCMPPD => {
  title => 'VCMPPD-Compare Float64 Vectors and Set Vector Mask',
};

PAGE XEON_VCMPPS => {
  title => 'VCMPPS-Compare Float32 Vectors and Set Vector Mask',
};

PAGE XEON_VCVTDQ2PD => {
  title => 'VCVTDQ2PD-Convert Int32 Vector to Float64 Vector',
};

PAGE XEON_VCVTFXPNTDQ2PS => {
  title => 'VCVTFXPNTDQ2PS-Convert Fixed Point Int32 Vector to Float32 Vector',
};

PAGE XEON_VCVTFXPNTPD2DQ => {
  title => 'VCVTFXPNTPD2DQ-Convert Float64 Vector to Fixed Point Int32 Vector',
};

PAGE XEON_VCVTFXPNTPD2UDQ => {
  title => 'VCVTFXPNTPD2UDQ-Convert Float64 Vector to Fixed Point Uint32 Vector',
};

PAGE XEON_VCVTFXPNTPS2DQ => {
  title => 'VCVTFXPNTPS2DQ-Convert Float32 Vector to Fixed Point Int32 Vector',
};

PAGE XEON_VCVTFXPNTPS2UDQ => {
  title => 'VCVTFXPNTPS2UDQ-Convert Float32 Vector to Fixed Point Uint32 Vector',
};

PAGE XEON_VCVTFXPNTUDQ2PS => {
  title => 'VCVTFXPNTUDQ2PS-Convert Fixed Point Uint32 Vector to Float32 Vector',
};

PAGE XEON_VCVTPD2PS => {
  title => 'VCVTPD2PS-Convert Float64 Vector to Float32 Vector',
};

PAGE XEON_VCVTPS2PD => {
  title => 'VCVTPS2PD-Convert Float32 Vector to Float64 Vector',
};

PAGE XEON_VCVTUDQ2PD => {
  title => 'VCVTUDQ2PD-Convert Uint32 Vector to Float64 Vector',
};

PAGE XEON_VEXP223PS => {
  title => 'VEXP223PS-Base-2 Exponential Calculation of Float32 Vector',
};

PAGE XEON_VEXP2PD => {
  title => 'VEXP2PD-Approximation to the Exponential 2^x of Packed Double-Precision Floating-Point Values with Less Than 2^-23 Relative Error',
};

PAGE XEON_VEXP2PS => {
  title => 'VEXP2PS-Approximation to the Exponential 2^x of Packed Single-Precision Floating-Point Values with Less Than 2^-23 Relative Error',
};

PAGE XEON_VFIXUPNANPD => {
  title => 'VFIXUPNANPD-Fix Up Special Float64 Vector Numbers With NaN Passthrough',
};

PAGE XEON_VFIXUPNANPS => {
  title => 'VFIXUPNANPS-Fix Up Special Float32 Vector Numbers With NaN Passthrough',
};

PAGE XEON_VFMADD132PD => {
  title => 'VFMADD132PD-Multiply Destination By Second Source and Add To First Source Float64 Vectors 201',
};

PAGE XEON_VFMADD132PS => {
  title => 'VFMADD132PS-Multiply Destination By Second Source and Add To First Source Float32 Vectors 205',
};

PAGE XEON_VFMADD213PD => {
  title => 'VFMADD213PD-Multiply First Source By Destination and Add Second Source Float64 Vectors . 208',
};

PAGE XEON_VFMADD213PS => {
  title => 'VFMADD213PS-Multiply First Source By Destination and Add Second Source Float32 Vectors . . 212',
};

PAGE XEON_VFMADD231PD => {
  title => 'VFMADD231PD-Multiply First Source By Second Source and Add To Destination Float64 Vectors 216',
};

PAGE XEON_VFMADD231PS => {
  title => 'VFMADD231PS-Multiply First Source By Second Source and Add To Destination Float32 Vectors 220',
};

PAGE XEON_VFMADD233PS => {
  title => 'VFMADD233PS-Multiply First Source By Specially Swizzled Second Source and Add To Second Source Float32 Vectors',
};

PAGE XEON_VFMSUB132PD => {
  title => 'VFMSUB132PD-Multiply Destination By Second Source and Subtract First Source Float64 Vectors228',
};

PAGE XEON_VFMSUB132PS => {
  title => 'VFMSUB132PS-Multiply Destination By Second Source and Subtract First Source Float32 Vectors232',
};

PAGE XEON_VFMSUB213PD => {
  title => 'VFMSUB213PD-Multiply First Source By Destination and Subtract Second Source Float64 Vectors235',
};

PAGE XEON_VFMSUB213PS => {
  title => 'VFMSUB213PS-Multiply First Source By Destination and Subtract Second Source Float32 Vectors239',
};

PAGE XEON_VFMSUB231PD => {
  title => 'VFMSUB231PD-Multiply First Source By Second Source and Subtract Destination Float64 Vectors242',
};

PAGE XEON_VFMSUB231PS => {
  title => 'VFMSUB231PS-Multiply First Source By Second Source and Subtract Destination Float32 Vectors246',
};

PAGE XEON_VFNMADD132PD => {
  title => 'VFNMADD132PD-Multiply Destination By Second Source and Subtract From First Source Float64 Vectors',
};

PAGE XEON_VFNMADD132PS => {
  title => 'VFNMADD132PS-Multiply Destination By Second Source and Subtract From First Source Float32 Vectors',
};

PAGE XEON_VFNMADD213PD => {
  title => 'VFNMADD213PD-Multiply First Source By Destination and Subtract From Second Source Float64 Vectors',
};

PAGE XEON_VFNMADD213PS => {
  title => 'VFNMADD213PS-Multiply First Source By Destination and Subtract From Second Source Float32 Vectors',
};

PAGE XEON_VFNMADD231PD => {
  title => 'VFNMADD231PD-Multiply First Source By Second Source and Subtract From Destination Float64 Vectors',
};

PAGE XEON_VFNMADD231PS => {
  title => 'VFNMADD231PS-Multiply First Source By Second Source and Subtract From Destination Float32 Vectors',
};

PAGE XEON_VFNMSUB132PD => {
  title => 'VFNMSUB132PD-Multiply Destination By Second Source, Negate, and Subtract First Source Float64 Vectors',
};

PAGE XEON_VFNMSUB132PS => {
  title => 'VFNMSUB132PS-Multiply Destination By Second Source, Negate, and Subtract First Source Float32 Vectors',
};

PAGE XEON_VFNMSUB213PD => {
  title => 'VFNMSUB213PD-Multiply First Source By Destination, Negate, and Subtract Second Source Float64 Vectors',
};

PAGE XEON_VFNMSUB213PS => {
  title => 'VFNMSUB213PS-Multiply First Source By Destination, Negate, and Subtract Second Source Float32 Vectors',
};

PAGE XEON_VFNMSUB231PD => {
  title => 'VFNMSUB231PD-Multiply First Source By Second Source, Negate, and Subtract Destination Float64 Vectors',
};

PAGE XEON_VFNMSUB231PS => {
  title => 'VFNMSUB231PS-Multiply First Source By Second Source, Negate, and Subtract Destination Float32 Vectors',
};

PAGE XEON_VGATHERDPD => {
  title => 'VGATHERDPD-Gather Float64 Vector With Signed Dword Indices',
};

PAGE XEON_VGATHERDPS => {
  title => 'VGATHERDPS-Gather Float32 Vector With Signed Dword Indices',
};

PAGE XEON_VGATHERPF0DPD => {
  title => 'VGATHERPF0DPD-Sparse Prefetch Packed SP/DP Data Values with Signed Dword, Signed Qword Indices Using T0 Hint',
};

PAGE XEON_VGATHERPF0DPS => {
  title => 'VGATHERPF0DPS-Gather Prefetch Float32 Vector With Signed Dword Indices Into L1',
};

PAGE XEON_VGATHERPF0HINTDPD => {
  title => 'VGATHERPF0HINTDPD-Gather Prefetch Float64 Vector Hint With Signed Dword Indices',
};

PAGE XEON_VGATHERPF0HINTDPS => {
  title => 'VGATHERPF0HINTDPS-Gather Prefetch Float32 Vector Hint With Signed Dword Indices',
};

PAGE XEON_VGATHERPF0QPD => {
  title => 'VGATHERPF0QPD-Sparse Prefetch Packed SP/DP Data Values with Signed Dword, Signed Qword Indices Using T0 Hint',
};

PAGE XEON_VGATHERPF0QPS => {
  title => 'VGATHERPF0QPS-Sparse Prefetch Packed SP/DP Data Values with Signed Dword, Signed Qword Indices Using T0 Hint',
};

PAGE XEON_VGATHERPF1DPD => {
  title => 'VGATHERPF1DPD-Sparse Prefetch Packed SP/DP Data Values with Signed Dword, Signed Qword Indices Using T1 Hint',
};

PAGE XEON_VGATHERPF1DPS => {
  title => 'VGATHERPF1DPS-Gather Prefetch Float32 Vector With Signed Dword Indices Into L2',
};

PAGE XEON_VGATHERPF1QPD => {
  title => 'VGATHERPF1QPD-Sparse Prefetch Packed SP/DP Data Values with Signed Dword, Signed Qword Indices Using T1 Hint',
};

PAGE XEON_VGATHERPF1QPS => {
  title => 'VGATHERPF1QPS-Sparse Prefetch Packed SP/DP Data Values with Signed Dword, Signed Qword Indices Using T1 Hint',
};

PAGE XEON_VGETEXPPD => {
  title => 'VGETEXPPD-Extract Float64 Vector of Exponents from Float64 Vector',
};

PAGE XEON_VGETEXPPS => {
  title => 'VGETEXPPS-Extract Float32 Vector of Exponents from Float32 Vector',
};

PAGE XEON_VGETMANTPD => {
  title => 'VGETMANTPD-Extract Float64 Vector of Normalized Mantissas from Float64 Vector',
};

PAGE XEON_VGETMANTPS => {
  title => 'VGETMANTPS-Extract Float32 Vector of Normalized Mantissas from Float32 Vector',
};

PAGE XEON_VGMAXABSPS => {
  title => 'VGMAXABSPS-Absolute Maximum of Float32 Vectors',
};

PAGE XEON_VGMAXPD => {
  title => 'VGMAXPD-Maximum of Float64 Vectors',
};

PAGE XEON_VGMAXPS => {
  title => 'VGMAXPS-Maximum of Float32 Vectors',
};

PAGE XEON_VGMINPD => {
  title => 'VGMINPD-Minimum of Float64 Vectors',
};

PAGE XEON_VGMINPS => {
  title => 'VGMINPS-Minimum of Float32 Vectors',
};

PAGE XEON_VLOADUNPACKHD => {
  title => 'VLOADUNPACKHD-Load Unaligned High And Unpack To Doubleword Vector',
};

PAGE XEON_VLOADUNPACKHPD => {
  title => 'VLOADUNPACKHPD-Load Unaligned High And Unpack To Float64 Vector',
};

PAGE XEON_VLOADUNPACKHPS => {
  title => 'VLOADUNPACKHPS-Load Unaligned High And Unpack To Float32 Vector',
};

PAGE XEON_VLOADUNPACKHQ => {
  title => 'VLOADUNPACKHQ-Load Unaligned High And Unpack To Int64 Vector',
};

PAGE XEON_VLOADUNPACKLD => {
  title => 'VLOADUNPACKLD-Load Unaligned Low And Unpack To Doubleword Vector',
};

PAGE XEON_VLOADUNPACKLPD => {
  title => 'VLOADUNPACKLPD-Load Unaligned Low And Unpack To Float64 Vector',
};

PAGE XEON_VLOADUNPACKLPS => {
  title => 'VLOADUNPACKLPS-Load Unaligned Low And Unpack To Float32 Vector',
};

PAGE XEON_VLOADUNPACKLQ => {
  title => 'VLOADUNPACKLQ-Load Unaligned Low And Unpack To Int64 Vector',
};

PAGE XEON_VLOG2PS => {
  title => 'VLOG2PS-Vector Logarithm Base-2 of Float32 Vector',
};

PAGE XEON_VMOVAPD => {
  title => 'VMOVAPD-Move Aligned Float64 Vector',
};

PAGE XEON_VMOVAPS => {
  title => 'VMOVAPS-Move Aligned Float32 Vector',
};

PAGE XEON_VMOVDQA32 => {
  title => 'VMOVDQA32-Move Aligned Int32 Vector',
};

PAGE XEON_VMOVDQA64 => {
  title => 'VMOVDQA64-Move Aligned Int64 Vector',
};

PAGE XEON_VMOVNRAPD => {
  title => 'VMOVNRAPD-Store Aligned Float64 Vector With No-Read Hint',
};

PAGE XEON_VMOVNRAPS => {
  title => 'VMOVNRAPS-Store Aligned Float32 Vector With No-Read Hint',
};

PAGE XEON_VMOVNRNGOAPD => {
  title => 'VMOVNRNGOAPD-Non-globally Ordered Store Aligned Float64 Vector With No-Read Hint',
};

PAGE XEON_VMOVNRNGOAPS => {
  title => 'VMOVNRNGOAPS-Non-globally Ordered Store Aligned Float32 Vector With No-Read Hint',
};

PAGE XEON_VMULPD => {
  title => 'VMULPD-Multiply Float64 Vectors',
};

PAGE XEON_VMULPS => {
  title => 'VMULPS-Multiply Float32 Vectors',
};

PAGE XEON_VP4DPWSSD => {
  title => 'VP4DPWSSD-Dot Product of Signed Words with Dword Accumulation (4-iterations)',
};

PAGE XEON_VP4DPWSSDS => {
  title => 'VP4DPWSSDS-Dot Product of Signed Words with Dword Accumulation and Saturation (4-iterations)',
};

PAGE XEON_VPACKSTOREHD => {
  title => 'VPACKSTOREHD-Pack And Store Unaligned High From Int32 Vector',
};

PAGE XEON_VPACKSTOREHPD => {
  title => 'VPACKSTOREHPD-Pack And Store Unaligned High From Float64 Vector',
};

PAGE XEON_VPACKSTOREHPS => {
  title => 'VPACKSTOREHPS-Pack And Store Unaligned High From Float32 Vector',
};

PAGE XEON_VPACKSTOREHQ => {
  title => 'VPACKSTOREHQ-Pack And Store Unaligned High From Int64 Vector',
};

PAGE XEON_VPACKSTORELD => {
  title => 'VPACKSTORELD-Pack and Store Unaligned Low From Int32 Vector',
};

PAGE XEON_VPACKSTORELPD => {
  title => 'VPACKSTORELPD-Pack and Store Unaligned Low From Float64 Vector',
};

PAGE XEON_VPACKSTORELPS => {
  title => 'VPACKSTORELPS-Pack and Store Unaligned Low From Float32 Vector',
};

PAGE XEON_VPACKSTORELQ => {
  title => 'VPACKSTORELQ-Pack and Store Unaligned Low From Int64 Vector',
};

PAGE XEON_VPADCD => {
  title => 'VPADCD-Add Int32 Vectors with Carry',
};

PAGE XEON_VPADDD => {
  title => 'VPADDD-Add Int32 Vectors',
};

PAGE XEON_VPADDSETCD => {
  title => 'VPADDSETCD-Add Int32 Vectors and Set Mask to Carry',
};

PAGE XEON_VPADDSETSD => {
  title => 'VPADDSETSD-Add Int32 Vectors and Set Mask to Sign',
};

PAGE XEON_VPBLENDMD => {
  title => 'VPBLENDMD-Blend Int32 Vectors using the Instruction Mask',
};

PAGE XEON_VPBLENDMQ => {
  title => 'VPBLENDMQ-Blend Int64 Vectors using the Instruction Mask',
};

PAGE XEON_VPBROADCASTD => {
  title => 'VPBROADCASTD-Broadcast Int32 Vector',
};

PAGE XEON_VPBROADCASTQ => {
  title => 'VPBROADCASTQ-Broadcast Int64 Vector',
};

PAGE XEON_VPCMPD => {
  title => 'VPCMPD-Compare Int32 Vectors and Set Vector Mask',
};

PAGE XEON_VPCMPEQD => {
  title => 'VPCMPEQD-Compare Equal Int32 Vectors and Set Vector Mask',
};

PAGE XEON_VPCMPGTD => {
  title => 'VPCMPGTD-Compare Greater Than Int32 Vectors and Set Vector Mask',
};

PAGE XEON_VPCMPLTD => {
  title => 'VPCMPLTD-Compare Less Than Int32 Vectors and Set Vector Mask',
};

PAGE XEON_VPCMPUD => {
  title => 'VPCMPUD-Compare Uint32 Vectors and Set Vector Mask',
};

PAGE XEON_VPERMD => {
  title => 'VPERMD-Permutes Int32 Vectors',
};

PAGE XEON_VPERMF32X4 => {
  title => 'VPERMF32X4-Shuffle Vector Dqwords',
};

PAGE XEON_VPGATHERDD => {
  title => 'VPGATHERDD-Gather Int32 Vector With Signed Dword Indices',
};

PAGE XEON_VPGATHERDQ => {
  title => 'VPGATHERDQ-Gather Int64 Vector With Signed Dword Indices',
};

PAGE XEON_VPMADD231D => {
  title => 'VPMADD231D-Multiply First Source By Second Source and Add To Destination Int32 Vectors',
};

PAGE XEON_VPMADD233D => {
  title => 'VPMADD233D-Multiply First Source By Specially Swizzled Second Source and Add To Second Source Int32 Vectors',
};

PAGE XEON_VPMAXSD => {
  title => 'VPMAXSD-Maximum of Int32 Vectors',
};

PAGE XEON_VPMAXUD => {
  title => 'VPMAXUD-Maximum of Uint32 Vectors',
};

PAGE XEON_VPMINSD => {
  title => 'VPMINSD-Minimum of Int32 Vectors',
};

PAGE XEON_VPMINUD => {
  title => 'VPMINUD-Minimum of Uint32 Vectors',
};

PAGE XEON_VPMULHD => {
  title => 'VPMULHD-Multiply Int32 Vectors And Store High Result',
};

PAGE XEON_VPMULHUD => {
  title => 'VPMULHUD-Multiply Uint32 Vectors And Store High Result',
};

PAGE XEON_VPMULLD => {
  title => 'VPMULLD-Multiply Int32 Vectors And Store Low Result',
};

PAGE XEON_VPREFETCH0 => {
  title => 'VPREFETCH0-Prefetch memory line using T0 hint',
};

PAGE XEON_VPREFETCH1 => {
  title => 'VPREFETCH1-Prefetch memory line using T1 hint',
};

PAGE XEON_VPREFETCH2 => {
  title => 'VPREFETCH2-Prefetch memory line using T2 hint',
};

PAGE XEON_VPREFETCHE0 => {
  title => 'VPREFETCHE0-Prefetch memory line using T0 hint, with intent to write',
};

PAGE XEON_VPREFETCHE1 => {
  title => 'VPREFETCHE1-Prefetch memory line using T1 hint, with intent to write',
};

PAGE XEON_VPREFETCHE2 => {
  title => 'VPREFETCHE2-Prefetch memory line using T2 hint, with intent to write',
};

PAGE XEON_VPREFETCHENTA => {
  title => 'VPREFETCHENTA-Prefetch memory line using NTA hint, with intent to write',
};

PAGE XEON_VPREFETCHNTA => {
  title => 'VPREFETCHNTA-Prefetch memory line using NTA hint',
};

PAGE XEON_VPSBBD => {
  title => 'VPSBBD-Subtract Int32 Vectors with Borrow',
};

PAGE XEON_VPSBBRD => {
  title => 'VPSBBRD-Reverse Subtract Int32 Vectors with Borrow',
};

PAGE XEON_VPSCATTERDD => {
  title => 'VPSCATTERDD-Scatter Int32 Vector With Signed Dword Indices',
};

PAGE XEON_VPSCATTERDQ => {
  title => 'VPSCATTERDQ-Scatter Int64 Vector With Signed Dword Indices',
};

PAGE XEON_VPSHUFD => {
  title => 'VPSHUFD-Shuffle Vector Doublewords',
};

PAGE XEON_VPSLLD => {
  title => 'VPSLLD-Shift Int32 Vector Immediate Left Logical',
};

PAGE XEON_VPSLLVD => {
  title => 'VPSLLVD-Shift Int32 Vector Left Logical',
};

PAGE XEON_VPSRAD => {
  title => 'VPSRAD-Shift Int32 Vector Immediate Right Arithmetic',
};

PAGE XEON_VPSRAVD => {
  title => 'VPSRAVD-Shift Int32 Vector Right Arithmetic',
};

PAGE XEON_VPSRLD => {
  title => 'VPSRLD-Shift Int32 Vector Immediate Right Logical',
};

PAGE XEON_VPSRLVD => {
  title => 'VPSRLVD-Shift Int32 Vector Right Logical',
};

PAGE XEON_VPSUBD => {
  title => 'VPSUBD-Subtract Int32 Vectors',
};

PAGE XEON_VPSUBRD => {
  title => 'VPSUBRD-Reverse Subtract Int32 Vectors',
};

PAGE XEON_VPSUBRSETBD => {
  title => 'VPSUBRSETBD-Reverse Subtract Int32 Vectors and Set Borrow',
};

PAGE XEON_VPSUBSETBD => {
  title => 'VPSUBSETBD-Subtract Int32 Vectors and Set Borrow',
};

PAGE XEON_VPTESTMD => {
  title => 'VPTESTMD-Logical AND Int32 Vectors and Set Vector Mask',
};

PAGE XEON_VRCP23PS => {
  title => 'VRCP23PS-Reciprocal of Float32 Vector',
};

PAGE XEON_VRCP28PD => {
  title => 'VRCP28PD-Approximation to the Reciprocal of Packed Double-Precision Floating-Point Values with Less Than 2^-28 Relative Error',
};

PAGE XEON_VRCP28PS => {
  title => 'VRCP28PS-Approximation to the Reciprocal of Packed Single-Precision Floating-Point Values with Less Than 2^-28 Relative Error',
};

PAGE XEON_VRCP28SD => {
  title => 'VRCP28SD-Approximation to the Reciprocal of Scalar Double-Precision Floating-Point Value with Less Than 2^-28 Relative Error',
};

PAGE XEON_VRCP28SS => {
  title => 'VRCP28SS-Approximation to the Reciprocal of Scalar Single-Precision Floating-Point Value with Less Than 2^-28 Relative Error',
};

PAGE XEON_VRNDFXPNTPD => {
  title => 'VRNDFXPNTPD-Round Float64 Vector',
};

PAGE XEON_VRNDFXPNTPS => {
  title => 'VRNDFXPNTPS-Round Float32 Vector',
};

PAGE XEON_VRSQRT23PS => {
  title => 'VRSQRT23PS-Vector Reciprocal Square Root of Float32 Vector',
};

PAGE XEON_VRSQRT28PD => {
  title => 'VRSQRT28PD-Approximation to the Reciprocal Square Root of Packed Double-Precision Floating-Point Values with Less Than 2^-28 Relative Error',
};

PAGE XEON_VRSQRT28PS => {
  title => 'VRSQRT28PS-Approximation to the Reciprocal Square Root of Packed Single-Precision Floating-Point Values with Less Than 2^-28 Relative Error',
};

PAGE XEON_VRSQRT28SD => {
  title => 'VRSQRT28SD-Approximation to the Reciprocal Square Root of Scalar Double-Precision Floating-Point Value with Less Than 2^-28 Relative Error',
};

PAGE XEON_VRSQRT28SS => {
  title => 'VRSQRT28SS-Approximation to the Reciprocal Square Root of Scalar Single-Precision Floating- Point Value with Less Than 2^-28 Relative Error',
};

PAGE XEON_VSCALEPS => {
  title => 'VSCALEPS-Scale Float32 Vectors',
};

PAGE XEON_VSCATTERDPD => {
  title => 'VSCATTERDPD-Scatter Float64 Vector With Signed Dword Indices',
};

PAGE XEON_VSCATTERDPS => {
  title => 'VSCATTERDPS-Scatter Float32 Vector With Signed Dword Indices',
};

PAGE XEON_VSCATTERPF0DPD => {
  title => 'VSCATTERPF0DPD-Sparse Prefetch Packed SP/DP Data Values with Signed Dword, Signed Qword Indices Using T0 Hint with Intent to Write',
};

PAGE XEON_VSCATTERPF0DPS => {
  title => 'VSCATTERPF0DPS-Scatter Prefetch Float32 Vector With Signed Dword Indices Into L1',
};

PAGE XEON_VSCATTERPF0HINTDPD => {
  title => 'VSCATTERPF0HINTDPD-Scatter Prefetch Float64 Vector Hint With Signed Dword Indices',
};

PAGE XEON_VSCATTERPF0HINTDPS => {
  title => 'VSCATTERPF0HINTDPS-Scatter Prefetch Float32 Vector Hint With Signed Dword Indices',
};

PAGE XEON_VSCATTERPF0QPD => {
  title => 'VSCATTERPF0QPD-Sparse Prefetch Packed SP/DP Data Values with Signed Dword, Signed Qword Indices Using T0 Hint with Intent to Write',
};

PAGE XEON_VSCATTERPF0QPS => {
  title => 'VSCATTERPF0QPS-Sparse Prefetch Packed SP/DP Data Values with Signed Dword, Signed Qword Indices Using T0 Hint with Intent to Write',
};

PAGE XEON_VSCATTERPF1DPD => {
  title => 'VSCATTERPF1DPD-Sparse Prefetch Packed SP/DP Data Values with Signed Dword, Signed Qword Indices Using T1 Hint with Intent to Write',
};

PAGE XEON_VSCATTERPF1DPS => {
  title => 'VSCATTERPF1DPS-Scatter Prefetch Float32 Vector With Signed Dword Indices Into L2',
};

PAGE XEON_VSCATTERPF1QPD => {
  title => 'VSCATTERPF1QPD-Sparse Prefetch Packed SP/DP Data Values with Signed Dword, Signed Qword Indices Using T1 Hint with Intent to Write',
};

PAGE XEON_VSCATTERPF1QPS => {
  title => 'VSCATTERPF1QPS-Sparse Prefetch Packed SP/DP Data Values with Signed Dword, Signed Qword Indices Using T1 Hint with Intent to Write',
};

PAGE XEON_VSUBPD => {
  title => 'VSUBPD-Subtract Float64 Vectors',
};

PAGE XEON_VSUBPS => {
  title => 'VSUBPS-Subtract Float32 Vectors',
};

PAGE XEON_VSUBRPD => {
  title => 'VSUBRPD-Reverse Subtract Float64 Vectors',
};

PAGE XEON_VSUBRPS => {
  title => 'VSUBRPS-Reverse Subtract Float32 Vectors',
};

