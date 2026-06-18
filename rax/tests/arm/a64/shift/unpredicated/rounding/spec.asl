__instruction ASRD_Z.P.ZI__
    __encoding ASRD_Z.P.ZI__
        __instruction_set A64
        __field tszh 22 +: 2
        __field Pg 10 +: 3
        __field tszl 8 +: 2
        __field imm3 5 +: 3
        __field Zdn 0 +: 5
        __opcode '00000100 xx000100 100xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            bits(4) tsize = tszh:tszl;
            case tsize of
                when '0000' UNDEFINED;
                when '0001' esize = 8;
                when '001x' esize = 16;
                when '01xx' esize = 32;
                when '1xxx' esize = 64;
            integer g = UInt(Pg);
            integer dn = UInt(Zdn);
            integer shift = (2 * esize) - UInt(tsize:imm3);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(VL) operand1 = Z[dn];
        bits(VL) result;

        for e = 0 to elements-1
            integer element1 = SInt(Elem[operand1, e, esize]);
            if ElemP[mask, e, esize] == '1' then
                if element1 < 0 then
                    element1 = element1 + ((1 << shift) - 1);
                Elem[result, e, esize] = (element1 >> shift)<esize-1:0>;
            else
                Elem[result, e, esize] = Elem[operand1, e, esize];

        Z[dn] = result;
