__instruction SXTB_Z.P.Z__
    __encoding SXTB_Z.P.Z__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '00000100 xx010000 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size == '00' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer s_esize = 8;
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Zd);
            boolean unsigned = FALSE;

    __encoding SXTH_Z.P.Z__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '00000100 xx010010 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size != '1x' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer s_esize = 16;
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Zd);
            boolean unsigned = FALSE;

    __encoding SXTW_Z.P.Z__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '00000100 xx010100 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size != '11' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer s_esize = 32;
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Zd);
            boolean unsigned = FALSE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(VL) operand  = Z[n];
        bits(VL) result = Z[d];

        for e = 0 to elements-1
            bits(esize) element = Elem[operand, e, esize];
            if ElemP[mask, e, esize] == '1' then
                Elem[result, e, esize] = Extend(element<s_esize-1:0>, esize, unsigned);

        Z[d] = result;
