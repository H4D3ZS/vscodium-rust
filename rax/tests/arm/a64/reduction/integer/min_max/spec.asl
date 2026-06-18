__instruction SMINV_R.P.Z__
    __encoding SMINV_R.P.Z__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Vd 0 +: 5
        __opcode '00000100 xx001010 001xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Vd);
            boolean unsigned = FALSE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(VL) operand = Z[n];
        integer minimum = if unsigned then (2^esize - 1) else (2^(esize-1) - 1);

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                integer element = Int(Elem[operand, e, esize], unsigned);
                minimum = Min(minimum, element);

        V[d] = minimum<esize-1:0>;

__instruction SMAXV_R.P.Z__
    __encoding SMAXV_R.P.Z__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Vd 0 +: 5
        __opcode '00000100 xx001000 001xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Vd);
            boolean unsigned = FALSE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(VL) operand = Z[n];
        integer maximum = if unsigned then 0 else -(2^(esize-1));

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                integer element = Int(Elem[operand, e, esize], unsigned);
                maximum = Max(maximum, element);

        V[d] = maximum<esize-1:0>;

__instruction UMINV_R.P.Z__
    __encoding UMINV_R.P.Z__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Vd 0 +: 5
        __opcode '00000100 xx001011 001xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Vd);
            boolean unsigned = TRUE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(VL) operand = Z[n];
        integer minimum = if unsigned then (2^esize - 1) else (2^(esize-1) - 1);

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                integer element = Int(Elem[operand, e, esize], unsigned);
                minimum = Min(minimum, element);

        V[d] = minimum<esize-1:0>;

__instruction UMAXV_R.P.Z__
    __encoding UMAXV_R.P.Z__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Vd 0 +: 5
        __opcode '00000100 xx001001 001xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Vd);
            boolean unsigned = TRUE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(VL) operand = Z[n];
        integer maximum = if unsigned then 0 else -(2^(esize-1));

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                integer element = Int(Elem[operand, e, esize], unsigned);
                maximum = Max(maximum, element);

        V[d] = maximum<esize-1:0>;
