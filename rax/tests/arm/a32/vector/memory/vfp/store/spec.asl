__instruction aarch32_VSTR_A
    __encoding aarch32_VSTR_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field U 23 +: 1
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 8 +: 2
        __field imm8 0 +: 8
        __opcode 'xxxx1101 xx00xxxx xxxx10xx xxxxxxxx'
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
            if n == 15 && CurrentInstrSet() != InstrSet_A32 then UNPREDICTABLE;

    __encoding aarch32_VSTR_T1_A
        __instruction_set T32
        __field U 23 +: 1
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 8 +: 2
        __field imm8 0 +: 8
        __opcode '11101101 xx00xxxx xxxx10xx xxxxxxxx'
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
            if n == 15 && CurrentInstrSet() != InstrSet_A32 then UNPREDICTABLE;

    __execute __conditional
        CheckVFPEnabled(TRUE);
        address = if add then (R[n] + imm32) else (R[n] - imm32);
        case esize of
            when 16
                MemA[address,2] = S[d]<15:0>;
            when 32
                MemA[address,4] = S[d];
            when 64
                // Store as two word-aligned words in the correct order for current endianness.
                MemA[address,4]   = if BigEndian() then D[d]<63:32> else D[d]<31:0>;
                MemA[address+4,4] = if BigEndian() then D[d]<31:0>  else D[d]<63:32>;

__instruction aarch32_VSTM_A
    __encoding aarch32_VSTM_T1A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field P 24 +: 1
        __field U 23 +: 1
        __field D 22 +: 1
        __field W 21 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field imm8 1 +: 0
        __opcode 'xxxx110x xxx0xxxx xxxx1011 xxxxxxx1'
        __guard cond != '1111'
        __decode
            if P == '0' && U == '0' && W == '0' then SEE "Related encodings";
            if P == '1' && W == '0' then SEE "VSTR";
            if P == U && W == '1' then UNDEFINED;
            // Remaining combinations are PUW = 010 (IA without !), 011 (IA with !), 101 (DB with !)
            single_regs = FALSE;  add = (U == '1');  wback = (W == '1');
            d = UInt(D:Vd);  n = UInt(Rn);  imm32 = ZeroExtend(imm8:'00', 32);
            regs = UInt(imm8) DIV 2;  // If UInt(imm8) is odd, see "FSTMX".
            if n == 15 && (wback || CurrentInstrSet() != InstrSet_A32) then UNPREDICTABLE;
            if regs == 0 || regs > 16 || (d+regs) > 32 then UNPREDICTABLE;
            if imm8<0> == '1' && (d+regs) > 16 then UNPREDICTABLE;

    __encoding aarch32_VSTM_T1A1_A
        __instruction_set T32
        __field P 24 +: 1
        __field U 23 +: 1
        __field D 22 +: 1
        __field W 21 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field imm8 1 +: 0
        __opcode '1110110x xxx0xxxx xxxx1011 xxxxxxx1'
        __guard TRUE
        __decode
            if P == '0' && U == '0' && W == '0' then SEE "Related encodings";
            if P == '1' && W == '0' then SEE "VSTR";
            if P == U && W == '1' then UNDEFINED;
            // Remaining combinations are PUW = 010 (IA without !), 011 (IA with !), 101 (DB with !)
            single_regs = FALSE;  add = (U == '1');  wback = (W == '1');
            d = UInt(D:Vd);  n = UInt(Rn);  imm32 = ZeroExtend(imm8:'00', 32);
            regs = UInt(imm8) DIV 2;  // If UInt(imm8) is odd, see "FSTMX".
            if n == 15 && (wback || CurrentInstrSet() != InstrSet_A32) then UNPREDICTABLE;
            if regs == 0 || regs > 16 || (d+regs) > 32 then UNPREDICTABLE;
            if imm8<0> == '1' && (d+regs) > 16 then UNPREDICTABLE;

    __execute __conditional
        CheckVFPEnabled(TRUE);
        address = if add then R[n] else R[n]-imm32;
        for r = 0 to regs-1
            if single_regs then
                MemA[address,4] = S[d+r];  address = address+4;
            else
                // Store as two word-aligned words in the correct order for current endianness.
                MemA[address,4] = if BigEndian() then D[d+r]<63:32> else D[d+r]<31:0>;
                MemA[address+4,4] = if BigEndian() then D[d+r]<31:0> else D[d+r]<63:32>;
                address = address+8;
        if wback then R[n] = if add then R[n]+imm32 else R[n]-imm32;
