__instruction aarch64_integer_arithmetic_mul_widening_32_64
    __encoding aarch64_integer_arithmetic_mul_widening_32_64
        __instruction_set A64
        __field U 23 +: 1
        __field Rm 16 +: 5
        __field o0 15 +: 1
        __field Ra 10 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '10011011 x01xxxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer a = UInt(Ra);
            integer destsize = 64;
            integer datasize = 32;
            boolean sub_op = (o0 == '1');
            boolean unsigned = (U == '1');

    __execute
        bits(datasize) operand1 = X[n];
        bits(datasize) operand2 = X[m];
        bits(destsize) operand3 = X[a];

        integer result;

        if sub_op then
            result = Int(operand3, unsigned) - (Int(operand1, unsigned) * Int(operand2, unsigned));
        else
            result = Int(operand3, unsigned) + (Int(operand1, unsigned) * Int(operand2, unsigned));

        X[d] = result<63:0>;

__instruction aarch64_integer_arithmetic_mul_widening_64_128hi
    __encoding aarch64_integer_arithmetic_mul_widening_64_128hi
        __instruction_set A64
        __field U 23 +: 1
        __field Rm 16 +: 5
        __field Ra 10 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '10011011 x10xxxxx 0xxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer a = UInt(Ra);           // ignored by UMULH/SMULH
            integer destsize = 64;
            integer datasize = destsize;
            boolean unsigned = (U == '1');

    __execute
        bits(datasize) operand1 = X[n];
        bits(datasize) operand2 = X[m];

        integer result;

        result = Int(operand1, unsigned) * Int(operand2, unsigned);

        X[d] = result<127:64>;
