__instruction aarch64_integer_arithmetic_mul_uniform_add_sub
    __encoding aarch64_integer_arithmetic_mul_uniform_add_sub
        __instruction_set A64
        __field sf 31 +: 1
        __field Rm 16 +: 5
        __field o0 15 +: 1
        __field Ra 10 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode 'x0011011 000xxxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer a = UInt(Ra);
            integer destsize = if sf == '1' then 64 else 32;
            integer datasize = destsize;
            boolean sub_op = (o0 == '1');

    __execute
        bits(datasize) operand1 = X[n];
        bits(datasize) operand2 = X[m];
        bits(destsize) operand3 = X[a];

        integer result;

        if sub_op then
            result = UInt(operand3) - (UInt(operand1) * UInt(operand2));
        else
            result = UInt(operand3) + (UInt(operand1) * UInt(operand2));

        X[d] = result<destsize-1:0>;
