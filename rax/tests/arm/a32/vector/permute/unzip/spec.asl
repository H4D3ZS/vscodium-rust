__instruction aarch32_VUZP_A
    __encoding aarch32_VUZP_T1A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 1x11xx10 xxxx0001 0xx0xxxx'
        __guard TRUE
        __decode
            if size == '11' || (Q == '0' && size == '10') then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            quadword_operation = (Q == '1');  esize = 8 << UInt(size);
            d = UInt(D:Vd);  m = UInt(M:Vm);

    __encoding aarch32_VUZP_T1A1_A
        __instruction_set T32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 1x11xx10 xxxx0001 0xx0xxxx'
        __guard TRUE
        __decode
            if size == '11' || (Q == '0' && size == '10') then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            quadword_operation = (Q == '1');  esize = 8 << UInt(size);
            d = UInt(D:Vd);  m = UInt(M:Vm);

    __execute __conditional
        CheckAdvSIMDEnabled();
        if quadword_operation then
            if d == m then
                Q[d>>1] = bits(128) UNKNOWN;  Q[m>>1] = bits(128) UNKNOWN;
            else
                zipped_q = Q[m>>1]:Q[d>>1];
                for e = 0 to (128 DIV esize) - 1
                    Elem[Q[d>>1],e,esize] = Elem[zipped_q,2*e,esize];
                    Elem[Q[m>>1],e,esize] = Elem[zipped_q,2*e+1,esize];
        else
            if d == m then
                D[d] = bits(64) UNKNOWN;  D[m] = bits(64) UNKNOWN;
            else
                zipped_d = D[m]:D[d];
                for e = 0 to (64 DIV esize) - 1
                    Elem[D[d],e,esize] = Elem[zipped_d,2*e,esize];
                    Elem[D[m],e,esize] = Elem[zipped_d,2*e+1,esize];
