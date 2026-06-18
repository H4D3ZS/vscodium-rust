__instruction aarch32_VST1_m_A
    __encoding aarch32_VST1_m_T1A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 6 +: 2
        __field align 4 +: 2
        __field Rm 0 +: 4
        __opcode '11110100 0x00xxxx xxxx0111 xxxxxxxx'
        __guard TRUE
        __decode
            regs = 1;  if align<1> == '1' then UNDEFINED;
            alignment = if align == '00' then 1 else 4 << UInt(align);
            ebytes = 1 << UInt(size);  elements = 8 DIV ebytes;
            d = UInt(D:Vd);  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 || d+regs > 32 then UNPREDICTABLE;

    __encoding aarch32_VST1_m_T2A2_A
        __instruction_set A32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 6 +: 2
        __field align 4 +: 2
        __field Rm 0 +: 4
        __opcode '11110100 0x00xxxx xxxx1010 xxxxxxxx'
        __guard TRUE
        __decode
            regs = 2;  if align == '11' then UNDEFINED;
            alignment = if align == '00' then 1 else 4 << UInt(align);
            ebytes = 1 << UInt(size);  elements = 8 DIV ebytes;
            d = UInt(D:Vd);  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 || d+regs > 32 then UNPREDICTABLE;

    __encoding aarch32_VST1_m_T3A3_A
        __instruction_set A32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 6 +: 2
        __field align 4 +: 2
        __field Rm 0 +: 4
        __opcode '11110100 0x00xxxx xxxx0110 xxxxxxxx'
        __guard TRUE
        __decode
            regs = 3;  if align<1> == '1' then UNDEFINED;
            alignment = if align == '00' then 1 else 4 << UInt(align);
            ebytes = 1 << UInt(size);  elements = 8 DIV ebytes;
            d = UInt(D:Vd);  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 || d+regs > 32 then UNPREDICTABLE;

    __encoding aarch32_VST1_m_T4A4_A
        __instruction_set A32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 6 +: 2
        __field align 4 +: 2
        __field Rm 0 +: 4
        __opcode '11110100 0x00xxxx xxxx0010 xxxxxxxx'
        __guard TRUE
        __decode
            regs = 4;
            alignment = if align == '00' then 1 else 4 << UInt(align);
            ebytes = 1 << UInt(size);  elements = 8 DIV ebytes;
            d = UInt(D:Vd);  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 || d+regs > 32 then UNPREDICTABLE;

    __encoding aarch32_VST1_m_T1A1_A
        __instruction_set T32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 6 +: 2
        __field align 4 +: 2
        __field Rm 0 +: 4
        __opcode '11111001 0x00xxxx xxxx0111 xxxxxxxx'
        __guard TRUE
        __decode
            regs = 1;  if align<1> == '1' then UNDEFINED;
            alignment = if align == '00' then 1 else 4 << UInt(align);
            ebytes = 1 << UInt(size);  elements = 8 DIV ebytes;
            d = UInt(D:Vd);  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 || d+regs > 32 then UNPREDICTABLE;

    __encoding aarch32_VST1_m_T2A2_A
        __instruction_set T32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 6 +: 2
        __field align 4 +: 2
        __field Rm 0 +: 4
        __opcode '11111001 0x00xxxx xxxx1010 xxxxxxxx'
        __guard TRUE
        __decode
            regs = 2;  if align == '11' then UNDEFINED;
            alignment = if align == '00' then 1 else 4 << UInt(align);
            ebytes = 1 << UInt(size);  elements = 8 DIV ebytes;
            d = UInt(D:Vd);  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 || d+regs > 32 then UNPREDICTABLE;

    __encoding aarch32_VST1_m_T3A3_A
        __instruction_set T32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 6 +: 2
        __field align 4 +: 2
        __field Rm 0 +: 4
        __opcode '11111001 0x00xxxx xxxx0110 xxxxxxxx'
        __guard TRUE
        __decode
            regs = 3;  if align<1> == '1' then UNDEFINED;
            alignment = if align == '00' then 1 else 4 << UInt(align);
            ebytes = 1 << UInt(size);  elements = 8 DIV ebytes;
            d = UInt(D:Vd);  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 || d+regs > 32 then UNPREDICTABLE;

    __encoding aarch32_VST1_m_T4A4_A
        __instruction_set T32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 6 +: 2
        __field align 4 +: 2
        __field Rm 0 +: 4
        __opcode '11111001 0x00xxxx xxxx0010 xxxxxxxx'
        __guard TRUE
        __decode
            regs = 4;
            alignment = if align == '00' then 1 else 4 << UInt(align);
            ebytes = 1 << UInt(size);  elements = 8 DIV ebytes;
            d = UInt(D:Vd);  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 || d+regs > 32 then UNPREDICTABLE;

    __execute __conditional
        CheckAdvSIMDEnabled();
        address = R[n];  iswrite = TRUE;
        - = AArch32.CheckAlignment(address, alignment, AccType_VEC, iswrite);
        for r = 0 to regs-1
            for e = 0 to elements-1
                if ebytes != 8 then
                    MemU[address,ebytes] = Elem[D[d+r],e];
                else
                    - = AArch32.CheckAlignment(address, ebytes, AccType_NORMAL, iswrite);
                    bits(64) data = Elem[D[d+r],e];
                    MemU[address,4] = if BigEndian() then data<63:32> else data<31:0>;
                    MemU[address+4,4] = if BigEndian() then data<31:0> else data<63:32>;
                address = address + ebytes;
        if wback then
            if register_index then
                R[n] = R[n] + R[m];
            else
                R[n] = R[n] + 8*regs;

