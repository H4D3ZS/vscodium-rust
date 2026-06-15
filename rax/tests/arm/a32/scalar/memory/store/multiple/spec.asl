__instruction aarch32_STM_A
    __encoding aarch32_STM_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field W 21 +: 1
        __field Rn 16 +: 4
        __field register_list 0 +: 16
        __opcode 'xxxx1000 10x0xxxx xxxxxxxx xxxxxxxx'
        __guard cond != '1111'
        __decode
            n = UInt(Rn);  registers = register_list;  wback = (W == '1');
            if n == 15 || BitCount(registers) < 1 then UNPREDICTABLE;

    __encoding aarch32_STM_T1_A
        __instruction_set T16
        __field Rn 24 +: 3
        __field register_list 16 +: 8
        __opcode '11000xxx xxxxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            n = UInt(Rn);  registers = '00000000':register_list;  wback = TRUE;
            if BitCount(registers) < 1 then UNPREDICTABLE;

    __encoding aarch32_STM_T2_A
        __instruction_set T32
        __field W 21 +: 1
        __field Rn 16 +: 4
        __field P 15 +: 1
        __field M 14 +: 1
        __field register_list 0 +: 14
        __opcode '11101000 10x0xxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            n = UInt(Rn);  registers = P:M:register_list;  wback = (W == '1');
            if n == 15 || BitCount(registers) < 2 then UNPREDICTABLE;
            if wback && registers<n> == '1' then UNPREDICTABLE;
            if registers<13> == '1' then UNPREDICTABLE;
            if registers<15> == '1' then UNPREDICTABLE;

    __execute __conditional
        address = R[n];
        for i = 0 to 14
            if registers<i> == '1' then
                if i == n && wback && i != LowestSetBit(registers) then
                    MemA[address,4] = bits(32) UNKNOWN;  // Only possible for encodings T1 and A1
                else
                    MemA[address,4] = R[i];
                address = address + 4;
        if registers<15> == '1' then  // Only possible for encoding A1
            MemA[address,4] = PCStoreValue();
        if wback then R[n] = R[n] + 4*BitCount(registers);

__instruction aarch32_STMDA_A
    __encoding aarch32_STMDA_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field W 21 +: 1
        __field Rn 16 +: 4
        __field register_list 0 +: 16
        __opcode 'xxxx1000 00x0xxxx xxxxxxxx xxxxxxxx'
        __guard cond != '1111'
        __decode
            n = UInt(Rn);  registers = register_list;  wback = (W == '1');
            if n == 15 || BitCount(registers) < 1 then UNPREDICTABLE;

    __execute __conditional
        address = R[n] - 4*BitCount(registers) + 4;
        for i = 0 to 14
            if registers<i> == '1' then
                if i == n && wback && i != LowestSetBit(registers) then
                    MemA[address,4] = bits(32) UNKNOWN;
                else
                    MemA[address,4] = R[i];
                address = address + 4;
        if registers<15> == '1' then
            MemA[address,4] = PCStoreValue();
        if wback then R[n] = R[n] - 4*BitCount(registers);

__instruction aarch32_STMDB_A
    __encoding aarch32_STMDB_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field W 21 +: 1
        __field Rn 16 +: 4
        __field register_list 0 +: 16
        __opcode 'xxxx1001 00x0xxxx xxxxxxxx xxxxxxxx'
        __guard cond != '1111'
        __decode
            n = UInt(Rn);  registers = register_list;  wback = (W == '1');
            if n == 15 || BitCount(registers) < 1 then UNPREDICTABLE;

    __encoding aarch32_STMDB_T1_A
        __instruction_set T32
        __field W 21 +: 1
        __field Rn 16 +: 4
        __field P 15 +: 1
        __field M 14 +: 1
        __field register_list 0 +: 14
        __opcode '11101001 00x0xxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            n = UInt(Rn);  registers = P:M:register_list;  wback = (W == '1');
            if n == 15 || BitCount(registers) < 2 then UNPREDICTABLE;
            if wback && registers<n> == '1' then UNPREDICTABLE;
            if registers<13> == '1' then UNPREDICTABLE;
            if registers<15> == '1' then UNPREDICTABLE;

    __execute __conditional
        address = R[n] - 4*BitCount(registers);
        for i = 0 to 14
            if registers<i> == '1' then
                if i == n && wback && i != LowestSetBit(registers) then
                    MemA[address,4] = bits(32) UNKNOWN;  // Only possible for encoding A1
                else
                    MemA[address,4] = R[i];
                address = address + 4;
        if registers<15> == '1' then  // Only possible for encoding A1
            MemA[address,4] = PCStoreValue();
        if wback then R[n] = R[n] - 4*BitCount(registers);

__instruction aarch32_STMIB_A
    __encoding aarch32_STMIB_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field W 21 +: 1
        __field Rn 16 +: 4
        __field register_list 0 +: 16
        __opcode 'xxxx1001 10x0xxxx xxxxxxxx xxxxxxxx'
        __guard cond != '1111'
        __decode
            n = UInt(Rn);  registers = register_list;  wback = (W == '1');
            if n == 15 || BitCount(registers) < 1 then UNPREDICTABLE;

    __execute __conditional
        address = R[n] + 4;
        for i = 0 to 14
            if registers<i> == '1' then
                if i == n && wback && i != LowestSetBit(registers) then
                    MemA[address,4] = bits(32) UNKNOWN;
                else
                    MemA[address,4] = R[i];
                address = address + 4;
        if registers<15> == '1' then
            MemA[address,4] = PCStoreValue();
        if wback then R[n] = R[n] + 4*BitCount(registers);
