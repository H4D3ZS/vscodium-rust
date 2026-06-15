__instruction MUL_Z.ZI__
    __encoding MUL_Z.ZI__
        __instruction_set A64
        __field size 22 +: 2
        __field imm8 5 +: 8
        __field Zdn 0 +: 5
        __opcode '00100101 xx110000 110xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer dn = UInt(Zdn);
            integer imm = SInt(imm8);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) operand1 = Z[dn];
        bits(VL) result;

        for e = 0 to elements-1
            integer element1 = SInt(Elem[operand1, e, esize]);
            Elem[result, e, esize] = (element1 * imm)<esize-1:0>;

        Z[dn] = result;

__instruction MUL_Z.P.ZZ__
    __encoding MUL_Z.P.ZZ__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 3
        __field Zm 5 +: 5
        __field Zdn 0 +: 5
        __opcode '00000100 xx010000 000xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer dn = UInt(Zdn);
            integer m = UInt(Zm);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(VL) operand1 = Z[dn];
        bits(VL) operand2 = Z[m];
        bits(VL) result;

        for e = 0 to elements-1
            integer element1 = UInt(Elem[operand1, e, esize]);
            integer element2 = UInt(Elem[operand2, e, esize]);
            if ElemP[mask, e, esize] == '1' then
                integer product = element1 * element2;
                Elem[result, e, esize] = product<esize-1:0>;
            else
                Elem[result, e, esize] = Elem[operand1, e, esize];

        Z[dn] = result;
