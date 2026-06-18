__instruction FMUL_Z.P.ZS__
    __encoding FMUL_Z.P.ZS__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 3
        __field i1 5 +: 1
        __field Zdn 0 +: 5
        __opcode '01100101 xx011010 100xxx00 00xxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size == '00' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer dn = UInt(Zdn);
            bits(esize) imm = if i1 == '0' then FPPointFive('0') else FPTwo('0');

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(VL) operand1 = Z[dn];
        bits(VL) result;

        for e = 0 to elements-1
            bits(esize) element1 = Elem[operand1, e, esize];
            if ElemP[mask, e, esize] == '1' then
                Elem[result, e, esize] = FPMul(element1, imm, FPCR);
            else
                Elem[result, e, esize] = element1;

        Z[dn] = result;
