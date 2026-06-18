__instruction PRFD_I.P.BI_S
    __encoding PRFD_I.P.BI_S
        __instruction_set A64
        __field imm6 16 +: 6
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field prfop 0 +: 4
        __opcode '10000101 11xxxxxx 011xxxxx xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 64;
            integer g = UInt(Pg);
            integer n = UInt(Rn);
            integer level = UInt(prfop<2:1>);
            boolean stream = (prfop<0> == '1');
            pref_hint = if prfop<3> == '0' then Prefetch_READ else Prefetch_WRITE;
            integer scale = 3;
            integer offset = SInt(imm6);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(64) base;
        bits(64) addr;

        if n == 31 then
            base = SP[];
        else
            base = X[n];

        addr = base + ((offset * elements) << scale);
        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                Hint_Prefetch(addr, pref_hint, level, stream);
            addr = addr + (1 << scale);

__instruction PRFD_I.P.BR_S
    __encoding PRFD_I.P.BR_S
        __instruction_set A64
        __field Rm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field prfop 0 +: 4
        __opcode '10000101 100xxxxx 110xxxxx xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            if Rm == '11111' then UNDEFINED;
            integer esize = 64;
            integer g = UInt(Pg);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer level = UInt(prfop<2:1>);
            boolean stream = (prfop<0> == '1');
            pref_hint = if prfop<3> == '0' then Prefetch_READ else Prefetch_WRITE;
            integer scale = 3;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(64) base;
        bits(64) offset = X[m];
        bits(64) addr;

        if n == 31 then
            base = SP[];
        else
            base = X[n];

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                addr = base + (UInt(offset) << scale);
                Hint_Prefetch(addr, pref_hint, level, stream);
            offset = offset + 1;

__instruction PRFD_I.P.AI_S
    __encoding PRFD_I.P.AI_S
        __instruction_set A64
        __field imm5 16 +: 5
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field prfop 0 +: 4
        __opcode '10000101 100xxxxx 111xxxxx xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 32;
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer level = UInt(prfop<2:1>);
            boolean stream = (prfop<0> == '1');
            pref_hint = if prfop<3> == '0' then Prefetch_READ else Prefetch_WRITE;
            integer scale = 3;
            integer offset = UInt(imm5);

    __encoding PRFD_I.P.AI_D
        __instruction_set A64
        __field imm5 16 +: 5
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field prfop 0 +: 4
        __opcode '11000101 100xxxxx 111xxxxx xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 64;
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer level = UInt(prfop<2:1>);
            boolean stream = (prfop<0> == '1');
            pref_hint = if prfop<3> == '0' then Prefetch_READ else Prefetch_WRITE;
            integer scale = 3;
            integer offset = UInt(imm5);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(VL) base;
        bits(64) addr;
        base = Z[n];

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                addr = ZeroExtend(Elem[base, e, esize], 64) + (offset << scale);
                Hint_Prefetch(addr, pref_hint, level, stream);

__instruction PRFD_I.P.BZ_S.x32.scaled
    __encoding PRFD_I.P.BZ_S.x32.scaled
        __instruction_set A64
        __field xs 22 +: 1
        __field Zm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field prfop 0 +: 4
        __opcode '10000100 0x1xxxxx 011xxxxx xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 32;
            integer g = UInt(Pg);
            integer n = UInt(Rn);
            integer m = UInt(Zm);
            integer level = UInt(prfop<2:1>);
            boolean stream = (prfop<0> == '1');
            pref_hint = if prfop<3> == '0' then Prefetch_READ else Prefetch_WRITE;
            integer offs_size = 32;
            boolean offs_unsigned = (xs == '0');
            integer scale = 3;

    __encoding PRFD_I.P.BZ_D.x32.scaled
        __instruction_set A64
        __field xs 22 +: 1
        __field Zm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field prfop 0 +: 4
        __opcode '11000100 0x1xxxxx 011xxxxx xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 64;
            integer g = UInt(Pg);
            integer n = UInt(Rn);
            integer m = UInt(Zm);
            integer level = UInt(prfop<2:1>);
            boolean stream = (prfop<0> == '1');
            pref_hint = if prfop<3> == '0' then Prefetch_READ else Prefetch_WRITE;
            integer offs_size = 32;
            boolean offs_unsigned = (xs == '0');
            integer scale = 3;

    __encoding PRFD_I.P.BZ_D.64.scaled
        __instruction_set A64
        __field Zm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field prfop 0 +: 4
        __opcode '11000100 011xxxxx 111xxxxx xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 64;
            integer g = UInt(Pg);
            integer n = UInt(Rn);
            integer m = UInt(Zm);
            integer level = UInt(prfop<2:1>);
            boolean stream = (prfop<0> == '1');
            pref_hint = if prfop<3> == '0' then Prefetch_READ else Prefetch_WRITE;
            integer offs_size = 64;
            boolean offs_unsigned = TRUE;
            integer scale = 3;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(64) base;
        bits(64) addr;
        bits(VL) offset;

        if n == 31 then
            base = SP[];
        else
            base = X[n];
        offset = Z[m];

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                integer off = Int(Elem[offset, e, esize]<offs_size-1:0>, offs_unsigned);
                addr = base + (off << scale);
                Hint_Prefetch(addr, pref_hint, level, stream);
