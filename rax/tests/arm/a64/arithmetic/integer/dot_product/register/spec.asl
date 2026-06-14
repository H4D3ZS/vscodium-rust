__instruction SDOT_Z.ZZZ__
    __encoding SDOT_Z.ZZZ__
        __instruction_set A64
        __field size 22 +: 2
        __field Zm 16 +: 5
        __field Zn 5 +: 5
        __field Zda 0 +: 5
        __opcode '01000100 xx0xxxxx 000000xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size == '0x' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer n = UInt(Zn);
            integer m = UInt(Zm);
            integer da = UInt(Zda);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) operand1 = Z[n];
        bits(VL) operand2 = Z[m];
        bits(VL) operand3 = Z[da];
        bits(VL) result;

        for e = 0 to elements-1
            bits(esize) res = Elem[operand3, e, esize];
            for i = 0 to 3
                integer element1 = SInt(Elem[operand1, 4 * e + i, esize DIV 4]);
                integer element2 = SInt(Elem[operand2, 4 * e + i, esize DIV 4]);
                res = res + element1 * element2;
            Elem[result, e, esize] = res;

        Z[da] = result;

__instruction UDOT_Z.ZZZ__
    __encoding UDOT_Z.ZZZ__
        __instruction_set A64
        __field size 22 +: 2
        __field Zm 16 +: 5
        __field Zn 5 +: 5
        __field Zda 0 +: 5
        __opcode '01000100 xx0xxxxx 000001xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size == '0x' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer n = UInt(Zn);
            integer m = UInt(Zm);
            integer da = UInt(Zda);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) operand1 = Z[n];
        bits(VL) operand2 = Z[m];
        bits(VL) operand3 = Z[da];
        bits(VL) result;

        for e = 0 to elements-1
            bits(esize) res = Elem[operand3, e, esize];
            for i = 0 to 3
                integer element1 = UInt(Elem[operand1, 4 * e + i, esize DIV 4]);
                integer element2 = UInt(Elem[operand2, 4 * e + i, esize DIV 4]);
                res = res + element1 * element2;
            Elem[result, e, esize] = res;

        Z[da] = result;
