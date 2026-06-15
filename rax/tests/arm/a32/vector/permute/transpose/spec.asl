__instruction aarch32_VTRN_A
    __encoding aarch32_VTRN_T1A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 1x11xx10 xxxx0000 1xx0xxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VTRN_T1A1_A
        __instruction_set T32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 1x11xx10 xxxx0000 1xx0xxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __execute __conditional
        CheckAdvSIMDEnabled();
        h = elements DIV 2;

        for r = 0 to regs-1
            if d == m then
                D[d+r] = bits(64) UNKNOWN;
            else
                for e = 0 to h-1
                    Elem[D[d+r],2*e+1,esize] = Elem[Din[m+r],2*e,esize];
                    Elem[D[m+r],2*e,esize] = Elem[Din[d+r],2*e+1,esize];
