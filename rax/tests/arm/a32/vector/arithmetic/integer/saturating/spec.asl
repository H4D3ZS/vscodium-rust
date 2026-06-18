__instruction aarch32_VQADD_A
    __encoding aarch32_VQADD_T1A1_A
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
        __opcode '1111001x 0xxxxxxx xxxx0000 xxx1xxxx'
        __guard TRUE
        __decode
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1') then UNDEFINED;
            unsigned = (U == '1');
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VQADD_T1A1_A
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
        __opcode '111x1111 0xxxxxxx xxxx0000 xxx1xxxx'
        __guard TRUE
        __decode
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1') then UNDEFINED;
            unsigned = (U == '1');
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __execute __conditional
        CheckAdvSIMDEnabled();
        for r = 0 to regs-1
            for e = 0 to elements-1
                sum = Int(Elem[D[n+r],e,esize], unsigned) + Int(Elem[D[m+r],e,esize], unsigned);
                (Elem[D[d+r],e,esize], sat) = SatQ(sum, esize, unsigned);
                if sat then FPSCR.QC = '1';

__instruction aarch32_VQSUB_A
    __encoding aarch32_VQSUB_T1A1_A
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
        __opcode '1111001x 0xxxxxxx xxxx0010 xxx1xxxx'
        __guard TRUE
        __decode
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1') then UNDEFINED;
            unsigned = (U == '1');
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VQSUB_T1A1_A
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
        __opcode '111x1111 0xxxxxxx xxxx0010 xxx1xxxx'
        __guard TRUE
        __decode
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1') then UNDEFINED;
            unsigned = (U == '1');
            esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __execute __conditional
        CheckAdvSIMDEnabled();
        for r = 0 to regs-1
            for e = 0 to elements-1
                diff = Int(Elem[D[n+r],e,esize], unsigned) - Int(Elem[D[m+r],e,esize], unsigned);
                (Elem[D[d+r],e,esize], sat) = SatQ(diff, esize, unsigned);
                if sat then FPSCR.QC = '1';

__instruction aarch32_VQDMULH_A
    __encoding aarch32_VQDMULH_T1A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110010 0xxxxxxx xxxx1011 xxx0xxxx'
        __guard TRUE
        __decode
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1') then UNDEFINED;
            if size == '00' || size == '11' then UNDEFINED;
            scalar_form = FALSE;  esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VQDMULH_T2A2_A
        __instruction_set A32
        __field Q 24 +: 1
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '1111001x 1xxxxxxx xxxx1100 x1x0xxxx'
        __guard TRUE
        __decode
            if size == '11' then SEE "Related encodings";
            if size == '00' then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1') then UNDEFINED;
            scalar_form = TRUE;  d = UInt(D:Vd);  n = UInt(N:Vn);  regs = if Q == '0' then 1 else 2;
            if size == '01' then esize = 16;  elements = 4;  m = UInt(Vm<2:0>);  index = UInt(M:Vm<3>);
            if size == '10' then esize = 32;  elements = 2;  m = UInt(Vm);  index = UInt(M);

    __encoding aarch32_VQDMULH_T1A1_A
        __instruction_set T32
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11101111 0xxxxxxx xxxx1011 xxx0xxxx'
        __guard TRUE
        __decode
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1') then UNDEFINED;
            if size == '00' || size == '11' then UNDEFINED;
            scalar_form = FALSE;  esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VQDMULH_T2A2_A
        __instruction_set T32
        __field Q 28 +: 1
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '111x1111 1xxxxxxx xxxx1100 x1x0xxxx'
        __guard TRUE
        __decode
            if size == '11' then SEE "Related encodings";
            if size == '00' then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1') then UNDEFINED;
            scalar_form = TRUE;  d = UInt(D:Vd);  n = UInt(N:Vn);  regs = if Q == '0' then 1 else 2;
            if size == '01' then esize = 16;  elements = 4;  m = UInt(Vm<2:0>);  index = UInt(M:Vm<3>);
            if size == '10' then esize = 32;  elements = 2;  m = UInt(Vm);  index = UInt(M);

    __execute __conditional
        CheckAdvSIMDEnabled();
        if scalar_form then op2 = SInt(Elem[D[m],index,esize]);
        for r = 0 to regs-1
            for e = 0 to elements-1
                if !scalar_form then op2 = SInt(Elem[D[m+r],e,esize]);
                op1 = SInt(Elem[D[n+r],e,esize]);
                // The following only saturates if both op1 and op2 equal -(2^(esize-1))
                (result, sat) = SignedSatQ((2*op1*op2) >> esize, esize);
                Elem[D[d+r],e,esize] = result;
                if sat then FPSCR.QC = '1';

__instruction aarch32_VQRDMULH_A
    __encoding aarch32_VQRDMULH_T1A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 0xxxxxxx xxxx1011 xxx0xxxx'
        __guard TRUE
        __decode
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1') then UNDEFINED;
            if size == '00' || size == '11' then UNDEFINED;
            scalar_form = FALSE;  esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VQRDMULH_T2A2_A
        __instruction_set A32
        __field Q 24 +: 1
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '1111001x 1xxxxxxx xxxx1101 x1x0xxxx'
        __guard TRUE
        __decode
            if size == '11' then SEE "Related encodings";
            if size == '00' then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1') then UNDEFINED;
            scalar_form = TRUE;  d = UInt(D:Vd);  n = UInt(N:Vn);  regs = if Q == '0' then 1 else 2;
            if size == '01' then esize = 16;  elements = 4;  m = UInt(Vm<2:0>);  index = UInt(M:Vm<3>);
            if size == '10' then esize = 32;  elements = 2;  m = UInt(Vm);  index = UInt(M);

    __encoding aarch32_VQRDMULH_T1A1_A
        __instruction_set T32
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 0xxxxxxx xxxx1011 xxx0xxxx'
        __guard TRUE
        __decode
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1' || Vm<0> == '1') then UNDEFINED;
            if size == '00' || size == '11' then UNDEFINED;
            scalar_form = FALSE;  esize = 8 << UInt(size);  elements = 64 DIV esize;
            d = UInt(D:Vd);  n = UInt(N:Vn);  m = UInt(M:Vm);  regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VQRDMULH_T2A2_A
        __instruction_set T32
        __field Q 28 +: 1
        __field D 22 +: 1
        __field size 20 +: 2
        __field Vn 16 +: 4
        __field Vd 12 +: 4
        __field N 7 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '111x1111 1xxxxxxx xxxx1101 x1x0xxxx'
        __guard TRUE
        __decode
            if size == '11' then SEE "Related encodings";
            if size == '00' then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vn<0> == '1') then UNDEFINED;
            scalar_form = TRUE;  d = UInt(D:Vd);  n = UInt(N:Vn);  regs = if Q == '0' then 1 else 2;
            if size == '01' then esize = 16;  elements = 4;  m = UInt(Vm<2:0>);  index = UInt(M:Vm<3>);
            if size == '10' then esize = 32;  elements = 2;  m = UInt(Vm);  index = UInt(M);

    __execute __conditional
        CheckAdvSIMDEnabled();
        round_const = 1 << (esize-1);
        if scalar_form then op2 = SInt(Elem[D[m],index,esize]);
        for r = 0 to regs-1
            for e = 0 to elements-1
                op1 = SInt(Elem[D[n+r],e,esize]);
                if !scalar_form then op2 = SInt(Elem[D[m+r],e,esize]);
                (result, sat) = SignedSatQ((2*op1*op2 + round_const) >> esize, esize);
                Elem[D[d+r],e,esize] = result;
                if sat then FPSCR.QC = '1';
