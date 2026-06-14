__instruction aarch32_VMOV_i_A
    __encoding aarch32_VMOV_i_T1A1_A
        __instruction_set A32
        __field i 24 +: 1
        __field D 22 +: 1
        __field imm3 16 +: 3
        __field Vd 12 +: 4
        __field cmode 8 +: 4
        __field Q 6 +: 1
        __field op 5 +: 1
        __field imm4 0 +: 4
        __opcode '1111001x 1x000xxx xxxx0xx0 0x01xxxx'
        __guard TRUE
        __decode
            if op == '0' && cmode<0> == '1' && cmode<3:2> != '11' then SEE "VORR (immediate)";
            if op == '1' && cmode != '1110' then SEE "Related encodings";
            if Q == '1' && Vd<0> == '1' then UNDEFINED;
            single_register = FALSE;  advsimd = TRUE;  imm64 = AdvSIMDExpandImm(op, cmode, i:imm3:imm4);
            d = UInt(D:Vd);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VMOV_i_A2_A
        __instruction_set A32
        __field cond 28 +: 4
        __field D 22 +: 1
        __field imm4H 16 +: 4
        __field Vd 12 +: 4
        __field size 8 +: 2
        __field imm4L 0 +: 4
        __opcode 'xxxx1110 1x11xxxx xxxx10xx x0x0xxxx'
        __guard cond != '1111'
        __decode
            if FPSCR.Len != '000' || FPSCR.Stride != '00' then UNDEFINED;
            if size == '00' || (size == '01' && !HaveFP16Ext()) then UNDEFINED;
            if size == '01' && cond != '1110' then UNPREDICTABLE;
            single_register = (size != '11'); advsimd = FALSE;
            bits(16) imm16;
            bits(32) imm32;
            bits(64) imm64;
            case size of
                when '01' d = UInt(Vd:D);  imm16 = VFPExpandImm(imm4H:imm4L); imm32 = Zeros(16) : imm16;
                when '10' d = UInt(Vd:D);  imm32 = VFPExpandImm(imm4H:imm4L);
                when '11' d = UInt(D:Vd);  imm64 = VFPExpandImm(imm4H:imm4L);  regs = 1;

    __encoding aarch32_VMOV_i_T3A3_A
        __instruction_set A32
        __field i 24 +: 1
        __field D 22 +: 1
        __field imm3 16 +: 3
        __field Vd 12 +: 4
        __field cmode 8 +: 4
        __field Q 6 +: 1
        __field op 5 +: 1
        __field imm4 0 +: 4
        __opcode '1111001x 1x000xxx xxxx10x0 0x01xxxx'
        __guard TRUE
        __decode
            if op == '0' && cmode<0> == '1' && cmode<3:2> != '11' then SEE "VORR (immediate)";
            if op == '1' && cmode != '1110' then SEE "Related encodings";
            if Q == '1' && Vd<0> == '1' then UNDEFINED;
            single_register = FALSE;  advsimd = TRUE;  imm64 = AdvSIMDExpandImm(op, cmode, i:imm3:imm4);
            d = UInt(D:Vd);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VMOV_i_T4A4_A
        __instruction_set A32
        __field i 24 +: 1
        __field D 22 +: 1
        __field imm3 16 +: 3
        __field Vd 12 +: 4
        __field cmode 8 +: 4
        __field Q 6 +: 1
        __field op 5 +: 1
        __field imm4 0 +: 4
        __opcode '1111001x 1x000xxx xxxx11xx 0x01xxxx'
        __guard TRUE
        __decode
            if op == '0' && cmode<0> == '1' && cmode<3:2> != '11' then SEE "VORR (immediate)";
            if op == '1' && cmode != '1110' then SEE "Related encodings";
            if Q == '1' && Vd<0> == '1' then UNDEFINED;
            single_register = FALSE;  advsimd = TRUE;  imm64 = AdvSIMDExpandImm(op, cmode, i:imm3:imm4);
            d = UInt(D:Vd);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VMOV_i_T5A5_A
        __instruction_set A32
        __field i 24 +: 1
        __field D 22 +: 1
        __field imm3 16 +: 3
        __field Vd 12 +: 4
        __field cmode 8 +: 4
        __field Q 6 +: 1
        __field op 5 +: 1
        __field imm4 0 +: 4
        __opcode '1111001x 1x000xxx xxxx1110 0x11xxxx'
        __guard TRUE
        __decode
            if op == '0' && cmode<0> == '1' && cmode<3:2> != '11' then SEE "VORR (immediate)";
            if op == '1' && cmode != '1110' then SEE "Related encodings";
            if Q == '1' && Vd<0> == '1' then UNDEFINED;
            single_register = FALSE;  advsimd = TRUE;  imm64 = AdvSIMDExpandImm(op, cmode, i:imm3:imm4);
            d = UInt(D:Vd);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VMOV_i_T1A1_A
        __instruction_set T32
        __field i 28 +: 1
        __field D 22 +: 1
        __field imm3 16 +: 3
        __field Vd 12 +: 4
        __field cmode 8 +: 4
        __field Q 6 +: 1
        __field op 5 +: 1
        __field imm4 0 +: 4
        __opcode '111x1111 1x000xxx xxxx0xx0 0x01xxxx'
        __guard TRUE
        __decode
            if op == '0' && cmode<0> == '1' && cmode<3:2> != '11' then SEE "VORR (immediate)";
            if op == '1' && cmode != '1110' then SEE "Related encodings";
            if Q == '1' && Vd<0> == '1' then UNDEFINED;
            single_register = FALSE;  advsimd = TRUE;  imm64 = AdvSIMDExpandImm(op, cmode, i:imm3:imm4);
            d = UInt(D:Vd);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VMOV_i_T2_A
        __instruction_set T32
        __field D 22 +: 1
        __field imm4H 16 +: 4
        __field Vd 12 +: 4
        __field size 8 +: 2
        __field imm4L 0 +: 4
        __opcode '11101110 1x11xxxx xxxx10xx x0x0xxxx'
        __guard TRUE
        __decode
            if FPSCR.Len != '000' || FPSCR.Stride != '00' then UNDEFINED;
            if size == '00' || (size == '01' && !HaveFP16Ext()) then UNDEFINED;
            if size == '01' && InITBlock()  then UNPREDICTABLE;
            single_register = (size != '11'); advsimd = FALSE;
            bits(16) imm16;
            bits(32) imm32;
            bits(64) imm64;
            case size of
                when '01' d = UInt(Vd:D);  imm16 = VFPExpandImm(imm4H:imm4L); imm32 = Zeros(16) : imm16;
                when '10' d = UInt(Vd:D);  imm32 = VFPExpandImm(imm4H:imm4L);
                when '11' d = UInt(D:Vd);  imm64 = VFPExpandImm(imm4H:imm4L);  regs = 1;

    __encoding aarch32_VMOV_i_T3A3_A
        __instruction_set T32
        __field i 28 +: 1
        __field D 22 +: 1
        __field imm3 16 +: 3
        __field Vd 12 +: 4
        __field cmode 8 +: 4
        __field Q 6 +: 1
        __field op 5 +: 1
        __field imm4 0 +: 4
        __opcode '111x1111 1x000xxx xxxx10x0 0x01xxxx'
        __guard TRUE
        __decode
            if op == '0' && cmode<0> == '1' && cmode<3:2> != '11' then SEE "VORR (immediate)";
            if op == '1' && cmode != '1110' then SEE "Related encodings";
            if Q == '1' && Vd<0> == '1' then UNDEFINED;
            single_register = FALSE;  advsimd = TRUE;  imm64 = AdvSIMDExpandImm(op, cmode, i:imm3:imm4);
            d = UInt(D:Vd);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VMOV_i_T4A4_A
        __instruction_set T32
        __field i 28 +: 1
        __field D 22 +: 1
        __field imm3 16 +: 3
        __field Vd 12 +: 4
        __field cmode 8 +: 4
        __field Q 6 +: 1
        __field op 5 +: 1
        __field imm4 0 +: 4
        __opcode '111x1111 1x000xxx xxxx11xx 0x01xxxx'
        __guard TRUE
        __decode
            if op == '0' && cmode<0> == '1' && cmode<3:2> != '11' then SEE "VORR (immediate)";
            if op == '1' && cmode != '1110' then SEE "Related encodings";
            if Q == '1' && Vd<0> == '1' then UNDEFINED;
            single_register = FALSE;  advsimd = TRUE;  imm64 = AdvSIMDExpandImm(op, cmode, i:imm3:imm4);
            d = UInt(D:Vd);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VMOV_i_T5A5_A
        __instruction_set T32
        __field i 28 +: 1
        __field D 22 +: 1
        __field imm3 16 +: 3
        __field Vd 12 +: 4
        __field cmode 8 +: 4
        __field Q 6 +: 1
        __field op 5 +: 1
        __field imm4 0 +: 4
        __opcode '111x1111 1x000xxx xxxx1110 0x11xxxx'
        __guard TRUE
        __decode
            if op == '0' && cmode<0> == '1' && cmode<3:2> != '11' then SEE "VORR (immediate)";
            if op == '1' && cmode != '1110' then SEE "Related encodings";
            if Q == '1' && Vd<0> == '1' then UNDEFINED;
            single_register = FALSE;  advsimd = TRUE;  imm64 = AdvSIMDExpandImm(op, cmode, i:imm3:imm4);
            d = UInt(D:Vd);  regs = if Q == '0' then 1 else 2;

    __execute __conditional
        CheckAdvSIMDOrVFPEnabled(TRUE, advsimd);
        if single_register then
            S[d] = imm32;
        else
            for r = 0 to regs-1
                D[d+r] = imm64;
