__instruction SQSUB_Z.ZZ__
    __encoding SQSUB_Z.ZZ__
        __instruction_set A64
        __field size 22 +: 2
        __field Zm 16 +: 5
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '00000100 xx1xxxxx 000110xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer n = UInt(Zn);
            integer m = UInt(Zm);
            integer d = UInt(Zd);
            boolean unsigned = FALSE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) operand1 = Z[n];
        bits(VL) operand2 = Z[m];
        bits(VL) result;

        for e = 0 to elements-1
            integer element1 = Int(Elem[operand1, e, esize], unsigned);
            integer element2 = Int(Elem[operand2, e, esize], unsigned);
            (Elem[result, e, esize], -) = SatQ(element1 - element2, esize, unsigned);

        Z[d] = result;

__instruction SQSUB_Z.ZI__
    __encoding SQSUB_Z.ZI__
        __instruction_set A64
        __field size 22 +: 2
        __field sh 13 +: 1
        __field imm8 5 +: 8
        __field Zdn 0 +: 5
        __opcode '00100101 xx100110 11xxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size:sh == '001' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer dn = UInt(Zdn);
            integer imm = UInt(imm8);
            if sh == '1' then imm = imm << 8;
            boolean unsigned = FALSE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) operand1 = Z[dn];
        bits(VL) result;

        for e = 0 to elements-1
            integer element1 = Int(Elem[operand1, e, esize], unsigned);
            (Elem[result, e, esize], -) = SatQ(element1 - imm, esize, unsigned);

        Z[dn] = result;
