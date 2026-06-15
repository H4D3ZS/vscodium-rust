__instruction aarch32_MOVT_A
    __encoding aarch32_MOVT_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field imm4 16 +: 4
        __field Rd 12 +: 4
        __field imm12 0 +: 12
        __opcode 'xxxx0011 0100xxxx xxxxxxxx xxxxxxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  imm16 = imm4:imm12;
            if d == 15 then UNPREDICTABLE;

    __encoding aarch32_MOVT_T1_A
        __instruction_set T32
        __field i 26 +: 1
        __field imm4 16 +: 4
        __field imm3 12 +: 3
        __field Rd 8 +: 4
        __field imm8 0 +: 8
        __opcode '11110x10 1100xxxx 0xxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  imm16 = imm4:i:imm3:imm8;
            if d == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        R[d]<31:16> = imm16;
        // R[d]<15:0> unchanged
