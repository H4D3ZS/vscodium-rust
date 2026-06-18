__instruction COMPACT_Z.P.Z__
    __encoding COMPACT_Z.P.Z__
        __instruction_set A64
        __field sz 22 +: 1
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '00000101 1x100001 100xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 32 << UInt(sz);
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Zd);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(VL) operand1 = Z[n];
        bits(VL) result;
        integer x = 0;

        for e = 0 to elements-1
            Elem[result, e, esize] = Zeros();
            if ElemP[mask, e, esize] == '1' then
                bits(esize) element = Elem[operand1, e, esize];
                Elem[result, x, esize] = element;
                x = x + 1;

        Z[d] = result;
