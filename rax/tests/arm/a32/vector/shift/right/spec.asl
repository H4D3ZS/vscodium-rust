__instruction aarch32_VSHR_A
    __encoding aarch32_VSHR_T1A1_A
        __instruction_set A32
        __field U 24 +: 1
        __field D 22 +: 1
        __field imm6 16 +: 6
        __field Vd 12 +: 4
        __field L 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '1111001x 1xxxxxxx xxxx0000 xxx1xxxx'
        __guard TRUE
        __decode
            if (L:imm6) == '0000xxx' then SEE "Related encodings";
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            case L:imm6 of
                when '0001xxx'  esize = 8;  elements = 8;  shift_amount = 16 - UInt(imm6);
                when '001xxxx'  esize = 16;  elements = 4;  shift_amount = 32 - UInt(imm6);
                when '01xxxxx'  esize = 32;  elements = 2;  shift_amount = 64 - UInt(imm6);
                when '1xxxxxx'  esize = 64;  elements = 1;  shift_amount = 64 - UInt(imm6);
            unsigned = (U == '1');  d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VSHR_T1A1_A
        __instruction_set T32
        __field U 28 +: 1
        __field D 22 +: 1
        __field imm6 16 +: 6
        __field Vd 12 +: 4
        __field L 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '111x1111 1xxxxxxx xxxx0000 xxx1xxxx'
        __guard TRUE
        __decode
            if (L:imm6) == '0000xxx' then SEE "Related encodings";
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            case L:imm6 of
                when '0001xxx'  esize = 8;  elements = 8;  shift_amount = 16 - UInt(imm6);
                when '001xxxx'  esize = 16;  elements = 4;  shift_amount = 32 - UInt(imm6);
                when '01xxxxx'  esize = 32;  elements = 2;  shift_amount = 64 - UInt(imm6);
                when '1xxxxxx'  esize = 64;  elements = 1;  shift_amount = 64 - UInt(imm6);
            unsigned = (U == '1');  d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __execute __conditional
        CheckAdvSIMDEnabled();
        for r = 0 to regs-1
            for e = 0 to elements-1
                result = Int(Elem[D[m+r],e,esize], unsigned) >> shift_amount;
                Elem[D[d+r],e,esize] = result<esize-1:0>;

__instruction aarch32_VRSHR_A
    __encoding aarch32_VRSHR_T1A1_A
        __instruction_set A32
        __field U 24 +: 1
        __field D 22 +: 1
        __field imm6 16 +: 6
        __field Vd 12 +: 4
        __field L 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '1111001x 1xxxxxxx xxxx0010 xxx1xxxx'
        __guard TRUE
        __decode
            if (L:imm6) == '0000xxx' then SEE "Related encodings";
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            case L:imm6 of
                when '0001xxx'  esize = 8;  elements = 8;  shift_amount = 16 - UInt(imm6);
                when '001xxxx'  esize = 16;  elements = 4;  shift_amount = 32 - UInt(imm6);
                when '01xxxxx'  esize = 32;  elements = 2;  shift_amount = 64 - UInt(imm6);
                when '1xxxxxx'  esize = 64;  elements = 1;  shift_amount = 64 - UInt(imm6);
            unsigned = (U == '1');  d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VRSHR_T1A1_A
        __instruction_set T32
        __field U 28 +: 1
        __field D 22 +: 1
        __field imm6 16 +: 6
        __field Vd 12 +: 4
        __field L 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '111x1111 1xxxxxxx xxxx0010 xxx1xxxx'
        __guard TRUE
        __decode
            if (L:imm6) == '0000xxx' then SEE "Related encodings";
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            case L:imm6 of
                when '0001xxx'  esize = 8;  elements = 8;  shift_amount = 16 - UInt(imm6);
                when '001xxxx'  esize = 16;  elements = 4;  shift_amount = 32 - UInt(imm6);
                when '01xxxxx'  esize = 32;  elements = 2;  shift_amount = 64 - UInt(imm6);
                when '1xxxxxx'  esize = 64;  elements = 1;  shift_amount = 64 - UInt(imm6);
            unsigned = (U == '1');  d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __execute __conditional
        CheckAdvSIMDEnabled();
        round_const = 1 << (shift_amount - 1);
        for r = 0 to regs-1
            for e = 0 to elements-1
                result = (Int(Elem[D[m+r],e,esize], unsigned) + round_const) >> shift_amount;
                Elem[D[d+r],e,esize] = result<esize-1:0>;
