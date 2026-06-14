### **1. Legacy & General Purpose Arithmetic**
*   **Legacy BCD/ASCII Adjust:** `aaa`, `aad`, `aam`, `aas`
*   **Legacy Decimal Adjust:** `daa`, `das`
*   **Integer Addition (Carry):** `adc`, `adcx`, `add`, `adox`
*   **Integer Subtraction:** `dec`, `inc`, `neg`, `sbb`
*   **Integer Subtraction (Base):** `sub`
*   **Integer Multiplication:** `imul`, `mul`, `mulx`
*   **Integer Division:** `div`, `idiv`
*   **Sign Extension (Word/Dword):** `cbw`, `cdq`, `cwde`, `cqo`
*   **Sign Extension (Word/Dword) 2:** `cwd`

### **2. Logic & Bit Manipulation**
*   **Basic Logic:** `and`, `not`, `or`, `test`
*   **Basic Logic (XOR):** `xor`
*   **Bit Testing:** `bt`, `btc`, `btr`, `bts`
*   **Bit Scanning:** `bsf`, `bsr`
*   **Bit Manipulation (BMI1):** `bextr`, `blsi`, `blsmsk`, `blsr`
*   **Bit Manipulation (BMI2):** `bzhi`, `pdep`, `pext`
*   **Bit Counting/Swap:** `bswap`, `lzcnt`, `tzcnt`
*   **Shifts (Logical):** `shl`, `shr`, `shlx`, `shrx`
*   **Shifts (Arithmetic):** `sal`, `sar`, `sarx`
*   **Shifts (Double Precision):** `shld`, `shrd`
*   **Rotates (Basic):** `rcl`, `rcr`, `rol`, `ror`
*   **Rotates (Advanced):** `rorx`

### **3. Control Flow & Stack**
*   **Jumps & Calls:** `call`, `jcc`, `jmp`, `ret`
*   **Loops:** `loop`, `loopcc`
*   **Interrupts:** `int1`, `int3`, `intn`, `into`
*   **Interrupt Return:** `iret`, `iretd`, `iretq`
*   **Stack Operations (Push):** `push`, `pusha`, `pushad`, `pushf`
*   **Stack Operations (Push 2):** `pushfd`, `pushfq`
*   **Stack Operations (Pop):** `pop`, `popa`, `popad`, `popf`
*   **Stack Operations (Pop 2):** `popfd`, `popfq`
*   **Stack Frame:** `enter`, `leave`
*   **System Control Flow:** `bound`, `ud`

### **4. Data Movement & Strings**
*   **General Move:** `mov`, `movbe`, `movsx`, `movsxd`
*   **General Move/Exchange:** `movzx`, `xchg`, `cmovcc`, `lea`
*   **Translation:** `xlat`, `xlatb`
*   **String Comparison:** `cmps`, `cmpsb`, `cmpsd`, `cmpsq`
*   **String Comparison 2:** `cmpss`, `cmpsw`
*   **String Scan:** `scas`, `scasb`, `scasd`, `scasw`
*   **String Load:** `lods`, `lodsb`, `lodsd`, `lodsq`
*   **String Load 2:** `lodsw`
*   **String Store:** `stos`, `stosb`, `stosd`, `stosq`
*   **String Store 2:** `stosw`
*   **String Move:** `movs`, `movsb`, `movsd`, `movsq`
*   **String Move 2:** `movsw`, `movshdup`, `movsldup`, `movss`
*   **I/O Port Strings:** `ins`, `insb`, `insd`, `insw`
*   **I/O Port Strings 2:** `outs`, `outsb`, `outsd`, `outsw`
*   **Repeat Prefixes:** `rep`, `repe`, `repne`, `repnz`
*   **Repeat Prefixes 2:** `repz`

