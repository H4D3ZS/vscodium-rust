__instruction SCVTF_Z.P.Z_H2FP16
    __encoding SCVTF_Z.P.Z_H2FP16
        __instruction_set A64
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '01100101 01010010 101xxxxx xxxxxxxx'
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
            FPRounding rounding = FPRoundingMode(FPCR);

    __encoding SCVTF_Z.P.Z_W2FP16
        __instruction_set A64
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '01100101 01010100 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 32;
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Zd);
            integer s_esize = 32;
            integer d_esize = 16;
            boolean unsigned = FALSE;
            FPRounding rounding = FPRoundingMode(FPCR);

    __encoding SCVTF_Z.P.Z_W2S
        __instruction_set A64
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '01100101 10010100 101xxxxx xxxxxxxx'
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
            FPRounding rounding = FPRoundingMode(FPCR);

    __encoding SCVTF_Z.P.Z_W2D
        __instruction_set A64
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '01100101 11010000 101xxxxx xxxxxxxx'
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
            FPRounding rounding = FPRoundingMode(FPCR);

    __encoding SCVTF_Z.P.Z_X2FP16
        __instruction_set A64
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '01100101 01010110 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 64;
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Zd);
            integer s_esize = 64;
            integer d_esize = 16;
            boolean unsigned = FALSE;
            FPRounding rounding = FPRoundingMode(FPCR);

    __encoding SCVTF_Z.P.Z_X2S
        __instruction_set A64
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '01100101 11010100 101xxxxx xxxxxxxx'
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
            FPRounding rounding = FPRoundingMode(FPCR);

    __encoding SCVTF_Z.P.Z_X2D
        __instruction_set A64
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '01100101 11010110 101xxxxx xxxxxxxx'
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
            FPRounding rounding = FPRoundingMode(FPCR);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(VL) operand  = Z[n];
        bits(VL) result = Z[d];

        for e = 0 to elements-1
            bits(esize) element = Elem[operand, e, esize];
            if ElemP[mask, e, esize] == '1' then
                bits(d_esize) fpval = FixedToFP(element<s_esize-1:0>, 0, unsigned, FPCR, rounding);
                Elem[result, e, esize] = ZeroExtend(fpval);

        Z[d] = result;

__instruction UCVTF_Z.P.Z_H2FP16
    __encoding UCVTF_Z.P.Z_H2FP16
        __instruction_set A64
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '01100101 01010011 101xxxxx xxxxxxxx'
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
            FPRounding rounding = FPRoundingMode(FPCR);

    __encoding UCVTF_Z.P.Z_W2FP16
        __instruction_set A64
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '01100101 01010101 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 32;
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Zd);
            integer s_esize = 32;
            integer d_esize = 16;
            boolean unsigned = TRUE;
            FPRounding rounding = FPRoundingMode(FPCR);

    __encoding UCVTF_Z.P.Z_W2S
        __instruction_set A64
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '01100101 10010101 101xxxxx xxxxxxxx'
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
            FPRounding rounding = FPRoundingMode(FPCR);

    __encoding UCVTF_Z.P.Z_W2D
        __instruction_set A64
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '01100101 11010001 101xxxxx xxxxxxxx'
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
            FPRounding rounding = FPRoundingMode(FPCR);

    __encoding UCVTF_Z.P.Z_X2FP16
        __instruction_set A64
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '01100101 01010111 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 64;
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Zd);
            integer s_esize = 64;
            integer d_esize = 16;
            boolean unsigned = TRUE;
            FPRounding rounding = FPRoundingMode(FPCR);

    __encoding UCVTF_Z.P.Z_X2S
        __instruction_set A64
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '01100101 11010101 101xxxxx xxxxxxxx'
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
            FPRounding rounding = FPRoundingMode(FPCR);

    __encoding UCVTF_Z.P.Z_X2D
        __instruction_set A64
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '01100101 11010111 101xxxxx xxxxxxxx'
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
            FPRounding rounding = FPRoundingMode(FPCR);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(VL) operand  = Z[n];
        bits(VL) result = Z[d];

        for e = 0 to elements-1
            bits(esize) element = Elem[operand, e, esize];
            if ElemP[mask, e, esize] == '1' then
                bits(d_esize) fpval = FixedToFP(element<s_esize-1:0>, 0, unsigned, FPCR, rounding);
                Elem[result, e, esize] = ZeroExtend(fpval);

        Z[d] = result;
