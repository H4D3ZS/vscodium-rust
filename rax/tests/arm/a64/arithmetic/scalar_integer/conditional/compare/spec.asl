__instruction aarch64_integer_conditional_compare_register
    __encoding aarch64_integer_conditional_compare_register
        __instruction_set A64
        __field sf 31 +: 1
        __field op 30 +: 1
        __field Rm 16 +: 5
        __field cond 12 +: 4
        __field Rn 5 +: 5
        __field nzcv 0 +: 4
        __opcode 'xx111010 010xxxxx xxxx00xx xxx0xxxx'
        __guard TRUE
        __decode
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer datasize = if sf == '1' then 64 else 32;
            boolean sub_op = (op == '1');
            bits(4) condition = cond;
            bits(4) flags = nzcv;

    __execute
        bits(datasize) operand1 = X[n];
        bits(datasize) operand2 = X[m];
        bit carry_in = '0';

        if ConditionHolds(condition) then
            if sub_op then
                operand2 = NOT(operand2);
                carry_in = '1';
            (-, flags) = AddWithCarry(operand1, operand2, carry_in);
        PSTATE.<N,Z,C,V> = flags;

__instruction aarch64_integer_conditional_compare_immediate
    __encoding aarch64_integer_conditional_compare_immediate
        __instruction_set A64
        __field sf 31 +: 1
        __field op 30 +: 1
        __field imm5 16 +: 5
        __field cond 12 +: 4
        __field Rn 5 +: 5
        __field nzcv 0 +: 4
        __opcode 'xx111010 010xxxxx xxxx10xx xxx0xxxx'
        __guard TRUE
        __decode
            integer n = UInt(Rn);
            integer datasize = if sf == '1' then 64 else 32;
            boolean sub_op = (op == '1');
            bits(4) condition = cond;
            bits(4) flags = nzcv;
            bits(datasize) imm = ZeroExtend(imm5, datasize);

    __execute
        bits(datasize) operand1 = X[n];
        bits(datasize) operand2 = imm;
        bit carry_in = '0';

        if ConditionHolds(condition) then
            if sub_op then
                operand2 = NOT(operand2);
                carry_in = '1';
            (-, flags) = AddWithCarry(operand1, operand2, carry_in);
        PSTATE.<N,Z,C,V> = flags;
