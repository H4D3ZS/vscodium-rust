__instruction aarch32_VCLS_A
    __encoding aarch32_VCLS_T1A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 1x11xx00 xxxx0100 0xx0xxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VCLS_T1A1_A
        __instruction_set T32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 1x11xx00 xxxx0100 0xx0xxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __execute __conditional
        CheckAdvSIMDEnabled();
        for r = 0 to regs-1
            for e = 0 to elements-1
                Elem[D[d+r],e,esize] = CountLeadingSignBits(Elem[D[m+r],e,esize])<esize-1:0>;

__instruction aarch32_VCLZ_A
    __encoding aarch32_VCLZ_T1A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 1x11xx00 xxxx0100 1xx0xxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VCLZ_T1A1_A
        __instruction_set T32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 1x11xx00 xxxx0100 1xx0xxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __execute __conditional
        CheckAdvSIMDEnabled();
        for r = 0 to regs-1
            for e = 0 to elements-1
                Elem[D[d+r],e,esize] = CountLeadingZeroBits(Elem[D[m+r],e,esize])<esize-1:0>;

__instruction aarch32_VCNT_A
    __encoding aarch32_VCNT_T1A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 1x11xx00 xxxx0101 0xx0xxxx'
        __guard TRUE
        __decode
            if size != '00' then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            esize = 8;  elements = 8;
            d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VCNT_T1A1_A
        __instruction_set T32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 1x11xx00 xxxx0101 0xx0xxxx'
        __guard TRUE
        __decode
            if size != '00' then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            esize = 8;  elements = 8;
            d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __execute __conditional
        CheckAdvSIMDEnabled();
        for r = 0 to regs-1
            for e = 0 to elements-1
                Elem[D[d+r],e,esize] = BitCount(Elem[D[m+r],e,esize])<esize-1:0>;