__instruction aarch32_VST2_m_A
    __encoding aarch32_VST2_m_T1A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field type1 8 +: 4
        __field size 6 +: 2
        __field align 4 +: 2
        __field Rm 0 +: 4
        __opcode '11110100 0x00xxxx xxxx100x xxxxxxxx'
        __guard TRUE
        __decode
            regs = 1;  if align == '11' then UNDEFINED;
            if size == '11' then UNDEFINED;
            inc = if type1 == '1001' then 2 else 1;
            alignment = if align == '00' then 1 else 4 << UInt(align);
            ebytes = 1 << UInt(size);  elements = 8 DIV ebytes;
            d = UInt(D:Vd);  d2 = d + inc;  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 || d2+regs > 32 then UNPREDICTABLE;

    __encoding aarch32_VST2_m_T2A2_A
        __instruction_set A32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 6 +: 2
        __field align 4 +: 2
        __field Rm 0 +: 4
        __opcode '11110100 0x00xxxx xxxx0011 xxxxxxxx'
        __guard TRUE
        __decode
            regs = 2;  inc = 2;
            if size == '11' then UNDEFINED;
            alignment = if align == '00' then 1 else 4 << UInt(align);
            ebytes = 1 << UInt(size);  elements = 8 DIV ebytes;
            d = UInt(D:Vd);  d2 = d + inc;  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 || d2+regs > 32 then UNPREDICTABLE;

    __encoding aarch32_VST2_m_T1A1_A
        __instruction_set T32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field type1 8 +: 4
        __field size 6 +: 2
        __field align 4 +: 2
        __field Rm 0 +: 4
        __opcode '11111001 0x00xxxx xxxx100x xxxxxxxx'
        __guard TRUE
        __decode
            regs = 1;  if align == '11' then UNDEFINED;
            if size == '11' then UNDEFINED;
            inc = if type1 == '1001' then 2 else 1;
            alignment = if align == '00' then 1 else 4 << UInt(align);
            ebytes = 1 << UInt(size);  elements = 8 DIV ebytes;
            d = UInt(D:Vd);  d2 = d + inc;  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 || d2+regs > 32 then UNPREDICTABLE;

    __encoding aarch32_VST2_m_T2A2_A
        __instruction_set T32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 6 +: 2
        __field align 4 +: 2
        __field Rm 0 +: 4
        __opcode '11111001 0x00xxxx xxxx0011 xxxxxxxx'
        __guard TRUE
        __decode
            regs = 2;  inc = 2;
            if size == '11' then UNDEFINED;
            alignment = if align == '00' then 1 else 4 << UInt(align);
            ebytes = 1 << UInt(size);  elements = 8 DIV ebytes;
            d = UInt(D:Vd);  d2 = d + inc;  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 || d2+regs > 32 then UNPREDICTABLE;

    __execute __conditional
        CheckAdvSIMDEnabled();
        address = R[n];  iswrite = TRUE;
        - = AArch32.CheckAlignment(address, alignment, AccType_VEC, iswrite);
        for r = 0 to regs-1
            for e = 0 to elements-1
                MemU[address,       ebytes] = Elem[D[d+r], e];
                MemU[address+ebytes,ebytes] = Elem[D[d2+r],e];
                address = address + 2*ebytes;
        if wback then
            if register_index then
                R[n] = R[n] + R[m];
            else
                R[n] = R[n] + 16*regs;

