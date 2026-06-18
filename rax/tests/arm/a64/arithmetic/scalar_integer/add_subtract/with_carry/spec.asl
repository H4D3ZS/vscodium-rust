__instruction aarch64_integer_arithmetic_add_sub_carry
    __encoding aarch64_integer_arithmetic_add_sub_carry
        __instruction_set A64
        __field sf 31 +: 1
        __field op 30 +: 1
        __field S 29 +: 1
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode 'xxx11010 000xxxxx 000000xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer datasize = if sf == '1' then 64 else 32;
            boolean sub_op = (op == '1');
            boolean setflags = (S == '1');

    __execute
        bits(datasize) result;
        bits(datasize) operand1 = X[n];
        bits(datasize) operand2 = X[m];
        bits(4) nzcv;

        if sub_op then
            operand2 = NOT(operand2);

        (result, nzcv) = AddWithCarry(operand1, operand2, PSTATE.C);

        if setflags then
            PSTATE.<N,Z,C,V> = nzcv;

        X[d] = result;
