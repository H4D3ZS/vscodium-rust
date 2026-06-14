__instruction LDNT1H_Z.P.BR_Contiguous
    __encoding LDNT1H_Z.P.BR_Contiguous
        __instruction_set A64
        __field Rm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10100100 100xxxxx 110xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if Rm == '11111' then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer g = UInt(Pg);
            integer esize = 16;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(64) base;
        bits(64) addr;
        bits(64) offset;
        bits(PL) mask = P[g];
        bits(VL) result;
        constant integer mbytes = esize DIV 8;

        if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);

        if n == 31 then
            CheckSPAlignment();
            base = SP[];
        else
            base = X[n];
        offset = X[m];

        for e = 0 to elements-1
            addr = base + UInt(offset) * mbytes;
            if ElemP[mask, e, esize] == '1' then
                Elem[result, e, esize] = Mem[addr, mbytes, AccType_STREAM];
            else
                Elem[result, e, esize] = Zeros();
            offset = offset + 1;

        Z[t] = result;

__instruction LDNT1H_Z.P.BI_Contiguous
    __encoding LDNT1H_Z.P.BI_Contiguous
        __instruction_set A64
        __field imm4 16 +: 4
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10100100 1000xxxx 111xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer g = UInt(Pg);
            integer esize = 16;
            integer offset = SInt(imm4);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(64) base;
        bits(64) addr;
        bits(PL) mask = P[g];
        bits(VL) result;
        constant integer mbytes = esize DIV 8;

        if n == 31 then
            CheckSPAlignment();
            if HaveMTEExt() then SetNotTagCheckedInstruction(TRUE);
            base = SP[];
        else
            if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);
            base = X[n];

        addr = base + offset * elements * mbytes;
        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                Elem[result, e, esize] = Mem[addr, mbytes, AccType_STREAM];
            else
                Elem[result, e, esize] = Zeros();
            addr = addr + mbytes;

        Z[t] = result;
