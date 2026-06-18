__instruction aarch32_VPMAX_i_A
    __encoding aarch32_VPMAX_i_T1A1_A
        __instruction_set A32
        __field U 24 +: 1
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field op 4 +: 1
        __field Vm 0 +: 4
        __opcode '1111001x 0xxxxxxx xxxx1010 x0x0xxxx'
        __guard TRUE
        __decode
            if size == '11' || Q == '1' then UNDEFINED;
            maximum = (op == '0');  unsigned = (U == '1');
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);

    __encoding aarch32_VPMAX_i_T1A1_A
        __instruction_set T32
        __field U 28 +: 1
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field op 4 +: 1
        __field Vm 0 +: 4
        __opcode '111x1111 0xxxxxxx xxxx1010 x0x0xxxx'
        __guard TRUE
        __decode
            if size == '11' || Q == '1' then UNDEFINED;
            maximum = (op == '0');  unsigned = (U == '1');
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);

    __execute __conditional
        CheckAdvSIMDEnabled();
        bits(64) dest;
        h = elements DIV 2;

        for e = 0 to h-1
            op1 = Int(Elem[D[n],2*e,esize], unsigned);
            op2 = Int(Elem[D[n],2*e+1,esize], unsigned);
            result = if maximum then Max(op1,op2) else Min(op1,op2);
            Elem[dest,e,esize] = result<esize-1:0>;
            op1 = Int(Elem[D[m],2*e,esize], unsigned);
            op2 = Int(Elem[D[m],2*e+1,esize], unsigned);
            result = if maximum then Max(op1,op2) else Min(op1,op2);
            Elem[dest,e+h,esize] = result<esize-1:0>;

        D[d] = dest;

__instruction aarch32_VPMAX_f_A
    __encoding aarch32_VPMAX_f_A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field op 21 +: 1
        __field sz 20 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 0x1xxxxx xxxx1111 x0x0xxxx'
        __guard TRUE
        __decode
            if Q == '1' then UNDEFINED;
            if sz == '1' && !HaveFP16Ext() then UNDEFINED;
            maximum = (op == '0');
            case sz of
                when '0' esize = 32; elements = 2;
                when '1' esize = 16; elements = 4;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);

    __encoding aarch32_VPMAX_f_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field op 21 +: 1
        __field sz 20 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 0x1xxxxx xxxx1111 x0x0xxxx'
        __guard TRUE
        __decode
            if Q == '1' then UNDEFINED;
            if sz == '1' && !HaveFP16Ext() then UNDEFINED;
            if sz == '1' && InITBlock() then UNPREDICTABLE;
            maximum = (op == '0');
            case sz of
                when '0' esize = 32; elements = 2;
                when '1' esize = 16; elements = 4;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);

    __execute __conditional
        CheckAdvSIMDEnabled();
        bits(64) dest;
        h = elements DIV 2;

        for e = 0 to h-1
            op1 = Elem[D[n],2*e,esize];  op2 = Elem[D[n],2*e+1,esize];
            Elem[dest,e,esize] = if maximum then FPMax(op1,op2,StandardFPSCRValue()) else FPMin(op1,op2,StandardFPSCRValue());
            op1 = Elem[D[m],2*e,esize];  op2 = Elem[D[m],2*e+1,esize];
            Elem[dest,e+h,esize] = if maximum then FPMax(op1,op2,StandardFPSCRValue()) else FPMin(op1,op2,StandardFPSCRValue());

        D[d] = dest;
