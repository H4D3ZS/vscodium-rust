__instruction aarch64_memory_exclusive_pair
    __encoding aarch64_memory_exclusive_pair
        __instruction_set A64
        __field sz 30 +: 1
        __field L 22 +: 1
        __field Rs 16 +: 5
        __field o0 15 +: 1
        __field Rt2 10 +: 5
        __field Rn 5 +: 5
        __field Rt 0 +: 5
        __opcode '1x001000 0x1xxxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            integer n = UInt(Rn);
            integer t = UInt(Rt);
            integer t2 = UInt(Rt2); // ignored by load/store single register
            integer s = UInt(Rs);   // ignored by all loads and store-release

            AccType acctype = if o0 == '1' then AccType_ORDEREDATOMIC else AccType_ATOMIC;
            boolean pair = TRUE;
            MemOp memop = if L == '1' then MemOp_LOAD else MemOp_STORE;
            integer elsize = 32 << UInt(sz);
            integer regsize = if elsize == 64 then 64 else 32;
            integer datasize = if pair then elsize * 2 else elsize;
            boolean tag_checked = n != 31;

    __execute
        bits(64) address;
        bits(datasize) data;
        constant integer dbytes = datasize DIV 8;
        boolean rt_unknown = FALSE;
        boolean rn_unknown = FALSE;

        if HaveMTEExt() then
            SetNotTagCheckedInstruction(!tag_checked);

        if memop == MemOp_LOAD && pair && t == t2 then
            Constraint c = ConstrainUnpredictable(Unpredictable_LDPOVERLAP);
            assert c IN {Constraint_UNKNOWN, Constraint_UNDEF, Constraint_NOP};
            case c of
                when Constraint_UNKNOWN    rt_unknown = TRUE;    // result is UNKNOWN
                when Constraint_UNDEF      UNDEFINED;
                when Constraint_NOP        EndOfInstruction();

        if memop == MemOp_STORE then
            if s == t || (pair && s == t2) then
                Constraint c = ConstrainUnpredictable(Unpredictable_DATAOVERLAP);
                assert c IN {Constraint_UNKNOWN, Constraint_NONE, Constraint_UNDEF, Constraint_NOP};
                case c of
                    when Constraint_UNKNOWN    rt_unknown = TRUE;    // store UNKNOWN value
                    when Constraint_NONE       rt_unknown = FALSE;   // store original value
                    when Constraint_UNDEF      UNDEFINED;
                    when Constraint_NOP        EndOfInstruction();
            if s == n && n != 31 then
                Constraint c = ConstrainUnpredictable(Unpredictable_BASEOVERLAP);
                assert c IN {Constraint_UNKNOWN, Constraint_NONE, Constraint_UNDEF, Constraint_NOP};
                case c of
                    when Constraint_UNKNOWN    rn_unknown = TRUE;    // address is UNKNOWN
                    when Constraint_NONE       rn_unknown = FALSE;   // address is original base
                    when Constraint_UNDEF      UNDEFINED;
                    when Constraint_NOP        EndOfInstruction();

        if n == 31 then
            CheckSPAlignment();
            address = SP[];
        elsif rn_unknown then
            address = bits(64) UNKNOWN;
        else
            address = X[n];

        case memop of
            when MemOp_STORE
                if rt_unknown then
                    data = bits(datasize) UNKNOWN;
                elsif pair then
                    bits(datasize DIV 2) el1 = X[t];
                    bits(datasize DIV 2) el2 = X[t2];
                    data = if BigEndian() then el1 : el2 else el2 : el1;
                else
                    data = X[t];

                bit status = '1';
                // Check whether the Exclusives monitors are set to include the
                // physical memory locations corresponding to virtual address
                // range [address, address+dbytes-1].
                if AArch64.ExclusiveMonitorsPass(address, dbytes) then
                    // This atomic write will be rejected if it does not refer
                    // to the same physical locations after address translation.
                    Mem[address, dbytes, acctype] = data;
                    status = ExclusiveMonitorsStatus();
                X[s] = ZeroExtend(status, 32);

            when MemOp_LOAD
                // Tell the Exclusives monitors to record a sequence of one or more atomic
                // memory reads from virtual address range [address, address+dbytes-1].
                // The Exclusives monitor will only be set if all the reads are from the
                // same dbytes-aligned physical address, to allow for the possibility of
                // an atomicity break if the translation is changed between reads.
                AArch64.SetExclusiveMonitors(address, dbytes);

                if pair then
                    if rt_unknown then
                        // ConstrainedUNPREDICTABLE case
                        X[t]  = bits(datasize) UNKNOWN;
                    elsif elsize == 32 then
                        // 32-bit load exclusive pair (atomic)
                        data = Mem[address, dbytes, acctype];
                        if BigEndian() then
                            X[t]  = data<datasize-1:elsize>;
                            X[t2] = data<elsize-1:0>;
                        else
                            X[t]  = data<elsize-1:0>;
                            X[t2] = data<datasize-1:elsize>;
                    else // elsize == 64
                        // 64-bit load exclusive pair (not atomic),
                        // but must be 128-bit aligned
                        if address != Align(address, dbytes) then
                            iswrite = FALSE;
                            secondstage = FALSE;
                            AArch64.Abort(address, AArch64.AlignmentFault(acctype, iswrite, secondstage));
                        X[t]  = Mem[address + 0, 8, acctype];
                        X[t2] = Mem[address + 8, 8, acctype];
                else
                    data = Mem[address, dbytes, acctype];
                    X[t] = ZeroExtend(data, regsize);
