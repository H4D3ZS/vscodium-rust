__instruction ADR_Z.AZ_SD.same.scaled
    __encoding ADR_Z.AZ_SD.same.scaled
        __instruction_set A64
        __field sz 22 +: 1
        __field Zm 16 +: 5
        __field msz 10 +: 2
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '00000100 1x1xxxxx 1010xxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 32 << UInt(sz);
            integer n = UInt(Zn);
            integer m = UInt(Zm);
            integer d = UInt(Zd);
            integer osize = esize;
            boolean unsigned = TRUE;
            integer mbytes = 1 << UInt(msz);

    __encoding ADR_Z.AZ_D.s32.scaled
        __instruction_set A64
        __field Zm 16 +: 5
        __field msz 10 +: 2
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '00000100 001xxxxx 1010xxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 64;
            integer n = UInt(Zn);
            integer m = UInt(Zm);
            integer d = UInt(Zd);
            integer osize = 32;
            boolean unsigned = FALSE;
            integer mbytes = 1 << UInt(msz);

    __encoding ADR_Z.AZ_D.u32.scaled
        __instruction_set A64
        __field Zm 16 +: 5
        __field msz 10 +: 2
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '00000100 011xxxxx 1010xxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 64;
            integer n = UInt(Zn);
            integer m = UInt(Zm);
            integer d = UInt(Zd);
            integer osize = 32;
            boolean unsigned = TRUE;
            integer mbytes = 1 << UInt(msz);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) base = Z[n];
        bits(VL) offs = Z[m];
        bits(VL) result;

        for e = 0 to elements-1
            bits(esize) addr = Elem[base, e, esize];
            integer offset = Int(Elem[offs, e, esize]<osize-1:0>, unsigned);
            Elem[result, e, esize] = addr + (offset * mbytes);

        Z[d] = result;