### **5. x87 Floating Point (FPU)**
*   **FPU Basic Math:** `fadd`, `faddp`, `fiadd`, `fsub`
*   **FPU Basic Math 2:** `fsubp`, `fisub`, `fsubr`, `fsubrp`
*   **FPU Basic Math 3:** `fisubr`, `fmul`, `fmulp`, `fimul`
*   **FPU Basic Math 4:** `fdiv`, `fdivp`, `fdivr`, `fdivrp`
*   **FPU Basic Math 5:** `fidiv`, `fidivr`, `fsqrt`, `fabs`
*   **FPU Basic Math 6:** `fchs`, `frndint`, `fscale`, `fprem`
*   **FPU Basic Math 7:** `fprem1`
*   **FPU Transcendental:** `fsin`, `fcos`, `fsincos`, `fpatan`
*   **FPU Transcendental 2:** `fptan`, `f2xm1`, `fyl2x`, `fyl2xp1`
*   **FPU Transcendental 3:** `fxtract`
*   **FPU Load/Store:** `fld`, `fst`, `fstp`, `fild`
*   **FPU Load/Store 2:** `fist`, `fistp`, `fisttp`, `fbld`
*   **FPU Load/Store 3:** `fbstp`
*   **FPU Constants:** `fld1`, `fldl2e`, `fldl2t`, `fldlg2`
*   **FPU Constants 2:** `fldln2`, `fldpi`, `fldz`
*   **FPU Comparison:** `fcom`, `fcomi`, `fcomip`, `fcomp`
*   **FPU Comparison 2:** `fcompp`, `ficom`, `ficomp`, `ftst`
*   **FPU Comparison 3:** `fucom`, `fucomi`, `fucomip`, `fucomp`
*   **FPU Comparison 4:** `fucompp`, `fxam`
*   **FPU State/Control:** `finit`, `fninit`, `fldcw`, `fstcw`
*   **FPU State/Control 2:** `fnstcw`, `fldenv`, `fstenv`, `fnstenv`
*   **FPU State/Control 3:** `fsave`, `fnsave`, `frstor`, `fstsw`
*   **FPU State/Control 4:** `fnstsw`, `fclex`, `fnclex`, `fdecstp`
*   **FPU State/Control 5:** `fincstp`, `ffree`, `fnop`, `fxch`
*   **FPU State/Control 6:** `fcmovcc`

### **6. Advanced Extensions (AMX, SGX, CET, KL)**
*   **AMX Tile Config:** `ldtilecfg`, `sttilecfg`
*   **AMX Matrix Math:** `tdpbf16ps`, `tdpbssd`, `tdpbsud`, `tdpbusd`
*   **AMX Matrix Math 2:** `tdpbuud`
*   **AMX Tile Ops:** `tileloadd`, `tileloaddt1`, `tilerelease`, `tilestored`
*   **AMX Tile Ops 2:** `tilezero`
*   **SGX Enclave Leaf:** `encls`, `enclu`, `enclv`
*   **SGX Enclave Management:** `ecreate`, `eadd`, `einit`, `eremove`
*   **SGX Enclave Management 2:** `eextend`, `eblock`, `etrack`, `ewb`
*   **SGX Enclave Management 3:** `eldb`, `eldu`, `epa`, `ereport`
*   **SGX Enclave Management 4:** `egetkey`, `eresume`, `eenter`, `eexit`
*   **SGX Enclave Management 5:** `eaccept`, `emodpr`, `emodt`, `emodpe`
*   **SGX Enclave Management 6:** `eacceptcopy`, `eaug`, `edbgrd`, `edbgwr`
*   **SGX Enclave Management 7:** `edeccssa`, `edecvirtchild`, `eincvirtchild`, `eldbc`
*   **SGX Enclave Management 8:** `elduc`, `erdinfo`, `esetcontext`, `etrackc`
*   **CET Indirect Branch:** `endbr32`, `endbr64`
*   **CET Shadow Stack:** `incsspd`, `incsspq`, `rdsspd`, `rdsspq`
*   **CET Shadow Stack 2:** `rstorssp`, `saveprevssp`, `setssbsy`, `clrssbsy`
*   **CET Shadow Stack 3:** `wrssd`, `wrssq`, `wrussd`, `wrussq`
*   **Key Locker Setup:** `loadiwkey`, `encodekey128`, `encodekey256`
*   **Key Locker AES:** `aesdec128kl`, `aesdec256kl`, `aesenc128kl`, `aesenc256kl`
*   **Key Locker Wide AES:** `aesdecwide128kl`, `aesdecwide256kl`, `aesencwide128kl`, `aesencwide256kl`

### **7. Cryptography & Hashing**
*   **AES-NI:** `aesdec`, `aesdeclast`, `aesenc`, `aesenclast`
*   **AES-NI Support:** `aesimc`, `aeskeygenassist`
*   **SHA-1 Extensions:** `sha1msg1`, `sha1msg2`, `sha1nexte`, `sha1rnds4`
*   **SHA-256 Extensions:** `sha256msg1`, `sha256msg2`, `sha256rnds2`
*   **GFNI / Carry-less:** `gf2p8affineinvqb`, `gf2p8affineqb`, `gf2p8mulb`, `pclmulqdq`

