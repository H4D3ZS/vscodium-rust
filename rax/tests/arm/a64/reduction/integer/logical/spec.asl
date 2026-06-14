__instruction ANDV_R.P.Z__
    __encoding ANDV_R.P.Z__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Vd 0 +: 5
        __opcode '00000100 xx011010 001xxxxx xxxxxxxx'
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
        bits(esize) result = Ones(esize);

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                result = result AND Elem[operand, e, esize];

        V[d] = result;

__instruction ORV_R.P.Z__
    __encoding ORV_R.P.Z__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Vd 0 +: 5
        __opcode '00000100 xx011000 001xxxxx xxxxxxxx'
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
        bits(esize) result = Zeros(esize);

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                result = result OR Elem[operand, e, esize];

        V[d] = result;

__instruction EORV_R.P.Z__
    __encoding EORV_R.P.Z__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Vd 0 +: 5
        __opcode '00000100 xx011001 001xxxxx xxxxxxxx'
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
        bits(esize) result = Zeros(esize);

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                result = result EOR Elem[operand, e, esize];

        V[d] = result;
