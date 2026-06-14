__instruction aarch32_VEXT_A
    __encoding aarch32_VEXT_T1A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field imm4 8 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110010 1x11xxxx xxxxxxxx xxx0xxxx'
        __guard TRUE
        __decode
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1') then UNDEFINED;
            if Q == '0' && imm4<3> == '1' then UNDEFINED;
            quadword_operation = (Q == '1');  position = 8 * UInt(imm4);
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);

    __encoding aarch32_VEXT_T1A1_A
        __instruction_set T32
        __field D 22 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field imm4 8 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11101111 1x11xxxx xxxxxxxx xxx0xxxx'
        __guard TRUE
        __decode
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1') then UNDEFINED;
            if Q == '0' && imm4<3> == '1' then UNDEFINED;
            quadword_operation = (Q == '1');  position = 8 * UInt(imm4);
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);

    __execute __conditional
        CheckAdvSIMDEnabled();
        if quadword_operation then
            Q[d>>1] = (Q[m>>1]:Q[n>>1])<position+127:position>;
        else
            D[d] = (D[m]:D[n])<position+63:position>;