### **8. System, Virtualization & Protection**
*   **VMX Control:** `vmcall`, `vmclear`, `vmfunc`, `vmlaunch`
*   **VMX Control 2:** `vmresume`, `vmptrld`, `vmptrst`, `vmread`
*   **VMX Control 3:** `vmwrite`, `vmxon`, `vmxoff`
*   **VMX Support:** `invept`, `invvpid`
*   **System Info:** `cpuid`, `rdpid`, `rdpmc`, `rdtsc`
*   **System Info 2:** `rdtscp`
*   **Control Registers:** `rdmsr`, `wrmsr`, `xgetbv`, `xsetbv`
*   **Protection Keys:** `rdpkru`, `wrpkru`
*   **Descriptor Tables:** `lgdt`, `lidt`, `lldt`, `ltr`
*   **Descriptor Tables 2:** `sgdt`, `sidt`, `sldt`, `str`
*   **Cache Control:** `clflush`, `clflushopt`, `clwb`, `cldemote`
*   **Cache Control 2:** `invd`, `wbinvd`, `wbnoinvd`, `invlpg`
*   **Cache Control 3:** `invpcid`
*   **State Save/Restore:** `xsave`, `xsavec`, `xsaveopt`, `xsaves`
*   **State Save/Restore 2:** `xrstor`, `xrstors`, `fxsave`, `fxrstor`
*   **System Entry/Exit:** `syscall`, `sysret`, `sysenter`, `sysexit`
*   **System Entry/Exit 2:** `swapgs`, `rsm`
*   **Flags Control:** `clc`, `stc`, `cld`, `std`
*   **Flags Control 2:** `cli`, `sti`, `cmc`, `lahf`
*   **Flags Control 3:** `sahf`, `clac`, `stac`
*   **System Status:** `smsw`, `lmsw`, `clts`, `arpl`
*   **System Status 2:** `lar`, `lsl`, `verr`, `verw`
*   **Modern System Ops:** `serialize`, `hreset`, `pconfig`, `ptwrite`
*   **Wait/Pause:** `hlt`, `pause`, `wait`, `fwait`
*   **Wait/Pause 2:** `tpause`, `umwait`, `umonitor`, `wakeup`
*   **Random Numbers:** `rdrand`, `rdseed`
*   **Segment Base:** `rdfsbase`, `rdgsbase`, `wrfsbase`, `wrgsbase`
*   **User Interrupts:** `clui`, `stui`, `testui`, `senduipi`
*   **User Interrupts 2:** `uiret`, `exitac`, `enteraccs`, `sexit`
*   **User Interrupts 3:** `senter`
*   **TSX Transactional:** `xbegin`, `xend`, `xabort`, `xtest`
*   **TSX Transactional 2:** `xacquire`, `xrelease`, `xresldtrk`, `xsusldtrk`

### **9. SIMD: AVX-512 FMA (Fused Multiply-Add)**
*   **FMA 132 Form (PD/PS):** `vfmadd132pd`, `vfmadd132ps`, `vfmaddsub132pd`, `vfmaddsub132ps`
*   **FMA 132 Form (SD/SS):** `vfmadd132sd`, `vfmadd132ss`, `vfmsub132pd`, `vfmsub132ps`
*   **FMA 132 Form (Other):** `vfmsub132sd`, `vfmsub132ss`, `vfmsubadd132pd`, `vfmsubadd132ps`
*   **FMA 132 Form (Neg):** `vfnmadd132pd`, `vfnmadd132ps`, `vfnmadd132sd`, `vfnmadd132ss`
*   **FMA 132 Form (Neg Sub):** `vfnmsub132pd`, `vfnmsub132ps`, `vfnmsub132sd`, `vfnmsub132ss`
*   **FMA 213 Form (PD/PS):** `vfmadd213pd`, `vfmadd213ps`, `vfmaddsub213pd`, `vfmaddsub213ps`
*   **FMA 213 Form (SD/SS):** `vfmadd213sd`, `vfmadd213ss`, `vfmsub213pd`, `vfmsub213ps`
*   **FMA 213 Form (Other):** `vfmsub213sd`, `vfmsub213ss`, `vfmsubadd213pd`, `vfmsubadd213ps`
*   **FMA 213 Form (Neg):** `vfnmadd213pd`, `vfnmadd213ps`, `vfnmadd213sd`, `vfnmadd213ss`
*   **FMA 213 Form (Neg Sub):** `vfnmsub213pd`, `vfnmsub213ps`, `vfnmsub213sd`, `vfnmsub213ss`
*   **FMA 231 Form (PD/PS):** `vfmadd231pd`, `vfmadd231ps`, `vfmaddsub231pd`, `vfmaddsub231ps`
*   **FMA 231 Form (SD/SS):** `vfmadd231sd`, `vfmadd231ss`, `vfmsub231pd`, `vfmsub231ps`
*   **FMA 231 Form (Other):** `vfmsub231sd`, `vfmsub231ss`, `vfmsubadd231pd`, `vfmsubadd231ps`
*   **FMA 231 Form (Neg):** `vfnmadd231pd`, `vfnmadd231ps`, `vfnmadd231sd`, `vfnmadd231ss`
*   **FMA 231 Form (Neg Sub):** `vfnmsub231pd`, `vfnmsub231ps`, `vfnmsub231sd`, `vfnmsub231ss`
*   **FMA Special:** `vfmaddrnd231pd`

