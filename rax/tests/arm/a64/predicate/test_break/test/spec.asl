__instruction PTEST_.P.P__
    __encoding PTEST_.P.P__
        __instruction_set A64
        __field Pg 10 +: 4
        __field Pn 5 +: 4
        __opcode '00100101 01010000 11xxxx0x xxx00000'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8;
            integer g = UInt(Pg);
            integer n = UInt(Pn);

    __execute
        CheckSVEEnabled();
        bits(PL) mask = P[g];
        bits(PL) result = P[n];

        PSTATE.<N,Z,C,V> = PredTest(mask, result, esize);
