__instruction UZP1_Z.ZZ__
    __encoding UZP1_Z.ZZ__
        __instruction_set A64
        __field size 22 +: 2
        __field Zm 16 +: 5
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '00000101 xx1xxxxx 011010xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer n = UInt(Zn);
            integer m = UInt(Zm);
            integer d = UInt(Zd);
            integer part = 0;

    __encoding UZP2_Z.ZZ__
        __instruction_set A64
        __field size 22 +: 2
        __field Zm 16 +: 5
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '00000101 xx1xxxxx 011011xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer n = UInt(Zn);
            integer m = UInt(Zm);
            integer d = UInt(Zd);
            integer part = 1;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) operand1 = Z[n];
        bits(VL) operand2 = Z[m];
        bits(VL) result;

        bits(VL*2) zipped = operand2:operand1;
        for e = 0 to elements-1
            Elem[result, e, esize] = Elem[zipped, 2*e+part, esize];

        Z[d] = result;

__instruction UZP1_P.PP__
    __encoding UZP1_P.PP__
        __instruction_set A64
        __field size 22 +: 2
        __field Pm 16 +: 4
        __field Pn 5 +: 4
        __field Pd 0 +: 4
        __opcode '00000101 xx10xxxx 0100100x xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer n = UInt(Pn);
            integer m = UInt(Pm);
            integer d = UInt(Pd);
            integer part = 0;

    __encoding UZP2_P.PP__
        __instruction_set A64
        __field size 22 +: 2
        __field Pm 16 +: 4
        __field Pn 5 +: 4
        __field Pd 0 +: 4
        __opcode '00000101 xx10xxxx 0100110x xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer n = UInt(Pn);
            integer m = UInt(Pm);
            integer d = UInt(Pd);
            integer part = 1;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) operand1 = P[n];
        bits(PL) operand2 = P[m];
        bits(PL) result;

        bits(PL*2) zipped = operand2:operand1;
        for e = 0 to elements-1
            Elem[result, e, esize DIV 8] = Elem[zipped, 2*e+part, esize DIV 8];

        P[d] = result;
