__instruction aarch32_VMUL_i_A
    __encoding aarch32_VMUL_i_A2_A
        __instruction_set A32
        __field U 24 +: 1
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field op 9 +: 1
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '1111001x 1xxxxxxx xxxx11x0 x0x0xxxx'
        __guard TRUE
        __decode
            if size == '11' then SEE "Related encodings";
            unsigned = (U == '1'); polynomial = (op == '1'); long_destination = TRUE;
            esize = 8 << UInt(size); elements = 64 DIV esize;
            if polynomial then
                if U == '1' || size == '01' then UNDEFINED;
                if size == '10' then    // .p64
                    if !HaveBit128PMULLExt() then UNDEFINED;
                    esize = 64; elements = 1;
            if Vd<0> == '1' then UNDEFINED;
            d = UInt(D:Vd); n = UInt(N:Vn); m = UInt(M:Vm); regs = 1;

    __encoding aarch32_VMUL_i_T2_A
        __instruction_set T32
        __field U 28 +: 1
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field op 9 +: 1
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '111x1111 1xxxxxxx xxxx11x0 x0x0xxxx'
        __guard TRUE
        __decode
            if size == '11' then SEE "Related encodings";
            unsigned = (U == '1'); polynomial = (op == '1'); long_destination = TRUE;
            esize = 8 << UInt(size); elements = 64 DIV esize;
            if polynomial then
                if U == '1' || size == '01' then UNDEFINED;
                if size == '10' then    // .p64
                    if InITBlock() then UNPREDICTABLE;
                    if !HaveBit128PMULLExt() then UNDEFINED;
                    esize = 64; elements = 1;
            if Vd<0> == '1' then UNDEFINED;
            d = UInt(D:Vd); n = UInt(N:Vn); m = UInt(M:Vm); regs = 1;

    __execute __conditional
        CheckAdvSIMDEnabled();
        for r = 0 to regs-1
            for e = 0 to elements-1
                op1 = Elem[Din[n+r],e,esize];  op1val = Int(op1, unsigned);
                op2 = Elem[Din[m+r],e,esize];  op2val = Int(op2, unsigned);
                if polynomial then
                    product = PolynomialMult(op1,op2);
                else
                    product = (op1val*op2val)<2*esize-1:0>;
                if long_destination then
                    Elem[Q[d>>1],e,2*esize] = product;
                else
                    Elem[D[d+r],e,esize] = product<esize-1:0>;

__instruction aarch32_VMLA_i_A
    __encoding aarch32_VMLA_i_T1A1_A
        __instruction_set A32
        __field op 24 +: 1
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 0xxxxxxx xxxx1001 xxx0xxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1') then UNDEFINED;
            add = (op == '0');  long_destination = FALSE;
            unsigned = FALSE;  // "Don't care" value: TRUE produces same functionality
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VMLA_i_T1A1_A
        __instruction_set T32
        __field op 28 +: 1
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 0xxxxxxx xxxx1001 xxx0xxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1') then UNDEFINED;
            add = (op == '0');  long_destination = FALSE;
            unsigned = FALSE;  // "Don't care" value: TRUE produces same functionality
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __execute __conditional
        CheckAdvSIMDEnabled();
        for r = 0 to regs-1
            for e = 0 to elements-1
                product = Int(Elem[Din[n+r],e,esize],unsigned) * Int(Elem[Din[m+r],e,esize],unsigned);
                addend = if add then product else -product;
                if long_destination then
                    Elem[Q[d>>1],e,2*esize] = Elem[Qin[d>>1],e,2*esize] + addend;
                else
                    Elem[D[d+r],e,esize] = Elem[Din[d+r],e,esize] + addend;
