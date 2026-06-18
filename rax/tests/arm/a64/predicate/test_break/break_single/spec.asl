__instruction BRKA_P.P.P__
    __encoding BRKA_P.P.P__
        __instruction_set A64
        __field Pg 10 +: 4
        __field Pn 5 +: 4
        __field M 4 +: 1
        __field Pd 0 +: 4
        __opcode '00100101 00010000 01xxxx0x xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8;
            integer g = UInt(Pg);
            integer n = UInt(Pn);
            integer d = UInt(Pd);
            boolean merging = (M == '1');
            boolean setflags = FALSE;

    __encoding BRKAS_P.P.P_Z
        __instruction_set A64
        __field Pg 10 +: 4
        __field Pn 5 +: 4
        __field Pd 0 +: 4
        __opcode '00100101 01010000 01xxxx0x xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8;
            integer g = UInt(Pg);
            integer n = UInt(Pn);
            integer d = UInt(Pd);
            boolean merging = FALSE;
            boolean setflags = TRUE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(PL) operand  = P[n];
        bits(PL) operand2 = P[d];
        boolean break = FALSE;
        bits(PL) result;

        for e = 0 to elements-1
            boolean element = ElemP[operand, e, esize] == '1';
            if ElemP[mask, e, esize] == '1' then
                ElemP[result, e, esize] = if !break then '1' else '0';
                break = break || element;
            elsif merging then
                ElemP[result, e, esize] = ElemP[operand2, e, esize];
            else
                ElemP[result, e, esize] = '0';

        if setflags then
            PSTATE.<N,Z,C,V> = PredTest(mask, result, esize);
        P[d] = result;

__instruction BRKB_P.P.P__
    __encoding BRKB_P.P.P__
        __instruction_set A64
        __field Pg 10 +: 4
        __field Pn 5 +: 4
        __field M 4 +: 1
        __field Pd 0 +: 4
        __opcode '00100101 10010000 01xxxx0x xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8;
            integer g = UInt(Pg);
            integer n = UInt(Pn);
            integer d = UInt(Pd);
            boolean merging = (M == '1');
            boolean setflags = FALSE;

    __encoding BRKBS_P.P.P_Z
        __instruction_set A64
        __field Pg 10 +: 4
        __field Pn 5 +: 4
        __field Pd 0 +: 4
        __opcode '00100101 11010000 01xxxx0x xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8;
            integer g = UInt(Pg);
            integer n = UInt(Pn);
            integer d = UInt(Pd);
            boolean merging = FALSE;
            boolean setflags = TRUE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(PL) operand  = P[n];
        bits(PL) operand2 = P[d];
        boolean break = FALSE;
        bits(PL) result;

        for e = 0 to elements-1
            boolean element = ElemP[operand, e, esize] == '1';
            if ElemP[mask, e, esize] == '1' then
                break = break || element;
                ElemP[result, e, esize] = if !break then '1' else '0';
            elsif merging then
                ElemP[result, e, esize] = ElemP[operand2, e, esize];
            else
                ElemP[result, e, esize] = '0';

        if setflags then
            PSTATE.<N,Z,C,V> = PredTest(mask, result, esize);
        P[d] = result;
