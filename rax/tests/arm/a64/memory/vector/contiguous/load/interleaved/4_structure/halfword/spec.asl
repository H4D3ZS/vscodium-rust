__instruction LD4H_Z.P.BR_Contiguous
    __encoding LD4H_Z.P.BR_Contiguous
        __instruction_set A64
        __field Rm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10100100 111xxxxx 110xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if Rm == '11111' then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer g = UInt(Pg);
            integer esize = 16;
            integer nreg = 4;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(64) base;
        bits(64) addr;
        bits(PL) mask = P[g];
        bits(64) offset = X[m];
        constant integer mbytes = esize DIV 8;
        array [0..3] of bits(VL) values;

        if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);

        if n == 31 then
            CheckSPAlignment();
            base = SP[];
        else
            base = X[n];

        for e = 0 to elements-1
            addr = base + UInt(offset) * mbytes;
            for r = 0 to nreg-1
                if ElemP[mask, e, esize] == '1' then
                    Elem[values[r], e, esize] = Mem[addr, mbytes, AccType_NORMAL];
                else
                    Elem[values[r], e, esize] = Zeros();
                addr = addr + mbytes;
            offset = offset + nreg;

        for r = 0 to nreg-1
            Z[(t+r) MOD 32] = values[r];
