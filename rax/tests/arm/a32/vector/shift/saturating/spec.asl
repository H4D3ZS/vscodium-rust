__instruction aarch32_VQSHL_i_A
    __encoding aarch32_VQSHL_i_T1A1_A
        __instruction_set A32
        __field U 24 +: 1
        __field D 22 +: 1
        __field imm6 16 +: 6
        __field Vd 12 +: 4
        __field op 8 +: 1
        __field L 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '1111001x 1xxxxxxx xxxx011x xxx1xxxx'
        __guard TRUE
        __decode
            if (L:imm6) == '0000xxx' then SEE "Related encodings";
            if U == '0' && op == '0' then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            case L:imm6 of
                when '0001xxx'  esize = 8;  elements = 8;  shift_amount = UInt(imm6) - 8;
                when '001xxxx'  esize = 16;  elements = 4;  shift_amount = UInt(imm6) - 16;
                when '01xxxxx'  esize = 32;  elements = 2;  shift_amount = UInt(imm6) - 32;
                when '1xxxxxx'  esize = 64;  elements = 1;  shift_amount = UInt(imm6);
            src_unsigned = (U == '1' && op == '1');  dest_unsigned = (U == '1');
            d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VQSHL_i_T1A1_A
        __instruction_set T32
        __field U 28 +: 1
        __field D 22 +: 1
        __field imm6 16 +: 6
        __field Vd 12 +: 4
        __field op 8 +: 1
        __field L 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '111x1111 1xxxxxxx xxxx011x xxx1xxxx'
        __guard TRUE
        __decode
            if (L:imm6) == '0000xxx' then SEE "Related encodings";
            if U == '0' && op == '0' then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;
            case L:imm6 of
                when '0001xxx'  esize = 8;  elements = 8;  shift_amount = UInt(imm6) - 8;
                when '001xxxx'  esize = 16;  elements = 4;  shift_amount = UInt(imm6) - 16;
                when '01xxxxx'  esize = 32;  elements = 2;  shift_amount = UInt(imm6) - 32;
                when '1xxxxxx'  esize = 64;  elements = 1;  shift_amount = UInt(imm6);
            src_unsigned = (U == '1' && op == '1');  dest_unsigned = (U == '1');
            d = UInt(D:Vd);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __execute __conditional
        CheckAdvSIMDEnabled();
        for r = 0 to regs-1
            for e = 0 to elements-1
                operand = Int(Elem[D[m+r],e,esize], src_unsigned);
                (result, sat) = SatQ(operand << shift_amount, esize, dest_unsigned);
                Elem[D[d+r],e,esize] = result;
                if sat then FPSCR.QC = '1';

__instruction aarch32_VQSHL_r_A
    __encoding aarch32_VQSHL_r_T1A1_A
        __instruction_set A32
        __field U 24 +: 1
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '1111001x 0xxxxxxx xxxx0100 xxx1xxxx'
        __guard TRUE
        __decode
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1' || Vn<0> == '1') then UNDEFINED;
            unsigned = (U == '1');
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  m = UInt(M:Vm);  n = UInt(N:Vn);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VQSHL_r_T1A1_A
        __instruction_set T32
        __field U 28 +: 1
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '111x1111 0xxxxxxx xxxx0100 xxx1xxxx'
        __guard TRUE
        __decode
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1' || Vn<0> == '1') then UNDEFINED;
            unsigned = (U == '1');
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  m = UInt(M:Vm);  n = UInt(N:Vn);  regs = if Q == '0' then 1 else 2;

    __execute __conditional
        CheckAdvSIMDEnabled();
        for r = 0 to regs-1
            for e = 0 to elements-1
                shift = SInt(Elem[D[n+r],e,esize]<7:0>);
                operand = Int(Elem[D[m+r],e,esize], unsigned);
                (result,sat) = SatQ(operand << shift, esize, unsigned);
                Elem[D[d+r],e,esize] = result;
                if sat then FPSCR.QC = '1';

