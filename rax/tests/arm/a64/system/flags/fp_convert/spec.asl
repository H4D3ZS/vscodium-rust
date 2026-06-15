__instruction aarch64_integer_flags_xaflag
    __encoding aarch64_integer_flags_xaflag
        __instruction_set A64
        __field CRm 8 +: 4
        __opcode '11010101 00000000 0100xxxx 00111111'
        __guard TRUE
        __decode
            if !HaveFlagFormatExt() then UNDEFINED;

    __execute
        bit N = NOT(PSTATE.C) AND NOT(PSTATE.Z);
        bit Z = PSTATE.Z AND PSTATE.C;
        bit C = PSTATE.C OR PSTATE.Z;
        bit V = NOT(PSTATE.C) AND PSTATE.Z;

        PSTATE.N = N;
        PSTATE.Z = Z;
        PSTATE.C = C;
        PSTATE.V = V;

__instruction aarch64_integer_flags_axflag
    __encoding aarch64_integer_flags_axflag
        __instruction_set A64
        __field CRm 8 +: 4
        __opcode '11010101 00000000 0100xxxx 01011111'
        __guard TRUE
        __decode
            if !HaveFlagFormatExt() then UNDEFINED;

    __execute
        bit N = '0';
        bit Z = PSTATE.Z OR PSTATE.V;
        bit C = PSTATE.C AND NOT(PSTATE.V);
        bit V = '0';

        PSTATE.N = N;
        PSTATE.Z = Z;
        PSTATE.C = C;
        PSTATE.V = V;