### **10. SIMD: AVX-512 FP16 & BFLOAT16**
*   **FP16 Arithmetic:** `vaddph`, `vaddsh`, `vsubph`, `vsubsh`
*   **FP16 Arithmetic 2:** `vmulph`, `vmulsh`, `vdivph`, `vdivsh`
*   **FP16 Arithmetic 3:** `vrcpph`, `vrcpsh`, `vrsqrtph`, `vrsqrtsh`
*   **FP16 Arithmetic 4:** `vsqrtph`, `vsqrtsh`
*   **FP16 Complex:** `vfcmaddcph`, `vfcmaddcsh`, `vfcmulcph`, `vfcmulcsh`
*   **FP16 Complex 2:** `vfmaddcph`, `vfmaddcsh`, `vfmulcph`, `vfmulcsh`
*   **FP16 Comparison:** `vcmpph`, `vcmpsh`, `vcomish`, `vucomish`
*   **FP16 Conversion:** `vcvtdq2ph`, `vcvtpd2ph`, `vcvtps2ph`, `vcvtqq2ph`
*   **FP16 Conversion 2:** `vcvtsd2sh`, `vcvtsi2sh`, `vcvtss2sh`, `vcvtudq2ph`
*   **FP16 Conversion 3:** `vcvtuqq2ph`, `vcvtusi2sh`, `vcvtuw2ph`, `vcvtw2ph`
*   **FP16 Special:** `vfpclassph`, `vfpclasssh`, `vgetexpph`, `vgetexpsh`
*   **FP16 Special 2:** `vgetmantph`, `vgetmantsh`, `vreduceph`, `vreducesh`
*   **FP16 Special 3:** `vrndscaleph`, `vrndscalesh`, `vscalefph`, `vscalefsh`
*   **BFLOAT16:** `vcvtne2ps2bf16`, `vcvtneps2bf16`, `vdpbf16ps`

