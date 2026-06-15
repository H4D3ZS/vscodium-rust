__instruction PTRUE_P.S__
    __encoding PTRUE_P.S__
        __instruction_set A64
        __field size 22 +: 2
        __field pattern 5 +: 5
        __field Pd 0 +: 4
        __opcode '00100101 xx011000 111000xx xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer d = UInt(Pd);
            boolean setflags = FALSE;
            bits(5) pat = pattern;

    __encoding PTRUES_P.S__
        __instruction_set A64
        __field size 22 +: 2
        __field pattern 5 +: 5
        __field Pd 0 +: 4
        __opcode '00100101 xx011001 111000xx xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer d = UInt(Pd);
            boolean setflags = TRUE;
            bits(5) pat = pattern;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        integer count = DecodePredCount(pat, esize);
        bits(PL) result;

        for e = 0 to elements-1
            ElemP[result, e, esize] = if e < count then '1' else '0';

        if setflags then
            PSTATE.<N,Z,C,V> = PredTest(result, result, esize);
        P[d] = result;

__instruction PFALSE_P__
    __encoding PFALSE_P__
        __instruction_set A64
        __field Pd 0 +: 4
        __opcode '00100101 00011000 11100100 0000xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer d = UInt(Pd);

    __execute
        CheckSVEEnabled();
        P[d] = Zeros(PL);
