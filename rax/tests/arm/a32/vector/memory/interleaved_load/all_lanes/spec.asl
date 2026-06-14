__instruction aarch32_VLD1_a_A
    __encoding aarch32_VLD1_a_T1A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 6 +: 2
        __field T 5 +: 1
        __field a 4 +: 1
        __field Rm 0 +: 4
        __opcode '11110100 1x10xxxx xxxx1100 xxxxxxxx'
        __guard TRUE
        __decode
            if size == '11' || (size == '00' && a == '1') then UNDEFINED;
            ebytes = 1 << UInt(size);  regs = if T == '0' then 1 else 2;
            alignment = if a == '0' then 1 else ebytes;
            d = UInt(D:Vd);  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 || d+regs > 32 then UNPREDICTABLE;

    __encoding aarch32_VLD1_a_T1A1_A
        __instruction_set T32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 6 +: 2
        __field T 5 +: 1
        __field a 4 +: 1
        __field Rm 0 +: 4
        __opcode '11111001 1x10xxxx xxxx1100 xxxxxxxx'
        __guard TRUE
        __decode
            if size == '11' || (size == '00' && a == '1') then UNDEFINED;
            ebytes = 1 << UInt(size);  regs = if T == '0' then 1 else 2;
            alignment = if a == '0' then 1 else ebytes;
            d = UInt(D:Vd);  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 || d+regs > 32 then UNPREDICTABLE;

    __execute __conditional
        CheckAdvSIMDEnabled();
        address = R[n];  iswrite = FALSE;
        - = AArch32.CheckAlignment(address, alignment, AccType_VEC, iswrite);
        bits(64) replicated_element = Replicate(MemU[address,ebytes]);
        for r = 0 to regs-1
            D[d+r] = replicated_element;
        if wback then
            if register_index then
                R[n] = R[n] + R[m];
            else
                R[n] = R[n] + ebytes;

__instruction aarch32_VLD2_a_A
    __encoding aarch32_VLD2_a_T1A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 6 +: 2
        __field T 5 +: 1
        __field a 4 +: 1
        __field Rm 0 +: 4
        __opcode '11110100 1x10xxxx xxxx1101 xxxxxxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            ebytes = 1 << UInt(size);
            alignment = if a == '0' then 1 else 2*ebytes;
            inc = if T == '0' then 1 else 2;
            d = UInt(D:Vd);  d2 = d + inc;  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 || d2 > 31 then UNPREDICTABLE;

    __encoding aarch32_VLD2_a_T1A1_A
        __instruction_set T32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 6 +: 2
        __field T 5 +: 1
        __field a 4 +: 1
        __field Rm 0 +: 4
        __opcode '11111001 1x10xxxx xxxx1101 xxxxxxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            ebytes = 1 << UInt(size);
            alignment = if a == '0' then 1 else 2*ebytes;
            inc = if T == '0' then 1 else 2;
            d = UInt(D:Vd);  d2 = d + inc;  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 || d2 > 31 then UNPREDICTABLE;

    __execute __conditional
        CheckAdvSIMDEnabled();
        address = R[n];  iswrite = FALSE;
        - = AArch32.CheckAlignment(address, alignment, AccType_VEC, iswrite);
        D[d] = Replicate(MemU[address,ebytes]);
        D[d2] = Replicate(MemU[address+ebytes,ebytes]);
        if wback then
            if register_index then
                R[n] = R[n] + R[m];
            else
                R[n] = R[n] + 2*ebytes;

__instruction aarch32_VLD3_a_A
    __encoding aarch32_VLD3_a_T1A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 6 +: 2
        __field T 5 +: 1
        __field a 4 +: 1
        __field Rm 0 +: 4
        __opcode '11110100 1x10xxxx xxxx1110 xxx0xxxx'
        __guard TRUE
        __decode
            if size == '11' || a == '1' then UNDEFINED;
            ebytes = 1 << UInt(size);
            inc = if T == '0' then 1 else 2;
            d = UInt(D:Vd);  d2 = d + inc;  d3 = d2 + inc;  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 || d3 > 31 then UNPREDICTABLE;

    __encoding aarch32_VLD3_a_T1A1_A
        __instruction_set T32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 6 +: 2
        __field T 5 +: 1
        __field a 4 +: 1
        __field Rm 0 +: 4
        __opcode '11111001 1x10xxxx xxxx1110 xxx0xxxx'
        __guard TRUE
        __decode
            if size == '11' || a == '1' then UNDEFINED;
            ebytes = 1 << UInt(size);
            inc = if T == '0' then 1 else 2;
            d = UInt(D:Vd);  d2 = d + inc;  d3 = d2 + inc;  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 || d3 > 31 then UNPREDICTABLE;

    __execute __conditional
        CheckAdvSIMDEnabled();
        address = R[n];
        D[d] = Replicate(MemU[address,ebytes]);
        D[d2] = Replicate(MemU[address+ebytes,ebytes]);
        D[d3] = Replicate(MemU[address+2*ebytes,ebytes]);
        if wback then
            if register_index then
                R[n] = R[n] + R[m];
            else
                R[n] = R[n] + 3*ebytes;

__instruction aarch32_VLD4_a_A
    __encoding aarch32_VLD4_a_T1A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 6 +: 2
        __field T 5 +: 1
        __field a 4 +: 1
        __field Rm 0 +: 4
        __opcode '11110100 1x10xxxx xxxx1111 xxxxxxxx'
        __guard TRUE
        __decode
            if size == '11' && a == '0' then UNDEFINED;
            if size == '11' then
                ebytes = 4;  alignment = 16;
            else
                ebytes = 1 << UInt(size);
                if size == '10' then
                    alignment = if a == '0' then 1 else 8;
                else
                    alignment = if a == '0' then 1 else 4*ebytes;
            inc = if T == '0' then 1 else 2;
            d = UInt(D:Vd);  d2 = d + inc;  d3 = d2 + inc;  d4 = d3 + inc;  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 || d4 > 31 then UNPREDICTABLE;

    __encoding aarch32_VLD4_a_T1A1_A
        __instruction_set T32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 6 +: 2
        __field T 5 +: 1
        __field a 4 +: 1
        __field Rm 0 +: 4
        __opcode '11111001 1x10xxxx xxxx1111 xxxxxxxx'
        __guard TRUE
        __decode
            if size == '11' && a == '0' then UNDEFINED;
            if size == '11' then
                ebytes = 4;  alignment = 16;
            else
                ebytes = 1 << UInt(size);
                if size == '10' then
                    alignment = if a == '0' then 1 else 8;
                else
                    alignment = if a == '0' then 1 else 4*ebytes;
            inc = if T == '0' then 1 else 2;
            d = UInt(D:Vd);  d2 = d + inc;  d3 = d2 + inc;  d4 = d3 + inc;  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 || d4 > 31 then UNPREDICTABLE;

    __execute __conditional
        CheckAdvSIMDEnabled();
        address = R[n];  iswrite = FALSE;
        - = AArch32.CheckAlignment(address, alignment, AccType_VEC, iswrite);
        D[d] = Replicate(MemU[address,ebytes]);
        D[d2] = Replicate(MemU[address+ebytes,ebytes]);
        D[d3] = Replicate(MemU[address+2*ebytes,ebytes]);
        D[d4] = Replicate(MemU[address+3*ebytes,ebytes]);
        if wback then
            if register_index then
                R[n] = R[n] + R[m];
            else
                R[n] = R[n] + 4*ebytes;
