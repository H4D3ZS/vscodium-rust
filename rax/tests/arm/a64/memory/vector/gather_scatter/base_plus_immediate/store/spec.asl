__instruction ST1B_Z.P.BI__
    __encoding ST1B_Z.P.BI__
        __instruction_set A64
        __field size 21 +: 2
        __field imm4 16 +: 4
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '11100100 0xx0xxxx 111xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer g = UInt(Pg);
            integer esize = 8 << UInt(size);
            integer msize = 8;
            integer offset = SInt(imm4);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(64) base;
        bits(64) addr;
        bits(PL) mask = P[g];
        bits(VL) src = Z[t];
        constant integer mbytes = msize DIV 8;

        if n == 31 then
            CheckSPAlignment();
            if HaveMTEExt() then SetNotTagCheckedInstruction(TRUE);
            base = SP[];
        else
            if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);
            base = X[n];

        addr = base + offset * elements * mbytes;
        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                Mem[addr, mbytes, AccType_NORMAL] = Elem[src, e, esize]<msize-1:0>;
            addr = addr + mbytes;

__instruction ST1H_Z.P.BI__
    __encoding ST1H_Z.P.BI__
        __instruction_set A64
        __field size 21 +: 2
        __field imm4 16 +: 4
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '11100100 1xx0xxxx 111xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size == '00' then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer g = UInt(Pg);
            integer esize = 8 << UInt(size);
            integer msize = 16;
            integer offset = SInt(imm4);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(64) base;
        bits(64) addr;
        bits(PL) mask = P[g];
        bits(VL) src = Z[t];
        constant integer mbytes = msize DIV 8;

        if n == 31 then
            CheckSPAlignment();
            if HaveMTEExt() then SetNotTagCheckedInstruction(TRUE);
            base = SP[];
        else
            if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);
            base = X[n];

        addr = base + offset * elements * mbytes;
        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                Mem[addr, mbytes, AccType_NORMAL] = Elem[src, e, esize]<msize-1:0>;
            addr = addr + mbytes;

__instruction ST1W_Z.P.BI__
    __encoding ST1W_Z.P.BI__
        __instruction_set A64
        __field size 21 +: 2
        __field imm4 16 +: 4
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '11100101 0xx0xxxx 111xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size != '1x' then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer g = UInt(Pg);
            integer esize = 8 << UInt(size);
            integer msize = 32;
            integer offset = SInt(imm4);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(64) base;
        bits(64) addr;
        bits(PL) mask = P[g];
        bits(VL) src = Z[t];
        constant integer mbytes = msize DIV 8;

        if n == 31 then
            CheckSPAlignment();
            if HaveMTEExt() then SetNotTagCheckedInstruction(TRUE);
            base = SP[];
        else
            if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);
            base = X[n];

        addr = base + offset * elements * mbytes;
        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                Mem[addr, mbytes, AccType_NORMAL] = Elem[src, e, esize]<msize-1:0>;
            addr = addr + mbytes;

__instruction ST1D_Z.P.BI__
    __encoding ST1D_Z.P.BI__
        __instruction_set A64
        __field size 21 +: 2
        __field imm4 16 +: 4
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '11100101 1xx0xxxx 111xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if size != '11' then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer g = UInt(Pg);
            integer esize = 8 << UInt(size);
            integer msize = 64;
            integer offset = SInt(imm4);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(64) base;
        bits(64) addr;
        bits(PL) mask = P[g];
        bits(VL) src = Z[t];
        constant integer mbytes = msize DIV 8;

        if n == 31 then
            CheckSPAlignment();
            if HaveMTEExt() then SetNotTagCheckedInstruction(TRUE);
            base = SP[];
        else
            if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);
            base = X[n];

        addr = base + offset * elements * mbytes;
        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                Mem[addr, mbytes, AccType_NORMAL] = Elem[src, e, esize]<msize-1:0>;
            addr = addr + mbytes;
