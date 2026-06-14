__instruction aarch32_VSHLL_A
    __encoding aarch32_VSHLL_T1A1_A
        __instruction_set A32
        __field U 24 +: 1
        __field D 22 +: 1
        __field imm6 16 +: 6
        __field Vd 12 +: 4
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '1111001x 1xxxxxxx xxxx1010 00x1xxxx'
        __guard TRUE
        __decode
            if imm6 == '000xxx' then SEE "Related encodings";
            if Vd<0> == '1' then UNDEFINED;
            case imm6 of
                when '001xxx'  esize = 8;  elements = 8;  shift_amount = UInt(imm6) - 8;
                when '01xxxx'  esize = 16;  elements = 4;  shift_amount = UInt(imm6) - 16;
                when '1xxxxx'  esize = 32;  elements = 2;  shift_amount = UInt(imm6) - 32;
            if shift_amount == 0 then SEE "VMOVL";
            unsigned = (U == '1');  d = UInt(D:Vd);  m = UInt(M:Vm);

    __encoding aarch32_VSHLL_T2A2_A
        __instruction_set A32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 1x11xx10 xxxx0011 00x0xxxx'
        __guard TRUE
        __decode
            if size == '11' || Vd<0> == '1' then UNDEFINED;
            esize = 8 << UInt(size);  elements = 64 DIV esize;  shift_amount = esize;
            unsigned = FALSE;  // Or TRUE without change of functionality
            d = UInt(D:Vd);  m = UInt(M:Vm);

    __encoding aarch32_VSHLL_T1A1_A
        __instruction_set T32
        __field U 28 +: 1
        __field D 22 +: 1
        __field imm6 16 +: 6
        __field Vd 12 +: 4
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '111x1111 1xxxxxxx xxxx1010 00x1xxxx'
        __guard TRUE
        __decode
            if imm6 == '000xxx' then SEE "Related encodings";
            if Vd<0> == '1' then UNDEFINED;
            case imm6 of
                when '001xxx'  esize = 8;  elements = 8;  shift_amount = UInt(imm6) - 8;
                when '01xxxx'  esize = 16;  elements = 4;  shift_amount = UInt(imm6) - 16;
                when '1xxxxx'  esize = 32;  elements = 2;  shift_amount = UInt(imm6) - 32;
            if shift_amount == 0 then SEE "VMOVL";
            unsigned = (U == '1');  d = UInt(D:Vd);  m = UInt(M:Vm);

    __encoding aarch32_VSHLL_T2A2_A
        __instruction_set T32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 1x11xx10 xxxx0011 00x0xxxx'
        __guard TRUE
        __decode
            if size == '11' || Vd<0> == '1' then UNDEFINED;
            esize = 8 << UInt(size);  elements = 64 DIV esize;  shift_amount = esize;
            unsigned = FALSE;  // Or TRUE without change of functionality
            d = UInt(D:Vd);  m = UInt(M:Vm);

    __execute __conditional
        CheckAdvSIMDEnabled();
        for e = 0 to elements-1
            result = Int(Elem[Din[m],e,esize], unsigned) << shift_amount;
            Elem[Q[d>>1],e,2*esize] = result<2*esize-1:0>;
