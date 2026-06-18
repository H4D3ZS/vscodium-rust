__instruction NAND_P.P.PP_Z
    __encoding NAND_P.P.PP_Z
        __instruction_set A64
        __field Pm 16 +: 4
        __field Pg 10 +: 4
        __field Pn 5 +: 4
        __field Pd 0 +: 4
        __opcode '00100101 1000xxxx 01xxxx1x xxx1xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8;
            integer g = UInt(Pg);
            integer n = UInt(Pn);
            integer m = UInt(Pm);
            integer d = UInt(Pd);
            boolean setflags = FALSE;

    __encoding NANDS_P.P.PP_Z
        __instruction_set A64
        __field Pm 16 +: 4
        __field Pg 10 +: 4
        __field Pn 5 +: 4
        __field Pd 0 +: 4
        __opcode '00100101 1100xxxx 01xxxx1x xxx1xxxx'
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
                ElemP[result, e, esize] = NOT(element1 AND element2);
            else
                ElemP[result, e, esize] = '0';

        if setflags then
            PSTATE.<N,Z,C,V> = PredTest(mask, result, esize);
        P[d] = result;

__instruction NOR_P.P.PP_Z
    __encoding NOR_P.P.PP_Z
        __instruction_set A64
        __field Pm 16 +: 4
        __field Pg 10 +: 4
        __field Pn 5 +: 4
        __field Pd 0 +: 4
        __opcode '00100101 1000xxxx 01xxxx1x xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8;
            integer g = UInt(Pg);
            integer n = UInt(Pn);
            integer m = UInt(Pm);
            integer d = UInt(Pd);
            boolean setflags = FALSE;

    __encoding NORS_P.P.PP_Z
        __instruction_set A64
        __field Pm 16 +: 4
        __field Pg 10 +: 4
        __field Pn 5 +: 4
        __field Pd 0 +: 4
        __opcode '00100101 1100xxxx 01xxxx1x xxx0xxxx'
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
                ElemP[result, e, esize] = NOT(element1 OR element2);
            else
                ElemP[result, e, esize] = '0';

        if setflags then
            PSTATE.<N,Z,C,V> = PredTest(mask, result, esize);
        P[d] = result;

__instruction ORN_P.P.PP_Z
    __encoding ORN_P.P.PP_Z
        __instruction_set A64
        __field Pm 16 +: 4
        __field Pg 10 +: 4
        __field Pn 5 +: 4
        __field Pd 0 +: 4
        __opcode '00100101 1000xxxx 01xxxx0x xxx1xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8;
            integer g = UInt(Pg);
            integer n = UInt(Pn);
            integer m = UInt(Pm);
            integer d = UInt(Pd);
            boolean setflags = FALSE;

    __encoding ORNS_P.P.PP_Z
        __instruction_set A64
        __field Pm 16 +: 4
        __field Pg 10 +: 4
        __field Pn 5 +: 4
        __field Pd 0 +: 4
        __opcode '00100101 1100xxxx 01xxxx0x xxx1xxxx'
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
                ElemP[result, e, esize] = element1 OR (NOT element2);
            else
                ElemP[result, e, esize] = '0';

        if setflags then
            PSTATE.<N,Z,C,V> = PredTest(mask, result, esize);
        P[d] = result;
