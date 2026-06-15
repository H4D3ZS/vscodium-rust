__instruction aarch64_memory_literal_simdfp
    __encoding aarch64_memory_literal_simdfp
        __instruction_set A64
        __field opc 30 +: 2
        __field imm19 5 +: 19
        __field Rt 0 +: 5
        __opcode 'xx011100 xxxxxxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            integer t = UInt(Rt);
            integer size;
            bits(64) offset;

            case opc of
                when '00'
                    size = 4;
                when '01'
                    size = 8;
                when '10'
                    size = 16;
                when '11'
                    UNDEFINED;

            offset = SignExtend(imm19:'00', 64);
            boolean tag_checked = FALSE;

    __execute
        bits(64) address = PC[] + offset;
        bits(size*8) data;

        if HaveMTEExt() then
            SetNotTagCheckedInstruction(!tag_checked);

        CheckFPAdvSIMDEnabled64();

        data = Mem[address, size, AccType_VEC];
        V[t] = data;
