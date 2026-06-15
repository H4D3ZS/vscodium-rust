__instruction aarch64_vector_logical
    __encoding aarch64_vector_logical
        __instruction_set A64
        __field Q 30 +: 1
        __field op 29 +: 1
        __field a 18 +: 1
        __field b 17 +: 1
        __field c 16 +: 1
        __field cmode 12 +: 4
        __field d 9 +: 1
        __field e 8 +: 1
        __field f 7 +: 1
        __field g 6 +: 1
        __field h 5 +: 1
        __field Rd 0 +: 5
        __opcode '0xx01111 00000xxx xxxx01xx xxxxxxxx'
        __guard TRUE
        __decode
            integer rd = UInt(Rd);

            integer datasize = if Q == '1' then 128 else 64;
            bits(datasize) imm;
            bits(64) imm64;

            ImmediateOp operation;
            case cmode:op of
                when '0xx00' operation = ImmediateOp_MOVI;
                when '0xx01' operation = ImmediateOp_MVNI;
                when '0xx10' operation = ImmediateOp_ORR;
                when '0xx11' operation = ImmediateOp_BIC;
                when '10x00' operation = ImmediateOp_MOVI;
                when '10x01' operation = ImmediateOp_MVNI;
                when '10x10' operation = ImmediateOp_ORR;
                when '10x11' operation = ImmediateOp_BIC;
                when '110x0' operation = ImmediateOp_MOVI;
                when '110x1' operation = ImmediateOp_MVNI;
                when '1110x' operation = ImmediateOp_MOVI;
                when '11110' operation = ImmediateOp_MOVI;
                when '11111'
                    // FMOV Dn,#imm is in main FP instruction set
                    if Q == '0' then UNDEFINED;
                    operation = ImmediateOp_MOVI;

            imm64 = AdvSIMDExpandImm(op, cmode, a:b:c:d:e:f:g:h);
            imm = Replicate(imm64, datasize DIV 64);

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand;
        bits(datasize) result;

        case operation of
            when ImmediateOp_MOVI
                result = imm;
            when ImmediateOp_MVNI
                result = NOT(imm);
            when ImmediateOp_ORR
                operand = V[rd];
                result = operand OR imm;
            when ImmediateOp_BIC
                operand = V[rd];
                result = operand AND NOT(imm);

        V[rd] = result;
