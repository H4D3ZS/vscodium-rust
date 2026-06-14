__instruction aarch32_VSLI_A
    __encoding aarch32_VSLI_T1A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field imm6 16 +: 6
        __field Vd 12 +: 4
        __field L 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 1xxxxxxx xxxx0101 xxx1xxxx'
        __guard TRUE
        __decode
            if (L:imm6) == '0000xxx' then SEE "Related encodings";
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            case L:imm6 of
                when '0001xxx'  esize = 8;  elements = 8;  shift_amount = UInt(imm6) - 8;
                when '001xxxx'  esize = 16;  elements = 4;  shift_amount = UInt(imm6) - 16;
                when '01xxxxx'  esize = 32;  elements = 2;  shift_amount = UInt(imm6) - 32;
                when '1xxxxxx'  esize = 64;  elements = 1;  shift_amount = UInt(imm6);
            d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VSLI_T1A1_A
        __instruction_set T32
        __field D 22 +: 1
        __field imm6 16 +: 6
        __field Vd 12 +: 4
        __field L 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 1xxxxxxx xxxx0101 xxx1xxxx'
        __guard TRUE
        __decode
            if (L:imm6) == '0000xxx' then SEE "Related encodings";
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            case L:imm6 of
                when '0001xxx'  esize = 8;  elements = 8;  shift_amount = UInt(imm6) - 8;
                when '001xxxx'  esize = 16;  elements = 4;  shift_amount = UInt(imm6) - 16;
                when '01xxxxx'  esize = 32;  elements = 2;  shift_amount = UInt(imm6) - 32;
                when '1xxxxxx'  esize = 64;  elements = 1;  shift_amount = UInt(imm6);
            d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __execute __conditional
        CheckAdvSIMDEnabled();
        mask = LSL(Ones(esize), shift_amount);
        for r = 0 to regs-1
            for e = 0 to elements-1
                shifted_op = LSL(Elem[D[m+r],e,esize], shift_amount);
                Elem[D[d+r],e,esize] = (Elem[D[d+r],e,esize] AND NOT(mask)) OR shifted_op;

__instruction aarch32_VSRI_A
    __encoding aarch32_VSRI_T1A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field imm6 16 +: 6
        __field Vd 12 +: 4
        __field L 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 1xxxxxxx xxxx0100 xxx1xxxx'
        __guard TRUE
        __decode
            if (L:imm6) == '0000xxx' then SEE "Related encodings";
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            case L:imm6 of
                when '0001xxx'  esize = 8;  elements = 8;  shift_amount = 16 - UInt(imm6);
                when '001xxxx'  esize = 16;  elements = 4;  shift_amount = 32 - UInt(imm6);
                when '01xxxxx'  esize = 32;  elements = 2;  shift_amount = 64 - UInt(imm6);
                when '1xxxxxx'  esize = 64;  elements = 1;  shift_amount = 64 - UInt(imm6);
            d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VSRI_T1A1_A
        __instruction_set T32
        __field D 22 +: 1
        __field imm6 16 +: 6
        __field Vd 12 +: 4
        __field L 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 1xxxxxxx xxxx0100 xxx1xxxx'
        __guard TRUE
        __decode
            if (L:imm6) == '0000xxx' then SEE "Related encodings";
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            case L:imm6 of
                when '0001xxx'  esize = 8;  elements = 8;  shift_amount = 16 - UInt(imm6);
                when '001xxxx'  esize = 16;  elements = 4;  shift_amount = 32 - UInt(imm6);
                when '01xxxxx'  esize = 32;  elements = 2;  shift_amount = 64 - UInt(imm6);
                when '1xxxxxx'  esize = 64;  elements = 1;  shift_amount = 64 - UInt(imm6);
            d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __execute __conditional
        CheckAdvSIMDEnabled();
        mask = LSR(Ones(esize), shift_amount);
        for r = 0 to regs-1
            for e = 0 to elements-1
                shifted_op = LSR(Elem[D[m+r],e,esize], shift_amount);
                Elem[D[d+r],e,esize] = (Elem[D[d+r],e,esize] AND NOT(mask)) OR shifted_op;
