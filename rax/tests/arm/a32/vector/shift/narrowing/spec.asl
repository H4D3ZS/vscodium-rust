__instruction aarch32_VSHRN_A
    __encoding aarch32_VSHRN_T1A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field imm6 16 +: 6
        __field Vd 12 +: 4
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110010 1xxxxxxx xxxx1000 00x1xxxx'
        __guard TRUE
        __decode
            if imm6 == '000xxx' then SEE "Related encodings";
            if Vm<0> == '1' then UNDEFINED;
            case imm6 of
                when '001xxx'  esize = 8;  elements = 8;  shift_amount = 16 - UInt(imm6);
                when '01xxxx'  esize = 16;  elements = 4;  shift_amount = 32 - UInt(imm6);
                when '1xxxxx'  esize = 32;  elements = 2;  shift_amount = 64 - UInt(imm6);
            d = UInt(D:Vd);  m = UInt(M:Vm);

    __encoding aarch32_VSHRN_T1A1_A
        __instruction_set T32
        __field D 22 +: 1
        __field imm6 16 +: 6
        __field Vd 12 +: 4
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11101111 1xxxxxxx xxxx1000 00x1xxxx'
        __guard TRUE
        __decode
            if imm6 == '000xxx' then SEE "Related encodings";
            if Vm<0> == '1' then UNDEFINED;
            case imm6 of
                when '001xxx'  esize = 8;  elements = 8;  shift_amount = 16 - UInt(imm6);
                when '01xxxx'  esize = 16;  elements = 4;  shift_amount = 32 - UInt(imm6);
                when '1xxxxx'  esize = 32;  elements = 2;  shift_amount = 64 - UInt(imm6);
            d = UInt(D:Vd);  m = UInt(M:Vm);

    __execute __conditional
        CheckAdvSIMDEnabled();
        for e = 0 to elements-1
            result = LSR(Elem[Qin[m>>1],e,2*esize], shift_amount);
            Elem[D[d],e,esize] = result<esize-1:0>;

__instruction aarch32_VRSHRN_A
    __encoding aarch32_VRSHRN_T1A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field imm6 16 +: 6
        __field Vd 12 +: 4
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110010 1xxxxxxx xxxx1000 01x1xxxx'
        __guard TRUE
        __decode
            if imm6 == '000xxx' then SEE "Related encodings";
            if Vm<0> == '1' then UNDEFINED;
            case imm6 of
                when '001xxx'  esize = 8;  elements = 8;  shift_amount = 16 - UInt(imm6);
                when '01xxxx'  esize = 16;  elements = 4;  shift_amount = 32 - UInt(imm6);
                when '1xxxxx'  esize = 32;  elements = 2;  shift_amount = 64 - UInt(imm6);
            d = UInt(D:Vd);  m = UInt(M:Vm);

    __encoding aarch32_VRSHRN_T1A1_A
        __instruction_set T32
        __field D 22 +: 1
        __field imm6 16 +: 6
        __field Vd 12 +: 4
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11101111 1xxxxxxx xxxx1000 01x1xxxx'
        __guard TRUE
        __decode
            if imm6 == '000xxx' then SEE "Related encodings";
            if Vm<0> == '1' then UNDEFINED;
            case imm6 of
                when '001xxx'  esize = 8;  elements = 8;  shift_amount = 16 - UInt(imm6);
                when '01xxxx'  esize = 16;  elements = 4;  shift_amount = 32 - UInt(imm6);
                when '1xxxxx'  esize = 32;  elements = 2;  shift_amount = 64 - UInt(imm6);
            d = UInt(D:Vd);  m = UInt(M:Vm);

    __execute __conditional
        CheckAdvSIMDEnabled();
        round_const = 1 << (shift_amount-1);
        for e = 0 to elements-1
            result = LSR(Elem[Qin[m>>1],e,2*esize] + round_const, shift_amount);
            Elem[D[d],e,esize] = result<esize-1:0>;
