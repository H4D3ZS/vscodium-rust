__instruction aarch64_memory_vector_single_no_wb
    __encoding aarch64_memory_vector_single_no_wb
        __instruction_set A64
        __field Q 30 +: 1
        __field L 22 +: 1
        __field R 21 +: 1
        __field opcode 13 +: 3
        __field S 12 +: 1
        __field size 10 +: 2
        __field Rn 5 +: 5
        __field Rt 0 +: 5
        __opcode '0x001101 0xx00000 xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            integer t = UInt(Rt);
            integer n = UInt(Rn);
            integer m = integer UNKNOWN;
            boolean wback = FALSE;
            boolean tag_checked = wback || n != 31;

    __encoding aarch64_memory_vector_single_post_inc
        __instruction_set A64
        __field Q 30 +: 1
        __field L 22 +: 1
        __field R 21 +: 1
        __field Rm 16 +: 5
        __field opcode 13 +: 3
        __field S 12 +: 1
        __field size 10 +: 2
        __field Rn 5 +: 5
        __field Rt 0 +: 5
        __opcode '0x001101 1xxxxxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            integer t = UInt(Rt);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            boolean wback = TRUE;
            boolean tag_checked = wback || n != 31;

    __postdecode
        integer scale = UInt(opcode<2:1>);
        integer selem = UInt(opcode<0>:R) + 1;
        boolean replicate = FALSE;
        integer index;

        case scale of
            when 3
                // load and replicate
                if L == '0' || S == '1' then UNDEFINED;
                scale = UInt(size);
                replicate = TRUE;
            when 0
                index = UInt(Q:S:size);         // B[0-15]
            when 1
                if size<0> == '1' then UNDEFINED;
                index = UInt(Q:S:size<1>);      // H[0-7]
            when 2
                if size<1> == '1' then UNDEFINED;
                if size<0> == '0' then
                    index = UInt(Q:S);          // S[0-3]
                else
                    if S == '1' then UNDEFINED;
                    index = UInt(Q);            // D[0-1]
                    scale = 3;

        MemOp memop = if L == '1' then MemOp_LOAD else MemOp_STORE;
        integer datasize = if Q == '1' then 128 else 64;
        integer esize = 8 << scale;
    __execute
        if HaveMTEExt() then
            SetNotTagCheckedInstruction(!tag_checked);

        CheckFPAdvSIMDEnabled64();

        bits(64) address;
        bits(64) offs;
        bits(128) rval;
        bits(esize) element;
        constant integer ebytes = esize DIV 8;

        if n == 31 then
            CheckSPAlignment();
            address = SP[];
        else
            address = X[n];

        offs = Zeros();
        if replicate then
            // load and replicate to all elements
            for s = 0 to selem-1
                element = Mem[address + offs, ebytes, AccType_VEC];
                // replicate to fill 128- or 64-bit register
                V[t] = Replicate(element, datasize DIV esize);
                offs = offs + ebytes;
                t = (t + 1) MOD 32;
        else
            // load/store one element per register
            for s = 0 to selem-1
                rval = V[t];
                if memop == MemOp_LOAD then
                    // insert into one lane of 128-bit register
                    Elem[rval, index, esize] = Mem[address + offs, ebytes, AccType_VEC];
                    V[t] = rval;
                else // memop == MemOp_STORE
                    // extract from one lane of 128-bit register
                    Mem[address + offs, ebytes, AccType_VEC] = Elem[rval, index, esize];
                offs = offs + ebytes;
                t = (t + 1) MOD 32;

        if wback then
            if m != 31 then
                offs = X[m];
            if n == 31 then
                SP[] = address + offs;
            else
                X[n] = address + offs;
