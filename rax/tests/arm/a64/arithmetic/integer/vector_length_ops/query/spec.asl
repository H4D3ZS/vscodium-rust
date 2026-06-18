__instruction CNTB_R.S__
    __encoding CNTB_R.S__
        __instruction_set A64
        __field imm4 16 +: 4
        __field pattern 5 +: 5
        __field Rd 0 +: 5
        __opcode '00000100 0010xxxx 111000xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8;
            integer d = UInt(Rd);
            bits(5) pat = pattern;
            integer imm = UInt(imm4) + 1;

    __encoding CNTD_R.S__
        __instruction_set A64
        __field imm4 16 +: 4
        __field pattern 5 +: 5
        __field Rd 0 +: 5
        __opcode '00000100 1110xxxx 111000xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 64;
            integer d = UInt(Rd);
            bits(5) pat = pattern;
            integer imm = UInt(imm4) + 1;

    __encoding CNTH_R.S__
        __instruction_set A64
        __field imm4 16 +: 4
        __field pattern 5 +: 5
        __field Rd 0 +: 5
        __opcode '00000100 0110xxxx 111000xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 16;
            integer d = UInt(Rd);
            bits(5) pat = pattern;
            integer imm = UInt(imm4) + 1;

    __encoding CNTW_R.S__
        __instruction_set A64
        __field imm4 16 +: 4
        __field pattern 5 +: 5
        __field Rd 0 +: 5
        __opcode '00000100 1010xxxx 111000xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 32;
            integer d = UInt(Rd);
            bits(5) pat = pattern;
            integer imm = UInt(imm4) + 1;

    __execute
        CheckSVEEnabled();
        integer count = DecodePredCount(pat, esize);

        X[d] = (count * imm)<63:0>;

__instruction CNTP_R.P.P__
    __encoding CNTP_R.P.P__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 4
        __field Pn 5 +: 4
        __field Rd 0 +: 5
        __opcode '00100101 xx100000 10xxxx0x xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer n = UInt(Pn);
            integer d = UInt(Rd);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(PL) operand = P[n];
        bits(64) sum = Zeros();

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' && ElemP[operand, e, esize] == '1' then
                sum = sum + 1;
        X[d] = sum;
