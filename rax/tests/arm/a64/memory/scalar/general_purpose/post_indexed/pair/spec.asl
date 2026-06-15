__instruction aarch64_memory_pair_general_post_idx
    __encoding aarch64_memory_pair_general_post_idx
        __instruction_set A64
        __field opc 30 +: 2
        __field L 22 +: 1
        __field imm7 15 +: 7
        __field Rt2 10 +: 5
        __field Rn 5 +: 5
        __field Rt 0 +: 5
        __opcode 'xx101000 1xxxxxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            boolean wback  = TRUE;
            boolean postindex = TRUE;

    __encoding aarch64_memory_pair_general_pre_idx
        __instruction_set A64
        __field opc 30 +: 2
        __field L 22 +: 1
        __field imm7 15 +: 7
        __field Rt2 10 +: 5
        __field Rn 5 +: 5
        __field Rt 0 +: 5
        __opcode 'xx101001 1xxxxxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            boolean wback  = TRUE;
            boolean postindex = FALSE;

    __encoding aarch64_memory_pair_general_offset
        __instruction_set A64
        __field opc 30 +: 2
        __field L 22 +: 1
        __field imm7 15 +: 7
        __field Rt2 10 +: 5
        __field Rn 5 +: 5
        __field Rt 0 +: 5
        __opcode 'xx101001 0xxxxxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            boolean wback  = FALSE;
            boolean postindex = FALSE;

    __postdecode
        integer n = UInt(Rn);
        integer t = UInt(Rt);
        integer t2 = UInt(Rt2);
        AccType acctype = AccType_NORMAL;
        MemOp memop = if L == '1' then MemOp_LOAD else MemOp_STORE;
        if L:opc<0> == '01' || opc == '11' then UNDEFINED;
        boolean signed = (opc<0> != '0');
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

        boolean wb_unknown = FALSE;

        if memop == MemOp_LOAD && wback && (t == n || t2 == n) && n != 31 then
            Constraint c = ConstrainUnpredictable(Unpredictable_WBOVERLAPLD);
            assert c IN {Constraint_WBSUPPRESS, Constraint_UNKNOWN, Constraint_UNDEF, Constraint_NOP};
            case c of
                when Constraint_WBSUPPRESS wback = FALSE;        // writeback is suppressed
                when Constraint_UNKNOWN    wb_unknown = TRUE;    // writeback is UNKNOWN
                when Constraint_UNDEF      UNDEFINED;
                when Constraint_NOP        EndOfInstruction();

        if memop == MemOp_STORE && wback && (t == n || t2 == n) && n != 31 then
            Constraint c = ConstrainUnpredictable(Unpredictable_WBOVERLAPST);
            assert c IN {Constraint_NONE, Constraint_UNKNOWN, Constraint_UNDEF, Constraint_NOP};
            case c of
                when Constraint_NONE       rt_unknown = FALSE;   // value stored is pre-writeback
                when Constraint_UNKNOWN    rt_unknown = TRUE;    // value stored is UNKNOWN
                when Constraint_UNDEF      UNDEFINED;
                when Constraint_NOP        EndOfInstruction();

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
                if signed then
                    X[t]  = SignExtend(data1, 64);
                    X[t2] = SignExtend(data2, 64);
                else
                    X[t]  = data1;
                    X[t2] = data2;

        if wback then
            if wb_unknown then
                address = bits(64) UNKNOWN;
            elsif postindex then
                address = address + offset;
            if n == 31 then
                SP[] = address;
            else
                X[n] = address;
