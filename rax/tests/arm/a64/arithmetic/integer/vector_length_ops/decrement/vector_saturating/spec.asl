__instruction UQDECH_Z.ZS__
    __encoding UQDECH_Z.ZS__
        __instruction_set A64
        __field imm4 16 +: 4
        __field pattern 5 +: 5
        __field Zdn 0 +: 5
        __opcode '00000100 0110xxxx 110011xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 16;
            integer dn = UInt(Zdn);
            bits(5) pat = pattern;
            integer imm = UInt(imm4) + 1;
            boolean unsigned = TRUE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        integer count = DecodePredCount(pat, esize);
        bits(VL) operand1 = Z[dn];
        bits(VL) result;

        for e = 0 to elements-1
            integer element1 = Int(Elem[operand1, e, esize], unsigned);
            (Elem[result, e, esize], -) = SatQ(element1 - (count * imm), esize, unsigned);

        Z[dn] = result;

__instruction UQDECW_Z.ZS__
    __encoding UQDECW_Z.ZS__
        __instruction_set A64
        __field imm4 16 +: 4
        __field pattern 5 +: 5
        __field Zdn 0 +: 5
        __opcode '00000100 1010xxxx 110011xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 32;
            integer dn = UInt(Zdn);
            bits(5) pat = pattern;
            integer imm = UInt(imm4) + 1;
            boolean unsigned = TRUE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        integer count = DecodePredCount(pat, esize);
        bits(VL) operand1 = Z[dn];
        bits(VL) result;

        for e = 0 to elements-1
            integer element1 = Int(Elem[operand1, e, esize], unsigned);
            (Elem[result, e, esize], -) = SatQ(element1 - (count * imm), esize, unsigned);

        Z[dn] = result;

__instruction UQDECD_Z.ZS__
    __encoding UQDECD_Z.ZS__
        __instruction_set A64
        __field imm4 16 +: 4
        __field pattern 5 +: 5
        __field Zdn 0 +: 5
        __opcode '00000100 1110xxxx 110011xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 64;
            integer dn = UInt(Zdn);
            bits(5) pat = pattern;
            integer imm = UInt(imm4) + 1;
            boolean unsigned = TRUE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        integer count = DecodePredCount(pat, esize);
        bits(VL) operand1 = Z[dn];
        bits(VL) result;

        for e = 0 to elements-1
            integer element1 = Int(Elem[operand1, e, esize], unsigned);
            (Elem[result, e, esize], -) = SatQ(element1 - (count * imm), esize, unsigned);

        Z[dn] = result;

__instruction SQDECD_Z.ZS__
    __encoding SQDECD_Z.ZS__
        __instruction_set A64
        __field imm4 16 +: 4
        __field pattern 5 +: 5
        __field Zdn 0 +: 5
        __opcode '00000100 1110xxxx 110010xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 64;
            integer dn = UInt(Zdn);
            bits(5) pat = pattern;
            integer imm = UInt(imm4) + 1;
            boolean unsigned = FALSE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        integer count = DecodePredCount(pat, esize);
        bits(VL) operand1 = Z[dn];
        bits(VL) result;

        for e = 0 to elements-1
            integer element1 = Int(Elem[operand1, e, esize], unsigned);
            (Elem[result, e, esize], -) = SatQ(element1 - (count * imm), esize, unsigned);

        Z[dn] = result;
