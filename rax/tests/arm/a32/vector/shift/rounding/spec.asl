__instruction aarch32_VRSHL_A
    __encoding aarch32_VRSHL_T1A1_A
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
        __opcode '1111001x 0xxxxxxx xxxx0101 xxx0xxxx'
        __guard TRUE
        __decode
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1' || Vn<0> == '1') then UNDEFINED;
            unsigned = (U == '1');
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  m = UInt(M:Vm);  n = UInt(N:Vn);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VRSHL_T1A1_A
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
        __opcode '111x1111 0xxxxxxx xxxx0101 xxx0xxxx'
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
                round_const = 1 << (-shift-1); // 0 for left shift, 2^(n-1) for right shift
                result = (Int(Elem[D[m+r],e,esize], unsigned) + round_const) << shift;
                Elem[D[d+r],e,esize] = result<esize-1:0>;

__instruction aarch32_VQRSHL_A
    __encoding aarch32_VQRSHL_T1A1_A
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
        __opcode '1111001x 0xxxxxxx xxxx0101 xxx1xxxx'
        __guard TRUE
        __decode
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1' || Vn<0> == '1') then UNDEFINED;
            unsigned = (U == '1');
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  m = UInt(M:Vm);  n = UInt(N:Vn);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VQRSHL_T1A1_A
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
        __opcode '111x1111 0xxxxxxx xxxx0101 xxx1xxxx'
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
                round_const = 1 << (-1-shift); // 0 for left shift, 2^(n-1) for right shift
                operand = Int(Elem[D[m+r],e,esize], unsigned);
                (result, sat) = SatQ((operand + round_const) << shift, esize, unsigned);
                Elem[D[d+r],e,esize] = result;
                if sat then FPSCR.QC = '1';
