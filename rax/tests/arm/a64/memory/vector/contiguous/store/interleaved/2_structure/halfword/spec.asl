__instruction ST2H_Z.P.BI_Contiguous
    __encoding ST2H_Z.P.BI_Contiguous
        __instruction_set A64
        __field imm4 16 +: 4
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '11100100 1011xxxx 111xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer g = UInt(Pg);
            integer esize = 16;
            integer offset = SInt(imm4);
            integer nreg = 2;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(64) base;
        bits(64) addr;
        bits(PL) mask = P[g];
        constant integer mbytes = esize DIV 8;
        array [0..1] of bits(VL) values;

        if n == 31 then
            CheckSPAlignment();
            if HaveMTEExt() then SetNotTagCheckedInstruction(TRUE);
            base = SP[];
        else
            if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);
            base = X[n];

        for r = 0 to nreg-1
            values[r] = Z[(t+r) MOD 32];

        addr = base + offset * elements * nreg * mbytes;
        for e = 0 to elements-1
            for r = 0 to nreg-1
                if ElemP[mask, e, esize] == '1' then
                    Mem[addr, mbytes, AccType_NORMAL] = Elem[values[r], e, esize];
                addr = addr + mbytes;

__instruction ST2H_Z.P.BR_Contiguous
    __encoding ST2H_Z.P.BR_Contiguous
        __instruction_set A64
        __field Rm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '11100100 101xxxxx 011xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if Rm == '11111' then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer g = UInt(Pg);
            integer esize = 16;
            integer nreg = 2;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(64) base;
        bits(64) addr;
        bits(PL) mask = P[g];
        bits(64) offset = X[m];
        constant integer mbytes = esize DIV 8;
        array [0..1] of bits(VL) values;

        if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);

        if n == 31 then
            CheckSPAlignment();
            base = SP[];
        else
            base = X[n];

        for r = 0 to nreg-1
            values[r] = Z[(t+r) MOD 32];

        for e = 0 to elements-1
            addr = base + UInt(offset) * mbytes;
            for r = 0 to nreg-1
                if ElemP[mask, e, esize] == '1' then
                    Mem[addr, mbytes, AccType_NORMAL] = Elem[values[r], e, esize];
                addr = addr + mbytes;
            offset = offset + nreg;
