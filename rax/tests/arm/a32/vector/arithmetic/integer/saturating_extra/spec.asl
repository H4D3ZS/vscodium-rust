__instruction aarch32_VQDMULL_A
    __encoding aarch32_VQDMULL_T1A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110010 1xxxxxxx xxxx1101 x0x0xxxx'
        __guard TRUE
        __decode
            if size == '11' then SEE "Related encodings";
            if size == '00' || Vd<0> == '1' then UNDEFINED;
            scalar_form = FALSE;  d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);
            esize = 8 << UInt(size);  elements = 64 DIV esize;

    __encoding aarch32_VQDMULL_T2A2_A
        __instruction_set A32
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110010 1xxxxxxx xxxx1011 x1x0xxxx'
        __guard TRUE
        __decode
            if size == '11' then SEE "Related encodings";
            if size == '00' || Vd<0> == '1' then UNDEFINED;
            scalar_form = TRUE;  d = UInt(D:Vd);  n = UInt(N:Vn);
            if size == '01' then esize = 16;  elements = 4;  m = UInt(Vm<2:0>);  index = UInt(M:Vm<3>);
            if size == '10' then esize = 32;  elements = 2;  m = UInt(Vm);  index = UInt(M);

    __encoding aarch32_VQDMULL_T1A1_A
        __instruction_set T32
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11101111 1xxxxxxx xxxx1101 x0x0xxxx'
        __guard TRUE
        __decode
            if size == '11' then SEE "Related encodings";
            if size == '00' || Vd<0> == '1' then UNDEFINED;
            scalar_form = FALSE;  d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);
            esize = 8 << UInt(size);  elements = 64 DIV esize;

    __encoding aarch32_VQDMULL_T2A2_A
        __instruction_set T32
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11101111 1xxxxxxx xxxx1011 x1x0xxxx'
        __guard TRUE
        __decode
            if size == '11' then SEE "Related encodings";
            if size == '00' || Vd<0> == '1' then UNDEFINED;
            scalar_form = TRUE;  d = UInt(D:Vd);  n = UInt(N:Vn);
            if size == '01' then esize = 16;  elements = 4;  m = UInt(Vm<2:0>);  index = UInt(M:Vm<3>);
            if size == '10' then esize = 32;  elements = 2;  m = UInt(Vm);  index = UInt(M);

    __execute __conditional
        CheckAdvSIMDEnabled();
        if scalar_form then op2 = SInt(Elem[Din[m],index,esize]);
        for e = 0 to elements-1
            if !scalar_form then op2 = SInt(Elem[Din[m],e,esize]);
            op1 = SInt(Elem[Din[n],e,esize]);
            // The following only saturates if both op1 and op2 equal -(2^(esize-1))
            (product, sat) = SignedSatQ(2*op1*op2, 2*esize);
            Elem[Q[d>>1],e,2*esize] = product;
            if sat then FPSCR.QC = '1';

