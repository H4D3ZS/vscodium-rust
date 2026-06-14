__instruction LD1RQB_Z.P.BI_U8
    __encoding LD1RQB_Z.P.BI_U8
        __instruction_set A64
        __field imm4 16 +: 4
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10100100 0000xxxx 001xxxxx xxxxxxxx'
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
        integer elements = 128 DIV esize;
        bits(64) base;
        bits(64) addr;
        bits(PL) mask = P[g]; // low 16 bits only
        bits(128) result;
        constant integer mbytes = esize DIV 8;

        if n == 31 then
            CheckSPAlignment();
            if HaveMTEExt() then SetNotTagCheckedInstruction(TRUE);
            base = SP[];
        else
            if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);
            base = X[n];

        addr = base + offset * 16;
        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                Elem[result, e, esize] = Mem[addr, mbytes, AccType_NORMAL];
            else
                Elem[result, e, esize] = Zeros();
            addr = addr + mbytes;

        Z[t] = Replicate(result, VL DIV 128);

__instruction LD1RQH_Z.P.BI_U16
    __encoding LD1RQH_Z.P.BI_U16
        __instruction_set A64
        __field imm4 16 +: 4
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10100100 1000xxxx 001xxxxx xxxxxxxx'
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
        integer elements = 128 DIV esize;
        bits(64) base;
        bits(64) addr;
        bits(PL) mask = P[g]; // low 16 bits only
        bits(128) result;
        constant integer mbytes = esize DIV 8;

        if n == 31 then
            CheckSPAlignment();
            if HaveMTEExt() then SetNotTagCheckedInstruction(TRUE);
            base = SP[];
        else
            if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);
            base = X[n];

        addr = base + offset * 16;
        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                Elem[result, e, esize] = Mem[addr, mbytes, AccType_NORMAL];
            else
                Elem[result, e, esize] = Zeros();
            addr = addr + mbytes;

        Z[t] = Replicate(result, VL DIV 128);

__instruction LD1RQW_Z.P.BI_U32
    __encoding LD1RQW_Z.P.BI_U32
        __instruction_set A64
        __field imm4 16 +: 4
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10100101 0000xxxx 001xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer g = UInt(Pg);
            integer esize = 32;
            integer offset = SInt(imm4);

    __execute
        CheckSVEEnabled();
        integer elements = 128 DIV esize;
        bits(64) base;
        bits(64) addr;
        bits(PL) mask = P[g]; // low 16 bits only
        bits(128) result;
        constant integer mbytes = esize DIV 8;

        if n == 31 then
            CheckSPAlignment();
            if HaveMTEExt() then SetNotTagCheckedInstruction(TRUE);
            base = SP[];
        else
            if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);
            base = X[n];

        addr = base + offset * 16;
        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                Elem[result, e, esize] = Mem[addr, mbytes, AccType_NORMAL];
            else
                Elem[result, e, esize] = Zeros();
            addr = addr + mbytes;

        Z[t] = Replicate(result, VL DIV 128);

__instruction LD1RQD_Z.P.BI_U64
    __encoding LD1RQD_Z.P.BI_U64
        __instruction_set A64
        __field imm4 16 +: 4
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10100101 1000xxxx 001xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer g = UInt(Pg);
            integer esize = 64;
            integer offset = SInt(imm4);

    __execute
        CheckSVEEnabled();
        integer elements = 128 DIV esize;
        bits(64) base;
        bits(64) addr;
        bits(PL) mask = P[g]; // low 16 bits only
        bits(128) result;
        constant integer mbytes = esize DIV 8;

        if n == 31 then
            CheckSPAlignment();
            if HaveMTEExt() then SetNotTagCheckedInstruction(TRUE);
            base = SP[];
        else
            if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);
            base = X[n];

        addr = base + offset * 16;
        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                Elem[result, e, esize] = Mem[addr, mbytes, AccType_NORMAL];
            else
                Elem[result, e, esize] = Zeros();
            addr = addr + mbytes;

        Z[t] = Replicate(result, VL DIV 128);
