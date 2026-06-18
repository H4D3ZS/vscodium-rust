__instruction aarch64_integer_flags_setf
    __encoding aarch64_integer_flags_setf
        __instruction_set A64
        __field sf 31 +: 1
        __field sz 14 +: 1
        __field Rn 5 +: 5
        __opcode 'x0111010 00000000 0x0010xx xxx01101'
        __guard TRUE
        __decode
            if !HaveFlagManipulateExt() || sf != '0' then UNDEFINED;
            integer msb = if sz=='1' then 15 else 7;
            integer n = UInt(Rn);

    __execute
        bits(32) tmpreg = X[n];
        PSTATE.N = tmpreg<msb>;
        PSTATE.Z = if (tmpreg<msb:0> == Zeros(msb+1)) then '1' else '0';
        PSTATE.V = tmpreg<msb+1> EOR tmpreg<msb>;
        //PSTATE.C unchanged;

__instruction aarch64_integer_flags_cfinv
    __encoding aarch64_integer_flags_cfinv
        __instruction_set A64
        __field CRm 8 +: 4
        __opcode '11010101 00000000 0100xxxx 00011111'
        __guard TRUE
        __decode
            if !HaveFlagManipulateExt() then UNDEFINED;

    __execute
        PSTATE.C = NOT(PSTATE.C);

__instruction aarch64_integer_flags_rmif
    __encoding aarch64_integer_flags_rmif
        __instruction_set A64
        __field sf 31 +: 1
        __field imm6 15 +: 6
        __field Rn 5 +: 5
        __field mask 0 +: 4
        __opcode 'x0111010 000xxxxx x00001xx xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveFlagManipulateExt() || sf != '1' then UNDEFINED;
            integer lsb = UInt(imm6);
            integer n = UInt(Rn);

    __execute
        bits(4) tmp;
        bits(64) tmpreg = X[n];
        tmp = (tmpreg:tmpreg)<lsb+3:lsb>;
        if mask<3> == '1' then PSTATE.N = tmp<3>;
        if mask<2> == '1' then PSTATE.Z = tmp<2>;
        if mask<1> == '1' then PSTATE.C = tmp<1>;
        if mask<0> == '1' then PSTATE.V = tmp<0>;