### **11. SIMD: Arithmetic (SSE/AVX/AVX-512)**
*   **Packed Add/Sub:** `addpd`, `addps`, `addsubpd`, `addsubps`
*   **Scalar Add/Sub:** `addsd`, `addss`, `subpd`, `subps`
*   **Scalar Add/Sub 2:** `subsd`, `subss`
*   **Packed Mul/Div:** `mulpd`, `mulps`, `divpd`, `divps`
*   **Scalar Mul/Div:** `mulsd`, `mulss`, `divsd`, `divss`
*   **Horizontal Add/Sub:** `haddpd`, `haddps`, `hsubpd`, `hsubps`
*   **Packed Integer Add:** `paddb`, `paddw`, `paddd`, `paddq`
*   **Packed Integer Add (Sat):** `paddsb`, `paddsw`, `paddusb`, `paddusw`
*   **Packed Integer Sub:** `psubb`, `psubw`, `psubd`, `psubq`
*   **Packed Integer Sub (Sat):** `psubsb`, `psubsw`, `psubusb`, `psubusw`
*   **Packed Integer Mul:** `pmulhuw`, `pmulhw`, `pmullw`, `pmuludq`
*   **Packed Integer Mul 2:** `pmuldq`, `pmulhrsw`, `pmulld`, `pmullq`
*   **Packed Integer Math:** `pmaddubsw`, `pmaddwd`, `vpmadd52huq`, `vpmadd52luq`
*   **Packed Min/Max:** `minpd`, `minps`, `minsd`, `minss`
*   **Packed Min/Max 2:** `maxpd`, `maxps`, `maxsd`, `maxss`
*   **Packed Integer Min/Max:** `pmaxsb`, `pmaxsw`, `pmaxsd`, `pmaxsq`
*   **Packed Integer Min/Max 2:** `pmaxub`, `pmaxuw`, `pmaxud`, `pmaxuq`
*   **Packed Integer Min/Max 3:** `pminsb`, `pminsw`, `pminsd`, `pminsq`
*   **Packed Integer Min/Max 4:** `pminub`, `pminuw`, `pminud`, `pminuq`
*   **Packed Absolute/Neg:** `pabsb`, `pabsw`, `pabsd`, `pabsq`
*   **Packed Misc Math:** `phaddw`, `phaddd`, `phaddsw`, `phsubw`
*   **Packed Misc Math 2:** `phsubd`, `phsubsw`, `phminposuw`, `psadbw`
*   **Packed Misc Math 3:** `mpsadbw`, `vdbpsadbw`, `vpmultishiftqb`
*   **Neural Network (VNNI):** `vpdpbusd`, `vpdpbusds`, `vpdpwssd`, `vpdpwssds`
*   **Neural Network (4VNNI):** `v4fmaddps`, `v4fnmaddps`, `v4fmaddss`, `v4fnmaddss`
*   **Neural Network (4VNNI) 2:** `vp4dpwssd`, `vp4dpwssds`
*   **Reciprocal/Sqrt:** `rcpps`, `rcpss`, `rsqrtps`, `rsqrtss`
*   **Reciprocal/Sqrt 2:** `sqrtpd`, `sqrtps`, `sqrtsd`, `sqrtss`
*   **AVX-512 Reciprocal:** `vrcp14pd`, `vrcp14ps`, `vrcp14sd`, `vrcp14ss`
*   **AVX-512 Reciprocal 2:** `vrcp28pd`, `vrcp28ps`, `vrcp28sd`, `vrcp28ss`
*   **AVX-512 Rsqrt:** `vrsqrt14pd`, `vrsqrt14ps`, `vrsqrt14sd`, `vrsqrt14ss`
*   **AVX-512 Rsqrt 2:** `vrsqrt28pd`, `vrsqrt28ps`, `vrsqrt28sd`, `vrsqrt28ss`
*   **Mask Arithmetic:** `kaddb`, `kaddw`, `kaddd`, `kaddq`

### **12. SIMD: Logic & Bitwise**
*   **Packed Logic:** `andpd`, `andps`, `andnpd`, `andnps`
*   **Packed Logic 2:** `orpd`, `orps`, `xorpd`, `xorps`
*   **Packed Integer Logic:** `pand`, `pandn`, `por`, `pxor`
*   **AVX-512 Ternary Logic:** `vpternlogd`, `vpternlogq`
*   **Mask Logic:** `kandb`, `kandw`, `kandd`, `kandq`
*   **Mask Logic 2:** `kandnb`, `kandnw`, `kandnd`, `kandnq`
*   **Mask Logic 3:** `korb`, `korw`, `kord`, `korq`
*   **Mask Logic 4:** `kxnorb`, `kxnorw`, `kxnord`, `kxnorq`
*   **Mask Logic 5:** `kxorb`, `kxorw`, `kxord`, `kxorq`
*   **Mask Logic 6:** `knotb`, `knotw`, `knotd`, `knotq`
*   **Mask Testing:** `kortestb`, `kortestw`, `kortestd`, `kortestq`
*   **Mask Testing 2:** `ktestb`, `ktestw`, `ktestd`, `ktestq`
*   **Packed Shifts:** `psllw`, `pslld`, `psllq`, `pslldq`
*   **Packed Shifts 2:** `psrlw`, `psrld`, `psrlq`, `psrldq`
*   **Packed Shifts 3:** `psraw`, `psrad`, `psraq`
*   **AVX-512 Var Shifts:** `vpsllvw`, `vpsllvd`, `vpsllvq`
*   **AVX-512 Var Shifts 2:** `vpsrlvw`, `vpsrlvd`, `vpsrlvq`
*   **AVX-512 Var Shifts 3:** `vpsravw`, `vpsravd`, `vpsravq`
*   **AVX-512 Rotates:** `vprold`, `vprolq`, `vprolvd`, `vprolvq`
*   **AVX-512 Rotates 2:** `vprord`, `vprorq`, `vprorvd`, `vprorvq`
*   **AVX-512 Bit Manipulation:** `vpshld`, `vpshldv`, `vpshrd`, `vpshrdv`
*   **Mask Shifts:** `kshiftlb`, `kshiftlw`, `kshiftld`, `kshiftlq`
*   **Mask Shifts 2:** `kshiftrb`, `kshiftrw`, `kshiftrd`, `kshiftrq`