__instruction aarch32_VQSHRN_A
    __encoding aarch32_VQSHRN_T1A1_A
        __instruction_set A32
        __field U 24 +: 1
        __field D 22 +: 1
        __field imm6 16 +: 6
        __field Vd 12 +: 4
        __field op 8 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '1111001x 1xxxxxxx xxxx100x 00x1xxxx'
        __guard TRUE
        __decode
            if imm6 == '000xxx' then SEE "Related encodings";
            if U == '0' && op == '0' then SEE "VSHRN";
            if Vm<0> == '1' then UNDEFINED;
            case imm6 of
                when '001xxx'  esize = 8;  elements = 8;  shift_amount = 16 - UInt(imm6);
                when '01xxxx'  esize = 16;  elements = 4;  shift_amount = 32 - UInt(imm6);
                when '1xxxxx'  esize = 32;  elements = 2;  shift_amount = 64 - UInt(imm6);
            src_unsigned = (U == '1' && op == '1');  dest_unsigned = (U == '1');
            d = UInt(D:Vd);  m = UInt(M:Vm);

    __encoding aarch32_VQSHRN_T1A1_A
        __instruction_set T32
        __field U 28 +: 1
        __field D 22 +: 1
        __field imm6 16 +: 6
        __field Vd 12 +: 4
        __field op 8 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '111x1111 1xxxxxxx xxxx100x 00x1xxxx'
        __guard TRUE
        __decode
            if imm6 == '000xxx' then SEE "Related encodings";
            if U == '0' && op == '0' then SEE "VSHRN";
            if Vm<0> == '1' then UNDEFINED;
            case imm6 of
                when '001xxx'  esize = 8;  elements = 8;  shift_amount = 16 - UInt(imm6);
                when '01xxxx'  esize = 16;  elements = 4;  shift_amount = 32 - UInt(imm6);
                when '1xxxxx'  esize = 32;  elements = 2;  shift_amount = 64 - UInt(imm6);
            src_unsigned = (U == '1' && op == '1');  dest_unsigned = (U == '1');
            d = UInt(D:Vd);  m = UInt(M:Vm);

    __execute __conditional
        CheckAdvSIMDEnabled();
        for e = 0 to elements-1
            operand = Int(Elem[Qin[m>>1],e,2*esize], src_unsigned);
            (result, sat) = SatQ(operand >> shift_amount, esize, dest_unsigned);
            Elem[D[d],e,esize] = result;
            if sat then FPSCR.QC = '1';

__instruction aarch32_VQRSHRN_A
    __encoding aarch32_VQRSHRN_T1A1_A
        __instruction_set A32
        __field U 24 +: 1
        __field D 22 +: 1
        __field imm6 16 +: 6
        __field Vd 12 +: 4
        __field op 8 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '1111001x 1xxxxxxx xxxx100x 01x1xxxx'
        __guard TRUE
        __decode
            if imm6 == '000xxx' then SEE "Related encodings";
            if U == '0' && op == '0' then SEE "VRSHRN";
            if Vm<0> == '1' then UNDEFINED;
            case imm6 of
                when '001xxx'  esize = 8;  elements = 8;  shift_amount = 16 - UInt(imm6);
                when '01xxxx'  esize = 16;  elements = 4;  shift_amount = 32 - UInt(imm6);
                when '1xxxxx'  esize = 32;  elements = 2;  shift_amount = 64 - UInt(imm6);
            src_unsigned = (U == '1' && op == '1');  dest_unsigned = (U == '1');
            d = UInt(D:Vd);  m = UInt(M:Vm);

    __encoding aarch32_VQRSHRN_T1A1_A
        __instruction_set T32
        __field U 28 +: 1
        __field D 22 +: 1
        __field imm6 16 +: 6
        __field Vd 12 +: 4
        __field op 8 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '111x1111 1xxxxxxx xxxx100x 01x1xxxx'
        __guard TRUE
        __decode
            if imm6 == '000xxx' then SEE "Related encodings";
            if U == '0' && op == '0' then SEE "VRSHRN";
            if Vm<0> == '1' then UNDEFINED;
            case imm6 of
                when '001xxx'  esize = 8;  elements = 8;  shift_amount = 16 - UInt(imm6);
                when '01xxxx'  esize = 16;  elements = 4;  shift_amount = 32 - UInt(imm6);
                when '1xxxxx'  esize = 32;  elements = 2;  shift_amount = 64 - UInt(imm6);
            src_unsigned = (U == '1' && op == '1');  dest_unsigned = (U == '1');
            d = UInt(D:Vd);  m = UInt(M:Vm);

    __execute __conditional
        CheckAdvSIMDEnabled();
        round_const = 1 << (shift_amount - 1);
        for e = 0 to elements-1
            operand = Int(Elem[Qin[m>>1],e,2*esize], src_unsigned);
            (result, sat) = SatQ((operand + round_const) >> shift_amount, esize, dest_unsigned);
            Elem[D[d],e,esize] = result;
            if sat then FPSCR.QC = '1';
