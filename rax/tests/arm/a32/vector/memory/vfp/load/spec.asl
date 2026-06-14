__instruction aarch32_VLDR_A
    __encoding aarch32_VLDR_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field U 23 +: 1
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 8 +: 2
        __field imm8 0 +: 8
        __opcode 'xxxx1101 xx01xxxx xxxx10xx xxxxxxxx'
        __guard cond != '1111'
        __decode
            if size == '00' || (size == '01' && !HaveFP16Ext()) then UNDEFINED;
            if size == '01' && cond != '1110' then UNPREDICTABLE;
            esize = 8 << UInt(size);  add = (U == '1');
            imm32 = if esize == 16 then ZeroExtend(imm8:'0', 32) else ZeroExtend(imm8:'00', 32);
            case size of
                when '01' d = UInt(Vd:D);
                when '10' d = UInt(Vd:D);
                when '11' d = UInt(D:Vd);
            n = UInt(Rn);

    __encoding aarch32_VLDR_T1_A
        __instruction_set T32
        __field U 23 +: 1
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 8 +: 2
        __field imm8 0 +: 8
        __opcode '11101101 xx01xxxx xxxx10xx xxxxxxxx'
        __guard TRUE
        __decode
            if size == '00' || (size == '01' && !HaveFP16Ext()) then UNDEFINED;
            if size == '01' && InITBlock()  then UNPREDICTABLE;
            esize = 8 << UInt(size);  add = (U == '1');
            imm32 = if esize == 16 then ZeroExtend(imm8:'0', 32) else ZeroExtend(imm8:'00', 32);
            case size of
                when '01' d = UInt(Vd:D);
                when '10' d = UInt(Vd:D);
                when '11' d = UInt(D:Vd);
            n = UInt(Rn);

    __execute __conditional
        CheckVFPEnabled(TRUE);
        base = if n == 15 then Align(PC,4) else R[n];
        address = if add then (base + imm32) else (base - imm32);
        case esize of
            when 16
                S[d] = Zeros(16) : MemA[address,2];
            when 32
                S[d] = MemA[address,4];
            when 64
                word1 = MemA[address,4];  word2 = MemA[address+4,4];
                // Combine the word-aligned words in the correct order for current endianness.
                D[d] = if BigEndian() then word1:word2 else word2:word1;

__instruction aarch32_VLDM_A
    __encoding aarch32_VLDM_T1A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field P 24 +: 1
        __field U 23 +: 1
        __field D 22 +: 1
        __field W 21 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field imm8 1 +: 0
        __opcode 'xxxx110x xxx1xxxx xxxx1011 xxxxxxx0'
        __guard cond != '1111'
        __decode
            if P == '0' && U == '0' && W == '0' then SEE "Related encodings";
            if P == '1' && W == '0' then SEE "VLDR";
            if P == U && W == '1' then UNDEFINED;
            // Remaining combinations are PUW = 010 (IA without !), 011 (IA with !), 101 (DB with !)
            single_regs = FALSE;  add = (U == '1');  wback = (W == '1');
            d = UInt(D:Vd);  n = UInt(Rn);  imm32 = ZeroExtend(imm8:'00', 32);
            regs = UInt(imm8) DIV 2;  // If UInt(imm8) is odd, see "FLDM*X".
            if n == 15 && (wback || CurrentInstrSet() != InstrSet_A32) then UNPREDICTABLE;
            if regs == 0 || regs > 16 || (d+regs) > 32 then UNPREDICTABLE;
            if imm8<0> == '1' && (d+regs) > 16 then UNPREDICTABLE;

    __encoding aarch32_VLDM_T2A2_A
        __instruction_set A32
        __field cond 28 +: 4
        __field P 24 +: 1
        __field U 23 +: 1
        __field D 22 +: 1
        __field W 21 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field imm8 0 +: 8
        __opcode 'xxxx110x xxx1xxxx xxxx1010 xxxxxxxx'
        __guard cond != '1111'
        __decode
            if P == '0' && U == '0' && W == '0' then SEE "Related encodings";
            if P == '1' && W == '0' then SEE "VLDR";
            if P == U && W == '1' then UNDEFINED;
            // Remaining combinations are PUW = 010 (IA without !), 011 (IA with !), 101 (DB with !)
            single_regs = TRUE;  add = (U == '1');  wback = (W == '1');  d = UInt(Vd:D);  n = UInt(Rn);
            imm32 = ZeroExtend(imm8:'00', 32);  regs = UInt(imm8);
            if n == 15 && (wback || CurrentInstrSet() != InstrSet_A32) then UNPREDICTABLE;
            if regs == 0 || (d+regs) > 32 then UNPREDICTABLE;

    __encoding aarch32_VLDM_T1A1_A
        __instruction_set T32
        __field P 24 +: 1
        __field U 23 +: 1
        __field D 22 +: 1
        __field W 21 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field imm8 1 +: 0
        __opcode '1110110x xxx1xxxx xxxx1011 xxxxxxx0'
        __guard TRUE
        __decode
            if P == '0' && U == '0' && W == '0' then SEE "Related encodings";
            if P == '1' && W == '0' then SEE "VLDR";
            if P == U && W == '1' then UNDEFINED;
            // Remaining combinations are PUW = 010 (IA without !), 011 (IA with !), 101 (DB with !)
            single_regs = FALSE;  add = (U == '1');  wback = (W == '1');
            d = UInt(D:Vd);  n = UInt(Rn);  imm32 = ZeroExtend(imm8:'00', 32);
            regs = UInt(imm8) DIV 2;  // If UInt(imm8) is odd, see "FLDM*X".
            if n == 15 && (wback || CurrentInstrSet() != InstrSet_A32) then UNPREDICTABLE;
            if regs == 0 || regs > 16 || (d+regs) > 32 then UNPREDICTABLE;
            if imm8<0> == '1' && (d+regs) > 16 then UNPREDICTABLE;

    __encoding aarch32_VLDM_T2A2_A
        __instruction_set T32
        __field P 24 +: 1
        __field U 23 +: 1
        __field D 22 +: 1
        __field W 21 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field imm8 0 +: 8
        __opcode '1110110x xxx1xxxx xxxx1010 xxxxxxxx'
        __guard TRUE
        __decode
            if P == '0' && U == '0' && W == '0' then SEE "Related encodings";
            if P == '1' && W == '0' then SEE "VLDR";
            if P == U && W == '1' then UNDEFINED;
            // Remaining combinations are PUW = 010 (IA without !), 011 (IA with !), 101 (DB with !)
            single_regs = TRUE;  add = (U == '1');  wback = (W == '1');  d = UInt(Vd:D);  n = UInt(Rn);
            imm32 = ZeroExtend(imm8:'00', 32);  regs = UInt(imm8);
            if n == 15 && (wback || CurrentInstrSet() != InstrSet_A32) then UNPREDICTABLE;
            if regs == 0 || (d+regs) > 32 then UNPREDICTABLE;

    __execute __conditional
        CheckVFPEnabled(TRUE);
        address = if add then R[n] else R[n]-imm32;
        for r = 0 to regs-1
            if single_regs then
                S[d+r] = MemA[address,4];  address = address+4;
            else
                word1 = MemA[address,4];  word2 = MemA[address+4,4];  address = address+8;
                // Combine the word-aligned words in the correct order for current endianness.
                D[d+r] = if BigEndian() then word1:word2 else word2:word1;
        if wback then R[n] = if add then R[n]+imm32 else R[n]-imm32;
