__instruction aarch64_integer_pac_pacia_dp_1src
    __encoding aarch64_integer_pac_pacia_dp_1src
        __instruction_set A64
        __field Z 13 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '11011010 11000001 00x000xx xxxxxxxx'
        __guard TRUE
        __decode
            boolean source_is_sp = FALSE;
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if !HavePACExt() then
                UNDEFINED;

            if Z == '0' then // PACIA
                if n == 31 then source_is_sp = TRUE;
            else // PACIZA
                if n != 31 then UNDEFINED;

    __encoding aarch64_integer_pac_pacia_hint
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
                when '0011 000' // PACIAZ
                    d = 30;
                    n = 31;
                when '0011 001' // PACIASP
                    d = 30;
                    source_is_sp = TRUE;
                    if HaveBTIExt() then
                        // Check for branch target compatibility between PSTATE.BTYPE
                        // and implicit branch target of PACIASP instruction.
                        BTypeCompatible = BTypeCompatible_PACIXSP();

                when '0001 000' // PACIA1716
                    d = 17;
                    n = 16;
                when '0001 010' SEE "PACIB";
                when '0001 100' SEE "AUTIA";
                when '0001 110' SEE "AUTIB";
                when '0011 01x' SEE "PACIB";
                when '0011 10x' SEE "AUTIA";
                when '0011 11x' SEE "AUTIB";
                when '0000 111' SEE "XPACLRI";

    __execute
        if HavePACExt() then
            if source_is_sp then
                X[d] = AddPACIA(X[d], SP[]);
            else
                X[d] = AddPACIA(X[d], X[n]);

__instruction aarch64_integer_pac_pacib_dp_1src
    __encoding aarch64_integer_pac_pacib_dp_1src
        __instruction_set A64
        __field Z 13 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '11011010 11000001 00x001xx xxxxxxxx'
        __guard TRUE
        __decode
            boolean source_is_sp = FALSE;
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if !HavePACExt() then
                UNDEFINED;

            if Z == '0' then // PACIB
                if n == 31 then source_is_sp = TRUE;
            else // PACIZB
                if n != 31 then UNDEFINED;

    __encoding aarch64_integer_pac_pacib_hint
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
                when '0011 010' // PACIBZ
                    d = 30;
                    n = 31;
                when '0011 011' // PACIBSP
                    d = 30;
                    source_is_sp = TRUE;
                    if HaveBTIExt() then
                        // Check for branch target compatibility between PSTATE.BTYPE
                        // and implicit branch target of PACIBSP instruction.
                        BTypeCompatible = BTypeCompatible_PACIXSP();
                when '0001 010' // PACIB1716
                    d = 17;
                    n = 16;
                when '0001 000' SEE "PACIA";
                when '0001 100' SEE "AUTIA";
                when '0001 110' SEE "AUTIB";
                when '0011 00x' SEE "PACIA";
                when '0011 10x' SEE "AUTIA";
                when '0011 11x' SEE "AUTIB";
                when '0000 111' SEE "XPACLRI";

    __execute
        if HavePACExt() then
            if source_is_sp then
                X[d] = AddPACIB(X[d], SP[]);
            else
                X[d] = AddPACIB(X[d], X[n]);