### **13. SIMD: Data Movement & Shuffles**
*   **Packed Move:** `movapd`, `movaps`, `movupd`, `movups`
*   **Packed Integer Move:** `movd`, `movq`, `movdqa`, `movdqu`
*   **AVX-512 Masked Move:** `vmovdqa32`, `vmovdqa64`, `vmovdqu8`, `vmovdqu16`
*   **AVX-512 Masked Move 2:** `vmovdqu32`, `vmovdqu64`
*   **Move High/Low:** `movhlps`, `movlhps`, `movhpd`, `movhps`
*   **Move High/Low 2:** `movlpd`, `movlps`, `movddup`
*   **Non-Temporal Move:** `movntdq`, `movntdqa`, `movnti`, `movntpd`
*   **Non-Temporal Move 2:** `movntps`, `movntq`, `movdir64b`, `movdiri`
*   **SIMD/MMX Exchange:** `movdq2q`, `movq2dq`
*   **Packed Shuffle:** `shufpd`, `shufps`, `pshufd`, `pshufb`
*   **Packed Shuffle 2:** `pshufhw`, `pshuflw`, `pshufw`
*   **AVX-512 Shuffle:** `vshuff32x4`, `vshuff64x2`, `vshufi32x4`, `vshufi64x2`
*   **Packed Permute:** `vpermilpd`, `vpermilps`, `vpermpd`, `vpermps`
*   **Packed Permute 2:** `vpermq`, `vpermb`, `vpermd`, `vpermw`
*   **AVX-512 Permute:** `vpermi2b`, `vpermi2w`, `vpermi2d`, `vpermi2q`
*   **AVX-512 Permute 2:** `vpermi2ps`, `vpermi2pd`, `vpermt2b`, `vpermt2w`
*   **AVX-512 Permute 3:** `vpermt2d`, `vpermt2q`, `vpermt2ps`, `vpermt2pd`
*   **AVX-512 Permute 4:** `vperm2f128`, `vperm2i128`
*   **Packed Blend:** `blendpd`, `blendps`, `blendvpd`, `blendvps`
*   **Packed Blend 2:** `pblendw`, `pblendvb`, `vpblendd`, `vpblendmb`
*   **Packed Blend 3:** `vpblendmw`, `vpblendmd`, `vpblendmq`
*   **Packed Unpack:** `punpckhbw`, `punpckhwd`, `punpckhdq`, `punpckhqdq`
*   **Packed Unpack 2:** `punpcklbw`, `punpcklwd`, `punpckldq`, `punpcklqdq`
*   **Packed Unpack 3:** `unpckhpd`, `unpckhps`, `unpcklpd`, `unpcklps`
*   **Packed Pack:** `packsswb`, `packssdw`, `packuswb`, `packusdw`
*   **Packed Align:** `palignr`, `valignd`, `valignq`
*   **Packed Broadcast:** `vbroadcast`, `vpbroadcast`, `vpbroadcastb`, `vpbroadcastw`
*   **Packed Broadcast 2:** `vpbroadcastd`, `vpbroadcastq`, `vpbroadcastm`
*   **Packed Insert/Extract:** `insertps`, `extractps`, `pinsrb`, `pinsrw`
*   **Packed Insert/Extract 2:** `pinsrd`, `pinsrq`, `pextrb`, `pextrw`
*   **Packed Insert/Extract 3:** `pextrd`, `pextrq`
*   **AVX-512 Insert/Extract:** `vinsertf128`, `vinsertf32x4`, `vinsertf64x2`, `vinsertf32x8`
*   **AVX-512 Insert/Extract 2:** `vinsertf64x4`, `vinserti128`, `vinserti32x4`, `vinserti64x2`
*   **AVX-512 Insert/Extract 3:** `vinserti32x8`, `vinserti64x4`, `vextractf128`, `vextractf32x4`
*   **AVX-512 Insert/Extract 4:** `vextractf64x2`, `vextractf32x8`, `vextractf64x4`, `vextracti128`
*   **AVX-512 Insert/Extract 5:** `vextracti32x4`, `vextracti64x2`, `vextracti32x8`, `vextracti64x4`
*   **Packed Compress/Expand:** `vcompresspd`, `vcompressps`, `vpcompressb`, `vpcompressw`
*   **Packed Compress/Expand 2:** `vpcompressd`, `vpcompressq`, `vexpandpd`, `vexpandps`
*   **Packed Compress/Expand 3:** `vpexpandb`, `vpexpandw`, `vpexpandd`, `vpexpandq`
*   **Gather/Scatter:** `vgatherdpd`, `vgatherqpd`, `vgatherdps`, `vgatherqps`
*   **Gather/Scatter 2:** `vpgatherdd`, `vpgatherdq`, `vpgatherqd`, `vpgatherqq`
*   **Gather/Scatter 3:** `vpscatterdd`, `vpscatterdq`, `vpscatterqd`, `vpscatterqq`
*   **Gather/Scatter 4:** `vscatterdps`, `vscatterdpd`, `vscatterqps`, `vscatterqpd`
*   **Gather/Scatter Prefetch:** `vgatherpf0dps`, `vgatherpf0qps`, `vgatherpf0dpd`, `vgatherpf0qpd`
*   **Gather/Scatter Prefetch 2:** `vgatherpf1dps`, `vgatherpf1qps`, `vgatherpf1dpd`, `vgatherpf1qpd`
*   **Gather/Scatter Prefetch 3:** `vscatterpf0dps`, `vscatterpf0qps`, `vscatterpf0dpd`, `vscatterpf0qpd`
*   **Gather/Scatter Prefetch 4:** `vscatterpf1dps`, `vscatterpf1qps`, `vscatterpf1dpd`, `vscatterpf1qpd`
*   **Mask Move:** `maskmovq`, `maskmovdqu`, `vmaskmov`, `vpmaskmov`
*   **Mask Move 2:** `kmovb`, `kmovw`, `kmovd`, `kmovq`
*   **Mask Move 3:** `kunpckbw`, `kunpckwd`, `kunpckdq`
*   **Packed Conversion:** `vpmovsx`, `vpmovzx`, `vpmovb2m`, `vpmovw2m`
*   **Packed Conversion 2:** `vpmovd2m`, `vpmovq2m`, `vpmovm2b`, `vpmovm2w`
*   **Packed Conversion 3:** `vpmovm2d`, `vpmovm2q`, `vpmovdb`, `vpmovdw`
*   **Packed Conversion 4:** `vpmovqb`, `vpmovqw`, `vpmovqd`, `vpmovsdb`
*   **Packed Conversion 5:** `vpmovsdw`, `vpmovsqb`, `vpmovsqw`, `vpmovsqd`
*   **Packed Conversion 6:** `vpmovusdb`, `vpmovusdw`, `vpmovusqb`, `vpmovusqw`
*   **Packed Conversion 7:** `vpmovusqd`, `vpmovwb`, `vpmovswb`, `vpmovuswb`

