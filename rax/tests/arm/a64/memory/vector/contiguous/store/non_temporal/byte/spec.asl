__instruction STNT1B_Z.P.BI_Contiguous
    __encoding STNT1B_Z.P.BI_Contiguous
        __instruction_set A64
        __field imm4 16 +: 4
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '11100100 0001xxxx 111xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer g = UInt(Pg);
            integer esize = 8;
            integer offset = SInt(imm4);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(64) base;
        bits(64) addr;
        constant integer mbytes = esize DIV 8;
        bits(VL) src;
        bits(PL) mask = P[g];

        if n == 31 then
            CheckSPAlignment();
            if HaveMTEExt() then SetNotTagCheckedInstruction(TRUE);
            base = SP[];
        else
            if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);
            base = X[n];
        src = Z[t];

        addr = base + offset * elements * mbytes;
        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                Mem[addr, mbytes, AccType_STREAM] = Elem[src, e, esize];
            addr = addr + mbytes;

__instruction STNT1B_Z.P.BR_Contiguous
    __encoding STNT1B_Z.P.BR_Contiguous
        __instruction_set A64
        __field Rm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '11100100 000xxxxx 011xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if Rm == '11111' then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer g = UInt(Pg);
            integer esize = 8;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(64) base;
        bits(64) addr;
        bits(64) offset = X[m];
        bits(VL) src;
        bits(PL) mask = P[g];
        constant integer mbytes = esize DIV 8;

        if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);

        if n == 31 then
            CheckSPAlignment();
            base = SP[];
        else
            base = X[n];

        src = Z[t];
        for e = 0 to elements-1
            addr = base + UInt(offset) * mbytes;
            if ElemP[mask, e, esize] == '1' then
                Mem[addr, mbytes, AccType_STREAM] = Elem[src, e, esize];
            offset = offset + 1;
