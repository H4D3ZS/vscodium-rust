__instruction DUP_Z.R__
    __encoding DUP_Z.R__
        __instruction_set A64
        __field size 22 +: 2
        __field Rn 5 +: 5
        __field Zd 0 +: 5
        __opcode '00000101 xx100000 001110xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer n = UInt(Rn);
            integer d = UInt(Zd);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(64) operand;
        if n == 31 then
            operand = SP[];
        else
            operand = X[n];
        bits(VL) result;

        for e = 0 to elements-1
            Elem[result, e, esize] = operand<esize-1:0>;

        Z[d] = result;
