__instruction ADD_Z.ZI__
    __encoding ADD_Z.ZI__
        __instruction_set A64
        __field size 22 +: 2
        __field sh 13 +: 1
        __field imm8 5 +: 8
        __field Zdn 0 +: 5
        __opcode '00100101 xx100000 11xxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size:sh == '001' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer dn = UInt(Zdn);
            integer imm = UInt(imm8);
            if sh == '1' then imm = imm << 8;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) operand1 = Z[dn];
        bits(VL) result;

        for e = 0 to elements-1
            bits(esize) element1 = Elem[operand1, e, esize];
            Elem[result, e, esize] = element1 + imm;

        Z[dn] = result;

__instruction SUB_Z.ZI__
    __encoding SUB_Z.ZI__
        __instruction_set A64
        __field size 22 +: 2
        __field sh 13 +: 1
        __field imm8 5 +: 8
        __field Zdn 0 +: 5
        __opcode '00100101 xx100001 11xxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size:sh == '001' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer dn = UInt(Zdn);
            integer imm = UInt(imm8);
            if sh == '1' then imm = imm << 8;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) operand1 = Z[dn];
        bits(VL) result;

        for e = 0 to elements-1
            bits(esize) element1 = Elem[operand1, e, esize];
            Elem[result, e, esize] = element1 - imm;

        Z[dn] = result;

__instruction SUBR_Z.ZI__
    __encoding SUBR_Z.ZI__
        __instruction_set A64
        __field size 22 +: 2
        __field sh 13 +: 1
        __field imm8 5 +: 8
        __field Zdn 0 +: 5
        __opcode '00100101 xx100011 11xxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size:sh == '001' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer dn = UInt(Zdn);
            integer imm = UInt(imm8);
            if sh == '1' then imm = imm << 8;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) operand1 = Z[dn];
        bits(VL) result;

        for e = 0 to elements-1
            integer element1 = UInt(Elem[operand1, e, esize]);
            Elem[result, e, esize] = (imm - element1)<esize-1:0>;

        Z[dn] = result;
