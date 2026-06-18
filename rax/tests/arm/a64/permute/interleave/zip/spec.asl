__instruction ZIP2_Z.ZZ__
    __encoding ZIP2_Z.ZZ__
        __instruction_set A64
        __field size 22 +: 2
        __field Zm 16 +: 5
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '00000101 xx1xxxxx 011001xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer n = UInt(Zn);
            integer m = UInt(Zm);
            integer d = UInt(Zd);
            integer part = 1;

    __encoding ZIP1_Z.ZZ__
        __instruction_set A64
        __field size 22 +: 2
        __field Zm 16 +: 5
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '00000101 xx1xxxxx 011000xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer n = UInt(Zn);
            integer m = UInt(Zm);
            integer d = UInt(Zd);
            integer part = 0;

    __execute
        CheckSVEEnabled();
        integer pairs = VL DIV (esize * 2);
        bits(VL) operand1 = Z[n];
        bits(VL) operand2 = Z[m];
        bits(VL) result;

        integer base = part * pairs;
        for p = 0 to pairs-1
            Elem[result, 2*p+0, esize] = Elem[operand1, base+p, esize];
            Elem[result, 2*p+1, esize] = Elem[operand2, base+p, esize];

        Z[d] = result;

__instruction ZIP2_P.PP__
    __encoding ZIP2_P.PP__
        __instruction_set A64
        __field size 22 +: 2
        __field Pm 16 +: 4
        __field Pn 5 +: 4
        __field Pd 0 +: 4
        __opcode '00000101 xx10xxxx 0100010x xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer n = UInt(Pn);
            integer m = UInt(Pm);
            integer d = UInt(Pd);
            integer part = 1;

    __encoding ZIP1_P.PP__
        __instruction_set A64
        __field size 22 +: 2
        __field Pm 16 +: 4
        __field Pn 5 +: 4
        __field Pd 0 +: 4
        __opcode '00000101 xx10xxxx 0100000x xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer n = UInt(Pn);
            integer m = UInt(Pm);
            integer d = UInt(Pd);
            integer part = 0;

    __execute
        CheckSVEEnabled();
        integer pairs = VL DIV (esize * 2);
        bits(PL) operand1 = P[n];
        bits(PL) operand2 = P[m];
        bits(PL) result;

        integer base = part * pairs;
        for p = 0 to pairs-1
            Elem[result, 2*p+0, esize DIV 8] = Elem[operand1, base+p, esize DIV 8];
            Elem[result, 2*p+1, esize DIV 8] = Elem[operand2, base+p, esize DIV 8];

        P[d] = result;
