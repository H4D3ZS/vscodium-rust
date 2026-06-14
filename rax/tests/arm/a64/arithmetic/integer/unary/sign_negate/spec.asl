__instruction ABS_Z.P.Z__
    __encoding ABS_Z.P.Z__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '00000100 xx010110 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Zd);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(VL) operand  = Z[n];
        bits(VL) result = Z[d];

        for e = 0 to elements-1
            integer element = SInt(Elem[operand, e, esize]);
            if ElemP[mask, e, esize] == '1' then
                element = Abs(element);
                Elem[result, e, esize] = element<esize-1:0>;

        Z[d] = result;

__instruction NEG_Z.P.Z__
    __encoding NEG_Z.P.Z__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '00000100 xx010111 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Zd);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(VL) operand  = Z[n];
        bits(VL) result = Z[d];

        for e = 0 to elements-1
            integer element = SInt(Elem[operand, e, esize]);
            if ElemP[mask, e, esize] == '1' then
                element = -element;
                Elem[result, e, esize] = element<esize-1:0>;

        Z[d] = result;
