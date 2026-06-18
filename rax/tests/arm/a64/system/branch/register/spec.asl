__instruction aarch64_branch_unconditional_register
    __encoding aarch64_branch_unconditional_register
        __instruction_set A64
        __field Z 24 +: 1
        __field op 21 +: 2
        __field A 11 +: 1
        __field M 10 +: 1
        __field Rn 5 +: 5
        __field Rm 0 +: 5
        __opcode '1101011x 0xx11111 0000xxxx xxxxxxxx'
        __guard TRUE
        __decode
            integer n = UInt(Rn);
            BranchType branch_type;
            integer m = UInt(Rm);
            boolean pac = (A == '1');
            boolean use_key_a = (M == '0');
            boolean source_is_sp = ((Z == '1') && (m == 31));

            if !pac && m != 0 then
                UNDEFINED;
            elsif pac && !HavePACExt() then
                UNDEFINED;

            case op of
                when '00' branch_type = BranchType_INDIR;
                when '01' branch_type = BranchType_INDCALL;
                when '10' branch_type = BranchType_RET;
                otherwise UNDEFINED;

            if pac then
                if Z == '0' && m != 31 then
                    UNDEFINED;

                if branch_type == BranchType_RET then
                    if n != 31 then UNDEFINED;
                    n = 30;
                    source_is_sp = TRUE;

    __execute
        bits(64) target = X[n];
        if pac then
            bits(64) modifier = if source_is_sp then SP[] else X[m];

            if use_key_a then
                target = AuthIA(target, modifier);
            else
                target = AuthIB(target, modifier);

        if branch_type == BranchType_INDCALL then X[30] = PC[] + 4;

        // Value in BTypeNext will be used to set PSTATE.BTYPE
        case branch_type of
            when BranchType_INDIR           // BR, BRAA, BRAB, BRAAZ, BRABZ
                if InGuardedPage then
                    if n == 16 || n == 17 then
                        BTypeNext = '01';
                    else
                        BTypeNext = '11';
                else
                    BTypeNext = '01';
            when BranchType_INDCALL         // BLR, BLRAA, BLRAB, BLRAAZ, BLRABZ
                BTypeNext = '10';
            when BranchType_RET             // RET, RETAA, RETAB
                BTypeNext = '00';

        BranchTo(target, branch_type);
