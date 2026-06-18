__instruction aarch64_integer_arithmetic_add_sub_shiftedreg
    __encoding aarch64_integer_arithmetic_add_sub_shiftedreg
        __instruction_set A64
        __field sf 31 +: 1
        __field op 30 +: 1
        __field S 29 +: 1
        __field shift 22 +: 2
        __field Rm 16 +: 5
        __field imm6 10 +: 6
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode 'xxx01011 xx0xxxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer datasize = if sf == '1' then 64 else 32;
            boolean sub_op = (op == '1');
            boolean setflags = (S == '1');

            if shift == '11' then UNDEFINED;
            if sf == '0' && imm6<5> == '1' then UNDEFINED;

            ShiftType shift_type = DecodeShift(shift);
            integer shift_amount = UInt(imm6);

    __execute
        bits(datasize) result;
        bits(datasize) operand1 = X[n];
        bits(datasize) operand2 = ShiftReg(m, shift_type, shift_amount);
        bits(4) nzcv;
        bit carry_in;

        if sub_op then
            operand2 = NOT(operand2);
            carry_in = '1';
        else
            carry_in = '0';

        (result, nzcv) = AddWithCarry(operand1, operand2, carry_in);

        if setflags then
            PSTATE.<N,Z,C,V> = nzcv;

        X[d] = result;
