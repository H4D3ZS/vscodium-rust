__instruction aarch64_memory_single_simdfp_register
    __encoding aarch64_memory_single_simdfp_register
        __instruction_set A64
        __field size 30 +: 2
        __field opc 22 +: 2
        __field Rm 16 +: 5
        __field option 13 +: 3
        __field S 12 +: 1
        __field Rn 5 +: 5
        __field Rt 0 +: 5
        __opcode 'xx111100 xx1xxxxx xxxx10xx xxxxxxxx'
        __guard TRUE
        __decode
            boolean wback = FALSE;
            boolean postindex = FALSE;
            integer scale = UInt(opc<1>:size);
            if scale > 4 then UNDEFINED;
            if option<1> == '0' then UNDEFINED;             // sub-word index
            ExtendType extend_type = DecodeRegExtend(option);
            integer shift = if S == '1' then scale else 0;

    __postdecode
        integer n = UInt(Rn);
        integer t = UInt(Rt);
        integer m = UInt(Rm);
        AccType acctype = AccType_VEC;
        MemOp memop = if opc<0> == '1' then MemOp_LOAD else MemOp_STORE;
        integer datasize = 8 << scale;
        boolean tag_checked = memop != MemOp_PREFETCH;
    __execute
        bits(64) offset = ExtendReg(m, extend_type, shift);
        if HaveMTEExt() then
            SetNotTagCheckedInstruction(!tag_checked);

        CheckFPAdvSIMDEnabled64();
        bits(64) address;
        bits(datasize) data;

        if n == 31 then
            CheckSPAlignment();
            address = SP[];
        else
            address = X[n];

        if ! postindex then
            address = address + offset;

        case memop of
            when MemOp_STORE
                data = V[t];
                Mem[address, datasize DIV 8, acctype] = data;

            when MemOp_LOAD
                data = Mem[address, datasize DIV 8, acctype];
                V[t] = data;

        if wback then
            if postindex then
                address = address + offset;
            if n == 31 then
                SP[] = address;
            else
                X[n] = address;