__instruction aarch32_VST3_m_A
    __encoding aarch32_VST3_m_T1A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field type1 8 +: 4
        __field size 6 +: 2
        __field align 4 +: 2
        __field Rm 0 +: 4
        __opcode '11110100 0x00xxxx xxxx010x xxxxxxxx'
        __guard TRUE
        __decode
            if size == '11' || align<1> == '1' then UNDEFINED;
            case type1 of
                when '0100'
                    inc = 1;
                when '0101'
                    inc = 2;
                otherwise
                    SEE "Related encodings";
            alignment = if align<0> == '0' then 1 else 8;
            ebytes = 1 << UInt(size);  elements = 8 DIV ebytes;
            d = UInt(D:Vd);  d2 = d + inc;  d3 = d2 + inc;  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 || d3 > 31 then UNPREDICTABLE;

    __encoding aarch32_VST3_m_T1A1_A
        __instruction_set T32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field type1 8 +: 4
        __field size 6 +: 2
        __field align 4 +: 2
        __field Rm 0 +: 4
        __opcode '11111001 0x00xxxx xxxx010x xxxxxxxx'
        __guard TRUE
        __decode
            if size == '11' || align<1> == '1' then UNDEFINED;
            case type1 of
                when '0100'
                    inc = 1;
                when '0101'
                    inc = 2;
                otherwise
                    SEE "Related encodings";
            alignment = if align<0> == '0' then 1 else 8;
            ebytes = 1 << UInt(size);  elements = 8 DIV ebytes;
            d = UInt(D:Vd);  d2 = d + inc;  d3 = d2 + inc;  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 || d3 > 31 then UNPREDICTABLE;

    __execute __conditional
        CheckAdvSIMDEnabled();
        address = R[n];  iswrite = TRUE;
        - = AArch32.CheckAlignment(address, alignment, AccType_VEC, iswrite);
        for e = 0 to elements-1
            MemU[address,         ebytes] = Elem[D[d], e];
            MemU[address+ebytes,  ebytes] = Elem[D[d2],e];
            MemU[address+2*ebytes,ebytes] = Elem[D[d3],e];
            address = address + 3*ebytes;
        if wback then
            if register_index then
                R[n] = R[n] + R[m];
            else
                R[n] = R[n] + 24;

__instruction aarch32_VST4_m_A
    __encoding aarch32_VST4_m_T1A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field type1 8 +: 4
        __field size 6 +: 2
        __field align 4 +: 2
        __field Rm 0 +: 4
        __opcode '11110100 0x00xxxx xxxx000x xxxxxxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            case type1 of
                when '0000'
                    inc = 1;
                when '0001'
                    inc = 2;
                otherwise
                    SEE "Related encodings";
            alignment = if align == '00' then 1 else 4 << UInt(align);
            ebytes = 1 << UInt(size);  elements = 8 DIV ebytes;
            d = UInt(D:Vd);  d2 = d + inc;  d3 = d2 + inc;  d4 = d3 + inc;  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 || d4 > 31 then UNPREDICTABLE;

    __encoding aarch32_VST4_m_T1A1_A
        __instruction_set T32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field type1 8 +: 4
        __field size 6 +: 2
        __field align 4 +: 2
        __field Rm 0 +: 4
        __opcode '11111001 0x00xxxx xxxx000x xxxxxxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            case type1 of
                when '0000'
                    inc = 1;
                when '0001'
                    inc = 2;
                otherwise
                    SEE "Related encodings";
            alignment = if align == '00' then 1 else 4 << UInt(align);
            ebytes = 1 << UInt(size);  elements = 8 DIV ebytes;
            d = UInt(D:Vd);  d2 = d + inc;  d3 = d2 + inc;  d4 = d3 + inc;  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 || d4 > 31 then UNPREDICTABLE;

    __execute __conditional
        CheckAdvSIMDEnabled();
        address = R[n];  iswrite = TRUE;
        - = AArch32.CheckAlignment(address, alignment, AccType_VEC, iswrite);
        for e = 0 to elements-1
            MemU[address,         ebytes] = Elem[D[d], e];
            MemU[address+ebytes,  ebytes] = Elem[D[d2],e];
            MemU[address+2*ebytes,ebytes] = Elem[D[d3],e];
            MemU[address+3*ebytes,ebytes] = Elem[D[d4],e];
            address = address + 4*ebytes;
        if wback then
            if register_index then
                R[n] = R[n] + R[m];
            else
                R[n] = R[n] + 32;
