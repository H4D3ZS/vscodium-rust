__instruction ASR_Z.P.ZI__
    __encoding ASR_Z.P.ZI__
        __instruction_set A64
        __field tszh 22 +: 2
        __field Pg 10 +: 3
        __field tszl 8 +: 2
        __field imm3 5 +: 3
        __field Zdn 0 +: 5
        __opcode '00000100 xx000000 100xxxxx xxxxxxxx'
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
        bits(VL) operand1 = Z[dn];
        bits(PL) mask = P[g];
        bits(VL) result;

        for e = 0 to elements-1
            bits(esize) element1 = Elem[operand1, e, esize];
            if ElemP[mask, e, esize] == '1' then
                Elem[result, e, esize] = ASR(element1, shift);
            else
                Elem[result, e, esize] = Elem[operand1, e, esize];

        Z[dn] = result;

__instruction LSR_Z.P.ZI__
    __encoding LSR_Z.P.ZI__
        __instruction_set A64
        __field tszh 22 +: 2
        __field Pg 10 +: 3
        __field tszl 8 +: 2
        __field imm3 5 +: 3
        __field Zdn 0 +: 5
        __opcode '00000100 xx000001 100xxxxx xxxxxxxx'
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
        bits(VL) operand1 = Z[dn];
        bits(PL) mask = P[g];
        bits(VL) result;

        for e = 0 to elements-1
            bits(esize) element1 = Elem[operand1, e, esize];
            if ElemP[mask, e, esize] == '1' then
                Elem[result, e, esize] = LSR(element1, shift);
            else
                Elem[result, e, esize] = Elem[operand1, e, esize];

        Z[dn] = result;

__instruction LSL_Z.P.ZI__
    __encoding LSL_Z.P.ZI__
        __instruction_set A64
        __field tszh 22 +: 2
        __field Pg 10 +: 3
        __field tszl 8 +: 2
        __field imm3 5 +: 3
        __field Zdn 0 +: 5
        __opcode '00000100 xx000011 100xxxxx xxxxxxxx'
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
            integer shift = UInt(tsize:imm3) - esize;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) operand1 = Z[dn];
        bits(PL) mask = P[g];
        bits(VL) result;

        for e = 0 to elements-1
            bits(esize) element1 = Elem[operand1, e, esize];
            if ElemP[mask, e, esize] == '1' then
                Elem[result, e, esize] = LSL(element1, shift);
            else
                Elem[result, e, esize] = Elem[operand1, e, esize];

        Z[dn] = result;
