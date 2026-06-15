__instruction DUP_Z.I__
    __encoding DUP_Z.I__
        __instruction_set A64
        __field size 22 +: 2
        __field sh 13 +: 1
        __field imm8 5 +: 8
        __field Zd 0 +: 5
        __opcode '00100101 xx111000 11xxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size:sh == '001' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer d = UInt(Zd);
            integer imm = SInt(imm8);
            if sh == '1' then imm = imm << 8;

    __execute
        CheckSVEEnabled();
        bits(VL) result = Replicate(imm<esize-1:0>);
        Z[d] = result;

__instruction DUPM_Z.I__
    __encoding DUPM_Z.I__
        __instruction_set A64
        __field imm13 5 +: 13
        __field Zd 0 +: 5
        __opcode '00000101 110000xx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 64;
            integer d = UInt(Zd);
            bits(esize) imm;
            (imm, -) = DecodeBitMasks(imm13<12>, imm13<5:0>, imm13<11:6>, TRUE);

    __execute
        CheckSVEEnabled();
        bits(VL) result = Replicate(imm);
        Z[d] = result;
