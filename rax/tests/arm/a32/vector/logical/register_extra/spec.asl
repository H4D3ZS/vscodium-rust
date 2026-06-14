__instruction aarch32_VORN_r_A
    __encoding aarch32_VORN_r_T1A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110010 0x11xxxx xxxx0001 xxx1xxxx'
        __guard TRUE
        __decode
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1') then UNDEFINED;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VORN_r_T1A1_A
        __instruction_set T32
        __field D 22 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11101111 0x11xxxx xxxx0001 xxx1xxxx'
        __guard TRUE
        __decode
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1') then UNDEFINED;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __execute __conditional
        CheckAdvSIMDEnabled();
        for r = 0 to regs-1
            D[d+r] = D[n+r] OR NOT(D[m+r]);

__instruction aarch32_VBIF_A
    __encoding aarch32_VBIF_T1A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field op 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 0x11xxxx xxxx0001 xxx1xxxx'
        __guard TRUE
        __decode
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1') then UNDEFINED;
            if op == '00' then SEE "VEOR";
            if op == '01' then operation = VBitOps_VBSL;
            if op == '10' then operation = VBitOps_VBIT;
            if op == '11' then operation = VBitOps_VBIF;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VBIF_T1A1_A
        __instruction_set T32
        __field D 22 +: 1
        __field op 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 0x11xxxx xxxx0001 xxx1xxxx'
        __guard TRUE
        __decode
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1') then UNDEFINED;
            if op == '00' then SEE "VEOR";
            if op == '01' then operation = VBitOps_VBSL;
            if op == '10' then operation = VBitOps_VBIT;
            if op == '11' then operation = VBitOps_VBIF;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __execute
        enumeration VBitOps {VBitOps_VBIF, VBitOps_VBIT, VBitOps_VBSL};

        if ConditionPassed() then
            EncodingSpecificOperations();  CheckAdvSIMDEnabled();
            for r = 0 to regs-1
                case operation of
                    when VBitOps_VBIF  D[d+r] = (D[d+r] AND D[m+r]) OR (D[n+r] AND NOT(D[m+r]));
                    when VBitOps_VBIT  D[d+r] = (D[n+r] AND D[m+r]) OR (D[d+r] AND NOT(D[m+r]));
                    when VBitOps_VBSL  D[d+r] = (D[n+r] AND D[d+r]) OR (D[m+r] AND NOT(D[d+r]));
