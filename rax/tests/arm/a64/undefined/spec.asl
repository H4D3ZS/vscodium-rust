__instruction aarch64_udf
    __encoding aarch64_udf
        __instruction_set A64
        __field imm16 0 +: 16
        __opcode '00000000 00000000 xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            // The imm16 field is ignored by hardware.
            UNDEFINED;

    __execute
        // No operation.
