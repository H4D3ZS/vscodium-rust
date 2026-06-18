__instruction aarch32_VMOV_sr_A
    __encoding aarch32_VMOV_sr_T1A1_A
        __instruction_set A32
        __field cond 28 +: 4
        __field U 23 +: 1
        __field opc1 21 +: 2
        __field Vn 16 +: 4
        __field Rt 12 +: 4
        __field N 7 +: 1
        __field opc2 5 +: 2
        __opcode 'xxxx1110 xxx1xxxx xxxx1011 xxx1xxxx'
        __guard cond != '1111'
        __decode
            case U:opc1:opc2 of
                when 'x1xxx'  advsimd = TRUE;  esize = 8;  index = UInt(opc1<0>:opc2);
                when 'x0xx1'  advsimd = TRUE;  esize = 16;  index = UInt(opc1<0>:opc2<1>);
                when '00x00'  advsimd = FALSE;  esize = 32;  index = UInt(opc1<0>);
                when '10x00'  UNDEFINED;
                when 'x0x10'  UNDEFINED;
            t = UInt(Rt);  n = UInt(N:Vn);  unsigned = (U == '1');
            if t == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __encoding aarch32_VMOV_sr_T1A1_A
        __instruction_set T32
        __field U 23 +: 1
        __field opc1 21 +: 2
        __field Vn 16 +: 4
        __field Rt 12 +: 4
        __field N 7 +: 1
        __field opc2 5 +: 2
        __opcode '11101110 xxx1xxxx xxxx1011 xxx1xxxx'
        __guard TRUE
        __decode
            case U:opc1:opc2 of
                when 'x1xxx'  advsimd = TRUE;  esize = 8;  index = UInt(opc1<0>:opc2);
                when 'x0xx1'  advsimd = TRUE;  esize = 16;  index = UInt(opc1<0>:opc2<1>);
                when '00x00'  advsimd = FALSE;  esize = 32;  index = UInt(opc1<0>);
                when '10x00'  UNDEFINED;
                when 'x0x10'  UNDEFINED;
            t = UInt(Rt);  n = UInt(N:Vn);  unsigned = (U == '1');
            if t == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        CheckAdvSIMDOrVFPEnabled(TRUE, advsimd);
        if unsigned then
            R[t] = ZeroExtend(Elem[D[n],index,esize], 32);
        else
            R[t] = SignExtend(Elem[D[n],index,esize], 32);
