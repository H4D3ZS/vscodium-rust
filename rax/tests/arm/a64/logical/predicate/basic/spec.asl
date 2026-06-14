__instruction AND_P.P.PP_Z
    __encoding AND_P.P.PP_Z
        __instruction_set A64
        __field S 22 +: 1
        __field Pm 16 +: 4
        __field Pg 10 +: 4
        __field Pn 5 +: 4
        __field Pd 0 +: 4
        __opcode '00100101 0000xxxx 01xxxx0x xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8;
            integer g = UInt(Pg);
            integer n = UInt(Pn);
            integer m = UInt(Pm);
            integer d = UInt(Pd);
            boolean setflags = FALSE;

    __encoding ANDS_P.P.PP_Z
        __instruction_set A64
        __field S 22 +: 1
        __field Pm 16 +: 4
        __field Pg 10 +: 4
        __field Pn 5 +: 4
        __field Pd 0 +: 4
        __opcode '00100101 0100xxxx 01xxxx0x xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8;
            integer g = UInt(Pg);
            integer n = UInt(Pn);
            integer m = UInt(Pm);
            integer d = UInt(Pd);
            boolean setflags = TRUE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(PL) operand1 = P[n];
        bits(PL) operand2 = P[m];
        bits(PL) result;

        for e = 0 to elements-1
            bit element1 = ElemP[operand1, e, esize];
            bit element2 = ElemP[operand2, e, esize];
            if ElemP[mask, e, esize] == '1' then
                ElemP[result, e, esize] = element1 AND element2;
            else
                ElemP[result, e, esize] = '0';

        if setflags then
            PSTATE.<N,Z,C,V> = PredTest(mask, result, esize);
        P[d] = result;

__instruction ORR_P.P.PP_Z
    __encoding ORR_P.P.PP_Z
        __instruction_set A64
        __field S 22 +: 1
        __field Pm 16 +: 4
        __field Pg 10 +: 4
        __field Pn 5 +: 4
        __field Pd 0 +: 4
        __opcode '00100101 1000xxxx 01xxxx0x xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8;
            integer g = UInt(Pg);
            integer n = UInt(Pn);
            integer m = UInt(Pm);
            integer d = UInt(Pd);
            boolean setflags = FALSE;

    __encoding ORRS_P.P.PP_Z
        __instruction_set A64
        __field S 22 +: 1
        __field Pm 16 +: 4
        __field Pg 10 +: 4
        __field Pn 5 +: 4
        __field Pd 0 +: 4
        __opcode '00100101 1100xxxx 01xxxx0x xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8;
            integer g = UInt(Pg);
            integer n = UInt(Pn);
            integer m = UInt(Pm);
            integer d = UInt(Pd);
            boolean setflags = TRUE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(PL) operand1 = P[n];
        bits(PL) operand2 = P[m];
        bits(PL) result;

        for e = 0 to elements-1
            bit element1 = ElemP[operand1, e, esize];
            bit element2 = ElemP[operand2, e, esize];
            if ElemP[mask, e, esize] == '1' then
                ElemP[result, e, esize] = element1 OR element2;
            else
                ElemP[result, e, esize] = '0';

        if setflags then
            PSTATE.<N,Z,C,V> = PredTest(mask, result, esize);
        P[d] = result;

__instruction EOR_P.P.PP_Z
    __encoding EOR_P.P.PP_Z
        __instruction_set A64
        __field Pm 16 +: 4
        __field Pg 10 +: 4
        __field Pn 5 +: 4
        __field Pd 0 +: 4
        __opcode '00100101 0000xxxx 01xxxx1x xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8;
            integer g = UInt(Pg);
            integer n = UInt(Pn);
            integer m = UInt(Pm);
            integer d = UInt(Pd);
            boolean setflags = FALSE;

    __encoding EORS_P.P.PP_Z
        __instruction_set A64
        __field Pm 16 +: 4
        __field Pg 10 +: 4
        __field Pn 5 +: 4
        __field Pd 0 +: 4
        __opcode '00100101 0100xxxx 01xxxx1x xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8;
            integer g = UInt(Pg);
            integer n = UInt(Pn);
            integer m = UInt(Pm);
            integer d = UInt(Pd);
            boolean setflags = TRUE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(PL) operand1 = P[n];
        bits(PL) operand2 = P[m];
        bits(PL) result;

        for e = 0 to elements-1
            bit element1 = ElemP[operand1, e, esize];
            bit element2 = ElemP[operand2, e, esize];
            if ElemP[mask, e, esize] == '1' then
                ElemP[result, e, esize] = element1 EOR element2;
            else
                ElemP[result, e, esize] = '0';

        if setflags then
            PSTATE.<N,Z,C,V> = PredTest(mask, result, esize);
        P[d] = result;

__instruction BIC_P.P.PP_Z
    __encoding BIC_P.P.PP_Z
        __instruction_set A64
        __field Pm 16 +: 4
        __field Pg 10 +: 4
        __field Pn 5 +: 4
        __field Pd 0 +: 4
        __opcode '00100101 0000xxxx 01xxxx0x xxx1xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8;
            integer g = UInt(Pg);
            integer n = UInt(Pn);
            integer m = UInt(Pm);
            integer d = UInt(Pd);
            boolean setflags = FALSE;

    __encoding BICS_P.P.PP_Z
        __instruction_set A64
        __field Pm 16 +: 4
        __field Pg 10 +: 4
        __field Pn 5 +: 4
        __field Pd 0 +: 4
        __opcode '00100101 0100xxxx 01xxxx0x xxx1xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8;
            integer g = UInt(Pg);
            integer n = UInt(Pn);
            integer m = UInt(Pm);
            integer d = UInt(Pd);
            boolean setflags = TRUE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(PL) operand1 = P[n];
        bits(PL) operand2 = P[m];
        bits(PL) result;

        for e = 0 to elements-1
            bit element1 = ElemP[operand1, e, esize];
            bit element2 = ElemP[operand2, e, esize];
            if ElemP[mask, e, esize] == '1' then
                ElemP[result, e, esize] = element1 AND (NOT element2);
            else
                ElemP[result, e, esize] = '0';

        if setflags then
            PSTATE.<N,Z,C,V> = PredTest(mask, result, esize);
        P[d] = result;
