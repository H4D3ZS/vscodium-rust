__instruction LDNT1D_Z.P.BR_Contiguous
    __encoding LDNT1D_Z.P.BR_Contiguous
        __instruction_set A64
        __field Rm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10100101 100xxxxx 110xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if Rm == '11111' then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer g = UInt(Pg);
            integer esize = 64;

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
