__instruction aarch32_VST1_1_A
    __encoding aarch32_VST1_1_T1A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 10 +: 2
        __field index_align 4 +: 4
        __field Rm 0 +: 4
        __opcode '11110100 1x00xxxx xxxx0000 xxxxxxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            if index_align<0> != '0' then UNDEFINED;
            ebytes = 1;  index = UInt(index_align<3:1>);  alignment = 1;
            d = UInt(D:Vd);  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 then UNPREDICTABLE;

    __encoding aarch32_VST1_1_T2A2_A
        __instruction_set A32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 10 +: 2
        __field index_align 4 +: 4
        __field Rm 0 +: 4
        __opcode '11110100 1x00xxxx xxxx0100 xxxxxxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            if index_align<1> != '0' then UNDEFINED;
            ebytes = 2;  index = UInt(index_align<3:2>);
            alignment = if index_align<0> == '0' then 1 else 2;
            d = UInt(D:Vd);  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 then UNPREDICTABLE;

    __encoding aarch32_VST1_1_T3A3_A
        __instruction_set A32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 10 +: 2
        __field index_align 4 +: 4
        __field Rm 0 +: 4
        __opcode '11110100 1x00xxxx xxxx1000 xxxxxxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            if index_align<2> != '0' then UNDEFINED;
            if index_align<1:0> != '00' && index_align<1:0> != '11' then UNDEFINED;
            ebytes = 4;  index = UInt(index_align<3>);
            alignment = if index_align<1:0> == '00' then 1 else 4;
            d = UInt(D:Vd);  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 then UNPREDICTABLE;

    __encoding aarch32_VST1_1_T1A1_A
        __instruction_set T32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 10 +: 2
        __field index_align 4 +: 4
        __field Rm 0 +: 4
        __opcode '11111001 1x00xxxx xxxx0000 xxxxxxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            if index_align<0> != '0' then UNDEFINED;
            ebytes = 1;  index = UInt(index_align<3:1>);  alignment = 1;
            d = UInt(D:Vd);  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 then UNPREDICTABLE;

    __encoding aarch32_VST1_1_T2A2_A
        __instruction_set T32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 10 +: 2
        __field index_align 4 +: 4
        __field Rm 0 +: 4
        __opcode '11111001 1x00xxxx xxxx0100 xxxxxxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            if index_align<1> != '0' then UNDEFINED;
            ebytes = 2;  index = UInt(index_align<3:2>);
            alignment = if index_align<0> == '0' then 1 else 2;
            d = UInt(D:Vd);  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 then UNPREDICTABLE;

    __encoding aarch32_VST1_1_T3A3_A
        __instruction_set T32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 10 +: 2
        __field index_align 4 +: 4
        __field Rm 0 +: 4
        __opcode '11111001 1x00xxxx xxxx1000 xxxxxxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            if index_align<2> != '0' then UNDEFINED;
            if index_align<1:0> != '00' && index_align<1:0> != '11' then UNDEFINED;
            ebytes = 4;  index = UInt(index_align<3>);
            alignment = if index_align<1:0> == '00' then 1 else 4;
            d = UInt(D:Vd);  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 then UNPREDICTABLE;

    __execute __conditional
        CheckAdvSIMDEnabled();
        address = R[n];  iswrite = TRUE;
        - = AArch32.CheckAlignment(address, alignment, AccType_VEC, iswrite);
        MemU[address,ebytes] = Elem[D[d],index];
        if wback then
            if register_index then
                R[n] = R[n] + R[m];
            else
                R[n] = R[n] + ebytes;

