__instruction aarch64_integer_shift_variable
    __encoding aarch64_integer_shift_variable
        __instruction_set A64
        __field sf 31 +: 1
        __field Rm 16 +: 5
        __field op2 10 +: 2
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode 'x0011010 110xxxxx 0010xxxx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer datasize = if sf == '1' then 64 else 32;
            ShiftType shift_type = DecodeShift(op2);

    __execute
        bits(datasize) result;
        bits(datasize) operand2 = X[m];

        result = ShiftReg(n, shift_type, UInt(operand2) MOD datasize);
        X[d] = result;
