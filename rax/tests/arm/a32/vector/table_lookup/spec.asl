__instruction aarch32_VTBL_A
    __encoding aarch32_VTBL_T1A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field len 8 +: 2
        __field N 7 +: 1
        __field op 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 1x11xxxx xxxx10xx xxx0xxxx'
        __guard TRUE
        __decode
            is_vtbl = (op == '0');  length = UInt(len)+1;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);
            if n+length > 32 then UNPREDICTABLE;

    __encoding aarch32_VTBL_T1A1_A
        __instruction_set T32
        __field D 22 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field len 8 +: 2
        __field N 7 +: 1
        __field op 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 1x11xxxx xxxx10xx xxx0xxxx'
        __guard TRUE
        __decode
            is_vtbl = (op == '0');  length = UInt(len)+1;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);
            if n+length > 32 then UNPREDICTABLE;

    __execute __conditional
        CheckAdvSIMDEnabled();

        // Create 256-bit = 32-byte table variable, with zeros in entries that will not be used.
        table3 = if length == 4 then D[n+3] else Zeros(64);
        table2 = if length >= 3 then D[n+2] else Zeros(64);
        table1 = if length >= 2 then D[n+1] else Zeros(64);
        table = table3 : table2 : table1 : D[n];

        for i = 0 to 7
            index = UInt(Elem[D[m],i,8]);
            if index < 8*length then
                Elem[D[d],i,8] = Elem[table,index,8];
            else
                if is_vtbl then
                    Elem[D[d],i,8] = Zeros(8);
                // else Elem[D[d],i,8] unchanged
