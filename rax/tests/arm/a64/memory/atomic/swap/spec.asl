__instruction aarch64_memory_atomicops_swp
    __encoding aarch64_memory_atomicops_swp
        __instruction_set A64
        __field size 30 +: 2
        __field A 23 +: 1
        __field R 22 +: 1
        __field Rs 16 +: 5
        __field Rn 5 +: 5
        __field Rt 0 +: 5
        __opcode 'xx111000 xx1xxxxx 100000xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveAtomicExt() then UNDEFINED;

            integer t = UInt(Rt);
            integer n = UInt(Rn);
            integer s = UInt(Rs);

            integer datasize = 8 << UInt(size);
            integer regsize = if datasize == 64 then 64 else 32;
            AccType ldacctype = if A == '1' && Rt != '11111' then AccType_ORDEREDATOMICRW else AccType_ATOMICRW;
            AccType stacctype = if R == '1' then AccType_ORDEREDATOMICRW else AccType_ATOMICRW;
            boolean tag_checked = n != 31;

    __execute
        bits(64) address;
        bits(datasize) data;

        if HaveMTEExt() then
            SetNotTagCheckedInstruction(!tag_checked);

        if n == 31 then
            CheckSPAlignment();
            address = SP[];
        else
            address = X[n];

        // All observers in the shareability domain observe the
        // following load and store atomically.
        data =  Mem[address, datasize DIV 8, ldacctype];
        Mem[address, datasize DIV 8, stacctype] = X[s];

        X[t] = ZeroExtend(data, regsize);
