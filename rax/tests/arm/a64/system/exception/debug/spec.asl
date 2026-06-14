__instruction aarch64_system_exceptions_debug_halt
    __encoding aarch64_system_exceptions_debug_halt
        __instruction_set A64
        __field imm16 5 +: 16
        __opcode '11010100 010xxxxx xxxxxxxx xxx00000'
        __guard TRUE
        __decode
            if EDSCR.HDE == '0' || !HaltingAllowed() then UndefinedFault();
            if HaveBTIExt() then
                BTypeCompatible = TRUE;

    __execute
        Halt(DebugHalt_HaltInstruction);

__instruction aarch64_system_exceptions_debug_breakpoint
    __encoding aarch64_system_exceptions_debug_breakpoint
        __instruction_set A64
        __field imm16 5 +: 16
        __opcode '11010100 001xxxxx xxxxxxxx xxx00000'
        __guard TRUE
        __decode
            bits(16) comment = imm16;
            if HaveBTIExt() then
                BTypeCompatible = TRUE;

    __execute
        AArch64.SoftwareBreakpoint(comment);

__instruction aarch64_system_exceptions_debug_exception
    __encoding aarch64_system_exceptions_debug_exception
        __instruction_set A64
        __field imm16 5 +: 16
        __field LL 0 +: 2
        __opcode '11010100 101xxxxx xxxxxxxx xxx000xx'
        __guard TRUE
        __decode
            bits(2) target_level = LL;
            if LL == '00' then UNDEFINED;
            if !Halted() then AArch64.UndefinedFault();

    __execute
        DCPSInstruction(target_level);
