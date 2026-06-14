__instruction FTMAD_Z.ZZI__
    __encoding FTMAD_Z.ZZI__
        __instruction_set A64
        __field size 22 +: 2
        __field imm3 16 +: 3
        __field Zm 5 +: 5
        __field Zdn 0 +: 5
        __opcode '01100101 xx010xxx 100000xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size == '00' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer dn = UInt(Zdn);
            integer m = UInt(Zm);
            integer imm = UInt(imm3);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) operand1 = Z[dn];
        bits(VL) operand2 = Z[m];
        bits(VL) result;

        for e = 0 to elements-1
            bits(esize) element1 = Elem[operand1, e, esize];
            bits(esize) element2 = Elem[operand2, e, esize];
            Elem[result, e, esize] = FPTrigMAdd(imm, element1, element2, FPCR);

        Z[dn] = result;

__instruction FTSMUL_Z.ZZ__
    __encoding FTSMUL_Z.ZZ__
        __instruction_set A64
        __field size 22 +: 2
        __field Zm 16 +: 5
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '01100101 xx0xxxxx 000011xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size == '00' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer n = UInt(Zn);
            integer m = UInt(Zm);
            integer d = UInt(Zd);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) operand1 = Z[n];
        bits(VL) operand2 = Z[m];
        bits(VL) result;

        for e = 0 to elements-1
            bits(esize) element1 = Elem[operand1, e, esize];
            bits(esize) element2 = Elem[operand2, e, esize];
            Elem[result, e, esize] = FPTrigSMul(element1, element2, FPCR);

        Z[d] = result;

__instruction FTSSEL_Z.ZZ__
    __encoding FTSSEL_Z.ZZ__
        __instruction_set A64
        __field size 22 +: 2
        __field Zm 16 +: 5
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '00000100 xx1xxxxx 101100xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size == '00' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer n = UInt(Zn);
            integer m = UInt(Zm);
            integer d = UInt(Zd);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) operand1 = Z[n];
        bits(VL) operand2 = Z[m];
        bits(VL) result;

        for e = 0 to elements-1
            bits(esize) element1 = Elem[operand1, e, esize];
            bits(esize) element2 = Elem[operand2, e, esize];
            Elem[result, e, esize] = FPTrigSSel(element1, element2);

        Z[d] = result;
