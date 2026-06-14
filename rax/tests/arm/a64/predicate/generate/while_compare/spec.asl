__instruction WHILELE_P.P.RR__
    __encoding WHILELE_P.P.RR__
        __instruction_set A64
        __field size 22 +: 2
        __field Rm 16 +: 5
        __field sf 12 +: 1
        __field Rn 5 +: 5
        __field Pd 0 +: 4
        __opcode '00100101 xx1xxxxx 000x01xx xxx1xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer rsize = 32 << UInt(sf);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer d = UInt(Pd);
            boolean unsigned = FALSE;
            SVECmp op = Cmp_LE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = Ones(PL);
        bits(rsize) operand1 = X[n];
        bits(rsize) operand2 = X[m];
        bits(PL) result;
        boolean last = TRUE;

        for e = 0 to elements-1
            boolean cond;
            case op of
                when Cmp_LT cond = (Int(operand1, unsigned) <  Int(operand2, unsigned));
                when Cmp_LE cond = (Int(operand1, unsigned) <= Int(operand2, unsigned));

            last = last && cond;
            ElemP[result, e, esize] = if last then '1' else '0';
            operand1 = operand1 + 1;

        PSTATE.<N,Z,C,V> = PredTest(mask, result, esize);
        P[d] = result;

__instruction WHILELS_P.P.RR__
    __encoding WHILELS_P.P.RR__
        __instruction_set A64
        __field size 22 +: 2
        __field Rm 16 +: 5
        __field sf 12 +: 1
        __field Rn 5 +: 5
        __field Pd 0 +: 4
        __opcode '00100101 xx1xxxxx 000x11xx xxx1xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer rsize = 32 << UInt(sf);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer d = UInt(Pd);
            boolean unsigned = TRUE;
            SVECmp op = Cmp_LE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = Ones(PL);
        bits(rsize) operand1 = X[n];
        bits(rsize) operand2 = X[m];
        bits(PL) result;
        boolean last = TRUE;

        for e = 0 to elements-1
            boolean cond;
            case op of
                when Cmp_LT cond = (Int(operand1, unsigned) <  Int(operand2, unsigned));
                when Cmp_LE cond = (Int(operand1, unsigned) <= Int(operand2, unsigned));

            last = last && cond;
            ElemP[result, e, esize] = if last then '1' else '0';
            operand1 = operand1 + 1;

        PSTATE.<N,Z,C,V> = PredTest(mask, result, esize);
        P[d] = result;

__instruction WHILELT_P.P.RR__
    __encoding WHILELT_P.P.RR__
        __instruction_set A64
        __field size 22 +: 2
        __field Rm 16 +: 5
        __field sf 12 +: 1
        __field Rn 5 +: 5
        __field Pd 0 +: 4
        __opcode '00100101 xx1xxxxx 000x01xx xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer rsize = 32 << UInt(sf);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer d = UInt(Pd);
            boolean unsigned = FALSE;
            SVECmp op = Cmp_LT;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = Ones(PL);
        bits(rsize) operand1 = X[n];
        bits(rsize) operand2 = X[m];
        bits(PL) result;
        boolean last = TRUE;

        for e = 0 to elements-1
            boolean cond;
            case op of
                when Cmp_LT cond = (Int(operand1, unsigned) <  Int(operand2, unsigned));
                when Cmp_LE cond = (Int(operand1, unsigned) <= Int(operand2, unsigned));

            last = last && cond;
            ElemP[result, e, esize] = if last then '1' else '0';
            operand1 = operand1 + 1;

        PSTATE.<N,Z,C,V> = PredTest(mask, result, esize);
        P[d] = result;

__instruction WHILELO_P.P.RR__
    __encoding WHILELO_P.P.RR__
        __instruction_set A64
        __field size 22 +: 2
        __field Rm 16 +: 5
        __field sf 12 +: 1
        __field Rn 5 +: 5
        __field Pd 0 +: 4
        __opcode '00100101 xx1xxxxx 000x11xx xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer rsize = 32 << UInt(sf);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer d = UInt(Pd);
            boolean unsigned = TRUE;
            SVECmp op = Cmp_LT;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = Ones(PL);
        bits(rsize) operand1 = X[n];
        bits(rsize) operand2 = X[m];
        bits(PL) result;
        boolean last = TRUE;

        for e = 0 to elements-1
            boolean cond;
            case op of
                when Cmp_LT cond = (Int(operand1, unsigned) <  Int(operand2, unsigned));
                when Cmp_LE cond = (Int(operand1, unsigned) <= Int(operand2, unsigned));

            last = last && cond;
            ElemP[result, e, esize] = if last then '1' else '0';
            operand1 = operand1 + 1;

        PSTATE.<N,Z,C,V> = PredTest(mask, result, esize);
        P[d] = result;
