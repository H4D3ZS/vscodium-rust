__instruction FCADD_Z.P.ZZ__
    __encoding FCADD_Z.P.ZZ__
        __instruction_set A64
        __field size 22 +: 2
        __field rot 16 +: 1
        __field Pg 10 +: 3
        __field Zm 5 +: 5
        __field Zdn 0 +: 5
        __opcode '01100100 xx00000x 100xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size == '00' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer dn = UInt(Zdn);
            integer m = UInt(Zm);
            boolean sub_i = (rot == '0');
            boolean sub_r = (rot == '1');

    __execute
        CheckSVEEnabled();
        integer pairs = VL DIV (2 * esize);
        bits(PL) mask = P[g];
        bits(VL) operand1 = Z[dn];
        bits(VL) operand2 = Z[m];
        bits(VL) result;

        for p = 0 to pairs-1
            acc_r  = Elem[operand1, 2 * p + 0, esize];
            acc_i  = Elem[operand1, 2 * p + 1, esize];
            elt2_r = Elem[operand2, 2 * p + 0, esize];
            elt2_i = Elem[operand2, 2 * p + 1, esize];
            if ElemP[mask, 2 * p + 0, esize] == '1' then
                if sub_i then elt2_i = FPNeg(elt2_i);
                acc_r = FPAdd(acc_r, elt2_i, FPCR);
            if ElemP[mask, 2 * p + 1, esize] == '1' then
                if sub_r then elt2_r = FPNeg(elt2_r);
                acc_i = FPAdd(acc_i, elt2_r, FPCR);
            Elem[result, 2 * p + 0, esize] = acc_r;
            Elem[result, 2 * p + 1, esize] = acc_i;

        Z[dn] = result;
