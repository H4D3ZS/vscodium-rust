__instruction aarch64_integer_tags_mcgettag
    __encoding aarch64_integer_tags_mcgettag
        __instruction_set A64
        __field imm9 12 +: 9
        __field Xn 5 +: 5
        __field Xt 0 +: 5
        __opcode '11011001 011xxxxx xxxx00xx xxxxxxxx'
        __guard TRUE
        __decode
            integer t = UInt(Xt);
            integer n = UInt(Xn);
            bits(64) offset = LSL(SignExtend(imm9, 64), LOG2_TAG_GRANULE);

    __execute
        bits(64) address;
        bits(4) tag;

        if n == 31 then
            CheckSPAlignment();
            address = SP[];
        else
            address = X[n];

        address = address + offset;
        address = Align(address, TAG_GRANULE);

        tag = MemTag[address];
        address = AddressWithAllocationTag(address, tag);

        X[t] = address;

__instruction aarch64_integer_tags_mcgettagarray
    __encoding aarch64_integer_tags_mcgettagarray
        __instruction_set A64
        __field Xn 5 +: 5
        __field Xt 0 +: 5
        __opcode '11011001 11100000 000000xx xxxxxxxx'
        __guard TRUE
        __decode
            integer t = UInt(Xt);
            integer n = UInt(Xn);
            boolean wback = TRUE;
            boolean wb_unknown = FALSE;

    __execute
        bits(64) data = Zeros(64);
        bits(64) address;
        integer count;

        if n == t then
            c = ConstrainUnpredictable(Unpredictable_WBOVERLAPLD);
            assert c IN {Constraint_WBSUPPRESS, Constraint_UNKNOWN, Constraint_UNDEF, Constraint_NOP};

            case c of
                when Constraint_WBSUPPRESS  wback = FALSE; // writeback is suppressed
                when Constraint_UNKNOWN     wb_unknown = TRUE; // writeback is UNKNOWN
                when Constraint_UNDEF       UnallocatedEncoding();
                when Constraint_NOP         EndOfInstruction();

        if n == 31 then
            CheckSPAlignment();
            address = SP[];
        else
            address = X[n];

        (address, count) = ImpDefTagArrayStartAndCount(address);

        for i = 0 to count-1
            integer index = UInt(address<LOG2_TAG_GRANULE+3:LOG2_TAG_GRANULE>);
            bits(4) tag = MemTag[address];
            data<(index*4)+3:(index*4)> = tag;
            address = address + TAG_GRANULE;

        X[t] = data;

        if wback then
            if wb_unknown then
                address = bits(64) UNKNOWN;
            if n == 31 then
                SP[] = address;
            else
                X[n] = address;
