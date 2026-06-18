__instruction FRECPE_Z.Z__
    __encoding FRECPE_Z.Z__
        __instruction_set A64
        __field size 22 +: 2
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '01100101 xx001110 001100xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size == '00' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer n = UInt(Zn);
            integer d = UInt(Zd);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) operand = Z[n];
        bits(VL) result;

        for e = 0 to elements-1
            bits(esize) element = Elem[operand, e, esize];
            Elem[result, e, esize] = FPRecipEstimate(element, FPCR);

        Z[d] = result;

__instruction FRSQRTE_Z.Z__
    __encoding FRSQRTE_Z.Z__
        __instruction_set A64
        __field size 22 +: 2
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '01100101 xx001111 001100xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size == '00' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer n = UInt(Zn);
            integer d = UInt(Zd);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) operand = Z[n];
        bits(VL) result;

        for e = 0 to elements-1
            bits(esize) element = Elem[operand, e, esize];
            Elem[result, e, esize] = FPRSqrtEstimate(element, FPCR);

        Z[d] = result;
