__instruction aarch64_integer_conditional_select
    __encoding aarch64_integer_conditional_select
        __instruction_set A64
        __field sf 31 +: 1
        __field op 30 +: 1
        __field Rm 16 +: 5
        __field cond 12 +: 4
        __field o2 10 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode 'xx011010 100xxxxx xxxx0xxx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer datasize = if sf == '1' then 64 else 32;
            bits(4) condition = cond;
            boolean else_inv = (op == '1');
            boolean else_inc = (o2 == '1');

    __execute
        bits(datasize) result;
        bits(datasize) operand1 = X[n];
        bits(datasize) operand2 = X[m];

        if ConditionHolds(condition) then
            result = operand1;
        else
            result = operand2;
            if else_inv then result = NOT(result);
            if else_inc then result = result + 1;

        X[d] = result;
