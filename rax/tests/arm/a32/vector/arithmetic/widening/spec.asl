__instruction aarch32_VADDL_A
    __encoding aarch32_VADDL_T1A1_A
        __instruction_set A32
        __field U 24 +: 1
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field op 8 +: 1
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '1111001x 1xxxxxxx xxxx0001 x0x0xxxx'
        __guard TRUE
        __decode
            if size == '11' then SEE "Related encodings";
            if Vd<0> == '1' || (op == '1' && Vn<0> == '1') then UNDEFINED;
            unsigned = (U == '1');
            esize = 8 << UInt(size);  elements = 64 DIV esize;  is_vaddw = (op == '1');
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);

    __encoding aarch32_VADDL_T1A1_A
        __instruction_set T32
        __field U 28 +: 1
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field op 8 +: 1
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '111x1111 1xxxxxxx xxxx0001 x0x0xxxx'
        __guard TRUE
        __decode
            if size == '11' then SEE "Related encodings";
            if Vd<0> == '1' || (op == '1' && Vn<0> == '1') then UNDEFINED;
            unsigned = (U == '1');
            esize = 8 << UInt(size);  elements = 64 DIV esize;  is_vaddw = (op == '1');
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);

    __execute __conditional
        CheckAdvSIMDEnabled();
        for e = 0 to elements-1
            if is_vaddw then
                op1 = Int(Elem[Qin[n>>1],e,2*esize], unsigned);
            else
                op1 = Int(Elem[Din[n],e,esize], unsigned);
            result = op1 + Int(Elem[Din[m],e,esize],unsigned);
            Elem[Q[d>>1],e,2*esize] = result<2*esize-1:0>;

__instruction aarch32_VSUBL_A
    __encoding aarch32_VSUBL_T1A1_A
        __instruction_set A32
        __field U 24 +: 1
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field op 8 +: 1
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '1111001x 1xxxxxxx xxxx0010 x0x0xxxx'
        __guard TRUE
        __decode
            if size == '11' then SEE "Related encodings";
            if Vd<0> == '1' || (op == '1' && Vn<0> == '1') then UNDEFINED;
            unsigned = (U == '1');
            esize = 8 << UInt(size);  elements = 64 DIV esize;  is_vsubw = (op == '1');
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);

    __encoding aarch32_VSUBL_T1A1_A
        __instruction_set T32
        __field U 28 +: 1
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field op 8 +: 1
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '111x1111 1xxxxxxx xxxx0010 x0x0xxxx'
        __guard TRUE
        __decode
            if size == '11' then SEE "Related encodings";
            if Vd<0> == '1' || (op == '1' && Vn<0> == '1') then UNDEFINED;
            unsigned = (U == '1');
            esize = 8 << UInt(size);  elements = 64 DIV esize;  is_vsubw = (op == '1');
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);

    __execute __conditional
        CheckAdvSIMDEnabled();
        for e = 0 to elements-1
            if is_vsubw then
                op1 = Int(Elem[Qin[n>>1],e,2*esize], unsigned);
            else
                op1 = Int(Elem[Din[n],e,esize], unsigned);
            result = op1 - Int(Elem[Din[m],e,esize], unsigned);
            Elem[Q[d>>1],e,2*esize] = result<2*esize-1:0>;
