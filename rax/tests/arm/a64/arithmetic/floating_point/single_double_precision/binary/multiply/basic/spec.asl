__instruction FMUL_Z.P.ZZ__
    __encoding FMUL_Z.P.ZZ__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 3
        __field Zm 5 +: 5
        __field Zdn 0 +: 5
        __opcode '01100101 xx000010 100xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size == '00' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer dn = UInt(Zdn);
            integer m = UInt(Zm);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(VL) operand1 = Z[dn];
        bits(VL) operand2 = Z[m];
        bits(VL) result;

        for e = 0 to elements-1
            bits(esize) element1 = Elem[operand1, e, esize];
            bits(esize) element2 = Elem[operand2, e, esize];
            if ElemP[mask, e, esize] == '1' then
                Elem[result, e, esize] = FPMul(element1, element2, FPCR);
            else
                Elem[result, e, esize] = element1;

        Z[dn] = result;

__instruction FMUL_Z.ZZ__
    __encoding FMUL_Z.ZZ__
        __instruction_set A64
        __field size 22 +: 2
        __field Zm 16 +: 5
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '01100101 xx0xxxxx 000010xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size == '00' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer n = UInt(Zn);
            integer m = UInt(Zm);
            integer d = UInt(Zd);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) operand1 = Z[n];
        bits(VL) operand2 = Z[m];
        bits(VL) result;

        for e = 0 to elements-1
            bits(esize) element1 = Elem[operand1, e, esize];
            bits(esize) element2 = Elem[operand2, e, esize];
            Elem[result, e, esize] = FPMul(element1, element2, FPCR);

        Z[d] = result;