__instruction aarch32_VST2_1_A
    __encoding aarch32_VST2_1_T1A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 10 +: 2
        __field index_align 4 +: 4
        __field Rm 0 +: 4
        __opcode '11110100 1x00xxxx xxxx0001 xxxxxxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            ebytes = 1;  index = UInt(index_align<3:1>);  inc = 1;
            alignment = if index_align<0> == '0' then 1 else 2;
            d = UInt(D:Vd);  d2 = d + inc;  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 || d2 > 31 then UNPREDICTABLE;

    __encoding aarch32_VST2_1_T2A2_A
        __instruction_set A32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 10 +: 2
        __field index_align 4 +: 4
        __field Rm 0 +: 4
        __opcode '11110100 1x00xxxx xxxx0101 xxxxxxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            ebytes = 2;  index = UInt(index_align<3:2>);
            inc = if index_align<1> == '0' then 1 else 2;
            alignment = if index_align<0> == '0' then 1 else 4;
            d = UInt(D:Vd);  d2 = d + inc;  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 || d2 > 31 then UNPREDICTABLE;

    __encoding aarch32_VST2_1_T3A3_A
        __instruction_set A32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 10 +: 2
        __field index_align 4 +: 4
        __field Rm 0 +: 4
        __opcode '11110100 1x00xxxx xxxx1001 xxxxxxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            if index_align<1> != '0' then UNDEFINED;
            ebytes = 4;  index = UInt(index_align<3>);
            inc = if index_align<2> == '0' then 1 else 2;
            alignment = if index_align<0> == '0' then 1 else 8;
            d = UInt(D:Vd);  d2 = d + inc;  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 || d2 > 31 then UNPREDICTABLE;

    __encoding aarch32_VST2_1_T1A1_A
        __instruction_set T32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 10 +: 2
        __field index_align 4 +: 4
        __field Rm 0 +: 4
        __opcode '11111001 1x00xxxx xxxx0001 xxxxxxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            ebytes = 1;  index = UInt(index_align<3:1>);  inc = 1;
            alignment = if index_align<0> == '0' then 1 else 2;
            d = UInt(D:Vd);  d2 = d + inc;  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 || d2 > 31 then UNPREDICTABLE;

    __encoding aarch32_VST2_1_T2A2_A
        __instruction_set T32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 10 +: 2
        __field index_align 4 +: 4
        __field Rm 0 +: 4
        __opcode '11111001 1x00xxxx xxxx0101 xxxxxxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            ebytes = 2;  index = UInt(index_align<3:2>);
            inc = if index_align<1> == '0' then 1 else 2;
            alignment = if index_align<0> == '0' then 1 else 4;
            d = UInt(D:Vd);  d2 = d + inc;  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 || d2 > 31 then UNPREDICTABLE;

    __encoding aarch32_VST2_1_T3A3_A
        __instruction_set T32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 10 +: 2
        __field index_align 4 +: 4
        __field Rm 0 +: 4
        __opcode '11111001 1x00xxxx xxxx1001 xxxxxxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            if index_align<1> != '0' then UNDEFINED;
            ebytes = 4;  index = UInt(index_align<3>);
            inc = if index_align<2> == '0' then 1 else 2;
            alignment = if index_align<0> == '0' then 1 else 8;
            d = UInt(D:Vd);  d2 = d + inc;  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 || d2 > 31 then UNPREDICTABLE;

    __execute __conditional
        CheckAdvSIMDEnabled();
        address = R[n];  iswrite = TRUE;
        - = AArch32.CheckAlignment(address, alignment, AccType_VEC, iswrite);
        MemU[address,       ebytes] = Elem[D[d], index];
        MemU[address+ebytes,ebytes] = Elem[D[d2],index];
        if wback then
            if register_index then
                R[n] = R[n] + R[m];
            else
                R[n] = R[n] + 2*ebytes;

