__instruction aarch32_VADDHN_A
    __encoding aarch32_VADDHN_T1A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110010 1xxxxxxx xxxx0100 x0x0xxxx'
        __guard TRUE
        __decode
            if size == '11' then SEE "Related encodings";
            if Vn<0> == '1' || Vm<0> == '1' then UNDEFINED;
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);

    __encoding aarch32_VADDHN_T1A1_A
        __instruction_set T32
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11101111 1xxxxxxx xxxx0100 x0x0xxxx'
        __guard TRUE
        __decode
            if size == '11' then SEE "Related encodings";
            if Vn<0> == '1' || Vm<0> == '1' then UNDEFINED;
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);

    __execute __conditional
        CheckAdvSIMDEnabled();
        for e = 0 to elements-1
            result = Elem[Qin[n>>1],e,2*esize] + Elem[Qin[m>>1],e,2*esize];
            Elem[D[d],e,esize] = result<2*esize-1:esize>;

__instruction aarch32_VSUBHN_A
    __encoding aarch32_VSUBHN_T1A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110010 1xxxxxxx xxxx0110 x0x0xxxx'
        __guard TRUE
        __decode
            if size == '11' then SEE "Related encodings";
            if Vn<0> == '1' || Vm<0> == '1' then UNDEFINED;
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);

    __encoding aarch32_VSUBHN_T1A1_A
        __instruction_set T32
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11101111 1xxxxxxx xxxx0110 x0x0xxxx'
        __guard TRUE
        __decode
            if size == '11' then SEE "Related encodings";
            if Vn<0> == '1' || Vm<0> == '1' then UNDEFINED;
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);

    __execute __conditional
        CheckAdvSIMDEnabled();
        for e = 0 to elements-1
            result = Elem[Qin[n>>1],e,2*esize] - Elem[Qin[m>>1],e,2*esize];
            Elem[D[d],e,esize] = result<2*esize-1:esize>;

__instruction aarch32_VMOVN_A
    __encoding aarch32_VMOVN_T1A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 1x11xx10 xxxx0010 00x0xxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            if Vm<0> == '1' then UNDEFINED;
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  m = UInt(M:Vm);

    __encoding aarch32_VMOVN_T1A1_A
        __instruction_set T32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 1x11xx10 xxxx0010 00x0xxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            if Vm<0> == '1' then UNDEFINED;
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  m = UInt(M:Vm);

    __execute __conditional
        CheckAdvSIMDEnabled();
        for e = 0 to elements-1
            Elem[D[d],e,esize] = Elem[Qin[m>>1],e,2*esize]<esize-1:0>;

__instruction aarch32_VQMOVN_A
    __encoding aarch32_VQMOVN_T1A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field op 6 +: 2
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 1x11xx10 xxxx0010 xxx0xxxx'
        __guard TRUE
        __decode
            if op == '00' then SEE "VMOVN";
            if size == '11' || Vm<0> == '1' then UNDEFINED;
            src_unsigned = (op == '11');  dest_unsigned = (op<0> == '1');
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  m = UInt(M:Vm);

    __encoding aarch32_VQMOVN_T1A1_A
        __instruction_set T32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field op 6 +: 2
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 1x11xx10 xxxx0010 xxx0xxxx'
        __guard TRUE
        __decode
            if op == '00' then SEE "VMOVN";
            if size == '11' || Vm<0> == '1' then UNDEFINED;
            src_unsigned = (op == '11');  dest_unsigned = (op<0> == '1');
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  m = UInt(M:Vm);

    __execute __conditional
        CheckAdvSIMDEnabled();
        for e = 0 to elements-1
            operand = Int(Elem[Qin[m>>1],e,2*esize], src_unsigned);
            (Elem[D[d],e,esize], sat) = SatQ(operand, esize, dest_unsigned);
            if sat then FPSCR.QC = '1';
