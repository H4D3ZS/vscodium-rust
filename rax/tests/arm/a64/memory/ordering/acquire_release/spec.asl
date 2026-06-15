__instruction aarch64_memory_ordered
    __encoding aarch64_memory_ordered
        __instruction_set A64
        __field size 30 +: 2
        __field L 22 +: 1
        __field Rs 16 +: 5
        __field o0 15 +: 1
        __field Rt2 10 +: 5
        __field Rn 5 +: 5
        __field Rt 0 +: 5
        __opcode 'xx001000 1x0xxxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            integer n = UInt(Rn);
            integer t = UInt(Rt);
            integer t2 = UInt(Rt2); // ignored by load/store single register
            integer s = UInt(Rs);   // ignored by all loads and store-release

            AccType acctype = if o0 == '0' then AccType_LIMITEDORDERED else AccType_ORDERED;
            MemOp memop = if L == '1' then MemOp_LOAD else MemOp_STORE;
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

        case memop of
            when MemOp_STORE
                data = X[t];
                Mem[address, dbytes, acctype] = data;

            when MemOp_LOAD
                data = Mem[address, dbytes, acctype];
                X[t] = ZeroExtend(data, regsize);
