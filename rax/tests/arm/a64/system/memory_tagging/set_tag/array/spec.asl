__instruction aarch64_integer_tags_mcsettagarray
    __encoding aarch64_integer_tags_mcsettagarray
        __instruction_set A64
        __field Xn 5 +: 5
        __field Xt 0 +: 5
        __opcode '11011001 10100000 000000xx xxxxxxxx'
        __guard TRUE
        __decode
            integer t = UInt(Xt);
            integer n = UInt(Xn);

    __execute
        bits(64) data = X[t];
        bits(64) address;

        if n == 31 then
            CheckSPAlignment();
            address = SP[];
        else
            address = X[n];

        // address<63:LOG2_TAG_GRANULE+4>:Zeros(LOG2_TAG_GRANULE+4) <= start <= address
        // 0 < count <= 16-start<LOG2_TAG_GRANULE+3:LOG2_TAG_GRANULE>
        integer count;
        (address, count) = ImpDefTagArrayStartAndCount(address);

        for i = 0 to count-1
            integer index = UInt(address<LOG2_TAG_GRANULE+3:LOG2_TAG_GRANULE>);
            bits(4) tag = data<(index*4)+3:index*4>;
            MemTag[address] = tag;
            address = address + TAG_GRANULE;

        if n == 31 then
            SP[] = address;
        else
            X[n] = address;
