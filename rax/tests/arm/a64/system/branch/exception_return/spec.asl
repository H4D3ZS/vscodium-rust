__instruction aarch64_branch_unconditional_eret
    __encoding aarch64_branch_unconditional_eret
        __instruction_set A64
        __field A 11 +: 1
        __field M 10 +: 1
        __field Rn 5 +: 5
        __field op4 0 +: 5
        __opcode '11010110 10011111 0000xxxx xxxxxxxx'
        __guard TRUE
        __decode
            if PSTATE.EL == EL0 then UNDEFINED;
            boolean pac = (A == '1');
            boolean use_key_a = (M == '0');

            if !pac && op4 != '00000' then
                UNDEFINED;
            elsif pac && (!HavePACExt() || op4 != '11111') then
                UNDEFINED;

            if Rn != '11111' then
                UNDEFINED;

    __execute
        AArch64.CheckForERetTrap(pac, use_key_a);
        bits(64) target = ELR[];

        if pac then
            if use_key_a then
                target = AuthIA(ELR[], SP[]);
            else
                target = AuthIB(ELR[], SP[]);

        AArch64.ExceptionReturn(target, SPSR[]);

__instruction aarch64_branch_unconditional_dret
    __encoding aarch64_branch_unconditional_dret
        __instruction_set A64
        __opcode '11010110 10111111 00000011 11100000'
        __guard TRUE
        __decode
            if !Halted() || PSTATE.EL == EL0 then UNDEFINED;

    __execute
        DRPSInstruction();
