__instruction aarch32_VABA_A
    __encoding aarch32_VABA_T1A1_A
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
        __opcode '1111001x 0xxxxxxx xxxx0111 xxx1xxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1') then UNDEFINED;
            unsigned = (U == '1');  long_destination = FALSE;
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VABA_T1A1_A
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
        __opcode '111x1111 0xxxxxxx xxxx0111 xxx1xxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1') then UNDEFINED;
            unsigned = (U == '1');  long_destination = FALSE;
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __execute __conditional
        CheckAdvSIMDEnabled();
        for r = 0 to regs-1
            for e = 0 to elements-1
                op1 = Elem[Din[n+r],e,esize];
                op2 = Elem[Din[m+r],e,esize];
                absdiff = Abs(Int(op1,unsigned) - Int(op2,unsigned));
                if long_destination then
                    Elem[Q[d>>1],e,2*esize] = Elem[Qin[d>>1],e,2*esize] + absdiff;
                else
                    Elem[D[d+r],e,esize] = Elem[Din[d+r],e,esize] + absdiff;

__instruction aarch32_VABD_i_A
    __encoding aarch32_VABD_i_T2A2_A
        __instruction_set A32
        __field U 24 +: 1
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '1111001x 1xxxxxxx xxxx0111 x0x0xxxx'
        __guard TRUE
        __decode
            if size == '11' then SEE "Related encodings";
            if Vd<0> == '1' then UNDEFINED;
            unsigned = (U == '1');  long_destination = TRUE;
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);  regs = 1;

    __encoding aarch32_VABD_i_T2A2_A
        __instruction_set T32
        __field U 28 +: 1
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '111x1111 1xxxxxxx xxxx0111 x0x0xxxx'
        __guard TRUE
        __decode
            if size == '11' then SEE "Related encodings";
            if Vd<0> == '1' then UNDEFINED;
            unsigned = (U == '1');  long_destination = TRUE;
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);  regs = 1;

    __execute __conditional
        CheckAdvSIMDEnabled();
        for r = 0 to regs-1
            for e = 0 to elements-1
                op1 = Elem[Din[n+r],e,esize];
                op2 = Elem[Din[m+r],e,esize];
                absdiff = Abs(Int(op1,unsigned) - Int(op2,unsigned));
                if long_destination then
                    Elem[Q[d>>1],e,2*esize] = absdiff<2*esize-1:0>;
                else
                    Elem[D[d+r],e,esize] = absdiff<esize-1:0>;
