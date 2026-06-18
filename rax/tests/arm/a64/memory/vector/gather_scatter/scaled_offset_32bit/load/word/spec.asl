__instruction LD1W_Z.P.BZ_S.x32.scaled
    __encoding LD1W_Z.P.BZ_S.x32.scaled
        __instruction_set A64
        __field xs 22 +: 1
        __field Zm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10000101 0x1xxxxx 010xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Zm);
            integer g = UInt(Pg);
            integer esize = 32;
            integer msize = 32;
            integer offs_size = 32;
            boolean unsigned = TRUE;
            boolean offs_unsigned = xs == '0';
            integer scale = 2;

    __encoding LD1W_Z.P.BZ_D.x32.scaled
        __instruction_set A64
        __field xs 22 +: 1
        __field Zm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '11000101 0x1xxxxx 010xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Zm);
            integer g = UInt(Pg);
            integer esize = 64;
            integer msize = 32;
            integer offs_size = 32;
            boolean unsigned = TRUE;
            boolean offs_unsigned = xs == '0';
            integer scale = 2;

    __encoding LD1W_Z.P.BZ_D.x32.unscaled
        __instruction_set A64
        __field xs 22 +: 1
        __field Zm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '11000101 0x0xxxxx 010xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Zm);
            integer g = UInt(Pg);
            integer esize = 64;
            integer msize = 32;
            integer offs_size = 32;
            boolean unsigned = TRUE;
            boolean offs_unsigned = xs == '0';
            integer scale = 0;

    __encoding LD1W_Z.P.BZ_S.x32.unscaled
        __instruction_set A64
        __field xs 22 +: 1
        __field Zm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10000101 0x0xxxxx 010xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Zm);
            integer g = UInt(Pg);
            integer esize = 32;
            integer msize = 32;
            integer offs_size = 32;
            boolean unsigned = TRUE;
            boolean offs_unsigned = xs == '0';
            integer scale = 0;

    __encoding LD1W_Z.P.BZ_D.64.scaled
        __instruction_set A64
        __field Zm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '11000101 011xxxxx 110xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Zm);
            integer g = UInt(Pg);
            integer esize = 64;
            integer msize = 32;
            integer offs_size = 64;
            boolean unsigned = TRUE;
            boolean offs_unsigned = TRUE;
            integer scale = 2;

    __encoding LD1W_Z.P.BZ_D.64.unscaled
        __instruction_set A64
        __field Zm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '11000101 010xxxxx 110xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Zm);
            integer g = UInt(Pg);
            integer esize = 64;
            integer msize = 32;
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

__instruction LDFF1W_Z.P.BZ_S.x32.scaled
    __encoding LDFF1W_Z.P.BZ_S.x32.scaled
        __instruction_set A64
        __field xs 22 +: 1
        __field Zm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10000101 0x1xxxxx 011xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Zm);
            integer g = UInt(Pg);
            integer esize = 32;
            integer msize = 32;
            integer offs_size = 32;
            boolean unsigned = TRUE;
            boolean offs_unsigned = xs == '0';
            integer scale = 2;

    __encoding LDFF1W_Z.P.BZ_D.x32.scaled
        __instruction_set A64
        __field xs 22 +: 1
        __field Zm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '11000101 0x1xxxxx 011xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Zm);
            integer g = UInt(Pg);
            integer esize = 64;
            integer msize = 32;
            integer offs_size = 32;
            boolean unsigned = TRUE;
            boolean offs_unsigned = xs == '0';
            integer scale = 2;

    __encoding LDFF1W_Z.P.BZ_D.x32.unscaled
        __instruction_set A64
        __field xs 22 +: 1
        __field Zm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '11000101 0x0xxxxx 011xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Zm);
            integer g = UInt(Pg);
            integer esize = 64;
            integer msize = 32;
            integer offs_size = 32;
            boolean unsigned = TRUE;
            boolean offs_unsigned = xs == '0';
            integer scale = 0;

    __encoding LDFF1W_Z.P.BZ_S.x32.unscaled
        __instruction_set A64
        __field xs 22 +: 1
        __field Zm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10000101 0x0xxxxx 011xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Zm);
            integer g = UInt(Pg);
            integer esize = 32;
            integer msize = 32;
            integer offs_size = 32;
            boolean unsigned = TRUE;
            boolean offs_unsigned = xs == '0';
            integer scale = 0;

    __encoding LDFF1W_Z.P.BZ_D.64.scaled
        __instruction_set A64
        __field Zm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '11000101 011xxxxx 111xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Zm);
            integer g = UInt(Pg);
            integer esize = 64;
            integer msize = 32;
            integer offs_size = 64;
            boolean unsigned = TRUE;
            boolean offs_unsigned = TRUE;
            integer scale = 2;

    __encoding LDFF1W_Z.P.BZ_D.64.unscaled
        __instruction_set A64
        __field Zm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '11000101 010xxxxx 111xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Zm);
            integer g = UInt(Pg);
            integer esize = 64;
            integer msize = 32;
            integer offs_size = 64;
            boolean unsigned = TRUE;
            boolean offs_unsigned = TRUE;
            integer scale = 0;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(64) base;
        bits(64) addr;
        bits(VL) offset;
        bits(PL) mask = P[g];
        bits(VL) result;
        bits(VL) orig = Z[t];
        bits(msize) data;
        constant integer mbytes = msize DIV 8;
        boolean first = TRUE;
        boolean fault = FALSE;
        boolean faulted = FALSE;
        boolean unknown = FALSE;

        if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);

        if n == 31 then
            CheckSPAlignment();
            base = SP[];
        else
            base = X[n];
        offset = Z[m];

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                integer off = Int(Elem[offset, e, esize]<offs_size-1:0>, offs_unsigned);
                addr = base + (off << scale);
                if first then
                    // Mem[] will not return if a fault is detected for the first active element
                    data = Mem[addr, mbytes, AccType_NORMAL];
                    first = FALSE;
                else
                    // MemNF[] will return fault=TRUE if access is not performed for any reason
                    (data, fault) = MemNF[addr, mbytes, AccType_NONFAULT];
            else
                (data, fault) = (Zeros(msize), FALSE);

            // FFR elements set to FALSE following a supressed access/fault
            faulted = faulted || fault;
            if faulted then
                ElemFFR[e, esize] = '0';

            // Value becomes CONSTRAINED UNPREDICTABLE after an FFR element is FALSE
            unknown = unknown || ElemFFR[e, esize] == '0';
            if unknown then
                if !fault && ConstrainUnpredictableBool(Unpredictable_SVELDNFDATA) then
                    Elem[result, e, esize] = Extend(data, esize, unsigned);
                elsif ConstrainUnpredictableBool(Unpredictable_SVELDNFZERO) then
                    Elem[result, e, esize] = Zeros();
                else  // merge
                    Elem[result, e, esize] = Elem[orig, e, esize];
            else
                Elem[result, e, esize] = Extend(data, esize, unsigned);

        Z[t] = result;
