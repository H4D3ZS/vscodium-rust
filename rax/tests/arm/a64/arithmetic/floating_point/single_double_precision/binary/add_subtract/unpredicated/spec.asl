__instruction FADD_Z.ZZ__
    __encoding FADD_Z.ZZ__
        __instruction_set A64
        __field size 22 +: 2
        __field Zm 16 +: 5
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '01100101 xx0xxxxx 000000xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size == '00' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer n = UInt(Zn);
            integer m = UInt(Zm);
            integer d = UInt(Zd);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) operand1 = Z[n];
        bits(VL) operand2 = Z[m];
        bits(VL) result;

        for e = 0 to elements-1
            bits(esize) element1 = Elem[operand1, e, esize];
            bits(esize) element2 = Elem[operand2, e, esize];
            Elem[result, e, esize] = FPAdd(element1, element2, FPCR);

        Z[d] = result;

__instruction FSUB_Z.ZZ__
    __encoding FSUB_Z.ZZ__
        __instruction_set A64
        __field size 22 +: 2
        __field Zm 16 +: 5
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '01100101 xx0xxxxx 000001xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size == '00' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer n = UInt(Zn);
            integer m = UInt(Zm);
            integer d = UInt(Zd);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) operand1 = Z[n];
        bits(VL) operand2 = Z[m];
        bits(VL) result;

        for e = 0 to elements-1
            bits(esize) element1 = Elem[operand1, e, esize];
            bits(esize) element2 = Elem[operand2, e, esize];
            Elem[result, e, esize] = FPSub(element1, element2, FPCR);

        Z[d] = result;
