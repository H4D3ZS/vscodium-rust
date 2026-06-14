__instruction BRKPA_P.P.PP__
    __encoding BRKPA_P.P.PP__
        __instruction_set A64
        __field Pm 16 +: 4
        __field Pg 10 +: 4
        __field Pn 5 +: 4
        __field Pd 0 +: 4
        __opcode '00100101 0000xxxx 11xxxx0x xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8;
            integer g = UInt(Pg);
            integer n = UInt(Pn);
            integer m = UInt(Pm);
            integer d = UInt(Pd);
            boolean setflags = FALSE;

    __encoding BRKPAS_P.P.PP__
        __instruction_set A64
        __field Pm 16 +: 4
        __field Pg 10 +: 4
        __field Pn 5 +: 4
        __field Pd 0 +: 4
        __opcode '00100101 0100xxxx 11xxxx0x xxx0xxxx'
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
        boolean last = (LastActive(mask, operand1, 8) == '1');

        for e = 0 to elements-1
            if ElemP[mask, e, 8] == '1' then
                ElemP[result, e, 8] = if last then '1' else '0';
                last = last && (ElemP[operand2, e, 8] == '0');
            else
                ElemP[result, e, 8] = '0';

        if setflags then
            PSTATE.<N,Z,C,V> = PredTest(mask, result, esize);
        P[d] = result;

__instruction BRKPB_P.P.PP__
    __encoding BRKPB_P.P.PP__
        __instruction_set A64
        __field Pm 16 +: 4
        __field Pg 10 +: 4
        __field Pn 5 +: 4
        __field Pd 0 +: 4
        __opcode '00100101 0000xxxx 11xxxx0x xxx1xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8;
            integer g = UInt(Pg);
            integer n = UInt(Pn);
            integer m = UInt(Pm);
            integer d = UInt(Pd);
            boolean setflags = FALSE;

    __encoding BRKPBS_P.P.PP__
        __instruction_set A64
        __field Pm 16 +: 4
        __field Pg 10 +: 4
        __field Pn 5 +: 4
        __field Pd 0 +: 4
        __opcode '00100101 0100xxxx 11xxxx0x xxx1xxxx'
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
        boolean last = (LastActive(mask, operand1, 8) == '1');

        for e = 0 to elements-1
            if ElemP[mask, e, 8] == '1' then
                last = last && (ElemP[operand2, e, 8] == '0');
                ElemP[result, e, 8] = if last then '1' else '0';
            else
                ElemP[result, e, 8] = '0';

        if setflags then
            PSTATE.<N,Z,C,V> = PredTest(mask, result, esize);
        P[d] = result;

__instruction BRKN_P.P.PP__
    __encoding BRKN_P.P.PP__
        __instruction_set A64
        __field Pg 10 +: 4
        __field Pn 5 +: 4
        __field Pdm 0 +: 4
        __opcode '00100101 00011000 01xxxx0x xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer g = UInt(Pg);
            integer n = UInt(Pn);
            integer dm = UInt(Pdm);
            boolean setflags = FALSE;

    __encoding BRKNS_P.P.PP__
        __instruction_set A64
        __field Pg 10 +: 4
        __field Pn 5 +: 4
        __field Pdm 0 +: 4
        __opcode '00100101 01011000 01xxxx0x xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer g = UInt(Pg);
            integer n = UInt(Pn);
            integer dm = UInt(Pdm);
            boolean setflags = TRUE;

    __execute
        CheckSVEEnabled();
        bits(PL) mask = P[g];
        bits(PL) operand1 = P[n];
        bits(PL) operand2 = P[dm];
        bits(PL) result;

        if LastActive(mask, operand1, 8) == '1' then
            result = operand2;
        else
            result = Zeros();

        if setflags then
            PSTATE.<N,Z,C,V> = PredTest(Ones(PL), result, 8);
        P[dm] = result;
