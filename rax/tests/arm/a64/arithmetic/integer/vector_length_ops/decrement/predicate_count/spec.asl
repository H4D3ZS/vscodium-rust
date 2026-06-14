__instruction DECP_R.P.R__
    __encoding DECP_R.P.R__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 5 +: 4
        __field Rdn 0 +: 5
        __opcode '00100101 xx101101 1000100x xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer dn = UInt(Rdn);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(64) operand = X[dn];
        integer count = 0;

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                count = count + 1;

        X[dn] = operand - count;

__instruction DECP_Z.P.Z__
    __encoding DECP_Z.P.Z__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 5 +: 4
        __field Zdn 0 +: 5
        __opcode '00100101 xx101101 1000000x xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size == '00' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer dn = UInt(Zdn);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(VL) operand = Z[dn];
        bits(VL) result;
        integer count = 0;

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                count = count + 1;

        for e = 0 to elements-1
            Elem[result, e, esize] = Elem[operand, e, esize] - count;

        Z[dn] = result;

__instruction SQDECP_R.P.R_SX
    __encoding SQDECP_R.P.R_SX
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 5 +: 4
        __field Rdn 0 +: 5
        __opcode '00100101 xx101010 1000100x xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer dn = UInt(Rdn);
            boolean unsigned = FALSE;
            integer ssize = 32;

    __encoding SQDECP_R.P.R_X
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 5 +: 4
        __field Rdn 0 +: 5
        __opcode '00100101 xx101010 1000110x xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer dn = UInt(Rdn);
            boolean unsigned = FALSE;
            integer ssize = 64;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(ssize) operand = X[dn];
        bits(ssize) result;
        integer count = 0;

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                count = count + 1;

        integer element = Int(operand, unsigned);
        (result, -) = SatQ(element - count, ssize, unsigned);
        X[dn] = Extend(result, 64, unsigned);

__instruction SQDECP_Z.P.Z__
    __encoding SQDECP_Z.P.Z__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 5 +: 4
        __field Zdn 0 +: 5
        __opcode '00100101 xx101010 1000000x xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size == '00' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer dn = UInt(Zdn);
            boolean unsigned = FALSE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(VL) operand = Z[dn];
        bits(VL) result;
        integer count = 0;

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                count = count + 1;

        for e = 0 to elements-1
            integer element = Int(Elem[operand, e, esize], unsigned);
            (Elem[result, e, esize], -) = SatQ(element - count, esize, unsigned);

        Z[dn] = result;

__instruction UQDECP_R.P.R_UW
    __encoding UQDECP_R.P.R_UW
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 5 +: 4
        __field Rdn 0 +: 5
        __opcode '00100101 xx101011 1000100x xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer dn = UInt(Rdn);
            boolean unsigned = TRUE;
            integer ssize = 32;

    __encoding UQDECP_R.P.R_X
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 5 +: 4
        __field Rdn 0 +: 5
        __opcode '00100101 xx101011 1000110x xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer dn = UInt(Rdn);
            boolean unsigned = TRUE;
            integer ssize = 64;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(ssize) operand = X[dn];
        bits(ssize) result;
        integer count = 0;

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                count = count + 1;

        integer element = Int(operand, unsigned);
        (result, -) = SatQ(element - count, ssize, unsigned);
        X[dn] = Extend(result, 64, unsigned);

__instruction UQDECP_Z.P.Z__
    __encoding UQDECP_Z.P.Z__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 5 +: 4
        __field Zdn 0 +: 5
        __opcode '00100101 xx101011 1000000x xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size == '00' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer dn = UInt(Zdn);
            boolean unsigned = TRUE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(VL) operand = Z[dn];
        bits(VL) result;
        integer count = 0;

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                count = count + 1;

        for e = 0 to elements-1
            integer element = Int(Elem[operand, e, esize], unsigned);
            (Elem[result, e, esize], -) = SatQ(element - count, esize, unsigned);

        Z[dn] = result;
