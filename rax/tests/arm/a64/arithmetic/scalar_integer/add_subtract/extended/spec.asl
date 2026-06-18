__instruction aarch64_integer_arithmetic_add_sub_extendedreg
    __encoding aarch64_integer_arithmetic_add_sub_extendedreg
        __instruction_set A64
        __field sf 31 +: 1
        __field op 30 +: 1
        __field S 29 +: 1
        __field Rm 16 +: 5
        __field option 13 +: 3
        __field imm3 10 +: 3
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode 'xxx01011 001xxxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer datasize = if sf == '1' then 64 else 32;
            boolean sub_op = (op == '1');
            boolean setflags = (S == '1');
            ExtendType extend_type = DecodeRegExtend(option);
            integer shift = UInt(imm3);
            if shift > 4 then UNDEFINED;

    __execute
        bits(datasize) result;
        bits(datasize) operand1 = if n == 31 then SP[] else X[n];
        bits(datasize) operand2 = ExtendReg(m, extend_type, shift);
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

        if d == 31 && !setflags then
            SP[] = result;
        else
            X[d] = result;
