__instruction aarch32_AESD_A
    __encoding aarch32_AESD_A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 1x11xx00 xxxx0011 01x0xxxx'
        __guard TRUE
        __decode
            if !HaveAESExt() then UNDEFINED;
            if size != '00' then UNDEFINED;
            if Vd<0> == '1' || Vm<0> == '1' then UNDEFINED;
            d = UInt(D:Vd); m = UInt(M:Vm);

    __encoding aarch32_AESD_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 1x11xx00 xxxx0011 01x0xxxx'
        __guard TRUE
        __decode
            if InITBlock() then UNPREDICTABLE;
            if !HaveAESExt() then UNDEFINED;
            if size != '00' then UNDEFINED;
            if Vd<0> == '1' || Vm<0> == '1' then UNDEFINED;
            d = UInt(D:Vd); m = UInt(M:Vm);

    __execute __conditional
        CheckCryptoEnabled32();
        op1 = Q[d>>1]; op2 = Q[m>>1];
        Q[d>>1] = AESInvSubBytes(AESInvShiftRows(op1 EOR op2));

__instruction aarch32_AESE_A
    __encoding aarch32_AESE_A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 1x11xx00 xxxx0011 00x0xxxx'
        __guard TRUE
        __decode
            if !HaveAESExt() then UNDEFINED;
            if size != '00' then UNDEFINED;
            if Vd<0> == '1' || Vm<0> == '1' then UNDEFINED;
            d = UInt(D:Vd); m = UInt(M:Vm);

    __encoding aarch32_AESE_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 1x11xx00 xxxx0011 00x0xxxx'
        __guard TRUE
        __decode
            if InITBlock() then UNPREDICTABLE;
            if !HaveAESExt() then UNDEFINED;
            if size != '00' then UNDEFINED;
            if Vd<0> == '1' || Vm<0> == '1' then UNDEFINED;
            d = UInt(D:Vd); m = UInt(M:Vm);

    __execute __conditional
        CheckCryptoEnabled32();
        op1 = Q[d>>1]; op2 = Q[m>>1];
        Q[d>>1] = AESSubBytes(AESShiftRows(op1 EOR op2));

__instruction aarch32_AESMC_A
    __encoding aarch32_AESMC_A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 1x11xx00 xxxx0011 10x0xxxx'
        __guard TRUE
        __decode
            if !HaveAESExt() then UNDEFINED;
            if size != '00' then UNDEFINED;
            if Vd<0> == '1' || Vm<0> == '1' then UNDEFINED;
            d = UInt(D:Vd); m = UInt(M:Vm);

    __encoding aarch32_AESMC_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 1x11xx00 xxxx0011 10x0xxxx'
        __guard TRUE
        __decode
            if InITBlock() then UNPREDICTABLE;
            if !HaveAESExt() then UNDEFINED;
            if size != '00' then UNDEFINED;
            if Vd<0> == '1' || Vm<0> == '1' then UNDEFINED;
            d = UInt(D:Vd); m = UInt(M:Vm);

    __execute __conditional
        CheckCryptoEnabled32();
        Q[d>>1] = AESMixColumns(Q[m>>1]);

__instruction aarch32_AESIMC_A
    __encoding aarch32_AESIMC_A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 1x11xx00 xxxx0011 11x0xxxx'
        __guard TRUE
        __decode
            if !HaveAESExt() then UNDEFINED;
            if size != '00' then UNDEFINED;
            if Vd<0> == '1' || Vm<0> == '1' then UNDEFINED;
            d = UInt(D:Vd); m = UInt(M:Vm);

    __encoding aarch32_AESIMC_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 1x11xx00 xxxx0011 11x0xxxx'
        __guard TRUE
        __decode
            if InITBlock() then UNPREDICTABLE;
            if !HaveAESExt() then UNDEFINED;
            if size != '00' then UNDEFINED;
            if Vd<0> == '1' || Vm<0> == '1' then UNDEFINED;
            d = UInt(D:Vd); m = UInt(M:Vm);

    __execute __conditional
        CheckCryptoEnabled32();
        Q[d>>1] = AESInvMixColumns(Q[m>>1]);
