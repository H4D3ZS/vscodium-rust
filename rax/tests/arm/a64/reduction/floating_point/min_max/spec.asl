__instruction FMINV_V.P.Z__
    __encoding FMINV_V.P.Z__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Vd 0 +: 5
        __opcode '01100101 xx000111 001xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size == '00' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Vd);

    __execute
        CheckSVEEnabled();
        bits(PL) mask = P[g];
        bits(VL) operand = Z[n];
        bits(esize) identity = FPInfinity('0');

        V[d] = ReducePredicated(ReduceOp_FMIN, operand, mask, identity);

__instruction FMAXV_V.P.Z__
    __encoding FMAXV_V.P.Z__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Vd 0 +: 5
        __opcode '01100101 xx000110 001xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size == '00' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Vd);

    __execute
        CheckSVEEnabled();
        bits(PL) mask = P[g];
        bits(VL) operand = Z[n];
        bits(esize) identity = FPInfinity('1');

        V[d] = ReducePredicated(ReduceOp_FMAX, operand, mask, identity);

__instruction FMINNMV_V.P.Z__
    __encoding FMINNMV_V.P.Z__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Vd 0 +: 5
        __opcode '01100101 xx000101 001xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size == '00' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Vd);

    __execute
        CheckSVEEnabled();
        bits(PL) mask = P[g];
        bits(VL) operand = Z[n];
        bits(esize) identity = FPDefaultNaN();

        V[d] = ReducePredicated(ReduceOp_FMINNUM, operand, mask, identity);

__instruction FMAXNMV_V.P.Z__
    __encoding FMAXNMV_V.P.Z__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Vd 0 +: 5
        __opcode '01100101 xx000100 001xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size == '00' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Vd);

    __execute
        CheckSVEEnabled();
        bits(PL) mask = P[g];
        bits(VL) operand = Z[n];
        bits(esize) identity = FPDefaultNaN();

        V[d] = ReducePredicated(ReduceOp_FMAXNUM, operand, mask, identity);
