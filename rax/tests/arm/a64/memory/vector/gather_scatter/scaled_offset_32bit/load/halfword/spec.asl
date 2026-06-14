__instruction LD1H_Z.P.BZ_S.x32.scaled
    __encoding LD1H_Z.P.BZ_S.x32.scaled
        __instruction_set A64
        __field xs 22 +: 1
        __field Zm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10000100 1x1xxxxx 010xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Zm);
            integer g = UInt(Pg);
            integer esize = 32;
            integer msize = 16;
            integer offs_size = 32;
            boolean unsigned = TRUE;
            boolean offs_unsigned = xs == '0';
            integer scale = 1;

    __encoding LD1H_Z.P.BZ_D.x32.scaled
        __instruction_set A64
        __field xs 22 +: 1
        __field Zm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '11000100 1x1xxxxx 010xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Zm);
            integer g = UInt(Pg);
            integer esize = 64;
            integer msize = 16;
            integer offs_size = 32;
            boolean unsigned = TRUE;
            boolean offs_unsigned = xs == '0';
            integer scale = 1;

    __encoding LD1H_Z.P.BZ_D.x32.unscaled
        __instruction_set A64
        __field xs 22 +: 1
        __field Zm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '11000100 1x0xxxxx 010xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Zm);
            integer g = UInt(Pg);
            integer esize = 64;
            integer msize = 16;
            integer offs_size = 32;
            boolean unsigned = TRUE;
            boolean offs_unsigned = xs == '0';
            integer scale = 0;

    __encoding LD1H_Z.P.BZ_S.x32.unscaled
        __instruction_set A64
        __field xs 22 +: 1
        __field Zm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10000100 1x0xxxxx 010xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Zm);
            integer g = UInt(Pg);
            integer esize = 32;
            integer msize = 16;
            integer offs_size = 32;
            boolean unsigned = TRUE;
            boolean offs_unsigned = xs == '0';
            integer scale = 0;

    __encoding LD1H_Z.P.BZ_D.64.scaled
        __instruction_set A64
        __field Zm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '11000100 111xxxxx 110xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Zm);
            integer g = UInt(Pg);
            integer esize = 64;
            integer msize = 16;
            integer offs_size = 64;
            boolean unsigned = TRUE;
            boolean offs_unsigned = TRUE;
            integer scale = 1;

    __encoding LD1H_Z.P.BZ_D.64.unscaled
        __instruction_set A64
        __field Zm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '11000100 110xxxxx 110xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Zm);
            integer g = UInt(Pg);
            integer esize = 64;
            integer msize = 16;
            integer offs_size = 64;
            boolean unsigned = TRUE;
            boolean offs_unsigned = TRUE;
            integer scale = 0;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(64) base;
        bits(64) addr;
        bits(VL) offset = Z[m];
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

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                integer off = Int(Elem[offset, e, esize]<offs_size-1:0>, offs_unsigned);
                addr = base + (off << scale);
                data = Mem[addr, mbytes, AccType_NORMAL];
                Elem[result, e, esize] = Extend(data, esize, unsigned);
            else
                Elem[result, e, esize] = Zeros();

        Z[t] = result;
