__instruction PNEXT_P.P.P__
    __encoding PNEXT_P.P.P__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 5 +: 4
        __field Pdn 0 +: 4
        __opcode '00100101 xx011001 1100010x xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer dn = UInt(Pdn);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(PL) operand = P[dn];
        bits(PL) result;

        integer next = LastActiveElement(operand, esize) + 1;

        while next < elements && (ElemP[mask, next, esize] == '0') do
            next = next + 1;

        result = Zeros();
        if next < elements then
            ElemP[result, next, esize] = '1';

        PSTATE.<N,Z,C,V> = PredTest(mask, result, esize);
        P[dn] = result;

__instruction PFIRST_P.P.P__
    __encoding PFIRST_P.P.P__
        __instruction_set A64
        __field Pg 5 +: 4
        __field Pdn 0 +: 4
        __opcode '00100101 01011000 1100000x xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8;
            integer g = UInt(Pg);
            integer dn = UInt(Pdn);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(PL) result = P[dn];
        integer first = -1;

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' && first == -1 then
                first = e;

        if first >= 0 then
            ElemP[result, first, esize] = '1';

        PSTATE.<N,Z,C,V> = PredTest(mask, result, esize);
        P[dn] = result;
