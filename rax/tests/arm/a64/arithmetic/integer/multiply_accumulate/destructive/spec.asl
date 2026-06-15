__instruction MAD_Z.P.ZZZ__
    __encoding MAD_Z.P.ZZZ__
        __instruction_set A64
        __field size 22 +: 2
        __field Zm 16 +: 5
        __field Pg 10 +: 3
        __field Za 5 +: 5
        __field Zdn 0 +: 5
        __opcode '00000100 xx0xxxxx 110xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer dn = UInt(Zdn);
            integer m = UInt(Zm);
            integer a = UInt(Za);
            boolean sub_op = FALSE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(VL) operand1 = Z[dn];
        bits(VL) operand2 = Z[m];
        bits(VL) operand3 = Z[a];
        bits(VL) result;

        for e = 0 to elements-1
            integer element1 = UInt(Elem[operand1, e, esize]);
            integer element2 = UInt(Elem[operand2, e, esize]);
            if ElemP[mask, e, esize] == '1' then
                integer product = element1 * element2;
                if sub_op then
                    Elem[result, e, esize] = Elem[operand3, e, esize] - product;
                else
                    Elem[result, e, esize] = Elem[operand3, e, esize] + product;
            else
                Elem[result, e, esize] = Elem[operand1, e, esize];

        Z[dn] = result;

__instruction MSB_Z.P.ZZZ__
    __encoding MSB_Z.P.ZZZ__
        __instruction_set A64
        __field size 22 +: 2
        __field Zm 16 +: 5
        __field Pg 10 +: 3
        __field Za 5 +: 5
        __field Zdn 0 +: 5
        __opcode '00000100 xx0xxxxx 111xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer dn = UInt(Zdn);
            integer m = UInt(Zm);
            integer a = UInt(Za);
            boolean sub_op = TRUE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(VL) operand1 = Z[dn];
        bits(VL) operand2 = Z[m];
        bits(VL) operand3 = Z[a];
        bits(VL) result;

        for e = 0 to elements-1
            integer element1 = UInt(Elem[operand1, e, esize]);
            integer element2 = UInt(Elem[operand2, e, esize]);
            if ElemP[mask, e, esize] == '1' then
                integer product = element1 * element2;
                if sub_op then
                    Elem[result, e, esize] = Elem[operand3, e, esize] - product;
                else
                    Elem[result, e, esize] = Elem[operand3, e, esize] + product;
            else
                Elem[result, e, esize] = Elem[operand1, e, esize];

        Z[dn] = result;
