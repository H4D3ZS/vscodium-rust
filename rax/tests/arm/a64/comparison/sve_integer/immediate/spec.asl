__instruction CMPEQ_P.P.ZI__
    __encoding CMPEQ_P.P.ZI__
        __instruction_set A64
        __field size 22 +: 2
        __field imm5 16 +: 5
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Pd 0 +: 4
        __opcode '00100101 xx0xxxxx 100xxxxx xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Pd);
            SVECmp op = Cmp_EQ;
            integer imm = SInt(imm5);
            boolean unsigned = FALSE;

    __encoding CMPGT_P.P.ZI__
        __instruction_set A64
        __field size 22 +: 2
        __field imm5 16 +: 5
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Pd 0 +: 4
        __opcode '00100101 xx0xxxxx 000xxxxx xxx1xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Pd);
            SVECmp op = Cmp_GT;
            integer imm = SInt(imm5);
            boolean unsigned = FALSE;

    __encoding CMPGE_P.P.ZI__
        __instruction_set A64
        __field size 22 +: 2
        __field imm5 16 +: 5
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Pd 0 +: 4
        __opcode '00100101 xx0xxxxx 000xxxxx xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Pd);
            SVECmp op = Cmp_GE;
            integer imm = SInt(imm5);
            boolean unsigned = FALSE;

    __encoding CMPHI_P.P.ZI__
        __instruction_set A64
        __field size 22 +: 2
        __field imm7 14 +: 7
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Pd 0 +: 4
        __opcode '00100100 xx1xxxxx xx0xxxxx xxx1xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Pd);
            SVECmp op = Cmp_GT;
            integer imm = UInt(imm7);
            boolean unsigned = TRUE;

    __encoding CMPHS_P.P.ZI__
        __instruction_set A64
        __field size 22 +: 2
        __field imm7 14 +: 7
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Pd 0 +: 4
        __opcode '00100100 xx1xxxxx xx0xxxxx xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Pd);
            SVECmp op = Cmp_GE;
            integer imm = UInt(imm7);
            boolean unsigned = TRUE;

    __encoding CMPLT_P.P.ZI__
        __instruction_set A64
        __field size 22 +: 2
        __field imm5 16 +: 5
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Pd 0 +: 4
        __opcode '00100101 xx0xxxxx 001xxxxx xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Pd);
            SVECmp op = Cmp_LT;
            integer imm = SInt(imm5);
            boolean unsigned = FALSE;

    __encoding CMPLE_P.P.ZI__
        __instruction_set A64
        __field size 22 +: 2
        __field imm5 16 +: 5
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Pd 0 +: 4
        __opcode '00100101 xx0xxxxx 001xxxxx xxx1xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Pd);
            SVECmp op = Cmp_LE;
            integer imm = SInt(imm5);
            boolean unsigned = FALSE;

    __encoding CMPLO_P.P.ZI__
        __instruction_set A64
        __field size 22 +: 2
        __field imm7 14 +: 7
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Pd 0 +: 4
        __opcode '00100100 xx1xxxxx xx1xxxxx xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Pd);
            SVECmp op = Cmp_LT;
            integer imm = UInt(imm7);
            boolean unsigned = TRUE;

    __encoding CMPLS_P.P.ZI__
        __instruction_set A64
        __field size 22 +: 2
        __field imm7 14 +: 7
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Pd 0 +: 4
        __opcode '00100100 xx1xxxxx xx1xxxxx xxx1xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Pd);
            SVECmp op = Cmp_LE;
            integer imm = UInt(imm7);
            boolean unsigned = TRUE;

    __encoding CMPNE_P.P.ZI__
        __instruction_set A64
        __field size 22 +: 2
        __field imm5 16 +: 5
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Pd 0 +: 4
        __opcode '00100101 xx0xxxxx 100xxxxx xxx1xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Pd);
            SVECmp op = Cmp_NE;
            integer imm = SInt(imm5);
            boolean unsigned = FALSE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(VL) operand1 = Z[n];
        bits(PL) result;

        for e = 0 to elements-1
            integer element1 = Int(Elem[operand1, e, esize], unsigned);
            if ElemP[mask, e, esize] == '1' then
                boolean cond;
                case op of
                    when Cmp_EQ cond = element1 == imm;
                    when Cmp_NE cond = element1 != imm;
                    when Cmp_GE cond = element1 >= imm;
                    when Cmp_LT cond = element1 <  imm;
                    when Cmp_GT cond = element1 >  imm;
                    when Cmp_LE cond = element1 <= imm;
                ElemP[result, e, esize] = if cond then '1' else '0';
            else
                ElemP[result, e, esize] = '0';

        PSTATE.<N,Z,C,V> = PredTest(mask, result, esize);
        P[d] = result;
