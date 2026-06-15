__instruction CLASTA_R.P.Z__
    __encoding CLASTA_R.P.Z__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 3
        __field Zm 5 +: 5
        __field Rdn 0 +: 5
        __opcode '00000101 xx110000 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer dn = UInt(Rdn);
            integer m = UInt(Zm);
            integer csize = if esize < 64 then 32 else 64;
            boolean isBefore = FALSE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(esize) operand1 = X[dn];
        bits(VL) operand2 = Z[m];
        bits(csize) result;
        integer last = LastActiveElement(mask, esize);

        if last < 0 then
            result = ZeroExtend(operand1);
        else
            if !isBefore then
                last = last + 1;
                if last >= elements then last = 0;
            result = ZeroExtend(Elem[operand2, last, esize]);

        X[dn] = result;

__instruction CLASTA_V.P.Z__
    __encoding CLASTA_V.P.Z__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 3
        __field Zm 5 +: 5
        __field Vdn 0 +: 5
        __opcode '00000101 xx101010 100xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer dn = UInt(Vdn);
            integer m = UInt(Zm);
            boolean isBefore = FALSE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(esize) operand1 = V[dn];
        bits(VL) operand2 = Z[m];
        bits(esize) result;
        integer last = LastActiveElement(mask, esize);

        if last < 0 then
            result = ZeroExtend(operand1);
        else
            if !isBefore then
                last = last + 1;
                if last >= elements then last = 0;
            result = Elem[operand2, last, esize];

        V[dn] = result;

__instruction CLASTA_Z.P.ZZ__
    __encoding CLASTA_Z.P.ZZ__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 3
        __field Zm 5 +: 5
        __field Zdn 0 +: 5
        __opcode '00000101 xx101000 100xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer dn = UInt(Zdn);
            integer m = UInt(Zm);
            boolean isBefore = FALSE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(VL) operand1 = Z[dn];
        bits(VL) operand2 = Z[m];
        bits(VL) result;
        integer last = LastActiveElement(mask, esize);

        if last < 0 then
            result = operand1;
        else
            if !isBefore then
                last = last + 1;
                if last >= elements then last = 0;
            for e = 0 to elements-1
                Elem[result, e, esize] = Elem[operand2, last, esize];

        Z[dn] = result;

__instruction CLASTB_V.P.Z__
    __encoding CLASTB_V.P.Z__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 3
        __field Zm 5 +: 5
        __field Vdn 0 +: 5
        __opcode '00000101 xx101011 100xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer dn = UInt(Vdn);
            integer m = UInt(Zm);
            boolean isBefore = TRUE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(esize) operand1 = V[dn];
        bits(VL) operand2 = Z[m];
        bits(esize) result;
        integer last = LastActiveElement(mask, esize);

        if last < 0 then
            result = ZeroExtend(operand1);
        else
            if !isBefore then
                last = last + 1;
                if last >= elements then last = 0;
            result = Elem[operand2, last, esize];

        V[dn] = result;

__instruction CLASTB_Z.P.ZZ__
    __encoding CLASTB_Z.P.ZZ__
        __instruction_set A64
        __field size 22 +: 2
        __field Pg 10 +: 3
        __field Zm 5 +: 5
        __field Zdn 0 +: 5
        __opcode '00000101 xx101001 100xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer g = UInt(Pg);
            integer dn = UInt(Zdn);
            integer m = UInt(Zm);
            boolean isBefore = TRUE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(PL) mask = P[g];
        bits(VL) operand1 = Z[dn];
        bits(VL) operand2 = Z[m];
        bits(VL) result;
        integer last = LastActiveElement(mask, esize);

        if last < 0 then
            result = operand1;
        else
            if !isBefore then
                last = last + 1;
                if last >= elements then last = 0;
            for e = 0 to elements-1
                Elem[result, e, esize] = Elem[operand2, last, esize];

        Z[dn] = result;
