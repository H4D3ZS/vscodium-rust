__instruction aarch32_CBNZ_A
    __encoding aarch32_CBNZ_T1_A
        __instruction_set T16
        __field op 27 +: 1
        __field i 25 +: 1
        __field imm5 19 +: 5
        __field Rn 16 +: 3
        __opcode '1011x0x1 xxxxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            n = UInt(Rn);  imm32 = ZeroExtend(i:imm5:'0', 32);  nonzero = (op == '1');
            if InITBlock() then UNPREDICTABLE;

    __execute
        if nonzero != IsZero(R[n]) then
            BranchWritePC(PC + imm32, BranchType_DIR);
