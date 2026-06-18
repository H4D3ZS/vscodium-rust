__instruction aarch64_integer_pac_pacda_dp_1src
    __encoding aarch64_integer_pac_pacda_dp_1src
        __instruction_set A64
        __field Z 13 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '11011010 11000001 00x010xx xxxxxxxx'
        __guard TRUE
        __decode
            boolean source_is_sp = FALSE;
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if !HavePACExt() then
                UNDEFINED;

            if Z == '0' then // PACDA
                if n == 31 then source_is_sp = TRUE;
            else // PACDZA
                if n != 31 then UNDEFINED;

    __execute
        if source_is_sp then
            X[d] = AddPACDA(X[d], SP[]);
        else
            X[d] = AddPACDA(X[d], X[n]);

__instruction aarch64_integer_pac_pacdb_dp_1src
    __encoding aarch64_integer_pac_pacdb_dp_1src
        __instruction_set A64
        __field Z 13 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '11011010 11000001 00x011xx xxxxxxxx'
        __guard TRUE
        __decode
            boolean source_is_sp = FALSE;
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if !HavePACExt() then
                UNDEFINED;

            if Z == '0' then // PACDB
                if n == 31 then source_is_sp = TRUE;
            else // PACDZB
                if n != 31 then UNDEFINED;

    __execute
        if source_is_sp then
            X[d] = AddPACDB(X[d], SP[]);
        else
            X[d] = AddPACDB(X[d], X[n]);