__instruction aarch32_VQDMLAL_A
    __encoding aarch32_VQDMLAL_T1A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field op 9 +: 1
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110010 1xxxxxxx xxxx1001 x0x0xxxx'
        __guard TRUE
        __decode
            if size == '11' then SEE "Related encodings";
            if size == '00' || Vd<0> == '1' then UNDEFINED;
            add = (op == '0');
            scalar_form = FALSE;  d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);
            esize = 8 << UInt(size);  elements = 64 DIV esize;

    __encoding aarch32_VQDMLAL_T2A2_A
        __instruction_set A32
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field op 10 +: 1
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110010 1xxxxxxx xxxx0011 x1x0xxxx'
        __guard TRUE
        __decode
            if size == '11' then SEE "Related encodings";
            if size == '00' || Vd<0> == '1' then UNDEFINED;
            add = (op == '0');
            scalar_form = TRUE;  d = UInt(D:Vd);  n = UInt(N:Vn);
            if size == '01' then esize = 16;  elements = 4;  m = UInt(Vm<2:0>);  index = UInt(M:Vm<3>);
            if size == '10' then esize = 32;  elements = 2;  m = UInt(Vm);  index = UInt(M);

    __encoding aarch32_VQDMLAL_T1A1_A
        __instruction_set T32
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field op 8 +: 1
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11101111 1xxxxxxx xxxx1001 x0x0xxxx'
        __guard TRUE
        __decode
            if size == '11' then SEE "Related encodings";
            if size == '00' || Vd<0> == '1' then UNDEFINED;
            add = (op == '0');
            scalar_form = FALSE;  d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);
            esize = 8 << UInt(size);  elements = 64 DIV esize;

    __encoding aarch32_VQDMLAL_T2A2_A
        __instruction_set T32
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field op 10 +: 1
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11101111 1xxxxxxx xxxx0011 x1x0xxxx'
        __guard TRUE
        __decode
            if size == '11' then SEE "Related encodings";
            if size == '00' || Vd<0> == '1' then UNDEFINED;
            add = (op == '0');
            scalar_form = TRUE;  d = UInt(D:Vd);  n = UInt(N:Vn);
            if size == '01' then esize = 16;  elements = 4;  m = UInt(Vm<2:0>);  index = UInt(M:Vm<3>);
            if size == '10' then esize = 32;  elements = 2;  m = UInt(Vm);  index = UInt(M);

    __execute __conditional
        CheckAdvSIMDEnabled();
        if scalar_form then op2 = SInt(Elem[Din[m],index,esize]);
        for e = 0 to elements-1
            if !scalar_form then op2 = SInt(Elem[Din[m],e,esize]);
            op1 = SInt(Elem[Din[n],e,esize]);
            // The following only saturates if both op1 and op2 equal -(2^(esize-1))
            (product, sat1) = SignedSatQ(2*op1*op2, 2*esize);
            if add then
                result = SInt(Elem[Qin[d>>1],e,2*esize]) + SInt(product);
            else
                result = SInt(Elem[Qin[d>>1],e,2*esize]) - SInt(product);
            (Elem[Q[d>>1],e,2*esize], sat2) = SignedSatQ(result, 2*esize);
            if sat1 || sat2 then FPSCR.QC = '1';

__instruction aarch32_VQRDMLAH_A
    __encoding aarch32_VQRDMLAH_A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 0xxxxxxx xxxx1011 xxx1xxxx'
        __guard TRUE
        __decode
            if !HaveQRDMLAHExt() then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1') then UNDEFINED;
            if size == '00' || size == '11' then UNDEFINED;
            add = TRUE;  scalar_form = FALSE;  esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VQRDMLAH_A2_A
        __instruction_set A32
        __field Q 24 +: 1
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '1111001x 1xxxxxxx xxxx1110 x1x0xxxx'
        __guard TRUE
        __decode
            if !HaveQRDMLAHExt() then UNDEFINED;
            if size == '11' then SEE "Related encodings";
            if size == '00' then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1') then UNDEFINED;
            add = TRUE;  scalar_form = TRUE;  d = UInt(D:Vd);  n = UInt(N:Vn);  regs = if Q == '0' then 1 else 2;
            if size == '01' then esize = 16;  elements = 4;  m = UInt(Vm<2:0>);  index = UInt(M:Vm<3>);
            if size == '10' then esize = 32;  elements = 2;  m = UInt(Vm);  index = UInt(M);

    __encoding aarch32_VQRDMLAH_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 0xxxxxxx xxxx1011 xxx1xxxx'
        __guard TRUE
        __decode
            if !HaveQRDMLAHExt() then UNDEFINED;
            if InITBlock() then UNPREDICTABLE;
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1') then UNDEFINED;
            if size == '00' || size == '11' then UNDEFINED;
            add = TRUE;  scalar_form = FALSE;  esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VQRDMLAH_T2_A
        __instruction_set T32
        __field Q 28 +: 1
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '111x1111 1xxxxxxx xxxx1110 x1x0xxxx'
        __guard TRUE
        __decode
            if !HaveQRDMLAHExt() then UNDEFINED;
            if InITBlock() then UNPREDICTABLE;
            if size == '11' then SEE "Related encodings";
            if size == '00' then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1') then UNDEFINED;
            add = TRUE;  scalar_form = TRUE;  d = UInt(D:Vd);  n = UInt(N:Vn);  regs = if Q == '0' then 1 else 2;
            if size == '01' then esize = 16;  elements = 4;  m = UInt(Vm<2:0>);  index = UInt(M:Vm<3>);
            if size == '10' then esize = 32;  elements = 2;  m = UInt(Vm);  index = UInt(M);

    __execute
        CheckAdvSIMDEnabled();
        round_const = 1 << (esize-1);
        if scalar_form then op2 = SInt(Elem[D[m],index,esize]);
        for r = 0 to regs-1
            for e = 0 to elements-1
                op1 = SInt(Elem[D[n+r],e,esize]);
                op3 = SInt(Elem[D[d+r],e,esize]) << esize;
                if !scalar_form then op2 = SInt(Elem[D[m+r],e,esize]);
                (result, sat) = SignedSatQ((op3 + 2*(op1*op2) + round_const) >> esize, esize);
                Elem[D[d+r],e,esize] = result;
                if sat then FPSCR.QC = '1';

