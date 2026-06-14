__instruction aarch32_VMOV_rs_A
    __encoding aarch32_VMOV_rs_T1A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field opc1 21 +: 2
        __field Vd 16 +: 4
        __field Rt 12 +: 4
        __field D 7 +: 1
        __field opc2 5 +: 2
        __opcode 'xxxx1110 0xx0xxxx xxxx1011 xxx1xxxx'
        __guard cond != '1111'
        __decode
            case opc1:opc2 of
                when '1xxx'  advsimd = TRUE;  esize = 8;  index = UInt(opc1<0>:opc2);
                when '0xx1'  advsimd = TRUE;  esize = 16;  index = UInt(opc1<0>:opc2<1>);
                when '0x00'  advsimd = FALSE;  esize = 32;  index = UInt(opc1<0>);
                when '0x10'  UNDEFINED;
            d = UInt(D:Vd);  t = UInt(Rt);
            if t == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __encoding aarch32_VMOV_rs_T1A1_A
        __instruction_set T32
        __field opc1 21 +: 2
        __field Vd 16 +: 4
        __field Rt 12 +: 4
        __field D 7 +: 1
        __field opc2 5 +: 2
        __opcode '11101110 0xx0xxxx xxxx1011 xxx1xxxx'
        __guard TRUE
        __decode
            case opc1:opc2 of
                when '1xxx'  advsimd = TRUE;  esize = 8;  index = UInt(opc1<0>:opc2);
                when '0xx1'  advsimd = TRUE;  esize = 16;  index = UInt(opc1<0>:opc2<1>);
                when '0x00'  advsimd = FALSE;  esize = 32;  index = UInt(opc1<0>);
                when '0x10'  UNDEFINED;
            d = UInt(D:Vd);  t = UInt(Rt);
            if t == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        CheckAdvSIMDOrVFPEnabled(TRUE, advsimd);
        Elem[D[d],index,esize] = R[t]<esize-1:0>;
