__instruction UDIV_Z.P.ZZ__
    __encoding UDIV_Z.P.ZZ__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 3
        __field Zm 5 +: 5
        __field Zdn 0 +: 5
        __opcode '00000100 xx010101 000xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size == '0x' then UNDEFINED;
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
                integer quotient;
                if element2 == 0 then
                    quotient = 0;
                else
                    quotient = RoundTowardsZero(Real(element1) / Real(element2));
                Elem[result, e, esize] = quotient<esize-1:0>;
            else
                Elem[result, e, esize] = Elem[operand1, e, esize];

        Z[dn] = result;

__instruction SDIV_Z.P.ZZ__
    __encoding SDIV_Z.P.ZZ__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 3
        __field Zm 5 +: 5
        __field Zdn 0 +: 5
        __opcode '00000100 xx010100 000xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size == '0x' then UNDEFINED;
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
                integer quotient;
                if element2 == 0 then
                    quotient = 0;
                else
                    quotient = RoundTowardsZero(Real(element1) / Real(element2));
                Elem[result, e, esize] = quotient<esize-1:0>;
            else
                Elem[result, e, esize] = Elem[operand1, e, esize];

        Z[dn] = result;

__instruction UDIVR_Z.P.ZZ__
    __encoding UDIVR_Z.P.ZZ__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 3
        __field Zm 5 +: 5
        __field Zdn 0 +: 5
        __opcode '00000100 xx010111 000xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size == '0x' then UNDEFINED;
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
                integer quotient;
                if element1 == 0 then
                    quotient = 0;
                else
                    quotient = RoundTowardsZero(Real(element2) / Real(element1));
                Elem[result, e, esize] = quotient<esize-1:0>;
            else
                Elem[result, e, esize] = Elem[operand1, e, esize];

        Z[dn] = result;

__instruction SDIVR_Z.P.ZZ__
    __encoding SDIVR_Z.P.ZZ__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 3
        __field Zm 5 +: 5
        __field Zdn 0 +: 5
        __opcode '00000100 xx010110 000xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size == '0x' then UNDEFINED;
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
                integer quotient;
                if element1 == 0 then
                    quotient = 0;
                else
                    quotient = RoundTowardsZero(Real(element2) / Real(element1));
                Elem[result, e, esize] = quotient<esize-1:0>;
            else
                Elem[result, e, esize] = Elem[operand1, e, esize];

        Z[dn] = result;
