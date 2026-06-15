__instruction aarch64_integer_pac_autia_dp_1src
    __encoding aarch64_integer_pac_autia_dp_1src
        __instruction_set A64
        __field Z 13 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '11011010 11000001 00x100xx xxxxxxxx'
        __guard TRUE
        __decode
            boolean source_is_sp = FALSE;
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if !HavePACExt() then
                UNDEFINED;

            if Z == '0' then // AUTIA
                if n == 31 then source_is_sp = TRUE;
            else // AUTIZA
                if n != 31 then UNDEFINED;

    __encoding aarch64_integer_pac_autia_hint
        __instruction_set A64
        __field CRm 8 +: 4
        __field op2 5 +: 3
        __opcode '11010101 00000011 0010xxxx xxx11111'
        __guard TRUE
        __decode
            integer d;
            integer n;
            boolean source_is_sp = FALSE;

            case CRm:op2 of
                when '0011 100' // AUTIAZ
                    d = 30;
                    n = 31;
                when '0011 101' // AUTIASP
                    d = 30;
                    source_is_sp = TRUE;
                when '0001 100' // AUTIA1716
                    d = 17;
                    n = 16;
                when '0001 000' SEE "PACIA";
                when '0001 010' SEE "PACIB";
                when '0001 110' SEE "AUTIB";
                when '0011 00x' SEE "PACIA";
                when '0011 01x' SEE "PACIB";
                when '0011 11x' SEE "AUTIB";
                when '0000 111' SEE "XPACLRI";
                otherwise SEE "HINT";

    __execute
        if HavePACExt() then
            if source_is_sp then
                X[d] = AuthIA(X[d], SP[]);
            else
                X[d] = AuthIA(X[d], X[n]);

__instruction aarch64_integer_pac_autib_dp_1src
    __encoding aarch64_integer_pac_autib_dp_1src
        __instruction_set A64
        __field Z 13 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '11011010 11000001 00x101xx xxxxxxxx'
        __guard TRUE
        __decode
            boolean source_is_sp = FALSE;
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if !HavePACExt() then
                UNDEFINED;

            if Z == '0' then // AUTIB
                if n == 31 then source_is_sp = TRUE;
            else // AUTIZB
                if n != 31 then UNDEFINED;

    __encoding aarch64_integer_pac_autib_hint
        __instruction_set A64
        __field CRm 8 +: 4
        __field op2 5 +: 3
        __opcode '11010101 00000011 0010xxxx xxx11111'
        __guard TRUE
        __decode
            integer d;
            integer n;
            boolean source_is_sp = FALSE;

            case CRm:op2 of
                when '0011 110' // AUTIBZ
                    d = 30;
                    n = 31;
                when '0011 111' // AUTIBSP
                    d = 30;
                    source_is_sp = TRUE;
                when '0001 110' // AUTIB1716
                    d = 17;
                    n = 16;
                when '0001 000' SEE "PACIA";
                when '0001 010' SEE "PACIB";
                when '0001 100' SEE "AUTIA";
                when '0011 00x' SEE "PACIA";
                when '0011 01x' SEE "PACIB";
                when '0011 10x' SEE "AUTIA";
                when '0000 111' SEE "XPACLRI";

    __execute
        if HavePACExt() then
            if source_is_sp then
                X[d] = AuthIB(X[d], SP[]);
            else
                X[d] = AuthIB(X[d], X[n]);
