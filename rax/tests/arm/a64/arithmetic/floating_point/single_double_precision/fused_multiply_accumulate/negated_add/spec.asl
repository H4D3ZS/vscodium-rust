__instruction FNMLA_Z.P.ZZZ__
    __encoding FNMLA_Z.P.ZZZ__
        __instruction_set A64
        __field size 22 +: 2
        __field Zm 16 +: 5
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zda 0 +: 5
        __opcode '01100101 xx1xxxxx 010xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size == '00' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer m = UInt(Zm);
            integer da = UInt(Zda);
            boolean op1_neg = TRUE;
            boolean op3_neg = TRUE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(VL) operand1 = Z[n];
        bits(VL) operand2 = Z[m];
        bits(VL) operand3 = Z[da];
        bits(VL) result;

        for e = 0 to elements-1
            bits(esize) element1 = Elem[operand1, e, esize];
            bits(esize) element2 = Elem[operand2, e, esize];
            bits(esize) element3 = Elem[operand3, e, esize];

            if ElemP[mask, e, esize] == '1' then
                if op1_neg then element1 = FPNeg(element1);
                if op3_neg then element3 = FPNeg(element3);
                Elem[result, e, esize] = FPMulAdd(element3, element1, element2, FPCR);
            else
                Elem[result, e, esize] = element3;

        Z[da] = result;

__instruction FNMAD_Z.P.ZZZ__
    __encoding FNMAD_Z.P.ZZZ__
        __instruction_set A64
        __field size 22 +: 2
        __field Za 16 +: 5
        __field Pg 10 +: 3
        __field Zm 5 +: 5
        __field Zdn 0 +: 5
        __opcode '01100101 xx1xxxxx 110xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size == '00' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer dn = UInt(Zdn);
            integer m = UInt(Zm);
            integer a = UInt(Za);
            boolean op1_neg = TRUE;
            boolean op3_neg = TRUE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(VL) operand1 = Z[dn];
        bits(VL) operand2 = Z[m];
        bits(VL) operand3 = Z[a];
        bits(VL) result;

        for e = 0 to elements-1
            bits(esize) element1 = Elem[operand1, e, esize];
            bits(esize) element2 = Elem[operand2, e, esize];
            bits(esize) element3 = Elem[operand3, e, esize];

            if ElemP[mask, e, esize] == '1' then
                if op1_neg then element1 = FPNeg(element1);
                if op3_neg then element3 = FPNeg(element3);
                Elem[result, e, esize] = FPMulAdd(element3, element1, element2, FPCR);
            else
                Elem[result, e, esize] = element1;

        Z[dn] = result;
