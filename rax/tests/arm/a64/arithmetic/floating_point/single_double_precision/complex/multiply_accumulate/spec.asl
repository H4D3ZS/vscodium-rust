__instruction FCMLA_Z.P.ZZZ__
    __encoding FCMLA_Z.P.ZZZ__
        __instruction_set A64
        __field size 22 +: 2
        __field Zm 16 +: 5
        __field rot 13 +: 2
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zda 0 +: 5
        __opcode '01100100 xx0xxxxx 0xxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size == '00' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer m = UInt(Zm);
            integer da = UInt(Zda);
            integer sel_a = UInt(rot<0>);
            integer sel_b = UInt(NOT(rot<0>));
            boolean neg_i = (rot<1> == '1');
            boolean neg_r = (rot<0> != rot<1>);

    __execute
        CheckSVEEnabled();
        integer pairs = VL DIV (2 * esize);
        bits(PL) mask = P[g];
        bits(VL) operand1 = Z[n];
        bits(VL) operand2 = Z[m];
        bits(VL) operand3 = Z[da];
        bits(VL) result;

        for p = 0 to pairs-1
            addend_r = Elem[operand3, 2 * p + 0, esize];
            addend_i = Elem[operand3, 2 * p + 1, esize];
            elt1_a   = Elem[operand1, 2 * p + sel_a, esize];
            elt2_a   = Elem[operand2, 2 * p + sel_a, esize];
            elt2_b   = Elem[operand2, 2 * p + sel_b, esize];
            if ElemP[mask, 2 * p + 0, esize] == '1' then
                if neg_r then elt2_a = FPNeg(elt2_a);
                addend_r = FPMulAdd(addend_r, elt1_a, elt2_a, FPCR);
            if ElemP[mask, 2 * p + 1, esize] == '1' then
                if neg_i then elt2_b = FPNeg(elt2_b);
                addend_i = FPMulAdd(addend_i, elt1_a, elt2_b, FPCR);
            Elem[result, 2 * p + 0, esize] = addend_r;
            Elem[result, 2 * p + 1, esize] = addend_i;

        Z[da] = result;

__instruction FCMLA_Z.ZZZi_H
    __encoding FCMLA_Z.ZZZi_H
        __instruction_set A64
        __field i2 19 +: 2
        __field Zm 16 +: 3
        __field rot 10 +: 2
        __field Zn 5 +: 5
        __field Zda 0 +: 5
        __opcode '01100100 101xxxxx 0001xxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 16;
            integer index = UInt(i2);
            integer n = UInt(Zn);
            integer m = UInt(Zm);
            integer da = UInt(Zda);
            integer sel_a = UInt(rot<0>);
            integer sel_b = UInt(NOT(rot<0>));
            boolean neg_i = (rot<1> == '1');
            boolean neg_r = (rot<0> != rot<1>);

    __encoding FCMLA_Z.ZZZi_S
        __instruction_set A64
        __field i1 20 +: 1
        __field Zm 16 +: 4
        __field rot 10 +: 2
        __field Zn 5 +: 5
        __field Zda 0 +: 5
        __opcode '01100100 111xxxxx 0001xxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 32;
            integer index = UInt(i1);
            integer n = UInt(Zn);
            integer m = UInt(Zm);
            integer da = UInt(Zda);
            integer sel_a = UInt(rot<0>);
            integer sel_b = UInt(NOT(rot<0>));
            boolean neg_i = (rot<1> == '1');
            boolean neg_r = (rot<0> != rot<1>);

    __execute
        CheckSVEEnabled();
        integer pairs = VL DIV (2 * esize);
        integer pairspersegment = 128 DIV (2 * esize);
        bits(VL) operand1 = Z[n];
        bits(VL) operand2 = Z[m];
        bits(VL) operand3 = Z[da];
        bits(VL) result;

        for p = 0 to pairs-1
            segmentbase = p - p MOD pairspersegment;
            s = segmentbase + index;
            addend_r = Elem[operand3, 2 * p + 0, esize];
            addend_i = Elem[operand3, 2 * p + 1, esize];
            elt1_a   = Elem[operand1, 2 * p + sel_a, esize];
            elt2_a   = Elem[operand2, 2 * s + sel_a, esize];
            elt2_b   = Elem[operand2, 2 * s + sel_b, esize];
            if neg_r then elt2_a = FPNeg(elt2_a);
            if neg_i then elt2_b = FPNeg(elt2_b);
            addend_r = FPMulAdd(addend_r, elt1_a, elt2_a, FPCR);
            addend_i = FPMulAdd(addend_i, elt1_a, elt2_b, FPCR);
            Elem[result, 2 * p + 0, esize] = addend_r;
            Elem[result, 2 * p + 1, esize] = addend_i;

        Z[da] = result;
