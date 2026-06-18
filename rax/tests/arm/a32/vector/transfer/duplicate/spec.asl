__instruction aarch32_VDUP_r_A
    __encoding aarch32_VDUP_r_T1A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field B 22 +: 1
        __field Q 21 +: 1
        __field Vd 16 +: 4
        __field Rt 12 +: 4
        __field D 7 +: 1
        __field E 5 +: 1
        __opcode 'xxxx1110 1xx0xxxx xxxx1011 x0x1xxxx'
        __guard cond != '1111'
        __decode
            if Q == '1' && Vd<0> == '1' then UNDEFINED;
            d = UInt(D:Vd);  t = UInt(Rt);  regs = if Q == '0' then 1 else 2;
            case B:E of
                when '00'  esize = 32;  elements = 2;
                when '01'  esize = 16;  elements = 4;
                when '10'  esize = 8;   elements = 8;
                when '11'  UNDEFINED;
            if t == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __encoding aarch32_VDUP_r_T1A1_A
        __instruction_set T32
        __field B 22 +: 1
        __field Q 21 +: 1
        __field Vd 16 +: 4
        __field Rt 12 +: 4
        __field D 7 +: 1
        __field E 5 +: 1
        __opcode '11101110 1xx0xxxx xxxx1011 x0x1xxxx'
        __guard TRUE
        __decode
            if Q == '1' && Vd<0> == '1' then UNDEFINED;
            d = UInt(D:Vd);  t = UInt(Rt);  regs = if Q == '0' then 1 else 2;
            case B:E of
                when '00'  esize = 32;  elements = 2;
                when '01'  esize = 16;  elements = 4;
                when '10'  esize = 8;   elements = 8;
                when '11'  UNDEFINED;
            if t == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        CheckAdvSIMDEnabled();
        scalar = R[t]<esize-1:0>;
        for r = 0 to regs-1
            for e = 0 to elements-1
                Elem[D[d+r],e,esize] = scalar;

__instruction aarch32_VDUP_s_A
    __encoding aarch32_VDUP_s_T1A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field imm4 16 +: 4
        __field Vd 12 +: 4
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 1x11xxxx xxxx1100 0xx0xxxx'
        __guard TRUE
        __decode
            if imm4 == 'x000' then UNDEFINED;
            if Q == '1' && Vd<0> == '1' then UNDEFINED;
            case imm4 of
                when 'xxx1'  esize = 8;  elements = 8;  index = UInt(imm4<3:1>);
                when 'xx10'  esize = 16;  elements = 4;  index = UInt(imm4<3:2>);
                when 'x100'  esize = 32;  elements = 2;  index = UInt(imm4<3>);
            d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VDUP_s_T1A1_A
        __instruction_set T32
        __field D 22 +: 1
        __field imm4 16 +: 4
        __field Vd 12 +: 4
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 1x11xxxx xxxx1100 0xx0xxxx'
        __guard TRUE
        __decode
            if imm4 == 'x000' then UNDEFINED;
            if Q == '1' && Vd<0> == '1' then UNDEFINED;
            case imm4 of
                when 'xxx1'  esize = 8;  elements = 8;  index = UInt(imm4<3:1>);
                when 'xx10'  esize = 16;  elements = 4;  index = UInt(imm4<3:2>);
                when 'x100'  esize = 32;  elements = 2;  index = UInt(imm4<3>);
            d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __execute __conditional
        CheckAdvSIMDEnabled();
        scalar = Elem[D[m],index,esize];
        for r = 0 to regs-1
            for e = 0 to elements-1
                Elem[D[d+r],e,esize] = scalar;