### **14. SIMD: Comparison & Special**
*   **Packed Comparison:** `cmppd`, `cmpps`, `cmpsd`, `cmpss`
*   **Scalar Comparison:** `comisd`, `comiss`, `ucomisd`, `ucomiss`
*   **Packed Integer Comparison:** `pcmpeqb`, `pcmpeqw`, `pcmpeqd`, `pcmpeqq`
*   **Packed Integer Comparison 2:** `pcmpgtb`, `pcmpgtw`, `pcmpgtd`, `pcmpgtq`
*   **Packed String Comparison:** `pcmpestri`, `pcmpestrm`, `pcmpistri`, `pcmpistrm`
*   **AVX-512 Comparison:** `vpcmpb`, `vpcmpub`, `vpcmpw`, `vpcmpuw`
*   **AVX-512 Comparison 2:** `vpcmpd`, `vpcmpud`, `vpcmpq`, `vpcmpuq`
*   **AVX-512 Special:** `vfpclasspd`, `vfpclassps`, `vfpclasssd`, `vfpclassss`
*   **AVX-512 Special 2:** `vgetexppd`, `vgetexpps`, `vgetexpsd`, `vgetexpss`
*   **AVX-512 Special 3:** `vgetmantpd`, `vgetmantps`, `vgetmantsd`, `vgetmantss`
*   **AVX-512 Special 4:** `vreducepd`, `vreduceps`, `vreducesd`, `vreducess`
*   **AVX-512 Special 5:** `vrndscalepd`, `vrndscaleps`, `vrndscalesd`, `vrndscaless`
*   **AVX-512 Special 6:** `vscalefpd`, `vscalefps`, `vscalefsd`, `vscalefss`
*   **AVX-512 Special 7:** `vfixupimmpd`, `vfixupimmps`, `vfixupimmsd`, `vfixupimmss`
*   **AVX-512 Special 8:** `vrangepd`, `vrangeps`, `vrangesd`, `vrangess`
*   **AVX-512 Special 9:** `vtestpd`, `vtestps`, `vptestmb`, `vptestmw`
*   **AVX-512 Special 10:** `vptestmd`, `vptestmq`, `vptestnmb`, `vptestnmw`
*   **AVX-512 Special 11:** `vptestnmd`, `vptestnmq`, `vpconflictd`, `vpconflictq`
*   **AVX-512 Special 12:** `vplzcntd`, `vplzcntq`, `vpshufbitqmb`, `vpopcnt`
*   **SIMD Conversion:** `cvtdq2pd`, `cvtdq2ps`, `cvtpd2dq`, `cvtpd2pi`
*   **SIMD Conversion 2:** `cvtpd2ps`, `cvtpi2pd`, `cvtpi2ps`, `cvtps2dq`
*   **SIMD Conversion 3:** `cvtps2pd`, `cvtps2pi`, `cvtsd2si`, `cvtsd2ss`
*   **SIMD Conversion 4:** `cvtsi2sd`, `cvtsi2ss`, `cvtss2sd`, `cvtss2si`
*   **SIMD Conversion 5:** `cvttpd2dq`, `cvttpd2pi`, `cvttps2dq`, `cvttps2pi`
*   **SIMD Conversion 6:** `cvttsd2si`, `cvttss2si`
*   **AVX-512 Conversion:** `vcvtpd2qq`, `vcvtpd2udq`, `vcvtpd2uqq`, `vcvtph2dq`
*   **AVX-512 Conversion 2:** `vcvtph2pd`, `vcvtph2ps`, `vcvtph2psx`, `vcvtph2qq`
*   **AVX-512 Conversion 3:** `vcvtph2udq`, `vcvtph2uqq`, `vcvtph2uw`, `vcvtph2w`
*   **AVX-512 Conversion 4:** `vcvtps2phx`, `vcvtps2qq`, `vcvtps2udq`, `vcvtps2uqq`
*   **AVX-512 Conversion 5:** `vcvtqq2pd`, `vcvtqq2ps`, `vcvtsd2usi`, `vcvtsh2sd`
*   **AVX-512 Conversion 6:** `vcvtsh2si`, `vcvtsh2ss`, `vcvtsh2usi`, `vcvtss2usi`
*   **AVX-512 Conversion 7:** `vcvttpd2qq`, `vcvttpd2udq`, `vcvttpd2uqq`, `vcvttph2dq`
*   **AVX-512 Conversion 8:** `vcvttph2qq`, `vcvttph2udq`, `vcvttph2uqq`, `vcvttph2uw`
*   **AVX-512 Conversion 9:** `vcvttph2w`, `vcvttps2qq`, `vcvttps2udq`, `vcvttps2uqq`
*   **AVX-512 Conversion 10:** `vcvttsd2usi`, `vcvttsh2si`, `vcvttsh2usi`, `vcvttss2usi`
*   **AVX-512 Conversion 11:** `vcvtudq2pd`, `vcvtudq2ps`, `vcvtuqq2pd`, `vcvtuqq2ps`
*   **AVX-512 Conversion 12:** `vcvtusi2sd`, `vcvtusi2ss`, `vcvtuw2ph`, `vcvtw2ph`

