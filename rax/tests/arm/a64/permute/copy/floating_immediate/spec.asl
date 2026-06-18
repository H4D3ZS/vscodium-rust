__instruction FCPY_Z.P.I__
    __encoding FCPY_Z.P.I__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 16 +: 4
        __field imm8 5 +: 8
        __field Zd 0 +: 5
        __opcode '00000101 xx01xxxx 110xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size == '00' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer d = UInt(Zd);
            bits(esize) imm = VFPExpandImm(imm8);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(VL) result = Z[d];

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                Elem[result, e, esize] = imm;

        Z[d] = result;

__instruction FDUP_Z.I__
    __encoding FDUP_Z.I__
        __instruction_set A64
        __field size 22 +: 2
        __field imm8 5 +: 8
        __field Zd 0 +: 5
        __opcode '00100101 xx111001 110xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size == '00' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer d = UInt(Zd);
            bits(esize) imm = VFPExpandImm(imm8);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) result;

        for e = 0 to elements-1
            Elem[result, e, esize] = imm;

        Z[d] = result;
