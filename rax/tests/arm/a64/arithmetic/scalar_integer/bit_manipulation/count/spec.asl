__instruction aarch64_integer_arithmetic_cnt
    __encoding aarch64_integer_arithmetic_cnt
        __instruction_set A64
        __field sf 31 +: 1
        __field op 10 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode 'x1011010 11000000 00010xxx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer datasize = if sf == '1' then 64 else 32;
            CountOp opcode = if op == '0' then CountOp_CLZ else CountOp_CLS;

    __execute
        integer result;
        bits(datasize) operand1 = X[n];

        if opcode == CountOp_CLZ then
            result = CountLeadingZeroBits(operand1);
        else
            result = CountLeadingSignBits(operand1);

        X[d] = result<datasize-1:0>;
