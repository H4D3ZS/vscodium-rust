__instruction LD2W_Z.P.BI_Contiguous
    __encoding LD2W_Z.P.BI_Contiguous
        __instruction_set A64
        __field imm4 16 +: 4
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10100101 0010xxxx 111xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer g = UInt(Pg);
            integer esize = 32;
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

        addr = base + offset * elements * nreg * mbytes;
        for e = 0 to elements-1
            for r = 0 to nreg-1
                if ElemP[mask, e, esize] == '1' then
                    Elem[values[r], e, esize] = Mem[addr, mbytes, AccType_NORMAL];
                else
                    Elem[values[r], e, esize] = Zeros();
                addr = addr + mbytes;

        for r = 0 to nreg-1
            Z[(t+r) MOD 32] = values[r];
