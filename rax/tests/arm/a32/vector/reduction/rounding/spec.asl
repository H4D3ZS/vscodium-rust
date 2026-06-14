__instruction aarch32_VRHADD_A
    __encoding aarch32_VRHADD_T1A1_A
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
        __opcode '1111001x 0xxxxxxx xxxx0001 xxx0xxxx'
        __guard TRUE
        __decode
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1') then UNDEFINED;
            if size == '11' then UNDEFINED;
            unsigned = (U == '1');
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VRHADD_T1A1_A
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
        __opcode '111x1111 0xxxxxxx xxxx0001 xxx0xxxx'
        __guard TRUE
        __decode
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1') then UNDEFINED;
            if size == '11' then UNDEFINED;
            unsigned = (U == '1');
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __execute __conditional
        CheckAdvSIMDEnabled();
        for r = 0 to regs-1
            for e = 0 to elements-1
                op1 = Int(Elem[D[n+r],e,esize], unsigned);
                op2 = Int(Elem[D[m+r],e,esize], unsigned);
                result = op1 + op2 + 1;
                Elem[D[d+r],e,esize] = result<esize:1>;

__instruction aarch32_VRADDHN_A
    __encoding aarch32_VRADDHN_T1A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 1xxxxxxx xxxx0100 x0x0xxxx'
        __guard TRUE
        __decode
            if size == '11' then SEE "Related encodings";
            if Vn<0> == '1' || Vm<0> == '1' then UNDEFINED;
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);

    __encoding aarch32_VRADDHN_T1A1_A
        __instruction_set T32
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 1xxxxxxx xxxx0100 x0x0xxxx'
        __guard TRUE
        __decode
            if size == '11' then SEE "Related encodings";
            if Vn<0> == '1' || Vm<0> == '1' then UNDEFINED;
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);

    __execute __conditional
        CheckAdvSIMDEnabled();
        round_const = 1 << (esize-1);
        for e = 0 to elements-1
            result = Elem[Qin[n>>1],e,2*esize] + Elem[Qin[m>>1],e,2*esize] + round_const;
            Elem[D[d],e,esize] = result<2*esize-1:esize>;