__instruction aarch32_VST3_1_A
    __encoding aarch32_VST3_1_T1A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 10 +: 2
        __field index_align 4 +: 4
        __field Rm 0 +: 4
        __opcode '11110100 1x00xxxx xxxx0010 xxxxxxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            if index_align<0> != '0' then UNDEFINED;
            ebytes = 1;  index = UInt(index_align<3:1>);  inc = 1;
            d = UInt(D:Vd);  d2 = d + inc;  d3 = d2 + inc;  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 || d3 > 31 then UNPREDICTABLE;

    __encoding aarch32_VST3_1_T2A2_A
        __instruction_set A32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 10 +: 2
        __field index_align 4 +: 4
        __field Rm 0 +: 4
        __opcode '11110100 1x00xxxx xxxx0110 xxxxxxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            if index_align<0> != '0' then UNDEFINED;
            ebytes = 2;  index = UInt(index_align<3:2>);
            inc = if index_align<1> == '0' then 1 else 2;
            d = UInt(D:Vd);  d2 = d + inc;  d3 = d2 + inc;  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 || d3 > 31 then UNPREDICTABLE;

    __encoding aarch32_VST3_1_T3A3_A
        __instruction_set A32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 10 +: 2
        __field index_align 4 +: 4
        __field Rm 0 +: 4
        __opcode '11110100 1x00xxxx xxxx1010 xxxxxxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            if index_align<1:0> != '00' then UNDEFINED;
            ebytes = 4;  index = UInt(index_align<3>);
            inc = if index_align<2> == '0' then 1 else 2;
            d = UInt(D:Vd);  d2 = d + inc;  d3 = d2 + inc;  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 || d3 > 31 then UNPREDICTABLE;

    __encoding aarch32_VST3_1_T1A1_A
        __instruction_set T32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 10 +: 2
        __field index_align 4 +: 4
        __field Rm 0 +: 4
        __opcode '11111001 1x00xxxx xxxx0010 xxxxxxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            if index_align<0> != '0' then UNDEFINED;
            ebytes = 1;  index = UInt(index_align<3:1>);  inc = 1;
            d = UInt(D:Vd);  d2 = d + inc;  d3 = d2 + inc;  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 || d3 > 31 then UNPREDICTABLE;

    __encoding aarch32_VST3_1_T2A2_A
        __instruction_set T32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 10 +: 2
        __field index_align 4 +: 4
        __field Rm 0 +: 4
        __opcode '11111001 1x00xxxx xxxx0110 xxxxxxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            if index_align<0> != '0' then UNDEFINED;
            ebytes = 2;  index = UInt(index_align<3:2>);
            inc = if index_align<1> == '0' then 1 else 2;
            d = UInt(D:Vd);  d2 = d + inc;  d3 = d2 + inc;  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 || d3 > 31 then UNPREDICTABLE;

    __encoding aarch32_VST3_1_T3A3_A
        __instruction_set T32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 10 +: 2
        __field index_align 4 +: 4
        __field Rm 0 +: 4
        __opcode '11111001 1x00xxxx xxxx1010 xxxxxxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            if index_align<1:0> != '00' then UNDEFINED;
            ebytes = 4;  index = UInt(index_align<3>);
            inc = if index_align<2> == '0' then 1 else 2;
            d = UInt(D:Vd);  d2 = d + inc;  d3 = d2 + inc;  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 || d3 > 31 then UNPREDICTABLE;

    __execute __conditional
        CheckAdvSIMDEnabled();
        address = R[n];
        MemU[address,         ebytes] = Elem[D[d], index];
        MemU[address+ebytes,  ebytes] = Elem[D[d2],index];
        MemU[address+2*ebytes,ebytes] = Elem[D[d3],index];
        if wback then
            if register_index then
                R[n] = R[n] + R[m];
            else
                R[n] = R[n] + 3*ebytes;

