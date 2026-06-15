__instruction aarch64_memory_single_simdfp_immediate_signed_offset_normal
    __encoding aarch64_memory_single_simdfp_immediate_signed_offset_normal
        __instruction_set A64
        __field size 30 +: 2
        __field opc 22 +: 2
        __field imm9 12 +: 9
        __field Rn 5 +: 5
        __field Rt 0 +: 5
        __opcode 'xx111100 xx0xxxxx xxxx00xx xxxxxxxx'
        __guard TRUE
        __decode
            boolean wback = FALSE;
            boolean postindex = FALSE;
            integer scale = UInt(opc<1>:size);
            if scale > 4 then UNDEFINED;
            bits(64) offset = SignExtend(imm9, 64);

    __postdecode
        integer n = UInt(Rn);
        integer t = UInt(Rt);
        AccType acctype = AccType_VEC;
        MemOp memop = if opc<0> == '1' then MemOp_LOAD else MemOp_STORE;
        integer datasize = 8 << scale;
        boolean tag_checked = memop != MemOp_PREFETCH && (wback || n != 31);
    __execute
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
