__instruction aarch32_VPADD_i_A
    __encoding aarch32_VPADD_i_T1A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110010 0xxxxxxx xxxx1011 xxx1xxxx'
        __guard TRUE
        __decode
            if size == '11' || Q == '1' then UNDEFINED;
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);

    __encoding aarch32_VPADD_i_T1A1_A
        __instruction_set T32
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11101111 0xxxxxxx xxxx1011 xxx1xxxx'
        __guard TRUE
        __decode
            if size == '11' || Q == '1' then UNDEFINED;
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);

    __execute __conditional
        CheckAdvSIMDEnabled();
        bits(64) dest;
        h = elements DIV 2;

        for e = 0 to h-1
            Elem[dest,e,esize]   = Elem[D[n],2*e,esize] + Elem[D[n],2*e+1,esize];
            Elem[dest,e+h,esize] = Elem[D[m],2*e,esize] + Elem[D[m],2*e+1,esize];

        D[d] = dest;

__instruction aarch32_VPADD_f_A
    __encoding aarch32_VPADD_f_A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field sz 20 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 0x0xxxxx xxxx1101 xxx0xxxx'
        __guard TRUE
        __decode
            if Q == '1' then UNDEFINED;
            if sz == '1' && !HaveFP16Ext() then UNDEFINED;
            case sz of
                when '0' esize = 32; elements = 2;
                when '1' esize = 16; elements = 4;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);

    __encoding aarch32_VPADD_f_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field sz 20 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 0x0xxxxx xxxx1101 xxx0xxxx'
        __guard TRUE
        __decode
            if Q == '1' then UNDEFINED;
            if sz == '1' && !HaveFP16Ext() then UNDEFINED;
            if sz == '1' && InITBlock() then UNPREDICTABLE;
            case sz of
                when '0' esize = 32; elements = 2;
                when '1' esize = 16; elements = 4;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);

    __execute __conditional
        CheckAdvSIMDEnabled();
        bits(64) dest;
        h = elements DIV 2;

        for e = 0 to h-1
            Elem[dest,e,esize]   = FPAdd(Elem[D[n],2*e,esize], Elem[D[n],2*e+1,esize], StandardFPSCRValue());
            Elem[dest,e+h,esize] = FPAdd(Elem[D[m],2*e,esize], Elem[D[m],2*e+1,esize], StandardFPSCRValue());

        D[d] = dest;

__instruction aarch32_VPADDL_A
    __encoding aarch32_VPADDL_T1A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field op 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 1x11xx00 xxxx0010 xxx0xxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            unsigned = (op == '1');
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VPADDL_T1A1_A
        __instruction_set T32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field op 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 1x11xx00 xxxx0010 xxx0xxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            unsigned = (op == '1');
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __execute __conditional
        CheckAdvSIMDEnabled();
        h = elements DIV 2;

        for r = 0 to regs-1
            for e = 0 to h-1
                op1 = Elem[D[m+r],2*e,esize];  op2 = Elem[D[m+r],2*e+1,esize];
                result = Int(op1, unsigned) + Int(op2, unsigned);
                Elem[D[d+r],e,2*esize] = result<2*esize-1:0>;

__instruction aarch32_VPADAL_A
    __encoding aarch32_VPADAL_T1A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field op 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 1x11xx00 xxxx0110 xxx0xxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            unsigned = (op == '1');
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VPADAL_T1A1_A
        __instruction_set T32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field op 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 1x11xx00 xxxx0110 xxx0xxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            unsigned = (op == '1');
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __execute __conditional
        CheckAdvSIMDEnabled();
        h = elements DIV 2;

        for r = 0 to regs-1
            for e = 0 to h-1
                op1 = Elem[D[m+r],2*e,esize];  op2 = Elem[D[m+r],2*e+1,esize];
                result = Int(op1, unsigned) + Int(op2, unsigned);
                Elem[D[d+r],e,2*esize] = Elem[D[d+r],e,2*esize] + result;
