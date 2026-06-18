__instruction MOVPRFX_Z.Z__
    __encoding MOVPRFX_Z.Z__
        __instruction_set A64
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '00000100 00100000 101111xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer n = UInt(Zn);
            integer d = UInt(Zd);

    __execute
        CheckSVEEnabled();
        bits(VL) result = Z[n];
        Z[d] = result;

__instruction MOVPRFX_Z.P.Z__
    __encoding MOVPRFX_Z.P.Z__
        __instruction_set A64
        __field size 22 +: 2
        __field M 16 +: 1
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '00000100 xx01000x 001xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Zd);
            boolean merging = (M == '1');

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(VL) operand1 = Z[n];
        bits(VL) dest = Z[d];
        bits(VL) result;

        for e = 0 to elements-1
            bits(esize) element = Elem[operand1, e, esize];
            if ElemP[mask, e, esize] == '1' then
                Elem[result, e, esize] = element;
            elsif merging then
                Elem[result, e, esize] = Elem[dest, e, esize];
            else
                Elem[result, e, esize] = Zeros();

        Z[d] = result;
