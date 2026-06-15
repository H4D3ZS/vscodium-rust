__instruction RDFFR_P.P.F__
    __encoding RDFFR_P.P.F__
        __instruction_set A64
        __field Pg 5 +: 4
        __field Pd 0 +: 4
        __opcode '00100101 00011000 1111000x xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer g = UInt(Pg);
            integer d = UInt(Pd);
            boolean setflags = FALSE;

    __encoding RDFFRS_P.P.F__
        __instruction_set A64
        __field Pg 5 +: 4
        __field Pd 0 +: 4
        __opcode '00100101 01011000 1111000x xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer g = UInt(Pg);
            integer d = UInt(Pd);
            boolean setflags = TRUE;

    __execute
        CheckSVEEnabled();
        bits(PL) mask = P[g];
        bits(PL) ffr = FFR[];
        bits(PL) result = ffr AND mask;

        if setflags then
            PSTATE.<N,Z,C,V> = PredTest(mask, result, 8);
        P[d] = result;

__instruction RDFFR_P.F__
    __encoding RDFFR_P.F__
        __instruction_set A64
        __field Pd 0 +: 4
        __opcode '00100101 00011001 11110000 0000xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer d = UInt(Pd);

    __execute
        CheckSVEEnabled();
        bits(PL) ffr = FFR[];
        P[d] = ffr;

__instruction SETFFR_F__
    __encoding SETFFR_F__
        __instruction_set A64
        __opcode '00100101 00101100 10010000 00000000'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;

    __execute
        CheckSVEEnabled();
        FFR[] = Ones(PL);

__instruction WRFFR_F.P__
    __encoding WRFFR_F.P__
        __instruction_set A64
        __field Pn 5 +: 4
        __opcode '00100101 00101000 1001000x xxx00000'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer n = UInt(Pn);

    __execute
        CheckSVEEnabled();
        bits(PL) operand = P[n];

        hsb = HighestSetBit(operand);
        if hsb < 0 || IsOnes(operand<hsb:0>) then
            FFR[] = operand;
        else // not a monotonic predicate
            FFR[] = bits(PL) UNKNOWN;
