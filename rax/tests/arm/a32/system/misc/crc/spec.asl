__instruction aarch32_CRC32_A
    __encoding aarch32_CRC32_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field sz 21 +: 2
        __field Rn 16 +: 4
        __field Rd 12 +: 4
        __field C 9 +: 1
        __field Rm 0 +: 4
        __opcode 'xxxx0001 0xx0xxxx xxxxxx1x 0100xxxx'
        __guard cond != '1111'
        __decode
            if ! HaveCRCExt() then UNDEFINED;
            d = UInt(Rd); n = UInt(Rn); m = UInt(Rm);
            size = 8 << UInt(sz);
            crc32c = (C == '1');
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE;
            if size == 64 then UNPREDICTABLE;
            if cond != '1110' then UNPREDICTABLE;

    __encoding aarch32_CRC32_T1_A
        __instruction_set T32
        __field C 20 +: 1
        __field Rn 16 +: 4
        __field Rd 8 +: 4
        __field sz 4 +: 2
        __field Rm 0 +: 4
        __opcode '11111010 1101xxxx 1111xxxx 10xxxxxx'
        __guard TRUE
        __decode
            if InITBlock() then UNPREDICTABLE;
            if ! HaveCRCExt() then UNDEFINED;
            d = UInt(Rd); n = UInt(Rn); m = UInt(Rm);
            size = 8 << UInt(sz);
            crc32c = (C == '1');
            if d == 15 || n == 15 || m == 15 then UNPREDICTABLE;
            if size == 64 then UNPREDICTABLE;

    __execute __conditional

        acc = R[n];             // accumulator
        val = R[m]<size-1:0>;   // input value
        poly = (if crc32c then 0x1EDC6F41 else 0x04C11DB7)<31:0>;
        tempacc = BitReverse(acc):Zeros(size);
        tempval = BitReverse(val):Zeros(32);
        // Poly32Mod2 on a bitstring does a polynomial Modulus over {0,1} operation
        R[d] = BitReverse(Poly32Mod2(tempacc EOR tempval, poly));