__instruction aarch32_VQRDMLSH_A
    __encoding aarch32_VQRDMLSH_A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 0xxxxxxx xxxx1100 xxx1xxxx'
        __guard TRUE
        __decode
            if !HaveQRDMLAHExt() then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1') then UNDEFINED;
            if size == '00' || size == '11' then UNDEFINED;
            add = FALSE;  scalar_form = FALSE;  esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VQRDMLSH_A2_A
        __instruction_set A32
        __field Q 24 +: 1
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '1111001x 1xxxxxxx xxxx1111 x1x0xxxx'
        __guard TRUE
        __decode
            if !HaveQRDMLAHExt() then UNDEFINED;
            if size == '11' then SEE "Related encodings";
            if size == '00' then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1') then UNDEFINED;
            add = FALSE;  scalar_form = TRUE;  d = UInt(D:Vd);  n = UInt(N:Vn);  regs = if Q == '0' then 1 else 2;
            if size == '01' then esize = 16;  elements = 4;  m = UInt(Vm<2:0>);  index = UInt(M:Vm<3>);
            if size == '10' then esize = 32;  elements = 2;  m = UInt(Vm);  index = UInt(M);

    __encoding aarch32_VQRDMLSH_T1_A
        __instruction_set T32
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 0xxxxxxx xxxx1100 xxx1xxxx'
        __guard TRUE
        __decode
            if !HaveQRDMLAHExt() then UNDEFINED;
            if InITBlock() then UNPREDICTABLE;
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1') then UNDEFINED;
            if size == '00' || size == '11' then UNDEFINED;
            add = FALSE;  scalar_form = FALSE;  esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VQRDMLSH_T2_A
        __instruction_set T32
        __field Q 28 +: 1
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '111x1111 1xxxxxxx xxxx1111 x1x0xxxx'
        __guard TRUE
        __decode
            if !HaveQRDMLAHExt() then UNDEFINED;
            if InITBlock() then UNPREDICTABLE;
            if size == '11' then SEE "Related encodings";
            if size == '00' then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1') then UNDEFINED;
            add = FALSE;  scalar_form = TRUE;  d = UInt(D:Vd);  n = UInt(N:Vn);  regs = if Q == '0' then 1 else 2;
            if size == '01' then esize = 16;  elements = 4;  m = UInt(Vm<2:0>);  index = UInt(M:Vm<3>);
            if size == '10' then esize = 32;  elements = 2;  m = UInt(Vm);  index = UInt(M);

    __execute
        CheckAdvSIMDEnabled();
        round_const = 1 << (esize-1);
        if scalar_form then op2 = SInt(Elem[D[m],index,esize]);
        for r = 0 to regs-1
            for e = 0 to elements-1
                op1 = SInt(Elem[D[n+r],e,esize]);
                op3 = SInt(Elem[D[d+r],e,esize]) << esize;
                if !scalar_form then op2 = SInt(Elem[D[m+r],e,esize]);
                (result, sat) = SignedSatQ((op3 - 2*(op1*op2) + round_const) >> esize, esize);
                Elem[D[d+r],e,esize] = result;
                if sat then FPSCR.QC = '1';
