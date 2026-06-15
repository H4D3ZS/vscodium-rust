__instruction aarch64_integer_tags_mcsettagpost
    __encoding aarch64_integer_tags_mcsettagpost
        __instruction_set A64
        __field imm9 12 +: 9
        __field Xn 5 +: 5
        __field Xt 0 +: 5
        __opcode '11011001 001xxxxx xxxx01xx xxxxxxxx'
        __guard TRUE
        __decode
            integer n = UInt(Xn);
            bits(64) offset = LSL(SignExtend(imm9, 64), LOG2_TAG_GRANULE);
            boolean writeback = TRUE;
            boolean postindex = TRUE;
            boolean zero_data = FALSE;

    __encoding aarch64_integer_tags_mcsettagpre
        __instruction_set A64
        __field imm9 12 +: 9
        __field Xn 5 +: 5
        __field Xt 0 +: 5
        __opcode '11011001 001xxxxx xxxx11xx xxxxxxxx'
        __guard TRUE
        __decode
            integer n = UInt(Xn);
            bits(64) offset = LSL(SignExtend(imm9, 64), LOG2_TAG_GRANULE);
            boolean writeback = TRUE;
            boolean postindex = FALSE;
            boolean zero_data = FALSE;

    __encoding aarch64_integer_tags_mcsettag
        __instruction_set A64
        __field imm9 12 +: 9
        __field Xn 5 +: 5
        __field Xt 0 +: 5
        __opcode '11011001 001xxxxx xxxx10xx xxxxxxxx'
        __guard TRUE
        __decode
            integer n = UInt(Xn);
            bits(64) offset = LSL(SignExtend(imm9, 64), LOG2_TAG_GRANULE);
            boolean writeback = FALSE;
            boolean postindex = FALSE;
            boolean zero_data = FALSE;

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

__instruction aarch64_integer_tags_mcsettagpairpost
    __encoding aarch64_integer_tags_mcsettagpairpost
        __instruction_set A64
        __field imm9 12 +: 9
        __field Xn 5 +: 5
        __field Xt 0 +: 5
        __opcode '11011001 101xxxxx xxxx01xx xxxxxxxx'
        __guard TRUE
        __decode
            integer n = UInt(Xn);
            bits(64) offset = LSL(SignExtend(imm9, 64), LOG2_TAG_GRANULE);
            boolean writeback = TRUE;
            boolean postindex = TRUE;
            boolean zero_data = FALSE;

    __encoding aarch64_integer_tags_mcsettagpairpre
        __instruction_set A64
        __field imm9 12 +: 9
        __field Xn 5 +: 5
        __field Xt 0 +: 5
        __opcode '11011001 101xxxxx xxxx11xx xxxxxxxx'
        __guard TRUE
        __decode
            integer n = UInt(Xn);
            bits(64) offset = LSL(SignExtend(imm9, 64), LOG2_TAG_GRANULE);
            boolean writeback = TRUE;
            boolean postindex = FALSE;
            boolean zero_data = FALSE;

    __encoding aarch64_integer_tags_mcsettagpair
        __instruction_set A64
        __field imm9 12 +: 9
        __field Xn 5 +: 5
        __field Xt 0 +: 5
        __opcode '11011001 101xxxxx xxxx10xx xxxxxxxx'
        __guard TRUE
        __decode
            integer n = UInt(Xn);
            bits(64) offset = LSL(SignExtend(imm9, 64), LOG2_TAG_GRANULE);
            boolean writeback = FALSE;
            boolean postindex = FALSE;
            boolean zero_data = FALSE;

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

__instruction aarch64_integer_tags_mcsettaganddatapairpost
    __encoding aarch64_integer_tags_mcsettaganddatapairpost
        __instruction_set A64
        __field simm7 15 +: 7
        __field Xt2 10 +: 5
        __field Xn 5 +: 5
        __field Xt 0 +: 5
        __opcode '01101000 10xxxxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            integer n = UInt(Xn);
            integer t = UInt(Xt);
            integer t2 = UInt(Xt2);
            bits(64) offset = LSL(SignExtend(simm7, 64), LOG2_TAG_GRANULE);
            boolean writeback = TRUE;
            boolean postindex = TRUE;

    __encoding aarch64_integer_tags_mcsettaganddatapairpre
        __instruction_set A64
        __field simm7 15 +: 7
        __field Xt2 10 +: 5
        __field Xn 5 +: 5
        __field Xt 0 +: 5
        __opcode '01101001 10xxxxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            integer n = UInt(Xn);
            integer t = UInt(Xt);
            integer t2 = UInt(Xt2);
            bits(64) offset = LSL(SignExtend(simm7, 64), LOG2_TAG_GRANULE);
            boolean writeback = TRUE;
            boolean postindex = FALSE;

    __encoding aarch64_integer_tags_mcsettaganddatapair
        __instruction_set A64
        __field simm7 15 +: 7
        __field Xt2 10 +: 5
        __field Xn 5 +: 5
        __field Xt 0 +: 5
        __opcode '01101001 00xxxxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            integer n = UInt(Xn);
            integer t = UInt(Xt);
            integer t2 = UInt(Xt2);
            bits(64) offset = LSL(SignExtend(simm7, 64), LOG2_TAG_GRANULE);
            boolean writeback = FALSE;
            boolean postindex = FALSE;

    __execute
        bits(64) address;
        bits(64) data1;
        bits(64) data2;

        SetNotTagCheckedInstruction(TRUE);

        if n == 31 then
            CheckSPAlignment();
            address = SP[];
        else
            address = X[n];

        data1 = X[t];
        data2 = X[t2];

        if !postindex then
            address = address + offset;

        Mem[address, 8, AccType_NORMAL] = data1;
        Mem[address+8, 8, AccType_NORMAL] = data2;

        MemTag[address] = AllocationTagFromAddress(address);

        if writeback then
            if postindex then
                address = address + offset;

            if n == 31 then
                SP[] = address;
            else
                X[n] = address;
