__instruction LD1RB_Z.P.BI_U8
    __encoding LD1RB_Z.P.BI_U8
        __instruction_set A64
        __field imm6 16 +: 6
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10000100 01xxxxxx 100xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer g = UInt(Pg);
            integer esize = 8;
            integer msize = 8;
            boolean unsigned = TRUE;
            integer offset = UInt(imm6);

    __encoding LD1RB_Z.P.BI_U16
        __instruction_set A64
        __field imm6 16 +: 6
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10000100 01xxxxxx 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer g = UInt(Pg);
            integer esize = 16;
            integer msize = 8;
            boolean unsigned = TRUE;
            integer offset = UInt(imm6);

    __encoding LD1RB_Z.P.BI_U32
        __instruction_set A64
        __field imm6 16 +: 6
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10000100 01xxxxxx 110xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer g = UInt(Pg);
            integer esize = 32;
            integer msize = 8;
            boolean unsigned = TRUE;
            integer offset = UInt(imm6);

    __encoding LD1RB_Z.P.BI_U64
        __instruction_set A64
        __field imm6 16 +: 6
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10000100 01xxxxxx 111xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer g = UInt(Pg);
            integer esize = 64;
            integer msize = 8;
            boolean unsigned = TRUE;
            integer offset = UInt(imm6);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(64) base;
        bits(64) addr;
        bits(PL) mask = P[g];
        bits(VL) result;
        bits(msize) data;
        constant integer mbytes = msize DIV 8;

        if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);

        if n == 31 then
            CheckSPAlignment();
            base = SP[];
        else
            base = X[n];

        integer last = LastActiveElement(mask, esize);
        if last >= 0 then
            addr = base + offset * mbytes;
            data = Mem[addr, mbytes, AccType_NORMAL];

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                Elem[result, e, esize] = Extend(data, esize, unsigned);
            else
                Elem[result, e, esize] = Zeros();

        Z[t] = result;

__instruction LD1RH_Z.P.BI_U16
    __encoding LD1RH_Z.P.BI_U16
        __instruction_set A64
        __field imm6 16 +: 6
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10000100 11xxxxxx 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer g = UInt(Pg);
            integer esize = 16;
            integer msize = 16;
            boolean unsigned = TRUE;
            integer offset = UInt(imm6);

    __encoding LD1RH_Z.P.BI_U32
        __instruction_set A64
        __field imm6 16 +: 6
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10000100 11xxxxxx 110xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer g = UInt(Pg);
            integer esize = 32;
            integer msize = 16;
            boolean unsigned = TRUE;
            integer offset = UInt(imm6);

    __encoding LD1RH_Z.P.BI_U64
        __instruction_set A64
        __field imm6 16 +: 6
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10000100 11xxxxxx 111xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer g = UInt(Pg);
            integer esize = 64;
            integer msize = 16;
            boolean unsigned = TRUE;
            integer offset = UInt(imm6);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(64) base;
        bits(64) addr;
        bits(PL) mask = P[g];
        bits(VL) result;
        bits(msize) data;
        constant integer mbytes = msize DIV 8;

        if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);

        if n == 31 then
            CheckSPAlignment();
            base = SP[];
        else
            base = X[n];

        integer last = LastActiveElement(mask, esize);
        if last >= 0 then
            addr = base + offset * mbytes;
            data = Mem[addr, mbytes, AccType_NORMAL];

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                Elem[result, e, esize] = Extend(data, esize, unsigned);
            else
                Elem[result, e, esize] = Zeros();

        Z[t] = result;

__instruction LD1RW_Z.P.BI_U32
    __encoding LD1RW_Z.P.BI_U32
        __instruction_set A64
        __field imm6 16 +: 6
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10000101 01xxxxxx 110xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer g = UInt(Pg);
            integer esize = 32;
            integer msize = 32;
            boolean unsigned = TRUE;
            integer offset = UInt(imm6);

    __encoding LD1RW_Z.P.BI_U64
        __instruction_set A64
        __field imm6 16 +: 6
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10000101 01xxxxxx 111xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer g = UInt(Pg);
            integer esize = 64;
            integer msize = 32;
            boolean unsigned = TRUE;
            integer offset = UInt(imm6);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(64) base;
        bits(64) addr;
        bits(PL) mask = P[g];
        bits(VL) result;
        bits(msize) data;
        constant integer mbytes = msize DIV 8;

        if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);

        if n == 31 then
            CheckSPAlignment();
            base = SP[];
        else
            base = X[n];

        integer last = LastActiveElement(mask, esize);
        if last >= 0 then
            addr = base + offset * mbytes;
            data = Mem[addr, mbytes, AccType_NORMAL];

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                Elem[result, e, esize] = Extend(data, esize, unsigned);
            else
                Elem[result, e, esize] = Zeros();

        Z[t] = result;

__instruction LD1RD_Z.P.BI_U64
    __encoding LD1RD_Z.P.BI_U64
        __instruction_set A64
        __field imm6 16 +: 6
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10000101 11xxxxxx 111xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer g = UInt(Pg);
            integer esize = 64;
            integer msize = 64;
            boolean unsigned = TRUE;
            integer offset = UInt(imm6);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(64) base;
        bits(64) addr;
        bits(PL) mask = P[g];
        bits(VL) result;
        bits(msize) data;
        constant integer mbytes = msize DIV 8;

        if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);

        if n == 31 then
            CheckSPAlignment();
            base = SP[];
        else
            base = X[n];

        integer last = LastActiveElement(mask, esize);
        if last >= 0 then
            addr = base + offset * mbytes;
            data = Mem[addr, mbytes, AccType_NORMAL];

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                Elem[result, e, esize] = Extend(data, esize, unsigned);
            else
                Elem[result, e, esize] = Zeros();

        Z[t] = result;
