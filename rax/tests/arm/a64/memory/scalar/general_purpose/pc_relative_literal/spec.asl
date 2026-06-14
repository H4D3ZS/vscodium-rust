__instruction aarch64_memory_literal_general
    __encoding aarch64_memory_literal_general
        __instruction_set A64
        __field opc 30 +: 2
        __field imm19 5 +: 19
        __field Rt 0 +: 5
        __opcode 'xx011000 xxxxxxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            integer t = UInt(Rt);
            MemOp memop = MemOp_LOAD;
            boolean signed = FALSE;
            integer size;
            bits(64) offset;

            case opc of
                when '00'
                    size = 4;
                when '01'
                    size = 8;
                when '10'
                    size = 4;
                    signed = TRUE;
                when '11'
                    memop = MemOp_PREFETCH;

            offset = SignExtend(imm19:'00', 64);
            boolean tag_checked = FALSE;

    __execute
        bits(64) address = PC[] + offset;
        bits(size*8) data;

        if HaveMTEExt() then
            SetNotTagCheckedInstruction(!tag_checked);

        case memop of
            when MemOp_LOAD
                data = Mem[address, size, AccType_NORMAL];
                if signed then
                    X[t] = SignExtend(data, 64);
                else
                    X[t] = data;

            when MemOp_PREFETCH
                Prefetch(address, t<4:0>);
