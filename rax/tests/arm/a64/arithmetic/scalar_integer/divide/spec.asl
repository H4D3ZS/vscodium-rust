__instruction aarch64_integer_arithmetic_div
    __encoding aarch64_integer_arithmetic_div
        __instruction_set A64
        __field sf 31 +: 1
        __field Rm 16 +: 5
        __field o1 10 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode 'x0011010 110xxxxx 00001xxx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer datasize = if sf == '1' then 64 else 32;
            boolean unsigned = (o1 == '0');

    __execute
        bits(datasize) operand1 = X[n];
        bits(datasize) operand2 = X[m];
        integer result;

        if IsZero(operand2) then
            result = 0;
        else
            result = RoundTowardsZero(Real(Int(operand1, unsigned)) / Real(Int(operand2, unsigned)));

        X[d] = result<datasize-1:0>;
