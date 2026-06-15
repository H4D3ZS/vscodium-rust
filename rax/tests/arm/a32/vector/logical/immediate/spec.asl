__instruction aarch32_VORR_i_A
    __encoding aarch32_VORR_i_T1A1_A
        __instruction_set A32
        __field i 24 +: 1
        __field D 22 +: 1
        __field imm3 16 +: 3
        __field Vd 12 +: 4
        __field cmode 8 +: 4
        __field Q 6 +: 1
        __field imm4 0 +: 4
        __opcode '1111001x 1x000xxx xxxx0xx1 0x01xxxx'
        __guard TRUE
        __decode
            if cmode<0> == '0' || cmode<3:2> == '11' then SEE "VMOV (immediate)";
            if Q == '1' && Vd<0> == '1' then UNDEFINED;
            imm64 = AdvSIMDExpandImm('0', cmode, i:imm3:imm4);
            d = UInt(D:Vd);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VORR_i_T2A2_A
        __instruction_set A32
        __field i 24 +: 1
        __field D 22 +: 1
        __field imm3 16 +: 3
        __field Vd 12 +: 4
        __field cmode 8 +: 4
        __field Q 6 +: 1
        __field imm4 0 +: 4
        __opcode '1111001x 1x000xxx xxxx10x1 0x01xxxx'
        __guard TRUE
        __decode
            if cmode<0> == '0' || cmode<3:2> == '11' then SEE "VMOV (immediate)";
            if Q == '1' && Vd<0> == '1' then UNDEFINED;
            imm64 = AdvSIMDExpandImm('0', cmode, i:imm3:imm4);
            d = UInt(D:Vd);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VORR_i_T1A1_A
        __instruction_set T32
        __field i 28 +: 1
        __field D 22 +: 1
        __field imm3 16 +: 3
        __field Vd 12 +: 4
        __field cmode 8 +: 4
        __field Q 6 +: 1
        __field imm4 0 +: 4
        __opcode '111x1111 1x000xxx xxxx0xx1 0x01xxxx'
        __guard TRUE
        __decode
            if cmode<0> == '0' || cmode<3:2> == '11' then SEE "VMOV (immediate)";
            if Q == '1' && Vd<0> == '1' then UNDEFINED;
            imm64 = AdvSIMDExpandImm('0', cmode, i:imm3:imm4);
            d = UInt(D:Vd);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VORR_i_T2A2_A
        __instruction_set T32
        __field i 28 +: 1
        __field D 22 +: 1
        __field imm3 16 +: 3
        __field Vd 12 +: 4
        __field cmode 8 +: 4
        __field Q 6 +: 1
        __field imm4 0 +: 4
        __opcode '111x1111 1x000xxx xxxx10x1 0x01xxxx'
        __guard TRUE
        __decode
            if cmode<0> == '0' || cmode<3:2> == '11' then SEE "VMOV (immediate)";
            if Q == '1' && Vd<0> == '1' then UNDEFINED;
            imm64 = AdvSIMDExpandImm('0', cmode, i:imm3:imm4);
            d = UInt(D:Vd);  regs = if Q == '0' then 1 else 2;

    __execute __conditional
        CheckAdvSIMDEnabled();
        for r = 0 to regs-1
            D[d+r] = D[d+r] OR imm64;

__instruction aarch32_VBIC_i_A
    __encoding aarch32_VBIC_i_T1A1_A
        __instruction_set A32
        __field i 24 +: 1
        __field D 22 +: 1
        __field imm3 16 +: 3
        __field Vd 12 +: 4
        __field cmode 8 +: 4
        __field Q 6 +: 1
        __field imm4 0 +: 4
        __opcode '1111001x 1x000xxx xxxx0xx1 0x11xxxx'
        __guard TRUE
        __decode
            if cmode<0> == '0' || cmode<3:2> == '11' then SEE "Related encodings";
            if Q == '1' && Vd<0> == '1' then UNDEFINED;
            imm64 = AdvSIMDExpandImm('1', cmode, i:imm3:imm4);
            d = UInt(D:Vd);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VBIC_i_T2A2_A
        __instruction_set A32
        __field i 24 +: 1
        __field D 22 +: 1
        __field imm3 16 +: 3
        __field Vd 12 +: 4
        __field cmode 8 +: 4
        __field Q 6 +: 1
        __field imm4 0 +: 4
        __opcode '1111001x 1x000xxx xxxx10x1 0x11xxxx'
        __guard TRUE
        __decode
            if cmode<0> == '0' || cmode<3:2> == '11' then SEE "Related encodings";
            if Q == '1' && Vd<0> == '1' then UNDEFINED;
            imm64 = AdvSIMDExpandImm('1', cmode, i:imm3:imm4);
            d = UInt(D:Vd);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VBIC_i_T1A1_A
        __instruction_set T32
        __field i 28 +: 1
        __field D 22 +: 1
        __field imm3 16 +: 3
        __field Vd 12 +: 4
        __field cmode 8 +: 4
        __field Q 6 +: 1
        __field imm4 0 +: 4
        __opcode '111x1111 1x000xxx xxxx0xx1 0x11xxxx'
        __guard TRUE
        __decode
            if cmode<0> == '0' || cmode<3:2> == '11' then SEE "Related encodings";
            if Q == '1' && Vd<0> == '1' then UNDEFINED;
            imm64 = AdvSIMDExpandImm('1', cmode, i:imm3:imm4);
            d = UInt(D:Vd);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VBIC_i_T2A2_A
        __instruction_set T32
        __field i 28 +: 1
        __field D 22 +: 1
        __field imm3 16 +: 3
        __field Vd 12 +: 4
        __field cmode 8 +: 4
        __field Q 6 +: 1
        __field imm4 0 +: 4
        __opcode '111x1111 1x000xxx xxxx10x1 0x11xxxx'
        __guard TRUE
        __decode
            if cmode<0> == '0' || cmode<3:2> == '11' then SEE "Related encodings";
            if Q == '1' && Vd<0> == '1' then UNDEFINED;
            imm64 = AdvSIMDExpandImm('1', cmode, i:imm3:imm4);
            d = UInt(D:Vd);  regs = if Q == '0' then 1 else 2;

    __execute __conditional
        CheckAdvSIMDEnabled();
        for r = 0 to regs-1
            D[d+r] = D[d+r] AND NOT(imm64);
