__instruction DECB_R.RS__
    __encoding DECB_R.RS__
        __instruction_set A64
        __field imm4 16 +: 4
        __field pattern 5 +: 5
        __field Rdn 0 +: 5
        __opcode '00000100 0011xxxx 111001xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8;
            integer dn = UInt(Rdn);
            bits(5) pat = pattern;
            integer imm = UInt(imm4) + 1;

    __encoding DECD_R.RS__
        __instruction_set A64
        __field imm4 16 +: 4
        __field pattern 5 +: 5
        __field Rdn 0 +: 5
        __opcode '00000100 1111xxxx 111001xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 64;
            integer dn = UInt(Rdn);
            bits(5) pat = pattern;
            integer imm = UInt(imm4) + 1;

    __encoding DECH_R.RS__
        __instruction_set A64
        __field imm4 16 +: 4
        __field pattern 5 +: 5
        __field Rdn 0 +: 5
        __opcode '00000100 0111xxxx 111001xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 16;
            integer dn = UInt(Rdn);
            bits(5) pat = pattern;
            integer imm = UInt(imm4) + 1;

    __encoding DECW_R.RS__
        __instruction_set A64
        __field imm4 16 +: 4
        __field pattern 5 +: 5
        __field Rdn 0 +: 5
        __opcode '00000100 1011xxxx 111001xx xxxxxxxx'
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

        X[dn] = operand1 - (count * imm);

__instruction DECD_Z.ZS__
    __encoding DECD_Z.ZS__
        __instruction_set A64
        __field imm4 16 +: 4
        __field pattern 5 +: 5
        __field Zdn 0 +: 5
        __opcode '00000100 1111xxxx 110001xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 64;
            integer dn = UInt(Zdn);
            bits(5) pat = pattern;
            integer imm = UInt(imm4) + 1;

    __encoding DECH_Z.ZS__
        __instruction_set A64
        __field imm4 16 +: 4
        __field pattern 5 +: 5
        __field Zdn 0 +: 5
        __opcode '00000100 0111xxxx 110001xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 16;
            integer dn = UInt(Zdn);
            bits(5) pat = pattern;
            integer imm = UInt(imm4) + 1;

    __encoding DECW_Z.ZS__
        __instruction_set A64
        __field imm4 16 +: 4
        __field pattern 5 +: 5
        __field Zdn 0 +: 5
        __opcode '00000100 1011xxxx 110001xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 32;
            integer dn = UInt(Zdn);
            bits(5) pat = pattern;
            integer imm = UInt(imm4) + 1;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        integer count = DecodePredCount(pat, esize);
        bits(VL) operand1 = Z[dn];
        bits(VL) result;

        for e = 0 to elements-1
            Elem[result, e, esize] = Elem[operand1, e, esize] - (count * imm);

        Z[dn] = result;
