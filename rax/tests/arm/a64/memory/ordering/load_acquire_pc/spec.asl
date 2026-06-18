__instruction aarch64_memory_ordered_rcpc
    __encoding aarch64_memory_ordered_rcpc
        __instruction_set A64
        __field size 30 +: 2
        __field Rs 16 +: 5
        __field Rn 5 +: 5
        __field Rt 0 +: 5
        __opcode 'xx111000 101xxxxx 110000xx xxxxxxxx'
        __guard TRUE
        __decode
            integer n = UInt(Rn);
            integer t = UInt(Rt);
            integer s = UInt(Rs);   // ignored by all loads and store-release

            AccType acctype = AccType_ORDERED;
            integer elsize = 8 << UInt(size);
            integer regsize = if elsize == 64 then 64 else 32;
            integer datasize = elsize;
            boolean tag_checked = n != 31;

    __execute
        bits(64) address;
        bits(datasize) data;
        constant integer dbytes = datasize DIV 8;

        if HaveMTEExt() then
            SetNotTagCheckedInstruction(!tag_checked);

        if n == 31 then
            CheckSPAlignment();
            address = SP[];
        else
            address = X[n];

        data = Mem[address, dbytes, acctype];
        X[t] = ZeroExtend(data, regsize);
