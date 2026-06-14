__instruction LD1SB_Z.P.BR_S16
    __encoding LD1SB_Z.P.BR_S16
        __instruction_set A64
        __field Rm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10100101 110xxxxx 010xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if Rm == '11111' then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer g = UInt(Pg);
            integer esize = 16;
            integer msize = 8;
            boolean unsigned = FALSE;

    __encoding LD1SB_Z.P.BR_S32
        __instruction_set A64
        __field Rm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10100101 101xxxxx 010xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if Rm == '11111' then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer g = UInt(Pg);
            integer esize = 32;
            integer msize = 8;
            boolean unsigned = FALSE;

    __encoding LD1SB_Z.P.BR_S64
        __instruction_set A64
        __field Rm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10100101 100xxxxx 010xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if Rm == '11111' then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer g = UInt(Pg);
            integer esize = 64;
            integer msize = 8;
            boolean unsigned = FALSE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(64) base;
        bits(64) addr;
        bits(PL) mask = P[g];
        bits(VL) result;
        bits(msize) data;
        bits(64) offset = X[m];
        constant integer mbytes = msize DIV 8;

        if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);

        if n == 31 then
            CheckSPAlignment();
            base = SP[];
        else
            base = X[n];

        for e = 0 to elements-1
            addr = base + UInt(offset) * mbytes;
            if ElemP[mask, e, esize] == '1' then
                data = Mem[addr, mbytes, AccType_NORMAL];
                Elem[result, e, esize] = Extend(data, esize, unsigned);
            else
                Elem[result, e, esize] = Zeros();
            offset = offset + 1;

        Z[t] = result;

__instruction LD1SH_Z.P.BR_S32
    __encoding LD1SH_Z.P.BR_S32
        __instruction_set A64
        __field Rm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10100101 001xxxxx 010xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if Rm == '11111' then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer g = UInt(Pg);
            integer esize = 32;
            integer msize = 16;
            boolean unsigned = FALSE;

    __encoding LD1SH_Z.P.BR_S64
        __instruction_set A64
        __field Rm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10100101 000xxxxx 010xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if Rm == '11111' then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer g = UInt(Pg);
            integer esize = 64;
            integer msize = 16;
            boolean unsigned = FALSE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(64) base;
        bits(64) addr;
        bits(PL) mask = P[g];
        bits(VL) result;
        bits(msize) data;
        bits(64) offset = X[m];
        constant integer mbytes = msize DIV 8;

        if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);

        if n == 31 then
            CheckSPAlignment();
            base = SP[];
        else
            base = X[n];

        for e = 0 to elements-1
            addr = base + UInt(offset) * mbytes;
            if ElemP[mask, e, esize] == '1' then
                data = Mem[addr, mbytes, AccType_NORMAL];
                Elem[result, e, esize] = Extend(data, esize, unsigned);
            else
                Elem[result, e, esize] = Zeros();
            offset = offset + 1;

        Z[t] = result;

__instruction LD1SW_Z.P.BR_S64
    __encoding LD1SW_Z.P.BR_S64
        __instruction_set A64
        __field Rm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10100100 100xxxxx 010xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if Rm == '11111' then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer g = UInt(Pg);
            integer esize = 64;
            integer msize = 32;
            boolean unsigned = FALSE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(64) base;
        bits(64) addr;
        bits(PL) mask = P[g];
        bits(VL) result;
        bits(msize) data;
        bits(64) offset = X[m];
        constant integer mbytes = msize DIV 8;

        if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);

        if n == 31 then
            CheckSPAlignment();
            base = SP[];
        else
            base = X[n];

        for e = 0 to elements-1
            addr = base + UInt(offset) * mbytes;
            if ElemP[mask, e, esize] == '1' then
                data = Mem[addr, mbytes, AccType_NORMAL];
                Elem[result, e, esize] = Extend(data, esize, unsigned);
            else
                Elem[result, e, esize] = Zeros();
            offset = offset + 1;

        Z[t] = result;
