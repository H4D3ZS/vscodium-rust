__instruction LASTA_R.P.Z__
    __encoding LASTA_R.P.Z__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Rd 0 +: 5
        __opcode '00000101 xx100000 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer rsize = if esize < 64 then 32 else 64;
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Rd);
            boolean isBefore = FALSE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(VL) operand = Z[n];
        bits(rsize) result;
        integer last = LastActiveElement(mask, esize);

        if isBefore then
            if last < 0 then last = elements - 1;
        else
            last = last + 1;
            if last >= elements then last = 0;
        result = ZeroExtend(Elem[operand, last, esize]);

        X[d] = result;

__instruction LASTA_V.P.Z__
    __encoding LASTA_V.P.Z__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Vd 0 +: 5
        __opcode '00000101 xx100010 100xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Vd);
            boolean isBefore = FALSE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(VL) operand = Z[n];
        integer last = LastActiveElement(mask, esize);

        if isBefore then
            if last < 0 then last = elements - 1;
        else
            last = last + 1;
            if last >= elements then last = 0;
        V[d] = Elem[operand, last, esize];

__instruction LASTB_R.P.Z__
    __encoding LASTB_R.P.Z__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Rd 0 +: 5
        __opcode '00000101 xx100001 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer rsize = if esize < 64 then 32 else 64;
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Rd);
            boolean isBefore = TRUE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(VL) operand = Z[n];
        bits(rsize) result;
        integer last = LastActiveElement(mask, esize);

        if isBefore then
            if last < 0 then last = elements - 1;
        else
            last = last + 1;
            if last >= elements then last = 0;
        result = ZeroExtend(Elem[operand, last, esize]);

        X[d] = result;

__instruction LASTB_V.P.Z__
    __encoding LASTB_V.P.Z__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Vd 0 +: 5
        __opcode '00000101 xx100011 100xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer n = UInt(Zn);
            integer d = UInt(Vd);
            boolean isBefore = TRUE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(VL) operand = Z[n];
        integer last = LastActiveElement(mask, esize);

        if isBefore then
            if last < 0 then last = elements - 1;
        else
            last = last + 1;
            if last >= elements then last = 0;
        V[d] = Elem[operand, last, esize];
