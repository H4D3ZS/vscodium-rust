__instruction aarch64_integer_tags_mcsettagandzerodatapost
    __encoding aarch64_integer_tags_mcsettagandzerodatapost
        __instruction_set A64
        __field imm9 12 +: 9
        __field Xn 5 +: 5
        __field Xt 0 +: 5
        __opcode '11011001 011xxxxx xxxx01xx xxxxxxxx'
        __guard TRUE
        __decode
            integer n = UInt(Xn);
            bits(64) offset = LSL(SignExtend(imm9, 64), LOG2_TAG_GRANULE);
            boolean writeback = TRUE;
            boolean postindex = TRUE;
            boolean zero_data = TRUE;

    __encoding aarch64_integer_tags_mcsettagandzerodatapre
        __instruction_set A64
        __field imm9 12 +: 9
        __field Xn 5 +: 5
        __field Xt 0 +: 5
        __opcode '11011001 011xxxxx xxxx11xx xxxxxxxx'
        __guard TRUE
        __decode
            integer n = UInt(Xn);
            bits(64) offset = LSL(SignExtend(imm9, 64), LOG2_TAG_GRANULE);
            boolean writeback = TRUE;
            boolean postindex = FALSE;
            boolean zero_data = TRUE;

    __encoding aarch64_integer_tags_mcsettagandzerodata
        __instruction_set A64
        __field imm9 12 +: 9
        __field Xn 5 +: 5
        __field Xt 0 +: 5
        __opcode '11011001 011xxxxx xxxx10xx xxxxxxxx'
        __guard TRUE
        __decode
            integer n = UInt(Xn);
            bits(64) offset = LSL(SignExtend(imm9, 64), LOG2_TAG_GRANULE);
            boolean writeback = FALSE;
            boolean postindex = FALSE;
            boolean zero_data = TRUE;

    __execute
        bits(64) address;

        SetNotTagCheckedInstruction(TRUE);

        if n == 31 then
            CheckSPAlignment();
            address = SP[];
        else
            address = X[n];

        if !postindex then
            address = address + offset;

        if zero_data then
            Mem[address, TAG_GRANULE, AccType_NORMAL] = Zeros(TAG_GRANULE * 8);

        MemTag[address] = AllocationTagFromAddress(address);

        if writeback then
            if postindex then
                address = address + offset;

            if n == 31 then
                SP[] = address;
            else
                X[n] = address;

__instruction aarch64_integer_tags_mcsettagpairandzerodatapost
    __encoding aarch64_integer_tags_mcsettagpairandzerodatapost
        __instruction_set A64
        __field imm9 12 +: 9
        __field Xn 5 +: 5
        __field Xt 0 +: 5
        __opcode '11011001 111xxxxx xxxx01xx xxxxxxxx'
        __guard TRUE
        __decode
            integer n = UInt(Xn);
            bits(64) offset = LSL(SignExtend(imm9, 64), LOG2_TAG_GRANULE);
            boolean writeback = TRUE;
            boolean postindex = TRUE;
            boolean zero_data = TRUE;

    __encoding aarch64_integer_tags_mcsettagpairandzerodatapre
        __instruction_set A64
        __field imm9 12 +: 9
        __field Xn 5 +: 5
        __field Xt 0 +: 5
        __opcode '11011001 111xxxxx xxxx11xx xxxxxxxx'
        __guard TRUE
        __decode
            integer n = UInt(Xn);
            bits(64) offset = LSL(SignExtend(imm9, 64), LOG2_TAG_GRANULE);
            boolean writeback = TRUE;
            boolean postindex = FALSE;
            boolean zero_data = TRUE;

    __encoding aarch64_integer_tags_mcsettagpairandzerodata
        __instruction_set A64
        __field imm9 12 +: 9
        __field Xn 5 +: 5
        __field Xt 0 +: 5
        __opcode '11011001 111xxxxx xxxx10xx xxxxxxxx'
        __guard TRUE
        __decode
            integer n = UInt(Xn);
            bits(64) offset = LSL(SignExtend(imm9, 64), LOG2_TAG_GRANULE);
            boolean writeback = FALSE;
            boolean postindex = FALSE;
            boolean zero_data = TRUE;

    __execute
        bits(64) address;
        bits(4) tag;

        SetNotTagCheckedInstruction(TRUE);

        if n == 31 then
            CheckSPAlignment();
            address = SP[];
        else
            address = X[n];

        if !postindex then
            address = address + offset;

        if zero_data then
            Mem[address, TAG_GRANULE, AccType_NORMAL] = Zeros(8*TAG_GRANULE);
            Mem[address+TAG_GRANULE, TAG_GRANULE, AccType_NORMAL] = Zeros(8*TAG_GRANULE);

        tag = AllocationTagFromAddress(address);
        MemTag[address] = tag;
        MemTag[address+TAG_GRANULE] = tag;

        if writeback then
            if postindex then
                address = address + offset;

            if n == 31 then
                SP[] = address;
            else
                X[n] = address;
