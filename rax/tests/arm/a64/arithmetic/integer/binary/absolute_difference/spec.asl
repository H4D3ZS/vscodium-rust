__instruction SABD_Z.P.ZZ__
    __encoding SABD_Z.P.ZZ__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 3
        __field Zm 5 +: 5
        __field Zdn 0 +: 5
        __opcode '00000100 xx001100 000xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer dn = UInt(Zdn);
            integer m = UInt(Zm);
            boolean unsigned = FALSE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(VL) operand1 = Z[dn];
        bits(VL) operand2 = Z[m];
        bits(VL) result;

        for e = 0 to elements-1
            integer element1 = Int(Elem[operand1, e, esize], unsigned);
            integer element2 = Int(Elem[operand2, e, esize], unsigned);
            if ElemP[mask, e, esize] == '1' then
                integer absdiff = Abs(element1 - element2);
                Elem[result, e, esize] = absdiff<esize-1:0>;
            else
                Elem[result, e, esize] = Elem[operand1, e, esize];

        Z[dn] = result;

__instruction UABD_Z.P.ZZ__
    __encoding UABD_Z.P.ZZ__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 3
        __field Zm 5 +: 5
        __field Zdn 0 +: 5
        __opcode '00000100 xx001101 000xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer dn = UInt(Zdn);
            integer m = UInt(Zm);
            boolean unsigned = TRUE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(VL) operand1 = Z[dn];
        bits(VL) operand2 = Z[m];
        bits(VL) result;

        for e = 0 to elements-1
            integer element1 = Int(Elem[operand1, e, esize], unsigned);
            integer element2 = Int(Elem[operand2, e, esize], unsigned);
            if ElemP[mask, e, esize] == '1' then
                integer absdiff = Abs(element1 - element2);
                Elem[result, e, esize] = absdiff<esize-1:0>;
            else
                Elem[result, e, esize] = Elem[operand1, e, esize];

        Z[dn] = result;
