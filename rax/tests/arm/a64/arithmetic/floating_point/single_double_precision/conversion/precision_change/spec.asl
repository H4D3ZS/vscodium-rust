__instruction FCVT_Z.P.Z_H2S
    __encoding FCVT_Z.P.Z_H2S
        __instruction_set A64
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '01100101 10001001 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 32;
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Zd);
            integer s_esize = 16;
            integer d_esize = 32;

    __encoding FCVT_Z.P.Z_H2D
        __instruction_set A64
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '01100101 11001001 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 64;
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Zd);
            integer s_esize = 16;
            integer d_esize = 64;

    __encoding FCVT_Z.P.Z_S2H
        __instruction_set A64
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '01100101 10001000 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 32;
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Zd);
            integer s_esize = 32;
            integer d_esize = 16;

    __encoding FCVT_Z.P.Z_S2D
        __instruction_set A64
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '01100101 11001011 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 64;
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Zd);
            integer s_esize = 32;
            integer d_esize = 64;

    __encoding FCVT_Z.P.Z_D2H
        __instruction_set A64
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '01100101 11001000 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 64;
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Zd);
            integer s_esize = 64;
            integer d_esize = 16;

    __encoding FCVT_Z.P.Z_D2S
        __instruction_set A64
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '01100101 11001010 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 64;
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Zd);
            integer s_esize = 64;
            integer d_esize = 32;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(VL) operand  = Z[n];
        bits(VL) result = Z[d];

        for e = 0 to elements-1
            bits(esize) element = Elem[operand, e, esize];
            if ElemP[mask, e, esize] == '1' then
                bits(d_esize) res = FPConvertSVE(element<s_esize-1:0>, FPCR);
                Elem[result, e, esize] = ZeroExtend(res);

        Z[d] = result;
