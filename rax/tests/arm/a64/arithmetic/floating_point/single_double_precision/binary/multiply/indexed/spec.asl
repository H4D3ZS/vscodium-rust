__instruction FMUL_Z.ZZi_H
    __encoding FMUL_Z.ZZi_H
        __instruction_set A64
        __field i3h 22 +: 1
        __field i3l 19 +: 2
        __field Zm 16 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '01100100 0x1xxxxx 001000xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 16;
            integer index = UInt(i3h:i3l);
            integer n = UInt(Zn);
            integer m = UInt(Zm);
            integer d = UInt(Zd);

    __encoding FMUL_Z.ZZi_S
        __instruction_set A64
        __field i2 19 +: 2
        __field Zm 16 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '01100100 101xxxxx 001000xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 32;
            integer index = UInt(i2);
            integer n = UInt(Zn);
            integer m = UInt(Zm);
            integer d = UInt(Zd);

    __encoding FMUL_Z.ZZi_D
        __instruction_set A64
        __field i1 20 +: 1
        __field Zm 16 +: 4
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '01100100 111xxxxx 001000xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 64;
            integer index = UInt(i1);
            integer n = UInt(Zn);
            integer m = UInt(Zm);
            integer d = UInt(Zd);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        integer eltspersegment = 128 DIV esize;
        bits(VL) operand1 = Z[n];
        bits(VL) operand2 = Z[m];
        bits(VL) result;

        for e = 0 to elements-1
            integer segmentbase = e - e MOD eltspersegment;
            integer s = segmentbase + index;
            bits(esize) element1 = Elem[operand1, e, esize];
            bits(esize) element2 = Elem[operand2, s, esize];
            Elem[result, e, esize] = FPMul(element1, element2, FPCR);

        Z[d] = result;
