__instruction LD1SB_Z.P.AI_S
    __encoding LD1SB_Z.P.AI_S
        __instruction_set A64
        __field imm5 16 +: 5
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10000100 001xxxxx 100xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Zn);
            integer g = UInt(Pg);
            integer esize = 32;
            integer msize = 8;
            boolean unsigned = FALSE;
            integer offset = UInt(imm5);

    __encoding LD1SB_Z.P.AI_D
        __instruction_set A64
        __field imm5 16 +: 5
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zt 0 +: 5
        __opcode '11000100 001xxxxx 100xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Zn);
            integer g = UInt(Pg);
            integer esize = 64;
            integer msize = 8;
            boolean unsigned = FALSE;
            integer offset = UInt(imm5);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) base = Z[n];
        bits(64) addr;
        bits(PL) mask = P[g];
        bits(VL) result;
        bits(msize) data;
        constant integer mbytes = msize DIV 8;

        if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                addr = ZeroExtend(Elem[base, e, esize], 64) + offset * mbytes;
                data = Mem[addr, mbytes, AccType_NORMAL];
                Elem[result, e, esize] = Extend(data, esize, unsigned);
            else
                Elem[result, e, esize] = Zeros();

        Z[t] = result;

__instruction LD1SH_Z.P.AI_S
    __encoding LD1SH_Z.P.AI_S
        __instruction_set A64
        __field imm5 16 +: 5
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10000100 101xxxxx 100xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Zn);
            integer g = UInt(Pg);
            integer esize = 32;
            integer msize = 16;
            boolean unsigned = FALSE;
            integer offset = UInt(imm5);

    __encoding LD1SH_Z.P.AI_D
        __instruction_set A64
        __field imm5 16 +: 5
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zt 0 +: 5
        __opcode '11000100 101xxxxx 100xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Zn);
            integer g = UInt(Pg);
            integer esize = 64;
            integer msize = 16;
            boolean unsigned = FALSE;
            integer offset = UInt(imm5);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) base = Z[n];
        bits(64) addr;
        bits(PL) mask = P[g];
        bits(VL) result;
        bits(msize) data;
        constant integer mbytes = msize DIV 8;

        if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                addr = ZeroExtend(Elem[base, e, esize], 64) + offset * mbytes;
                data = Mem[addr, mbytes, AccType_NORMAL];
                Elem[result, e, esize] = Extend(data, esize, unsigned);
            else
                Elem[result, e, esize] = Zeros();

        Z[t] = result;

__instruction LD1SW_Z.P.AI_D
    __encoding LD1SW_Z.P.AI_D
        __instruction_set A64
        __field imm5 16 +: 5
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zt 0 +: 5
        __opcode '11000101 001xxxxx 100xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Zn);
            integer g = UInt(Pg);
            integer esize = 64;
            integer msize = 32;
            boolean unsigned = FALSE;
            integer offset = UInt(imm5);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) base = Z[n];
        bits(64) addr;
        bits(PL) mask = P[g];
        bits(VL) result;
        bits(msize) data;
        constant integer mbytes = msize DIV 8;

        if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                addr = ZeroExtend(Elem[base, e, esize], 64) + offset * mbytes;
                data = Mem[addr, mbytes, AccType_NORMAL];
                Elem[result, e, esize] = Extend(data, esize, unsigned);
            else
                Elem[result, e, esize] = Zeros();

        Z[t] = result;
