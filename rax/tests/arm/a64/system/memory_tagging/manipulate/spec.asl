__instruction aarch64_integer_tags_mcinsertrandomtag
    __encoding aarch64_integer_tags_mcinsertrandomtag
        __instruction_set A64
        __field Xm 16 +: 5
        __field Xn 5 +: 5
        __field Xd 0 +: 5
        __opcode '10011010 110xxxxx 000100xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Xd);
            integer n = UInt(Xn);
            integer m = UInt(Xm);

    __execute
        bits(64) operand = if n == 31 then SP[] else X[n];
        bits(64) xm = X[m];
        bits(16) exclude = xm<15:0> OR GCR_EL1.Exclude;

        if AllocationTagAccessIsEnabled() then
            if GCR_EL1.RRND == '1' then
                RGSR_EL1 = bits(32) UNKNOWN;
                rtag = _ChooseRandomNonExcludedTag(exclude);
            else
                bits(4) start = RGSR_EL1.TAG;
                bits(4) offset = RandomTag();

                rtag = ChooseNonExcludedTag(start, offset, exclude);

                RGSR_EL1.TAG = rtag;
        else
            rtag = '0000';

        bits(64) result = AddressWithAllocationTag(operand, rtag);

        if d == 31 then
            SP[] = result;
        else
            X[d] = result;

__instruction aarch64_integer_tags_mcinserttagmask
    __encoding aarch64_integer_tags_mcinserttagmask
        __instruction_set A64
        __field Xm 16 +: 5
        __field Xn 5 +: 5
        __field Xd 0 +: 5
        __opcode '10011010 110xxxxx 000101xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Xd);
            integer n = UInt(Xn);
            integer m = UInt(Xm);

    __execute
        bits(64) address = if n == 31 then SP[] else X[n];
        bits(64) mask = X[m];
        bits(4) tag = AllocationTagFromAddress(address);

        mask<UInt(tag)> = '1';
        X[d] = mask;

__instruction aarch64_integer_tags_mcsubtag
    __encoding aarch64_integer_tags_mcsubtag
        __instruction_set A64
        __field uimm6 16 +: 6
        __field op3 14 +: 2
        __field uimm4 10 +: 4
        __field Xn 5 +: 5
        __field Xd 0 +: 5
        __opcode '11010001 10xxxxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Xd);
            integer n = UInt(Xn);
            bits(4) tag_offset = uimm4;
            bits(64) offset = LSL(ZeroExtend(uimm6, 64), LOG2_TAG_GRANULE);
            boolean ADD = FALSE;

    __execute
        bits(64) operand1 = if n == 31 then SP[] else X[n];
        bits(4) start_tag = AllocationTagFromAddress(operand1);
        bits(16) exclude = GCR_EL1.Exclude;
        bits(64) result;
        bits(4) rtag;

        if AllocationTagAccessIsEnabled() then
            rtag = ChooseNonExcludedTag(start_tag, tag_offset, exclude);
        else
            rtag = '0000';

        if ADD then
            (result, -) = AddWithCarry(operand1, offset, '0');
        else
            (result, -) = AddWithCarry(operand1, NOT(offset), '1');

        result = AddressWithAllocationTag(result, rtag);

        if d == 31 then
            SP[] = result;
        else
            X[d] = result;

__instruction aarch64_integer_tags_mcaddtag
    __encoding aarch64_integer_tags_mcaddtag
        __instruction_set A64
        __field uimm6 16 +: 6
        __field op3 14 +: 2
        __field uimm4 10 +: 4
        __field Xn 5 +: 5
        __field Xd 0 +: 5
        __opcode '10010001 10xxxxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Xd);
            integer n = UInt(Xn);
            bits(4) tag_offset = uimm4;
            bits(64) offset = LSL(ZeroExtend(uimm6, 64), LOG2_TAG_GRANULE);
            boolean ADD = TRUE;

    __execute
        bits(64) operand1 = if n == 31 then SP[] else X[n];
        bits(4) start_tag = AllocationTagFromAddress(operand1);
        bits(16) exclude = GCR_EL1.Exclude;
        bits(64) result;
        bits(4) rtag;

        if AllocationTagAccessIsEnabled() then
            rtag = ChooseNonExcludedTag(start_tag, tag_offset, exclude);
        else
            rtag = '0000';

        if ADD then
            (result, -) = AddWithCarry(operand1, offset, '0');
        else
            (result, -) = AddWithCarry(operand1, NOT(offset), '1');

        result = AddressWithAllocationTag(result, rtag);

        if d == 31 then
            SP[] = result;
        else
            X[d] = result;
