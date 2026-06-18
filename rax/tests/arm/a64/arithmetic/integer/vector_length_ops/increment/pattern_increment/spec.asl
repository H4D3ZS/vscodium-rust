__instruction INCB_R.RS__
    __encoding INCB_R.RS__
        __instruction_set A64
        __field imm4 16 +: 4
        __field pattern 5 +: 5
        __field Rdn 0 +: 5
        __opcode '00000100 0011xxxx 111000xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8;
            integer dn = UInt(Rdn);
            bits(5) pat = pattern;
            integer imm = UInt(imm4) + 1;

    __encoding INCD_R.RS__
        __instruction_set A64
        __field imm4 16 +: 4
        __field pattern 5 +: 5
        __field Rdn 0 +: 5
        __opcode '00000100 1111xxxx 111000xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 64;
            integer dn = UInt(Rdn);
            bits(5) pat = pattern;
            integer imm = UInt(imm4) + 1;

    __encoding INCH_R.RS__
        __instruction_set A64
        __field imm4 16 +: 4
        __field pattern 5 +: 5
        __field Rdn 0 +: 5
        __opcode '00000100 0111xxxx 111000xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 16;
            integer dn = UInt(Rdn);
            bits(5) pat = pattern;
            integer imm = UInt(imm4) + 1;

    __encoding INCW_R.RS__
        __instruction_set A64
        __field imm4 16 +: 4
        __field pattern 5 +: 5
        __field Rdn 0 +: 5
        __opcode '00000100 1011xxxx 111000xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 32;
            integer dn = UInt(Rdn);
            bits(5) pat = pattern;
            integer imm = UInt(imm4) + 1;

    __execute
        CheckSVEEnabled();
        integer count = DecodePredCount(pat, esize);
        bits(64) operand1 = X[dn];

        X[dn] = operand1 + (count * imm);
