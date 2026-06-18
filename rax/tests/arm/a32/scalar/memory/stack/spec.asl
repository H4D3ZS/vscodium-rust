__instruction aarch32_PUSH_A
    __encoding aarch32_PUSH_T1_A
        __instruction_set T16
        __field M 24 +: 1
        __field register_list 16 +: 8
        __opcode '1011010x xxxxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            registers = '0':M:'000000':register_list;  UnalignedAllowed = FALSE;
            if BitCount(registers) < 1 then UNPREDICTABLE;

    __execute __conditional
        address = SP - 4*BitCount(registers);
        for i = 0 to 14
            if registers<i> == '1' then
                if i == 13 && i != LowestSetBit(registers) then  // Only possible for encoding A1
                    MemA[address,4] = bits(32) UNKNOWN;
                else
                    if UnalignedAllowed then
                        MemU[address,4] = R[i];
                    else
                        MemA[address,4] = R[i];
                address = address + 4;
        if registers<15> == '1' then  // Only possible for encoding A1 or A2
            if UnalignedAllowed then
                MemU[address,4] = PCStoreValue();
            else
                MemA[address,4] = PCStoreValue();
        SP = SP - 4*BitCount(registers);

__instruction aarch32_POP_A
    __encoding aarch32_POP_T1_A
        __instruction_set T16
        __field P 24 +: 1
        __field register_list 16 +: 8
        __opcode '1011110x xxxxxxxx 00000000 00000000'
        __guard TRUE
        __decode
            registers = P:'0000000':register_list;   UnalignedAllowed = FALSE;
            if BitCount(registers) < 1 then UNPREDICTABLE;
            if registers<15> == '1' && InITBlock() && !LastInITBlock() then UNPREDICTABLE;

    __execute __conditional
        address = SP;
        for i = 0 to 14
            if registers<i> == '1' then
                R[i] = if UnalignedAllowed then MemU[address,4] else MemA[address,4];
                address = address + 4;
        if registers<15> == '1' then
            if UnalignedAllowed then
                if address<1:0> == '00' then
                    LoadWritePC(MemU[address,4]);
                else
                    UNPREDICTABLE;
            else
                LoadWritePC(MemA[address,4]);
        if registers<13> == '0' then SP = SP + 4*BitCount(registers);
        if registers<13> == '1' then SP = bits(32) UNKNOWN;
