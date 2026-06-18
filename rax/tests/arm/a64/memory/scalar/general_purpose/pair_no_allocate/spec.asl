__instruction aarch64_memory_pair_general_no_alloc
    __encoding aarch64_memory_pair_general_no_alloc
        __instruction_set A64
        __field opc 30 +: 2
        __field L 22 +: 1
        __field imm7 15 +: 7
        __field Rt2 10 +: 5
        __field Rn 5 +: 5
        __field Rt 0 +: 5
        __opcode 'xx101000 0xxxxxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            boolean wback  = FALSE;
            boolean postindex = FALSE;

    __postdecode
        integer n = UInt(Rn);
        integer t = UInt(Rt);
        integer t2 = UInt(Rt2);
        AccType acctype = AccType_STREAM;
        MemOp memop = if L == '1' then MemOp_LOAD else MemOp_STORE;
        if opc<0> == '1' then UNDEFINED;
        integer scale = 2 + UInt(opc<1>);
        integer datasize = 8 << scale;
        bits(64) offset = LSL(SignExtend(imm7, 64), scale);
        boolean tag_checked = wback || n != 31;
    __execute
        bits(64) address;
        bits(datasize) data1;
        bits(datasize) data2;
        constant integer dbytes = datasize DIV 8;
        boolean rt_unknown = FALSE;

        if HaveMTEExt() then
            SetNotTagCheckedInstruction(!tag_checked);

        if memop == MemOp_LOAD && t == t2 then
            Constraint c = ConstrainUnpredictable(Unpredictable_LDPOVERLAP);
            assert c IN {Constraint_UNKNOWN, Constraint_UNDEF, Constraint_NOP};
            case c of
                when Constraint_UNKNOWN    rt_unknown = TRUE;    // result is UNKNOWN
                when Constraint_UNDEF      UNDEFINED;
                when Constraint_NOP        EndOfInstruction();

        if n == 31 then
            CheckSPAlignment();
            address = SP[];
        else
            address = X[n];

        if ! postindex then
            address = address + offset;

        case memop of
            when MemOp_STORE
                if rt_unknown && t == n then
                    data1 = bits(datasize) UNKNOWN;
                else
                    data1 = X[t];
                if rt_unknown && t2 == n then
                    data2 = bits(datasize) UNKNOWN;
                else
                    data2 = X[t2];
                Mem[address + 0     , dbytes, acctype] = data1;
                Mem[address + dbytes, dbytes, acctype] = data2;

            when MemOp_LOAD
                data1 = Mem[address + 0     , dbytes, acctype];
                data2 = Mem[address + dbytes, dbytes, acctype];
                if rt_unknown then
                    data1 = bits(datasize) UNKNOWN;
                    data2 = bits(datasize) UNKNOWN;
                X[t]  = data1;
                X[t2] = data2;

        if wback then
            if postindex then
                address = address + offset;
            if n == 31 then
                SP[] = address;
            else
                X[n] = address;
