__instruction UADDV_R.P.Z__
    __encoding UADDV_R.P.Z__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Vd 0 +: 5
        __opcode '00000100 xx000001 001xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Vd);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(VL) operand = Z[n];
        integer sum = 0;

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                integer element = UInt(Elem[operand, e, esize]);
                sum = sum + element;

        V[d] = sum<63:0>;

__instruction SADDV_R.P.Z__
    __encoding SADDV_R.P.Z__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Vd 0 +: 5
        __opcode '00000100 xx000000 001xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size == '11' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Vd);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(VL) operand = Z[n];
        integer sum = 0;

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                integer element = SInt(Elem[operand, e, esize]);
                sum = sum + element;

        V[d] = sum<63:0>;
