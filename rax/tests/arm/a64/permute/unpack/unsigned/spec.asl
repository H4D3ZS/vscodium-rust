__instruction UUNPKHI_Z.Z__
    __encoding UUNPKHI_Z.Z__
        __instruction_set A64
        __field size 22 +: 2
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '00000101 xx110011 001110xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size == '00' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer n = UInt(Zn);
            integer d = UInt(Zd);
            boolean unsigned = TRUE;
            boolean hi = TRUE;

    __encoding UUNPKLO_Z.Z__
        __instruction_set A64
        __field size 22 +: 2
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '00000101 xx110010 001110xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size == '00' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer n = UInt(Zn);
            integer d = UInt(Zd);
            boolean unsigned = TRUE;
            boolean hi = FALSE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        integer hsize = esize DIV 2;
        bits(VL) operand = Z[n];
        bits(VL) result;

        for e = 0 to elements-1
            bits(hsize) element = if hi then Elem[operand, e + elements, hsize] else Elem[operand, e, hsize];
            Elem[result, e, esize] = Extend(element, esize, unsigned);

        Z[d] = result;
