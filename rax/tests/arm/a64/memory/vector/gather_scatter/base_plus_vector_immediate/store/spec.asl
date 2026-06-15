__instruction ST1B_Z.P.AI_S
    __encoding ST1B_Z.P.AI_S
        __instruction_set A64
        __field imm5 16 +: 5
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zt 0 +: 5
        __opcode '11100100 011xxxxx 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Zn);
            integer g = UInt(Pg);
            integer esize = 32;
            integer msize = 8;
            integer offset = UInt(imm5);

    __encoding ST1B_Z.P.AI_D
        __instruction_set A64
        __field imm5 16 +: 5
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zt 0 +: 5
        __opcode '11100100 010xxxxx 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Zn);
            integer g = UInt(Pg);
            integer esize = 64;
            integer msize = 8;
            integer offset = UInt(imm5);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) base = Z[n];
        bits(VL) src = Z[t];
        bits(PL) mask = P[g];
        bits(64) addr;
        constant integer mbytes = msize DIV 8;

        if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                addr = ZeroExtend(Elem[base, e, esize], 64) + offset * mbytes;
                Mem[addr, mbytes, AccType_NORMAL] = Elem[src, e, esize]<msize-1:0>;

__instruction ST1W_Z.P.AI_S
    __encoding ST1W_Z.P.AI_S
        __instruction_set A64
        __field imm5 16 +: 5
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zt 0 +: 5
        __opcode '11100101 011xxxxx 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Zn);
            integer g = UInt(Pg);
            integer esize = 32;
            integer msize = 32;
            integer offset = UInt(imm5);

    __encoding ST1W_Z.P.AI_D
        __instruction_set A64
        __field imm5 16 +: 5
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zt 0 +: 5
        __opcode '11100101 010xxxxx 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Zn);
            integer g = UInt(Pg);
            integer esize = 64;
            integer msize = 32;
            integer offset = UInt(imm5);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) base = Z[n];
        bits(VL) src = Z[t];
        bits(PL) mask = P[g];
        bits(64) addr;
        constant integer mbytes = msize DIV 8;

        if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                addr = ZeroExtend(Elem[base, e, esize], 64) + offset * mbytes;
                Mem[addr, mbytes, AccType_NORMAL] = Elem[src, e, esize]<msize-1:0>;

__instruction ST1D_Z.P.AI_D
    __encoding ST1D_Z.P.AI_D
        __instruction_set A64
        __field imm5 16 +: 5
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zt 0 +: 5
        __opcode '11100101 110xxxxx 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Zn);
            integer g = UInt(Pg);
            integer esize = 64;
            integer msize = 64;
            integer offset = UInt(imm5);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) base = Z[n];
        bits(VL) src = Z[t];
        bits(PL) mask = P[g];
        bits(64) addr;
        constant integer mbytes = msize DIV 8;

        if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                addr = ZeroExtend(Elem[base, e, esize], 64) + offset * mbytes;
                Mem[addr, mbytes, AccType_NORMAL] = Elem[src, e, esize]<msize-1:0>;
