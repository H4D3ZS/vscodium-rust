__instruction aarch64_system_monitors
    __encoding aarch64_system_monitors
        __instruction_set A64
        __field CRm 8 +: 4
        __opcode '11010101 00000011 0011xxxx 01011111'
        __guard TRUE
        __decode
            // CRm field is ignored

    __execute
        ClearExclusiveLocal(ProcessorID());
