__instruction aarch32_VDOT_A
    __encoding aarch32_VDOT_A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field U 4 +: 1
        __field Vm 0 +: 4
        __opcode '11111100 0x10xxxx xxxx1101 xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveDOTPExt() then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1') then UNDEFINED;
            boolean signed = U=='0';
            integer d = UInt(D:Vd);
            integer n = UInt(N:Vn);
            integer m = UInt(M:Vm);
            integer esize = 32;
            integer regs = if Q == '1' then 2 else 1;

    __encoding aarch32_VDOT_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field U 4 +: 1
        __field Vm 0 +: 4
        __opcode '11111100 0x10xxxx xxxx1101 xxx0xxxx'
        __guard TRUE
        __decode
            if InITBlock() then UNPREDICTABLE;
            if !HaveDOTPExt() then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1') then UNDEFINED;
            boolean signed = U=='0';
            integer d = UInt(D:Vd);
            integer n = UInt(N:Vn);
            integer m = UInt(M:Vm);
            integer esize = 32;
            integer regs = if Q == '1' then 2 else 1;

    __execute
        bits(64) operand1;
        bits(64) operand2;
        bits(64) result;
        CheckAdvSIMDEnabled();
        for r = 0 to regs-1
            operand1 = D[n+r];
            operand2 = D[m+r];
            result = D[d+r];
            integer element1, element2;
            for e = 0 to 1
                integer res = 0;
                for i = 0 to 3
                    if signed then
                        element1 = SInt(Elem[operand1, 4 * e + i, esize DIV 4]);
                        element2 = SInt(Elem[operand2, 4 * e + i, esize DIV 4]);
                    else
                        element1 = UInt(Elem[operand1, 4 * e + i, esize DIV 4]);
                        element2 = UInt(Elem[operand2, 4 * e + i, esize DIV 4]);
                    res = res + element1 * element2;
                Elem[result, e, esize] = Elem[result, e, esize] + res;
            D[d+r] = result;

__instruction aarch32_VDOT_s_A
    __encoding aarch32_VDOT_s_A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field U 4 +: 1
        __field Vm 0 +: 4
        __opcode '11111110 0x10xxxx xxxx1101 xxx0xxxx'
        __guard TRUE
        __decode
            if !HaveDOTPExt() then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1') then UNDEFINED;
            boolean signed = (U=='0');
            integer d = UInt(D:Vd);
            integer n = UInt(N:Vn);
            integer m = UInt(Vm<3:0>);
            integer index = UInt(M);
            integer esize = 32;
            integer regs = if Q == '1' then 2 else 1;

    __encoding aarch32_VDOT_s_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field U 4 +: 1
        __field Vm 0 +: 4
        __opcode '11111110 0x10xxxx xxxx1101 xxx0xxxx'
        __guard TRUE
        __decode
            if InITBlock() then UNPREDICTABLE;
            if !HaveDOTPExt() then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1') then UNDEFINED;
            boolean signed = (U=='0');
            integer d = UInt(D:Vd);
            integer n = UInt(N:Vn);
            integer m = UInt(Vm<3:0>);
            integer index = UInt(M);
            integer esize = 32;
            integer regs = if Q == '1' then 2 else 1;

    __execute
        bits(64) operand1;
        bits(64) operand2 = D[m];
        bits(64) result;
        CheckAdvSIMDEnabled();
        for r = 0 to regs-1
            operand1 = D[n+r];
            result = D[d+r];
            integer element1, element2;
            for e = 0 to 1
                integer res = 0;
                for i = 0 to 3
                    if signed then
                        element1 = SInt(Elem[operand1, 4 * e + i, esize DIV 4]);
                        element2 = SInt(Elem[operand2, 4 * index + i, esize DIV 4]);
                    else
                        element1 = UInt(Elem[operand1, 4 * e + i, esize DIV 4]);
                        element2 = UInt(Elem[operand2, 4 * index + i, esize DIV 4]);
                    res = res + element1 * element2;
                Elem[result, e, esize] = Elem[result, e, esize] + res;
            D[d+r] = result;
