__instruction REVB_Z.Z__
    __encoding REVB_Z.Z__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '00000101 xx100100 100xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size == '00' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Zd);
            integer swsize = 8;

    __encoding REVH_Z.Z__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '00000101 xx100101 100xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size != '1x' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Zd);
            integer swsize = 16;

    __encoding REVW_Z.Z__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '00000101 xx100110 100xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size != '11' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Zd);
            integer swsize = 32;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(VL) operand = Z[n];
        bits(VL) result = Z[d];

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                bits(esize) element = Elem[operand, e, esize];
                Elem[result, e, esize] = Reverse(element, swsize);

        Z[d] = result;
