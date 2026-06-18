__instruction aarch64_memory_atomicops_cas_single
    __encoding aarch64_memory_atomicops_cas_single
        __instruction_set A64
        __field size 30 +: 2
        __field L 22 +: 1
        __field Rs 16 +: 5
        __field o0 15 +: 1
        __field Rn 5 +: 5
        __field Rt 0 +: 5
        __opcode 'xx001000 1x1xxxxx x11111xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveAtomicExt() then UNDEFINED;

            integer n = UInt(Rn);
            integer t = UInt(Rt);
            integer s = UInt(Rs);

            integer datasize = 8 << UInt(size);
            integer regsize = if datasize == 64 then 64 else 32;
            AccType ldacctype = if L == '1' then AccType_ORDEREDATOMICRW else AccType_ATOMICRW;
            AccType stacctype = if o0 == '1' then AccType_ORDEREDATOMICRW else AccType_ATOMICRW;
            boolean tag_checked = n != 31;

    __execute
        bits(64) address;
        bits(datasize) comparevalue;
        bits(datasize) newvalue;
        bits(datasize) data;

        if HaveMTEExt() then
            SetNotTagCheckedInstruction(!tag_checked);

        comparevalue = X[s];
        newvalue = X[t];

        if n == 31 then
            CheckSPAlignment();
            address = SP[];
        else
            address = X[n];

        // All observers in the shareability domain observe the
        // following load and store atomically.
        data = Mem[address, datasize DIV 8, ldacctype];
        if data == comparevalue then
            Mem[address, datasize DIV 8, stacctype] = newvalue;

        X[s] = ZeroExtend(data, regsize);
