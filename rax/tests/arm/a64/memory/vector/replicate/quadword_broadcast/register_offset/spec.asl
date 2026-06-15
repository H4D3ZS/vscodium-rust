__instruction LD1RQB_Z.P.BR_Contiguous
    __encoding LD1RQB_Z.P.BR_Contiguous
        __instruction_set A64
        __field Rm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10100100 000xxxxx 000xxxxx xxxxxxxx'
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
        integer elements = 128 DIV esize;
        bits(64) base;
        bits(64) addr;
        bits(PL) mask = P[g]; // low 16 bits only
        bits(64) offset;
        bits(128) result;
        constant integer mbytes = esize DIV 8;

        if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);

        if n == 31 then
            CheckSPAlignment();
            base = SP[];
        else
            base = X[n];
        offset = X[m];

        addr = base + UInt(offset) * mbytes;
        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                Elem[result, e, esize] = Mem[addr, mbytes, AccType_NORMAL];
            else
                Elem[result, e, esize] = Zeros();
            addr = addr + mbytes;

        Z[t] = Replicate(result, VL DIV 128);

__instruction LD1RQH_Z.P.BR_Contiguous
    __encoding LD1RQH_Z.P.BR_Contiguous
        __instruction_set A64
        __field Rm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10100100 100xxxxx 000xxxxx xxxxxxxx'
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
        integer elements = 128 DIV esize;
        bits(64) base;
        bits(64) addr;
        bits(PL) mask = P[g]; // low 16 bits only
        bits(64) offset;
        bits(128) result;
        constant integer mbytes = esize DIV 8;

        if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);

        if n == 31 then
            CheckSPAlignment();
            base = SP[];
        else
            base = X[n];
        offset = X[m];

        addr = base + UInt(offset) * mbytes;
        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                Elem[result, e, esize] = Mem[addr, mbytes, AccType_NORMAL];
            else
                Elem[result, e, esize] = Zeros();
            addr = addr + mbytes;

        Z[t] = Replicate(result, VL DIV 128);

__instruction LD1RQW_Z.P.BR_Contiguous
    __encoding LD1RQW_Z.P.BR_Contiguous
        __instruction_set A64
        __field Rm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10100101 000xxxxx 000xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if Rm == '11111' then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer g = UInt(Pg);
            integer esize = 32;

    __execute
        CheckSVEEnabled();
        integer elements = 128 DIV esize;
        bits(64) base;
        bits(64) addr;
        bits(PL) mask = P[g]; // low 16 bits only
        bits(64) offset;
        bits(128) result;
        constant integer mbytes = esize DIV 8;

        if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);

        if n == 31 then
            CheckSPAlignment();
            base = SP[];
        else
            base = X[n];
        offset = X[m];

        addr = base + UInt(offset) * mbytes;
        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                Elem[result, e, esize] = Mem[addr, mbytes, AccType_NORMAL];
            else
                Elem[result, e, esize] = Zeros();
            addr = addr + mbytes;

        Z[t] = Replicate(result, VL DIV 128);

__instruction LD1RQD_Z.P.BR_Contiguous
    __encoding LD1RQD_Z.P.BR_Contiguous
        __instruction_set A64
        __field Rm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10100101 100xxxxx 000xxxxx xxxxxxxx'
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
        integer elements = 128 DIV esize;
        bits(64) base;
        bits(64) addr;
        bits(PL) mask = P[g]; // low 16 bits only
        bits(64) offset;
        bits(128) result;
        constant integer mbytes = esize DIV 8;

        if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);

        if n == 31 then
            CheckSPAlignment();
            base = SP[];
        else
            base = X[n];
        offset = X[m];

        addr = base + UInt(offset) * mbytes;
        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                Elem[result, e, esize] = Mem[addr, mbytes, AccType_NORMAL];
            else
                Elem[result, e, esize] = Zeros();
            addr = addr + mbytes;

        Z[t] = Replicate(result, VL DIV 128);
