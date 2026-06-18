__instruction ASR_Z.ZI__
    __encoding ASR_Z.ZI__
        __instruction_set A64
        __field tszh 22 +: 2
        __field tszl 19 +: 2
        __field imm3 16 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '00000100 xx1xxxxx 100100xx xxxxxxxx'
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
            integer n = UInt(Zn);
            integer d = UInt(Zd);
            integer shift = (2 * esize) - UInt(tsize:imm3);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) operand1 = Z[n];
        bits(VL) result;

        for e = 0 to elements-1
            bits(esize) element1 = Elem[operand1, e, esize];
            Elem[result, e, esize] = ASR(element1, shift);

        Z[d] = result;

__instruction LSR_Z.ZI__
    __encoding LSR_Z.ZI__
        __instruction_set A64
        __field tszh 22 +: 2
        __field tszl 19 +: 2
        __field imm3 16 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '00000100 xx1xxxxx 100101xx xxxxxxxx'
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
            integer n = UInt(Zn);
            integer d = UInt(Zd);
            integer shift = (2 * esize) - UInt(tsize:imm3);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) operand1 = Z[n];
        bits(VL) result;

        for e = 0 to elements-1
            bits(esize) element1 = Elem[operand1, e, esize];
            Elem[result, e, esize] = LSR(element1, shift);

        Z[d] = result;

__instruction LSL_Z.ZI__
    __encoding LSL_Z.ZI__
        __instruction_set A64
        __field tszh 22 +: 2
        __field tszl 19 +: 2
        __field imm3 16 +: 3
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '00000100 xx1xxxxx 100111xx xxxxxxxx'
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
            integer n = UInt(Zn);
            integer d = UInt(Zd);
            integer shift = UInt(tsize:imm3) - esize;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) operand1 = Z[n];
        bits(VL) result;

        for e = 0 to elements-1
            bits(esize) element1 = Elem[operand1, e, esize];
            Elem[result, e, esize] = LSL(element1, shift);

        Z[d] = result;
