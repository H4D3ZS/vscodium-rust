__instruction AND_Z.ZI__
    __encoding AND_Z.ZI__
        __instruction_set A64
        __field imm13 5 +: 13
        __field Zdn 0 +: 5
        __opcode '00000101 100000xx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer dn = UInt(Zdn);
            bits(64) imm;
            (imm, -) = DecodeBitMasks(imm13<12>, imm13<5:0>, imm13<11:6>, TRUE);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV 64;
        bits(VL) operand = Z[dn];
        bits(VL) result;

        for e = 0 to elements-1
            bits(64) element1 = Elem[operand, e, 64];
            Elem[result, e, 64] = element1 AND imm;

        Z[dn] = result;

__instruction ORR_Z.ZI__
    __encoding ORR_Z.ZI__
        __instruction_set A64
        __field imm13 5 +: 13
        __field Zdn 0 +: 5
        __opcode '00000101 000000xx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer dn = UInt(Zdn);
            bits(64) imm;
            (imm, -) = DecodeBitMasks(imm13<12>, imm13<5:0>, imm13<11:6>, TRUE);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV 64;
        bits(VL) operand = Z[dn];
        bits(VL) result;

        for e = 0 to elements-1
            bits(64) element1 = Elem[operand, e, 64];
            Elem[result, e, 64] = element1 OR imm;

        Z[dn] = result;

__instruction EOR_Z.ZI__
    __encoding EOR_Z.ZI__
        __instruction_set A64
        __field imm13 5 +: 13
        __field Zdn 0 +: 5
        __opcode '00000101 010000xx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer dn = UInt(Zdn);
            bits(64) imm;
            (imm, -) = DecodeBitMasks(imm13<12>, imm13<5:0>, imm13<11:6>, TRUE);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV 64;
        bits(VL) operand = Z[dn];
        bits(VL) result;

        for e = 0 to elements-1
            bits(64) element1 = Elem[operand, e, 64];
            Elem[result, e, 64] = element1 EOR imm;

        Z[dn] = result;

__instruction MUL_Z.ZI__
    __encoding MUL_Z.ZI__
        __instruction_set A64
        __field size 22 +: 2
        __field imm8 5 +: 8
        __field Zdn 0 +: 5
        __opcode '00100101 xx110000 110xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer dn = UInt(Zdn);
            integer imm = SInt(imm8);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) operand1 = Z[dn];
        bits(VL) result;

        for e = 0 to elements-1
            integer element1 = SInt(Elem[operand1, e, esize]);
            Elem[result, e, esize] = (element1 * imm)<esize-1:0>;

        Z[dn] = result;
