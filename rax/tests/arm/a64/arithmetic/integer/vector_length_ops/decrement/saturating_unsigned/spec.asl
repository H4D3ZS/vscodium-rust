__instruction UQDECB_R.RS_UW
    __encoding UQDECB_R.RS_UW
        __instruction_set A64
        __field imm4 16 +: 4
        __field pattern 5 +: 5
        __field Rdn 0 +: 5
        __opcode '00000100 0010xxxx 111111xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8;
            integer dn = UInt(Rdn);
            bits(5) pat = pattern;
            integer imm = UInt(imm4) + 1;
            boolean unsigned = TRUE;
            integer ssize = 32;

    __encoding UQDECB_R.RS_X
        __instruction_set A64
        __field imm4 16 +: 4
        __field pattern 5 +: 5
        __field Rdn 0 +: 5
        __opcode '00000100 0011xxxx 111111xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8;
            integer dn = UInt(Rdn);
            bits(5) pat = pattern;
            integer imm = UInt(imm4) + 1;
            boolean unsigned = TRUE;
            integer ssize = 64;

    __execute
        CheckSVEEnabled();
        integer count = DecodePredCount(pat, esize);
        bits(ssize) operand1 = X[dn];
        bits(ssize) result;

        integer element1 = Int(operand1, unsigned);
        (result, -) = SatQ(element1 - (count * imm), ssize, unsigned);
        X[dn] = Extend(result, 64, unsigned);

__instruction UQDECH_R.RS_UW
    __encoding UQDECH_R.RS_UW
        __instruction_set A64
        __field imm4 16 +: 4
        __field pattern 5 +: 5
        __field Rdn 0 +: 5
        __opcode '00000100 0110xxxx 111111xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 16;
            integer dn = UInt(Rdn);
            bits(5) pat = pattern;
            integer imm = UInt(imm4) + 1;
            boolean unsigned = TRUE;
            integer ssize = 32;

    __encoding UQDECH_R.RS_X
        __instruction_set A64
        __field imm4 16 +: 4
        __field pattern 5 +: 5
        __field Rdn 0 +: 5
        __opcode '00000100 0111xxxx 111111xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 16;
            integer dn = UInt(Rdn);
            bits(5) pat = pattern;
            integer imm = UInt(imm4) + 1;
            boolean unsigned = TRUE;
            integer ssize = 64;

    __execute
        CheckSVEEnabled();
        integer count = DecodePredCount(pat, esize);
        bits(ssize) operand1 = X[dn];
        bits(ssize) result;

        integer element1 = Int(operand1, unsigned);
        (result, -) = SatQ(element1 - (count * imm), ssize, unsigned);
        X[dn] = Extend(result, 64, unsigned);

__instruction UQDECW_R.RS_UW
    __encoding UQDECW_R.RS_UW
        __instruction_set A64
        __field imm4 16 +: 4
        __field pattern 5 +: 5
        __field Rdn 0 +: 5
        __opcode '00000100 1010xxxx 111111xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 32;
            integer dn = UInt(Rdn);
            bits(5) pat = pattern;
            integer imm = UInt(imm4) + 1;
            boolean unsigned = TRUE;
            integer ssize = 32;

    __encoding UQDECW_R.RS_X
        __instruction_set A64
        __field imm4 16 +: 4
        __field pattern 5 +: 5
        __field Rdn 0 +: 5
        __opcode '00000100 1011xxxx 111111xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 32;
            integer dn = UInt(Rdn);
            bits(5) pat = pattern;
            integer imm = UInt(imm4) + 1;
            boolean unsigned = TRUE;
            integer ssize = 64;

    __execute
        CheckSVEEnabled();
        integer count = DecodePredCount(pat, esize);
        bits(ssize) operand1 = X[dn];
        bits(ssize) result;

        integer element1 = Int(operand1, unsigned);
        (result, -) = SatQ(element1 - (count * imm), ssize, unsigned);
        X[dn] = Extend(result, 64, unsigned);

__instruction UQDECD_R.RS_UW
    __encoding UQDECD_R.RS_UW
        __instruction_set A64
        __field imm4 16 +: 4
        __field pattern 5 +: 5
        __field Rdn 0 +: 5
        __opcode '00000100 1110xxxx 111111xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 64;
            integer dn = UInt(Rdn);
            bits(5) pat = pattern;
            integer imm = UInt(imm4) + 1;
            boolean unsigned = TRUE;
            integer ssize = 32;

    __encoding UQDECD_R.RS_X
        __instruction_set A64
        __field imm4 16 +: 4
        __field pattern 5 +: 5
        __field Rdn 0 +: 5
        __opcode '00000100 1111xxxx 111111xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 64;
            integer dn = UInt(Rdn);
            bits(5) pat = pattern;
            integer imm = UInt(imm4) + 1;
            boolean unsigned = TRUE;
            integer ssize = 64;

    __execute
        CheckSVEEnabled();
        integer count = DecodePredCount(pat, esize);
        bits(ssize) operand1 = X[dn];
        bits(ssize) result;

        integer element1 = Int(operand1, unsigned);
        (result, -) = SatQ(element1 - (count * imm), ssize, unsigned);
        X[dn] = Extend(result, 64, unsigned);
