__instruction aarch64_memory_atomicops_cas_pair
    __encoding aarch64_memory_atomicops_cas_pair
        __instruction_set A64
        __field sz 30 +: 1
        __field L 22 +: 1
        __field Rs 16 +: 5
        __field o0 15 +: 1
        __field Rt2 10 +: 5
        __field Rn 5 +: 5
        __field Rt 0 +: 5
        __opcode '0x001000 0x1xxxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveAtomicExt() then UNDEFINED;
            if Rs<0> == '1' then UNDEFINED;
            if Rt<0> == '1' then UNDEFINED;

            integer n = UInt(Rn);
            integer t = UInt(Rt);
            integer s = UInt(Rs);

            integer datasize = 32 << UInt(sz);
            integer regsize = datasize;
            AccType ldacctype = if L == '1' then AccType_ORDEREDATOMICRW else AccType_ATOMICRW;
            AccType stacctype = if o0 == '1' then AccType_ORDEREDATOMICRW else AccType_ATOMICRW;
            boolean tag_checked = n != 31;

    __execute
        bits(64) address;
        bits(2*datasize) comparevalue;
        bits(2*datasize) newvalue;
        bits(2*datasize) data;

        bits(datasize) s1 = X[s];
        bits(datasize) s2 = X[s+1];
        bits(datasize) t1 = X[t];
        bits(datasize) t2 = X[t+1];
        comparevalue = if BigEndian() then s1:s2 else s2:s1;
        newvalue     = if BigEndian() then t1:t2 else t2:t1;

        if HaveMTEExt() then
            SetNotTagCheckedInstruction(!tag_checked);

        if n == 31 then
            CheckSPAlignment();
            address = SP[];
        else
            address = X[n];

        // All observers in the shareability domain observe the
        // following load and store atomically.
        data =  Mem[address, (2 * datasize) DIV 8, ldacctype];
        if data == comparevalue then
            Mem[address, (2 * datasize) DIV 8, stacctype] = newvalue;

        if BigEndian() then
            X[s]   = ZeroExtend(data<2*datasize-1:datasize>, regsize);
            X[s+1] = ZeroExtend(data<datasize-1:0>, regsize);
        else
            X[s]   = ZeroExtend(data<datasize-1:0>, regsize);
            X[s+1] = ZeroExtend(data<2*datasize-1:datasize>, regsize);
