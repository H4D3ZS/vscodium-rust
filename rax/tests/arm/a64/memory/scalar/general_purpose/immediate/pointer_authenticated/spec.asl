__instruction aarch64_memory_single_general_immediate_signed_pac
    __encoding aarch64_memory_single_general_immediate_signed_pac
        __instruction_set A64
        __field size 30 +: 2
        __field M 23 +: 1
        __field S 22 +: 1
        __field imm9 12 +: 9
        __field W 11 +: 1
        __field Rn 5 +: 5
        __field Rt 0 +: 5
        __opcode 'xx111000 xx1xxxxx xxxxx1xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HavePACExt() || size != '11' then UNDEFINED;
            integer t = UInt(Rt);
            integer n = UInt(Rn);
            boolean wback = (W == '1');
            boolean use_key_a = (M == '0');
            bits(10) S10 = S:imm9;
            integer scale = 3;
            bits(64) offset = LSL(SignExtend(S10, 64), scale);
            boolean tag_checked = wback || n != 31;

    __execute
        bits(64) address;
        bits(64) data;
        boolean wb_unknown = FALSE;

        if HaveMTEExt() then
            SetNotTagCheckedInstruction(!tag_checked);

        if wback && n == t && n != 31 then
            c = ConstrainUnpredictable(Unpredictable_WBOVERLAPLD);
            assert c IN {Constraint_WBSUPPRESS, Constraint_UNKNOWN, Constraint_UNDEF, Constraint_NOP};
            case c of
                when Constraint_WBSUPPRESS wback = FALSE;       // writeback is suppressed
                when Constraint_UNKNOWN    wb_unknown = TRUE;   // writeback is UNKNOWN
                when Constraint_UNDEF      UNDEFINED;
                when Constraint_NOP        EndOfInstruction();

        if n == 31 then
            CheckSPAlignment();
            address = SP[];
        else
            address = X[n];

        if use_key_a then
            address = AuthDA(address, X[31]);
        else
            address = AuthDB(address, X[31]);

        address = address + offset;
        data = Mem[address, 8, AccType_NORMAL];
        X[t] = data;

        if wback then
            if wb_unknown then
                address = bits(64) UNKNOWN;
            if n == 31 then
                SP[] = address;
            else
                X[n] = address;
