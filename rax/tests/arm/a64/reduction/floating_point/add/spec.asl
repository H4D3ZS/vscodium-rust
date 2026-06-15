__instruction FADDV_V.P.Z__
    __encoding FADDV_V.P.Z__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Vd 0 +: 5
        __opcode '01100101 xx000000 001xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size == '00' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Vd);

    __execute
        CheckSVEEnabled();
        bits(PL) mask = P[g];
        bits(VL) operand = Z[n];
        bits(esize) identity = FPZero('0');

        V[d] = ReducePredicated(ReduceOp_FADD, operand, mask, identity);

__instruction FADDA_V.P.Z__
    __encoding FADDA_V.P.Z__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 3
        __field Zm 5 +: 5
        __field Vdn 0 +: 5
        __opcode '01100101 xx011000 001xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size == '00' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer dn = UInt(Vdn);
            integer m = UInt(Zm);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(esize) operand1 = V[dn];
        bits(VL) operand2 = Z[m];
        bits(esize) result = operand1;

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                bits(esize) element = Elem[operand2, e, esize];
                result = FPAdd(result, element, FPCR);

        V[dn] = result;
