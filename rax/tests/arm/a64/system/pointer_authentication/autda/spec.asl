__instruction aarch64_integer_pac_autda_dp_1src
    __encoding aarch64_integer_pac_autda_dp_1src
        __instruction_set A64
        __field Z 13 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '11011010 11000001 00x110xx xxxxxxxx'
        __guard TRUE
        __decode
            boolean source_is_sp = FALSE;
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if !HavePACExt() then
                UNDEFINED;

            if Z == '0' then // AUTDA
                if n == 31 then source_is_sp = TRUE;
            else // AUTDZA
                if n != 31 then UNDEFINED;

    __execute
        if source_is_sp then
            X[d] = AuthDA(X[d], SP[]);
        else
            X[d] = AuthDA(X[d], X[n]);

__instruction aarch64_integer_pac_autdb_dp_1src
    __encoding aarch64_integer_pac_autdb_dp_1src
        __instruction_set A64
        __field Z 13 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '11011010 11000001 00x111xx xxxxxxxx'
        __guard TRUE
        __decode
            boolean source_is_sp = FALSE;
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if !HavePACExt() then
                UNDEFINED;

            if Z == '0' then // AUTDB
                if n == 31 then source_is_sp = TRUE;
            else // AUTDZB
                if n != 31 then UNDEFINED;

    __execute
        if source_is_sp then
            X[d] = AuthDB(X[d], SP[]);
        else
            X[d] = AuthDB(X[d], X[n]);