### **15. Miscellaneous & State Management**
*   **MPX Protection:** `bndcl`, `bndcu`, `bndcn`, `bndldx`
*   **MPX Protection 2:** `bndmk`, `bndstx`
*   **Atomic/Lock:** `lock`, `cmpxchg`, `cmpxchg8b`, `cmpxchg16b`
*   **Atomic/Lock 2:** `xadd`
*   **SIMD State:** `ldmxcsr`, `stmxcsr`, `emms`, `vzeroall`
*   **SIMD State 2:** `vzeroupper`
*   **Hashing/CRC:** `crc32`
*   **Memory Ordering:** `lfence`, `mfence`, `sfence`
*   **Enqueue Command:** `enqcmd`, `enqcmds`
*   **Misc SIMD:** `psignb`, `psignw`, `psignd`, `roundpd`
*   **Misc SIMD 2:** `roundps`, `roundsd`, `roundss`, `pavgb`
*   **Misc SIMD 3:** `pavgw`, `ptest`, `vptest`, `vtestpd`
*   **Misc SIMD 4:** `vtestps`, `vp2intersectd`, `vp2intersectq`, `vexp2pd`
*   **Misc SIMD 5:** `vexp2ps`, `ptwrite`, `pconfig`, `setcc`
*   **Misc SIMD 6:** `nop`
*   **Prefetch:** `prefetchh`, `prefetchw`, `prefetchwt1`
