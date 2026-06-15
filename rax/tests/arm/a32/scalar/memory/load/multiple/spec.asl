__instruction aarch32_LDM_A
    __encoding aarch32_LDM_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field W 21 +: 1
        __field Rn 16 +: 4
        __field register_list 0 +: 16
        __opcode 'xxxx1000 10x1xxxx xxxxxxxx xxxxxxxx'
        __guard cond != '1111'
        __decode
            n = UInt(Rn);  registers = register_list;  wback = (W == '1');
            if n == 15 || BitCount(registers) < 1 then UNPREDICTABLE;
            if wback && registers<n> == '1' then UNPREDICTABLE;

    __encoding aarch32_LDM_T1_A
        __instruction_set T16
        __field Rn 24 +: 3
        __field register_list 16 +: 8
        __opcode '11001xxx xxxxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            n = UInt(Rn);  registers = '00000000':register_list;  wback = (registers<n> == '0');
            if BitCount(registers) < 1 then UNPREDICTABLE;

    __encoding aarch32_LDM_T2_A
        __instruction_set T32
        __field W 21 +: 1
        __field Rn 16 +: 4
        __field P 15 +: 1
        __field M 14 +: 1
        __field register_list 0 +: 14
        __opcode '11101000 10x1xxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            n = UInt(Rn);  registers = P:M:register_list;  wback = (W == '1');
            if n == 15 || BitCount(registers) < 2 || (P == '1' && M == '1') then UNPREDICTABLE;
            if wback && registers<n> == '1' then UNPREDICTABLE;
            if registers<13> == '1' then UNPREDICTABLE;
            if registers<15> == '1' && InITBlock() && !LastInITBlock() then UNPREDICTABLE;

    __execute __conditional
        address = R[n];
        for i = 0 to 14
            if registers<i> == '1' then
                R[i] = MemA[address,4];  address = address + 4;
        if registers<15> == '1' then
            LoadWritePC(MemA[address,4]);
        if wback && registers<n> == '0' then R[n] = R[n] + 4*BitCount(registers);
        if wback && registers<n> == '1' then R[n] = bits(32) UNKNOWN;

__instruction aarch32_LDMDA_A
    __encoding aarch32_LDMDA_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field W 21 +: 1
        __field Rn 16 +: 4
        __field register_list 0 +: 16
        __opcode 'xxxx1000 00x1xxxx xxxxxxxx xxxxxxxx'
        __guard cond != '1111'
        __decode
            n = UInt(Rn);  registers = register_list;  wback = (W == '1');
            if n == 15 || BitCount(registers) < 1 then UNPREDICTABLE;
            if wback && registers<n> == '1' then UNPREDICTABLE;

    __execute __conditional
        address = R[n] - 4*BitCount(registers) + 4;
        for i = 0 to 14
            if registers<i> == '1' then
                R[i] = MemA[address,4];  address = address + 4;
        if registers<15> == '1' then
            LoadWritePC(MemA[address,4]);
        if wback && registers<n> == '0' then R[n] = R[n] - 4*BitCount(registers);
        if wback && registers<n> == '1' then R[n] = bits(32) UNKNOWN;

__instruction aarch32_LDMDB_A
    __encoding aarch32_LDMDB_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field W 21 +: 1
        __field Rn 16 +: 4
        __field register_list 0 +: 16
        __opcode 'xxxx1001 00x1xxxx xxxxxxxx xxxxxxxx'
        __guard cond != '1111'
        __decode
            n = UInt(Rn);  registers = register_list;  wback = (W == '1');
            if n == 15 || BitCount(registers) < 1 then UNPREDICTABLE;
            if wback && registers<n> == '1' then UNPREDICTABLE;

    __encoding aarch32_LDMDB_T1_A
        __instruction_set T32
        __field W 21 +: 1
        __field Rn 16 +: 4
        __field P 15 +: 1
        __field M 14 +: 1
        __field register_list 0 +: 14
        __opcode '11101001 00x1xxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            n = UInt(Rn);  registers = P:M:register_list;  wback = (W == '1');
            if n == 15 || BitCount(registers) < 2 || (P == '1' && M == '1') then UNPREDICTABLE;
            if wback && registers<n> == '1' then UNPREDICTABLE;
            if registers<13> == '1' then UNPREDICTABLE;
            if registers<15> == '1' && InITBlock() && !LastInITBlock() then UNPREDICTABLE;

    __execute __conditional
        address = R[n] - 4*BitCount(registers);
        for i = 0 to 14
            if registers<i> == '1' then
                R[i] = MemA[address,4];  address = address + 4;
        if registers<15> == '1' then
            LoadWritePC(MemA[address,4]);
        if wback && registers<n> == '0' then R[n] = R[n] - 4*BitCount(registers);
        if wback && registers<n> == '1' then R[n] = bits(32) UNKNOWN;

__instruction aarch32_LDMIB_A
    __encoding aarch32_LDMIB_A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field W 21 +: 1
        __field Rn 16 +: 4
        __field register_list 0 +: 16
        __opcode 'xxxx1001 10x1xxxx xxxxxxxx xxxxxxxx'
        __guard cond != '1111'
        __decode
            n = UInt(Rn);  registers = register_list;  wback = (W == '1');
            if n == 15 || BitCount(registers) < 1 then UNPREDICTABLE;
            if wback && registers<n> == '1' then UNPREDICTABLE;

    __execute __conditional
        address = R[n] + 4;
        for i = 0 to 14
            if registers<i> == '1' then
                R[i] = MemA[address,4];  address = address + 4;
        if registers<15> == '1' then
            LoadWritePC(MemA[address,4]);
        if wback && registers<n> == '0' then R[n] = R[n] + 4*BitCount(registers);
        if wback && registers<n> == '1' then R[n] = bits(32) UNKNOWN;
