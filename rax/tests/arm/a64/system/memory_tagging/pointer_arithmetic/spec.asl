__instruction aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
    __encoding aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
        __instruction_set A64
        __field Xm 16 +: 5
        __field Xn 5 +: 5
        __field Xd 0 +: 5
        __opcode '10011010 110xxxxx 000000xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Xd);
            integer n = UInt(Xn);
            integer m = UInt(Xm);
            boolean setflags = FALSE;

    __execute
        bits(64) operand1 = if n == 31 then SP[] else X[n];
        bits(64) operand2 = if m == 31 then SP[] else X[m];
        operand1 = SignExtend(operand1<55:0>, 64);
        operand2 = SignExtend(operand2<55:0>, 64);

        bits(64) result;
        bits(4) nzcv;

        operand2 = NOT(operand2);
        (result, nzcv) = AddWithCarry(operand1, operand2, '1');

        if setflags then
            PSTATE.<N,Z,C,V> = nzcv;
        X[d] = result;

__instruction aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags
    __encoding aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags
        __instruction_set A64
        __field Xm 16 +: 5
        __field Xn 5 +: 5
        __field Xd 0 +: 5
        __opcode '10111010 110xxxxx 000000xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Xd);
            integer n = UInt(Xn);
            integer m = UInt(Xm);
            boolean setflags = TRUE;

    __execute
        bits(64) operand1 = if n == 31 then SP[] else X[n];
        bits(64) operand2 = if m == 31 then SP[] else X[m];
        operand1 = SignExtend(operand1<55:0>, 64);
        operand2 = SignExtend(operand2<55:0>, 64);

        bits(64) result;
        bits(4) nzcv;

        operand2 = NOT(operand2);
        (result, nzcv) = AddWithCarry(operand1, operand2, '1');

        if setflags then
            PSTATE.<N,Z,C,V> = nzcv;
        X[d] = result;