__instruction aarch32_VST4_1_A
    __encoding aarch32_VST4_1_T1A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 10 +: 2
        __field index_align 4 +: 4
        __field Rm 0 +: 4
        __opcode '11110100 1x00xxxx xxxx0011 xxxxxxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            if size != '00' then SEE "Related encodings";
            ebytes = 1;  index = UInt(index_align<3:1>);  inc = 1;
            alignment = if index_align<0> == '0' then 1 else 4;
            d = UInt(D:Vd);  d2 = d + inc;  d3 = d2 + inc;  d4 = d3 + inc;  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 || d4 > 31 then UNPREDICTABLE;

    __encoding aarch32_VST4_1_T2A2_A
        __instruction_set A32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 10 +: 2
        __field index_align 4 +: 4
        __field Rm 0 +: 4
        __opcode '11110100 1x00xxxx xxxx0111 xxxxxxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            if size != '01' then SEE "Related encodings";
            ebytes = 2;  index = UInt(index_align<3:2>);
            inc = if index_align<1> == '0' then 1 else 2;
            alignment = if index_align<0> == '0' then 1 else 8;
            d = UInt(D:Vd);  d2 = d + inc;  d3 = d2 + inc;  d4 = d3 + inc;  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 || d4 > 31 then UNPREDICTABLE;

    __encoding aarch32_VST4_1_T3A3_A
        __instruction_set A32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 10 +: 2
        __field index_align 4 +: 4
        __field Rm 0 +: 4
        __opcode '11110100 1x00xxxx xxxx1011 xxxxxxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            if size != '00' then SEE "Related encodings";
            if index_align<1:0> == '11' then UNDEFINED;
            ebytes = 4;  index = UInt(index_align<3>);
            inc = if index_align<2> == '0' then 1 else 2;
            alignment = if index_align<1:0> == '00' then 1 else 4 << UInt(index_align<1:0>);
            d = UInt(D:Vd);  d2 = d + inc;  d3 = d2 + inc;  d4 = d3 + inc;  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 || d4 > 31 then UNPREDICTABLE;

    __encoding aarch32_VST4_1_T1A1_A
        __instruction_set T32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 10 +: 2
        __field index_align 4 +: 4
        __field Rm 0 +: 4
        __opcode '11111001 1x00xxxx xxxx0011 xxxxxxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            if size != '00' then SEE "Related encodings";
            ebytes = 1;  index = UInt(index_align<3:1>);  inc = 1;
            alignment = if index_align<0> == '0' then 1 else 4;
            d = UInt(D:Vd);  d2 = d + inc;  d3 = d2 + inc;  d4 = d3 + inc;  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 || d4 > 31 then UNPREDICTABLE;

    __encoding aarch32_VST4_1_T2A2_A
        __instruction_set T32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 10 +: 2
        __field index_align 4 +: 4
        __field Rm 0 +: 4
        __opcode '11111001 1x00xxxx xxxx0111 xxxxxxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            if size != '01' then SEE "Related encodings";
            ebytes = 2;  index = UInt(index_align<3:2>);
            inc = if index_align<1> == '0' then 1 else 2;
            alignment = if index_align<0> == '0' then 1 else 8;
            d = UInt(D:Vd);  d2 = d + inc;  d3 = d2 + inc;  d4 = d3 + inc;  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 || d4 > 31 then UNPREDICTABLE;

    __encoding aarch32_VST4_1_T3A3_A
        __instruction_set T32
        __field D 22 +: 1
        __field Rn 16 +: 4
        __field Vd 12 +: 4
        __field size 10 +: 2
        __field index_align 4 +: 4
        __field Rm 0 +: 4
        __opcode '11111001 1x00xxxx xxxx1011 xxxxxxxx'
        __guard TRUE
        __decode
            if size == '11' then UNDEFINED;
            if size != '00' then SEE "Related encodings";
            if index_align<1:0> == '11' then UNDEFINED;
            ebytes = 4;  index = UInt(index_align<3>);
            inc = if index_align<2> == '0' then 1 else 2;
            alignment = if index_align<1:0> == '00' then 1 else 4 << UInt(index_align<1:0>);
            d = UInt(D:Vd);  d2 = d + inc;  d3 = d2 + inc;  d4 = d3 + inc;  n = UInt(Rn);  m = UInt(Rm);
            wback = (m != 15);  register_index = (m != 15 && m != 13);
            if n == 15 || d4 > 31 then UNPREDICTABLE;

    __execute __conditional
        CheckAdvSIMDEnabled();
        address = R[n];  iswrite = TRUE;
        - = AArch32.CheckAlignment(address, alignment, AccType_VEC, iswrite);
        MemU[address,         ebytes] = Elem[D[d], index];
        MemU[address+ebytes,  ebytes] = Elem[D[d2],index];
        MemU[address+2*ebytes,ebytes] = Elem[D[d3],index];
        MemU[address+3*ebytes,ebytes] = Elem[D[d4],index];
        if wback then
            if register_index then
                R[n] = R[n] + R[m];
            else
                R[n] = R[n] + 4*ebytes;
