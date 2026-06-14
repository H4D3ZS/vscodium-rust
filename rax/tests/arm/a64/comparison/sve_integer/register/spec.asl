__instruction CMPEQ_P.P.ZZ__
    __encoding CMPEQ_P.P.ZZ__
        __instruction_set A64
        __field size 22 +: 2
        __field Zm 16 +: 5
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Pd 0 +: 4
        __opcode '00100100 xx0xxxxx 101xxxxx xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer m = UInt(Zm);
            integer d = UInt(Pd);
            SVECmp op = Cmp_EQ;
            boolean unsigned = FALSE;

    __encoding CMPGT_P.P.ZZ__
        __instruction_set A64
        __field size 22 +: 2
        __field Zm 16 +: 5
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Pd 0 +: 4
        __opcode '00100100 xx0xxxxx 100xxxxx xxx1xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer m = UInt(Zm);
            integer d = UInt(Pd);
            SVECmp op = Cmp_GT;
            boolean unsigned = FALSE;

    __encoding CMPGE_P.P.ZZ__
        __instruction_set A64
        __field size 22 +: 2
        __field Zm 16 +: 5
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Pd 0 +: 4
        __opcode '00100100 xx0xxxxx 100xxxxx xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer m = UInt(Zm);
            integer d = UInt(Pd);
            SVECmp op = Cmp_GE;
            boolean unsigned = FALSE;

    __encoding CMPHI_P.P.ZZ__
        __instruction_set A64
        __field size 22 +: 2
        __field Zm 16 +: 5
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Pd 0 +: 4
        __opcode '00100100 xx0xxxxx 000xxxxx xxx1xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer m = UInt(Zm);
            integer d = UInt(Pd);
            SVECmp op = Cmp_GT;
            boolean unsigned = TRUE;

    __encoding CMPHS_P.P.ZZ__
        __instruction_set A64
        __field size 22 +: 2
        __field Zm 16 +: 5
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Pd 0 +: 4
        __opcode '00100100 xx0xxxxx 000xxxxx xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer m = UInt(Zm);
            integer d = UInt(Pd);
            SVECmp op = Cmp_GE;
            boolean unsigned = TRUE;

    __encoding CMPNE_P.P.ZZ__
        __instruction_set A64
        __field size 22 +: 2
        __field Zm 16 +: 5
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Pd 0 +: 4
        __opcode '00100100 xx0xxxxx 101xxxxx xxx1xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer m = UInt(Zm);
            integer d = UInt(Pd);
            SVECmp op = Cmp_NE;
            boolean unsigned = FALSE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(VL) operand1 = Z[n];
        bits(VL) operand2 = Z[m];
        bits(PL) result;

        for e = 0 to elements-1
            integer element1 = Int(Elem[operand1, e, esize], unsigned);
            integer element2 = Int(Elem[operand2, e, esize], unsigned);
            if ElemP[mask, e, esize] == '1' then
                boolean cond;
                case op of
                    when Cmp_EQ cond = element1 == element2;
                    when Cmp_NE cond = element1 != element2;
                    when Cmp_GE cond = element1 >= element2;
                    when Cmp_LT cond = element1 <  element2;
                    when Cmp_GT cond = element1 >  element2;
                    when Cmp_LE cond = element1 <= element2;
                ElemP[result, e, esize] = if cond then '1' else '0';
            else
                ElemP[result, e, esize] = '0';

        PSTATE.<N,Z,C,V> = PredTest(mask, result, esize);
        P[d] = result;
