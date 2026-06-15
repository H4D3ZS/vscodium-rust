__instruction FEXPA_Z.Z__
    __encoding FEXPA_Z.Z__
        __instruction_set A64
        __field size 22 +: 2
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '00000100 xx100000 101110xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size == '00' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer n = UInt(Zn);
            integer d = UInt(Zd);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) operand  = Z[n];
        bits(VL) result;

        for e = 0 to elements-1
            bits(esize) element = Elem[operand, e, esize];
            Elem[result, e, esize] = FPExpA(element);

        Z[d] = result;
