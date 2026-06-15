__instruction aarch64_vector_arithmetic_binary_uniform_logical_bsl_eor
    __encoding aarch64_vector_arithmetic_binary_uniform_logical_bsl_eor
        __instruction_set A64
        __field Q 30 +: 1
        __field opc2 22 +: 2
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x101110 xx1xxxxx 000111xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer esize = 8;
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;

            VBitOp op;

            case opc2 of
                when '00' op = VBitOp_VEOR;
                when '01' op = VBitOp_VBSL;
                when '10' op = VBitOp_VBIT;
                when '11' op = VBitOp_VBIF;

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand1;
        bits(datasize) operand2;
        bits(datasize) operand3;
        bits(datasize) operand4 = V[n];

        case op of
            when VBitOp_VEOR
                operand1 = V[m];
                operand2 = Zeros();
                operand3 = Ones();
            when VBitOp_VBSL
                operand1 = V[m];
                operand2 = operand1;
                operand3 = V[d];
            when VBitOp_VBIT
                operand1 = V[d];
                operand2 = operand1;
                operand3 = V[m];
            when VBitOp_VBIF
                operand1 = V[d];
                operand2 = operand1;
                operand3 = NOT(V[m]);

        V[d] = operand1 EOR ((operand2 EOR operand4) AND operand3);
