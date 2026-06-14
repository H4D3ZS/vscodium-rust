__instruction aarch64_memory_vector_multiple_no_wb
    __encoding aarch64_memory_vector_multiple_no_wb
        __instruction_set A64
        __field Q 30 +: 1
        __field L 22 +: 1
        __field opcode 12 +: 4
        __field size 10 +: 2
        __field Rn 5 +: 5
        __field Rt 0 +: 5
        __opcode '0x001100 0x000000 xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            integer t = UInt(Rt);
            integer n = UInt(Rn);
            integer m = integer UNKNOWN;
            boolean wback = FALSE;
            boolean tag_checked = wback || n != 31;

    __encoding aarch64_memory_vector_multiple_post_inc
        __instruction_set A64
        __field Q 30 +: 1
        __field L 22 +: 1
        __field Rm 16 +: 5
        __field opcode 12 +: 4
        __field size 10 +: 2
        __field Rn 5 +: 5
        __field Rt 0 +: 5
        __opcode '0x001100 1x0xxxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            integer t = UInt(Rt);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            boolean wback = TRUE;
            boolean tag_checked = wback || n != 31;

    __postdecode
        MemOp memop = if L == '1' then MemOp_LOAD else MemOp_STORE;
        integer datasize = if Q == '1' then 128 else 64;
        integer esize = 8 << UInt(size);
        integer elements = datasize DIV esize;

        integer rpt;    // number of iterations
        integer selem;  // structure elements

        case opcode of
            when '0000' rpt = 1; selem = 4;     // LD/ST4 (4 registers)
            when '0010' rpt = 4; selem = 1;     // LD/ST1 (4 registers)
            when '0100' rpt = 1; selem = 3;     // LD/ST3 (3 registers)
            when '0110' rpt = 3; selem = 1;     // LD/ST1 (3 registers)
            when '0111' rpt = 1; selem = 1;     // LD/ST1 (1 register)
            when '1000' rpt = 1; selem = 2;     // LD/ST2 (2 registers)
            when '1010' rpt = 2; selem = 1;     // LD/ST1 (2 registers)
            otherwise UNDEFINED;

        // .1D format only permitted with LD1 & ST1
        if size:Q == '110' && selem != 1 then UNDEFINED;
    __execute
        CheckFPAdvSIMDEnabled64();

        bits(64) address;
        bits(64) offs;
        bits(datasize) rval;
        integer tt;
        constant integer ebytes = esize DIV 8;

        if HaveMTEExt() then
            SetNotTagCheckedInstruction(!tag_checked);

        if n == 31 then
            CheckSPAlignment();
            address = SP[];
        else
            address = X[n];

        offs = Zeros();
        for r = 0 to rpt-1
            for e = 0 to elements-1
                tt = (t + r) MOD 32;
                for s = 0 to selem-1
                    rval = V[tt];
                    if memop == MemOp_LOAD then
                        Elem[rval, e, esize] = Mem[address + offs, ebytes, AccType_VEC];
                        V[tt] = rval;
                    else // memop == MemOp_STORE
                        Mem[address + offs, ebytes, AccType_VEC] = Elem[rval, e, esize];
                    offs = offs + ebytes;
                    tt = (tt + 1) MOD 32;

        if wback then
            if m != 31 then
                offs = X[m];
            if n == 31 then
                SP[] = address + offs;
            else
                X[n] = address + offs;
