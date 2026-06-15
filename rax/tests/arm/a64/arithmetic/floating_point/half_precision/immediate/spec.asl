__instruction aarch64_vector_fp16_movi
    __encoding aarch64_vector_fp16_movi
        __instruction_set A64
        __field Q 30 +: 1
        __field a 18 +: 1
        __field b 17 +: 1
        __field c 16 +: 1
        __field d 9 +: 1
        __field e 8 +: 1
        __field f 7 +: 1
        __field g 6 +: 1
        __field h 5 +: 1
        __field Rd 0 +: 5
        __opcode '0x001111 00000xxx 111111xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveFP16Ext() then UNDEFINED;

            integer rd = UInt(Rd);

            integer datasize = if Q == '1' then 128 else 64;
            bits(datasize) imm;

            imm8 = a:b:c:d:e:f:g:h;
            imm16 = imm8<7>:NOT(imm8<6>):Replicate(imm8<6>,2):imm8<5:0>:Zeros(6);

            imm = Replicate(imm16, datasize DIV 16);

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

        V[rd] = imm;
