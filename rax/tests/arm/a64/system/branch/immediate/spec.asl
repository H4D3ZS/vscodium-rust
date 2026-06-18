__instruction aarch64_branch_unconditional_immediate
    __encoding aarch64_branch_unconditional_immediate
        __instruction_set A64
        __field op 31 +: 1
        __field imm26 0 +: 26
        __opcode 'x00101xx xxxxxxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            BranchType branch_type = if op == '1' then BranchType_DIRCALL else BranchType_DIR;
            bits(64) offset = SignExtend(imm26:'00', 64);

    __execute
        if branch_type == BranchType_DIRCALL then X[30] = PC[] + 4;

        BranchTo(PC[] + offset, branch_type);
