__instruction ST1B_Z.P.BR__
    __encoding ST1B_Z.P.BR__
        __instruction_set A64
        __field size 21 +: 2
        __field Rm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '11100100 0xxxxxxx 010xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if Rm == '11111' then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer g = UInt(Pg);
            integer esize = 8 << UInt(size);
            integer msize = 8;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(64) base;
        bits(64) addr;
        bits(PL) mask = P[g];
        bits(64) offset = X[m];
        bits(VL) src = Z[t];
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
                Mem[addr, mbytes, AccType_NORMAL] = Elem[src, e, esize]<msize-1:0>;
            offset = offset + 1;

__instruction ST1W_Z.P.BR__
    __encoding ST1W_Z.P.BR__
        __instruction_set A64
        __field size 21 +: 2
        __field Rm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '11100101 0xxxxxxx 010xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size != '1x' then UNDEFINED;
            if Rm == '11111' then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer g = UInt(Pg);
            integer esize = 8 << UInt(size);
            integer msize = 32;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(64) base;
        bits(64) addr;
        bits(PL) mask = P[g];
        bits(64) offset = X[m];
        bits(VL) src = Z[t];
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
                Mem[addr, mbytes, AccType_NORMAL] = Elem[src, e, esize]<msize-1:0>;
            offset = offset + 1;

__instruction ST1D_Z.P.BR__
    __encoding ST1D_Z.P.BR__
        __instruction_set A64
        __field size 21 +: 2
        __field Rm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '11100101 1xxxxxxx 010xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size == '0x' then SEE(sve_mem_spill);
            if size == '10' then UNDEFINED;
            if Rm == '11111' then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer g = UInt(Pg);
            integer esize = 8 << UInt(size);
            integer msize = 64;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(64) base;
        bits(64) addr;
        bits(PL) mask = P[g];
        bits(64) offset = X[m];
        bits(VL) src = Z[t];
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
                Mem[addr, mbytes, AccType_NORMAL] = Elem[src, e, esize]<msize-1:0>;
            offset = offset + 1;
