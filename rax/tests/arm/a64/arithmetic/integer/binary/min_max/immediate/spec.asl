__instruction SMIN_Z.ZI__
    __encoding SMIN_Z.ZI__
        __instruction_set A64
        __field size 22 +: 2
        __field imm8 5 +: 8
        __field Zdn 0 +: 5
        __opcode '00100101 xx101010 110xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer dn = UInt(Zdn);
            boolean unsigned = FALSE;
            integer imm = Int(imm8, unsigned);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) operand1 = Z[dn];
        bits(VL) result;

        for e = 0 to elements-1
            integer element1 = Int(Elem[operand1, e, esize], unsigned);
            Elem[result, e, esize] = Min(element1, imm)<esize-1:0>;

        Z[dn] = result;

__instruction SMAX_Z.ZI__
    __encoding SMAX_Z.ZI__
        __instruction_set A64
        __field size 22 +: 2
        __field imm8 5 +: 8
        __field Zdn 0 +: 5
        __opcode '00100101 xx101000 110xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer dn = UInt(Zdn);
            boolean unsigned = FALSE;
            integer imm = Int(imm8, unsigned);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) operand1 = Z[dn];
        bits(VL) result;

        for e = 0 to elements-1
            integer element1 = Int(Elem[operand1, e, esize], unsigned);
            Elem[result, e, esize] = Max(element1, imm)<esize-1:0>;

        Z[dn] = result;

__instruction UMIN_Z.ZI__
    __encoding UMIN_Z.ZI__
        __instruction_set A64
        __field size 22 +: 2
        __field imm8 5 +: 8
        __field Zdn 0 +: 5
        __opcode '00100101 xx101011 110xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer dn = UInt(Zdn);
            boolean unsigned = TRUE;
            integer imm = Int(imm8, unsigned);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) operand1 = Z[dn];
        bits(VL) result;

        for e = 0 to elements-1
            integer element1 = Int(Elem[operand1, e, esize], unsigned);
            Elem[result, e, esize] = Min(element1, imm)<esize-1:0>;

        Z[dn] = result;

__instruction UMAX_Z.ZI__
    __encoding UMAX_Z.ZI__
        __instruction_set A64
        __field size 22 +: 2
        __field imm8 5 +: 8
        __field Zdn 0 +: 5
        __opcode '00100101 xx101001 110xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer dn = UInt(Zdn);
            boolean unsigned = TRUE;
            integer imm = Int(imm8, unsigned);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) operand1 = Z[dn];
        bits(VL) result;

        for e = 0 to elements-1
            integer element1 = Int(Elem[operand1, e, esize], unsigned);
            Elem[result, e, esize] = Max(element1, imm)<esize-1:0>;

        Z[dn] = result;
