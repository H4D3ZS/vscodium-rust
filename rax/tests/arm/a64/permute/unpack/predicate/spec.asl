__instruction PUNPKHI_P.P__
    __encoding PUNPKHI_P.P__
        __instruction_set A64
        __field Pn 5 +: 4
        __field Pd 0 +: 4
        __opcode '00000101 00110001 0100000x xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 16;
            integer n = UInt(Pn);
            integer d = UInt(Pd);
            boolean hi = TRUE;

    __encoding PUNPKLO_P.P__
        __instruction_set A64
        __field Pn 5 +: 4
        __field Pd 0 +: 4
        __opcode '00000101 00110000 0100000x xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 16;
            integer n = UInt(Pn);
            integer d = UInt(Pd);
            boolean hi = FALSE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) operand = P[n];
        bits(PL) result;

        for e = 0 to elements-1
            ElemP[result, e, esize] = ElemP[operand, if hi then e + elements else e, esize DIV 2];

        P[d] = result;
