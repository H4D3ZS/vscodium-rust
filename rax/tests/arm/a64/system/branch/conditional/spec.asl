__instruction aarch64_branch_conditional_cond
    __encoding aarch64_branch_conditional_cond
        __instruction_set A64
        __field imm19 5 +: 19
        __field cond 0 +: 4
        __opcode '01010100 xxxxxxxx xxxxxxxx xxx0xxxx'
        __guard TRUE
        __decode
            bits(64) offset = SignExtend(imm19:'00', 64);
            bits(4) condition = cond;

    __execute
        if ConditionHolds(condition) then
            BranchTo(PC[] + offset, BranchType_DIR);

__instruction aarch64_branch_conditional_compare
    __encoding aarch64_branch_conditional_compare
        __instruction_set A64
        __field sf 31 +: 1
        __field op 24 +: 1
        __field imm19 5 +: 19
        __field Rt 0 +: 5
        __opcode 'x011010x xxxxxxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            integer t = UInt(Rt);
            integer datasize = if sf == '1' then 64 else 32;
            boolean iszero = (op == '0');
            bits(64) offset = SignExtend(imm19:'00', 64);

    __execute
        bits(datasize) operand1 = X[t];

        if IsZero(operand1) == iszero then
            BranchTo(PC[] + offset, BranchType_DIR);

__instruction aarch64_branch_conditional_test
    __encoding aarch64_branch_conditional_test
        __instruction_set A64
        __field b5 31 +: 1
        __field op 24 +: 1
        __field b40 19 +: 5
        __field imm14 5 +: 14
        __field Rt 0 +: 5
        __opcode 'x011011x xxxxxxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            integer t = UInt(Rt);

            integer datasize = if b5 == '1' then 64 else 32;
            integer bit_pos = UInt(b5:b40);
            bit bit_val = op;
            bits(64) offset = SignExtend(imm14:'00', 64);

    __execute
        bits(datasize) operand = X[t];

        if operand<bit_pos> == bit_val then
            BranchTo(PC[] + offset, BranchType_DIR);
