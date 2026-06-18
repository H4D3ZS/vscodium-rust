__instruction aarch32_UBFX_A
    __encoding aarch32_UBFX_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field widthm1 16 +: 5
        __field Rd 12 +: 4
        __field lsb 7 +: 5
        __field Rn 0 +: 4
        __opcode 'xxxx0111 111xxxxx xxxxxxxx x101xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);
            lsbit = UInt(lsb);  widthminus1 = UInt(widthm1);
            if d == 15 || n == 15 then UNPREDICTABLE;

    __encoding aarch32_UBFX_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field imm3 12 +: 3
        __field Rd 8 +: 4
        __field imm2 6 +: 2
        __field widthm1 0 +: 5
        __opcode '11110x11 1100xxxx 0xxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  n = UInt(Rn);
            lsbit = UInt(imm3:imm2);  widthminus1 = UInt(widthm1);
            if d == 15 || n == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        msbit = lsbit + widthminus1;
        if msbit <= 31 then
            R[d] = ZeroExtend(R[n]<msbit:lsbit>, 32);
        else
            UNPREDICTABLE;

__instruction aarch32_SBFX_A
    __encoding aarch32_SBFX_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field widthm1 16 +: 5
        __field Rd 12 +: 4
        __field lsb 7 +: 5
        __field Rn 0 +: 4
        __opcode 'xxxx0111 101xxxxx xxxxxxxx x101xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  n = UInt(Rn);
            lsbit = UInt(lsb);  widthminus1 = UInt(widthm1);
            if d == 15 || n == 15 then UNPREDICTABLE;

    __encoding aarch32_SBFX_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field imm3 12 +: 3
        __field Rd 8 +: 4
        __field imm2 6 +: 2
        __field widthm1 0 +: 5
        __opcode '11110x11 0100xxxx 0xxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  n = UInt(Rn);
            lsbit = UInt(imm3:imm2);  widthminus1 = UInt(widthm1);
            if d == 15 || n == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        msbit = lsbit + widthminus1;
        if msbit <= 31 then
            R[d] = SignExtend(R[n]<msbit:lsbit>, 32);
        else
            UNPREDICTABLE;

__instruction aarch32_BFI_A
    __encoding aarch32_BFI_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field msb 16 +: 5
        __field Rd 12 +: 4
        __field lsb 7 +: 5
        __field Rn 0 +: 4
        __opcode 'xxxx0111 110xxxxx xxxxxxxx x001xxxx'
        __guard cond != '1111'
        __decode
            if Rn == '1111' then SEE "BFC";
            d = UInt(Rd);  n = UInt(Rn);  msbit = UInt(msb);  lsbit = UInt(lsb);
            if d == 15 then UNPREDICTABLE;

    __encoding aarch32_BFI_T1_A
        __instruction_set T32
        __field Rn 16 +: 4
        __field imm3 12 +: 3
        __field Rd 8 +: 4
        __field imm2 6 +: 2
        __field msb 0 +: 5
        __opcode '11110x11 0110xxxx 0xxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if Rn == '1111' then SEE "BFC";
            d = UInt(Rd);  n = UInt(Rn);  msbit = UInt(msb);  lsbit = UInt(imm3:imm2);
            if d == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        if msbit >= lsbit then
            R[d]<msbit:lsbit> = R[n]<(msbit-lsbit):0>;
            // Other bits of R[d] are unchanged
        else
            UNPREDICTABLE;

__instruction aarch32_BFC_A
    __encoding aarch32_BFC_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field msb 16 +: 5
        __field Rd 12 +: 4
        __field lsb 7 +: 5
        __opcode 'xxxx0111 110xxxxx xxxxxxxx x0011111'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  msbit = UInt(msb);  lsbit = UInt(lsb);
            if d == 15 then UNPREDICTABLE;

    __encoding aarch32_BFC_T1_A
        __instruction_set T32
        __field imm3 12 +: 3
        __field Rd 8 +: 4
        __field imm2 6 +: 2
        __field msb 0 +: 5
        __opcode '11110x11 01101111 0xxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  msbit = UInt(msb);  lsbit = UInt(imm3:imm2);
            if d == 15 then UNPREDICTABLE;  // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        if msbit >= lsbit then
            R[d]<msbit:lsbit> = Replicate('0', msbit-lsbit+1);
            // Other bits of R[d] are unchanged
        else
            UNPREDICTABLE;
