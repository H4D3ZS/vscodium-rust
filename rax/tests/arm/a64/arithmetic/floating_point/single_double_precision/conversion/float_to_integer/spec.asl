__instruction FCVTZS_Z.P.Z_FP162H
    __encoding FCVTZS_Z.P.Z_FP162H
        __instruction_set A64
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '01100101 01011010 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 16;
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Zd);
            integer s_esize = 16;
            integer d_esize = 16;
            boolean unsigned = FALSE;
            FPRounding rounding = FPRounding_ZERO;

    __encoding FCVTZS_Z.P.Z_FP162W
        __instruction_set A64
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '01100101 01011100 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 32;
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Zd);
            integer s_esize = 16;
            integer d_esize = 32;
            boolean unsigned = FALSE;
            FPRounding rounding = FPRounding_ZERO;

    __encoding FCVTZS_Z.P.Z_FP162X
        __instruction_set A64
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '01100101 01011110 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 64;
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Zd);
            integer s_esize = 16;
            integer d_esize = 64;
            boolean unsigned = FALSE;
            FPRounding rounding = FPRounding_ZERO;

    __encoding FCVTZS_Z.P.Z_S2W
        __instruction_set A64
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '01100101 10011100 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 32;
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Zd);
            integer s_esize = 32;
            integer d_esize = 32;
            boolean unsigned = FALSE;
            FPRounding rounding = FPRounding_ZERO;

    __encoding FCVTZS_Z.P.Z_S2X
        __instruction_set A64
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '01100101 11011100 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 64;
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Zd);
            integer s_esize = 32;
            integer d_esize = 64;
            boolean unsigned = FALSE;
            FPRounding rounding = FPRounding_ZERO;

    __encoding FCVTZS_Z.P.Z_D2W
        __instruction_set A64
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '01100101 11011000 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 64;
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Zd);
            integer s_esize = 64;
            integer d_esize = 32;
            boolean unsigned = FALSE;
            FPRounding rounding = FPRounding_ZERO;

    __encoding FCVTZS_Z.P.Z_D2X
        __instruction_set A64
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '01100101 11011110 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 64;
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Zd);
            integer s_esize = 64;
            integer d_esize = 64;
            boolean unsigned = FALSE;
            FPRounding rounding = FPRounding_ZERO;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(VL) operand  = Z[n];
        bits(VL) result = Z[d];

        for e = 0 to elements-1
            bits(esize) element = Elem[operand, e, esize];
            if ElemP[mask, e, esize] == '1' then
                bits(d_esize) res = FPToFixed(element<s_esize-1:0>, 0, unsigned, FPCR, rounding);
                Elem[result, e, esize] = Extend(res, unsigned);

        Z[d] = result;

__instruction FCVTZU_Z.P.Z_FP162H
    __encoding FCVTZU_Z.P.Z_FP162H
        __instruction_set A64
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '01100101 01011011 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 16;
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Zd);
            integer s_esize = 16;
            integer d_esize = 16;
            boolean unsigned = TRUE;
            FPRounding rounding = FPRounding_ZERO;

    __encoding FCVTZU_Z.P.Z_FP162W
        __instruction_set A64
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '01100101 01011101 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 32;
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Zd);
            integer s_esize = 16;
            integer d_esize = 32;
            boolean unsigned = TRUE;
            FPRounding rounding = FPRounding_ZERO;

    __encoding FCVTZU_Z.P.Z_FP162X
        __instruction_set A64
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '01100101 01011111 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 64;
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Zd);
            integer s_esize = 16;
            integer d_esize = 64;
            boolean unsigned = TRUE;
            FPRounding rounding = FPRounding_ZERO;

    __encoding FCVTZU_Z.P.Z_S2W
        __instruction_set A64
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '01100101 10011101 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 32;
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Zd);
            integer s_esize = 32;
            integer d_esize = 32;
            boolean unsigned = TRUE;
            FPRounding rounding = FPRounding_ZERO;

    __encoding FCVTZU_Z.P.Z_S2X
        __instruction_set A64
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '01100101 11011101 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 64;
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Zd);
            integer s_esize = 32;
            integer d_esize = 64;
            boolean unsigned = TRUE;
            FPRounding rounding = FPRounding_ZERO;

    __encoding FCVTZU_Z.P.Z_D2W
        __instruction_set A64
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '01100101 11011001 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 64;
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Zd);
            integer s_esize = 64;
            integer d_esize = 32;
            boolean unsigned = TRUE;
            FPRounding rounding = FPRounding_ZERO;

    __encoding FCVTZU_Z.P.Z_D2X
        __instruction_set A64
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '01100101 11011111 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 64;
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Zd);
            integer s_esize = 64;
            integer d_esize = 64;
            boolean unsigned = TRUE;
            FPRounding rounding = FPRounding_ZERO;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(VL) operand  = Z[n];
        bits(VL) result = Z[d];

        for e = 0 to elements-1
            bits(esize) element = Elem[operand, e, esize];
            if ElemP[mask, e, esize] == '1' then
                bits(d_esize) res = FPToFixed(element<s_esize-1:0>, 0, unsigned, FPCR, rounding);
                Elem[result, e, esize] = Extend(res, unsigned);

        Z[d] = result;
