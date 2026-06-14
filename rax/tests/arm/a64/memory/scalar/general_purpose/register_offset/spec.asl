__instruction aarch64_memory_single_general_register
    __encoding aarch64_memory_single_general_register
        __instruction_set A64
        __field size 30 +: 2
        __field opc 22 +: 2
        __field Rm 16 +: 5
        __field option 13 +: 3
        __field S 12 +: 1
        __field Rn 5 +: 5
        __field Rt 0 +: 5
        __opcode 'xx111000 xx1xxxxx xxxx10xx xxxxxxxx'
        __guard TRUE
        __decode
            boolean wback = FALSE;
            boolean postindex = FALSE;
            integer scale = UInt(size);
            if option<1> == '0' then UNDEFINED;             // sub-word index
            ExtendType extend_type = DecodeRegExtend(option);
            integer shift = if S == '1' then scale else 0;

    __postdecode
        integer n = UInt(Rn);
        integer t = UInt(Rt);
        integer m = UInt(Rm);
        AccType acctype = AccType_NORMAL;
        MemOp memop;
        boolean signed;
        integer regsize;

        if opc<1> == '0' then
            // store or zero-extending load
            memop = if opc<0> == '1' then MemOp_LOAD else MemOp_STORE;
            regsize = if size == '11' then 64 else 32;
            signed = FALSE;
        else
            if size == '11' then
                memop = MemOp_PREFETCH;
                if opc<0> == '1' then UNDEFINED;
            else
                // sign-extending load
                memop = MemOp_LOAD;
                if size == '10' && opc<0> == '1' then UNDEFINED;
                regsize = if opc<0> == '1' then 32 else 64;
                signed = TRUE;

        integer datasize = 8 << scale;
        boolean tag_checked = memop != MemOp_PREFETCH;
    __execute
        bits(64) offset = ExtendReg(m, extend_type, shift);
        if HaveMTEExt() then
            SetNotTagCheckedInstruction(!tag_checked);

        bits(64) address;
        bits(datasize) data;

        boolean wb_unknown = FALSE;
        boolean rt_unknown = FALSE;

        if memop == MemOp_LOAD && wback && n == t && n != 31 then
            c = ConstrainUnpredictable(Unpredictable_WBOVERLAPLD);
            assert c IN {Constraint_WBSUPPRESS, Constraint_UNKNOWN, Constraint_UNDEF, Constraint_NOP};
            case c of
                when Constraint_WBSUPPRESS wback = FALSE;       // writeback is suppressed
                when Constraint_UNKNOWN    wb_unknown = TRUE;   // writeback is UNKNOWN
                when Constraint_UNDEF      UNDEFINED;
                when Constraint_NOP        EndOfInstruction();

        if memop == MemOp_STORE && wback && n == t && n != 31 then
            c = ConstrainUnpredictable(Unpredictable_WBOVERLAPST);
            assert c IN {Constraint_NONE, Constraint_UNKNOWN, Constraint_UNDEF, Constraint_NOP};
            case c of
                when Constraint_NONE       rt_unknown = FALSE;  // value stored is original value
                when Constraint_UNKNOWN    rt_unknown = TRUE;   // value stored is UNKNOWN
                when Constraint_UNDEF      UNDEFINED;
                when Constraint_NOP        EndOfInstruction();

        if n == 31 then
            if memop != MemOp_PREFETCH then CheckSPAlignment();
            address = SP[];
        else
            address = X[n];

        if ! postindex then
            address = address + offset;

        case memop of
            when MemOp_STORE
                if rt_unknown then
                    data = bits(datasize) UNKNOWN;
                else
                    data = X[t];
                Mem[address, datasize DIV 8, acctype] = data;

            when MemOp_LOAD
                data = Mem[address, datasize DIV 8, acctype];
                if signed then
                    X[t] = SignExtend(data, regsize);
                else
                    X[t] = ZeroExtend(data, regsize);

            when MemOp_PREFETCH
                Prefetch(address, t<4:0>);

        if wback then
            if wb_unknown then
                address = bits(64) UNKNOWN;
            elsif postindex then
                address = address + offset;
            if n == 31 then
                SP[] = address;
            else
                X[n] = address;
