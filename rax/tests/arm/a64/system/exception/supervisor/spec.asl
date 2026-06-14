__instruction aarch64_system_exceptions_runtime_svc
    __encoding aarch64_system_exceptions_runtime_svc
        __instruction_set A64
        __field imm16 5 +: 16
        __opcode '11010100 000xxxxx xxxxxxxx xxx00001'
        __guard TRUE
        __decode
            bits(16) imm = imm16;

    __execute
        AArch64.CallSupervisor(imm);
