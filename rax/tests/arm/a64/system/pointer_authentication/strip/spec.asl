__instruction aarch64_integer_pac_strip_dp_1src
    __encoding aarch64_integer_pac_strip_dp_1src
        __instruction_set A64
        __field D 10 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '11011010 11000001 01000xxx xxxxxxxx'
        __guard TRUE
        __decode
            boolean data = (D == '1');
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if !HavePACExt() then
                UNDEFINED;

            if n != 31 then UNDEFINED;

    __encoding aarch64_integer_pac_strip_hint
        __instruction_set A64
        __opcode '11010101 00000011 00100000 11111111'
        __guard TRUE
        __decode
            integer d = 30;
            boolean data = FALSE;

    __execute
        if HavePACExt() then
            X[d] = Strip(X[d], data);
