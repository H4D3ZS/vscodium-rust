__instruction SQINCH_R.RS_SX
    __encoding SQINCH_R.RS_SX
        __instruction_set A64
        __field imm4 16 +: 4
        __field pattern 5 +: 5
        __field Rdn 0 +: 5
        __opcode '00000100 0110xxxx 111100xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 16;
            integer dn = UInt(Rdn);
            bits(5) pat = pattern;
            integer imm = UInt(imm4) + 1;
            boolean unsigned = FALSE;
            integer ssize = 32;

    __encoding SQINCH_R.RS_X
        __instruction_set A64
        __field imm4 16 +: 4
        __field pattern 5 +: 5
        __field Rdn 0 +: 5
        __opcode '00000100 0111xxxx 111100xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 16;
            integer dn = UInt(Rdn);
            bits(5) pat = pattern;
            integer imm = UInt(imm4) + 1;
            boolean unsigned = FALSE;
            integer ssize = 64;

    __execute
        CheckSVEEnabled();
        integer count = DecodePredCount(pat, esize);
        bits(ssize) operand1 = X[dn];
        bits(ssize) result;

        integer element1 = Int(operand1, unsigned);
        (result, -) = SatQ(element1 + (count * imm), ssize, unsigned);
        X[dn] = Extend(result, 64, unsigned);

__instruction SQINCW_R.RS_SX
    __encoding SQINCW_R.RS_SX
        __instruction_set A64
        __field imm4 16 +: 4
        __field pattern 5 +: 5
        __field Rdn 0 +: 5
        __opcode '00000100 1010xxxx 111100xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 32;
            integer dn = UInt(Rdn);
            bits(5) pat = pattern;
            integer imm = UInt(imm4) + 1;
            boolean unsigned = FALSE;
            integer ssize = 32;

    __encoding SQINCW_R.RS_X
        __instruction_set A64
        __field imm4 16 +: 4
        __field pattern 5 +: 5
        __field Rdn 0 +: 5
        __opcode '00000100 1011xxxx 111100xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 32;
            integer dn = UInt(Rdn);
            bits(5) pat = pattern;
            integer imm = UInt(imm4) + 1;
            boolean unsigned = FALSE;
            integer ssize = 64;

    __execute
        CheckSVEEnabled();
        integer count = DecodePredCount(pat, esize);
        bits(ssize) operand1 = X[dn];
        bits(ssize) result;

        integer element1 = Int(operand1, unsigned);
        (result, -) = SatQ(element1 + (count * imm), ssize, unsigned);
        X[dn] = Extend(result, 64, unsigned);

__instruction SQINCD_R.RS_SX
    __encoding SQINCD_R.RS_SX
        __instruction_set A64
        __field imm4 16 +: 4
        __field pattern 5 +: 5
        __field Rdn 0 +: 5
        __opcode '00000100 1110xxxx 111100xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 64;
            integer dn = UInt(Rdn);
            bits(5) pat = pattern;
            integer imm = UInt(imm4) + 1;
            boolean unsigned = FALSE;
            integer ssize = 32;

    __encoding SQINCD_R.RS_X
        __instruction_set A64
        __field imm4 16 +: 4
        __field pattern 5 +: 5
        __field Rdn 0 +: 5
        __opcode '00000100 1111xxxx 111100xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 64;
            integer dn = UInt(Rdn);
            bits(5) pat = pattern;
            integer imm = UInt(imm4) + 1;
            boolean unsigned = FALSE;
            integer ssize = 64;

    __execute
        CheckSVEEnabled();
        integer count = DecodePredCount(pat, esize);
        bits(ssize) operand1 = X[dn];
        bits(ssize) result;

        integer element1 = Int(operand1, unsigned);
        (result, -) = SatQ(element1 + (count * imm), ssize, unsigned);
        X[dn] = Extend(result, 64, unsigned);
