__instruction CPY_Z.P.I__
    __encoding CPY_Z.P.I__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 16 +: 4
        __field M 14 +: 1
        __field sh 13 +: 1
        __field imm8 5 +: 8
        __field Zd 0 +: 5
        __opcode '00000101 xx01xxxx 0xxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size:sh == '001' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer d = UInt(Zd);
            boolean merging = (M == '1');
            integer imm = SInt(imm8);
            if sh == '1' then imm = imm << 8;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(VL) dest = Z[d];
        bits(VL) result;

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                Elem[result, e, esize] = imm<esize-1:0>;
            elsif merging then
                Elem[result, e, esize] = Elem[dest, e, esize];
            else
                Elem[result, e, esize] = Zeros();

        Z[d] = result;

__instruction CPY_Z.P.R__
    __encoding CPY_Z.P.R__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zd 0 +: 5
        __opcode '00000101 xx101000 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer n = UInt(Rn);
            integer d = UInt(Zd);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(64) operand1;
        if n == 31 then
            operand1 = SP[];
        else
            operand1 = X[n];
        bits(VL) result = Z[d];

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                Elem[result, e, esize] = operand1<esize-1:0>;

        Z[d] = result;

__instruction CPY_Z.P.V__
    __encoding CPY_Z.P.V__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 3
        __field Vn 5 +: 5
        __field Zd 0 +: 5
        __opcode '00000101 xx100000 100xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer n = UInt(Vn);
            integer d = UInt(Zd);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(esize) operand1 = V[n];
        bits(VL) result = Z[d];

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                Elem[result, e, esize] = operand1;

        Z[d] = result;
