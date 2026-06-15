__instruction aarch32_VSHL_i_A
    __encoding aarch32_VSHL_i_T1A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field imm6 16 +: 6
        __field Vd 12 +: 4
        __field L 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110010 1xxxxxxx xxxx0101 xxx1xxxx'
        __guard TRUE
        __decode
            if L:imm6 == '0000xxx' then SEE "Related encodings";
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            case L:imm6 of
                when '0001xxx'  esize = 8;  elements = 8;  shift_amount = UInt(imm6) - 8;
                when '001xxxx'  esize = 16;  elements = 4;  shift_amount = UInt(imm6) - 16;
                when '01xxxxx'  esize = 32;  elements = 2;  shift_amount = UInt(imm6) - 32;
                when '1xxxxxx'  esize = 64;  elements = 1;  shift_amount = UInt(imm6);
            d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VSHL_i_T1A1_A
        __instruction_set T32
        __field D 22 +: 1
        __field imm6 16 +: 6
        __field Vd 12 +: 4
        __field L 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11101111 1xxxxxxx xxxx0101 xxx1xxxx'
        __guard TRUE
        __decode
            if L:imm6 == '0000xxx' then SEE "Related encodings";
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            case L:imm6 of
                when '0001xxx'  esize = 8;  elements = 8;  shift_amount = UInt(imm6) - 8;
                when '001xxxx'  esize = 16;  elements = 4;  shift_amount = UInt(imm6) - 16;
                when '01xxxxx'  esize = 32;  elements = 2;  shift_amount = UInt(imm6) - 32;
                when '1xxxxxx'  esize = 64;  elements = 1;  shift_amount = UInt(imm6);
            d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __execute __conditional
        CheckAdvSIMDEnabled();
        for r = 0 to regs-1
            for e = 0 to elements-1
                Elem[D[d+r],e,esize] = LSL(Elem[D[m+r],e,esize], shift_amount);

__instruction aarch32_VSHL_r_A
    __encoding aarch32_VSHL_r_T1A1_A
        __instruction_set A32
        __field U 24 +: 1
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '1111001x 0xxxxxxx xxxx0100 xxx0xxxx'
        __guard TRUE
        __decode
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1' || Vn<0> == '1') then UNDEFINED;
            unsigned = (U == '1');
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  m = UInt(M:Vm);  n = UInt(N:Vn);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VSHL_r_T1A1_A
        __instruction_set T32
        __field U 28 +: 1
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '111x1111 0xxxxxxx xxxx0100 xxx0xxxx'
        __guard TRUE
        __decode
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1' || Vn<0> == '1') then UNDEFINED;
            unsigned = (U == '1');
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  m = UInt(M:Vm);  n = UInt(N:Vn);  regs = if Q == '0' then 1 else 2;

    __execute __conditional
        CheckAdvSIMDEnabled();
        for r = 0 to regs-1
            for e = 0 to elements-1
                shift = SInt(Elem[D[n+r],e,esize]<7:0>);
                result = Int(Elem[D[m+r],e,esize], unsigned) << shift;
                Elem[D[d+r],e,esize] = result<esize-1:0>;
