__instruction aarch64_integer_pac_pacga_dp_2src
    __encoding aarch64_integer_pac_pacga_dp_2src
        __instruction_set A64
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '10011010 110xxxxx 001100xx xxxxxxxx'
        __guard TRUE
        __decode
            boolean source_is_sp = FALSE;
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);

            if !HavePACExt() then
                UNDEFINED;

            if m == 31 then source_is_sp = TRUE;

    __execute
        if source_is_sp then
            X[d] = AddPACGA(X[n], SP[]);
        else
            X[d] = AddPACGA(X[n], X[m]);
