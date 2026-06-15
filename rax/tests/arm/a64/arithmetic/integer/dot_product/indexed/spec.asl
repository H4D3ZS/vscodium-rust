__instruction SDOT_Z.ZZZi_S
    __encoding SDOT_Z.ZZZi_S
        __instruction_set A64
        __field i2 19 +: 2
        __field Zm 16 +: 3
        __field Zn 5 +: 5
        __field Zda 0 +: 5
        __opcode '01000100 101xxxxx 000000xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 32;
            integer index = UInt(i2);
            integer n = UInt(Zn);
            integer m = UInt(Zm);
            integer da = UInt(Zda);

    __encoding SDOT_Z.ZZZi_D
        __instruction_set A64
        __field i1 20 +: 1
        __field Zm 16 +: 4
        __field Zn 5 +: 5
        __field Zda 0 +: 5
        __opcode '01000100 111xxxxx 000000xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 64;
            integer index = UInt(i1);
            integer n = UInt(Zn);
            integer m = UInt(Zm);
            integer da = UInt(Zda);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        integer eltspersegment = 128 DIV esize;
        bits(VL) operand1 = Z[n];
        bits(VL) operand2 = Z[m];
        bits(VL) operand3 = Z[da];
        bits(VL) result;

        for e = 0 to elements-1
            integer segmentbase = e - e MOD eltspersegment;
            integer s = segmentbase + index;
            bits(esize) res = Elem[operand3, e, esize];
            for i = 0 to 3
                integer element1 = SInt(Elem[operand1, 4 * e + i, esize DIV 4]);
                integer element2 = SInt(Elem[operand2, 4 * s + i, esize DIV 4]);
                res = res + element1 * element2;
            Elem[result, e, esize] = res;

        Z[da] = result;

__instruction UDOT_Z.ZZZi_S
    __encoding UDOT_Z.ZZZi_S
        __instruction_set A64
        __field i2 19 +: 2
        __field Zm 16 +: 3
        __field Zn 5 +: 5
        __field Zda 0 +: 5
        __opcode '01000100 101xxxxx 000001xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 32;
            integer index = UInt(i2);
            integer n = UInt(Zn);
            integer m = UInt(Zm);
            integer da = UInt(Zda);

    __encoding UDOT_Z.ZZZi_D
        __instruction_set A64
        __field i1 20 +: 1
        __field Zm 16 +: 4
        __field Zn 5 +: 5
        __field Zda 0 +: 5
        __opcode '01000100 111xxxxx 000001xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 64;
            integer index = UInt(i1);
            integer n = UInt(Zn);
            integer m = UInt(Zm);
            integer da = UInt(Zda);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        integer eltspersegment = 128 DIV esize;
        bits(VL) operand1 = Z[n];
        bits(VL) operand2 = Z[m];
        bits(VL) operand3 = Z[da];
        bits(VL) result;

        for e = 0 to elements-1
            integer segmentbase = e - e MOD eltspersegment;
            integer s = segmentbase + index;
            bits(esize) res = Elem[operand3, e, esize];
            for i = 0 to 3
                integer element1 = UInt(Elem[operand1, 4 * e + i, esize DIV 4]);
                integer element2 = UInt(Elem[operand2, 4 * s + i, esize DIV 4]);
                res = res + element1 * element2;
            Elem[result, e, esize] = res;

        Z[da] = result;
