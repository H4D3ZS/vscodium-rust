__instruction aarch64_vector_arithmetic_unary_add_pairwise
    __encoding aarch64_vector_arithmetic_unary_add_pairwise
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field size 22 +: 2
        __field op 14 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01110 xx100000 0x1010xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if size == '11' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV (2*esize);
            boolean acc = (op == '1');
            boolean unsigned = (U == '1');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand = V[n];
        bits(datasize) result;

        bits(2*esize) sum;
        integer op1;
        integer op2;

        result = if acc then V[d] else Zeros();
        for e = 0 to elements-1
            op1 = Int(Elem[operand, 2*e+0, esize], unsigned);
            op2 = Int(Elem[operand, 2*e+1, esize], unsigned);
            sum = (op1 + op2)<2*esize-1:0>;
            Elem[result, e, 2*esize] = Elem[result, e, 2*esize] + sum;

        V[d] = result;
