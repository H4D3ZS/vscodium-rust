__instruction TBL_Z.ZZ_1
    __encoding TBL_Z.ZZ_1
        __instruction_set A64
        __field size 22 +: 2
        __field Zm 16 +: 5
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '00000101 xx1xxxxx 001100xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer n = UInt(Zn);
            integer m = UInt(Zm);
            integer d = UInt(Zd);
            boolean double_table = FALSE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) indexes = Z[m];
        bits(VL) result;
        integer table_size = if double_table then VL*2 else VL;
        integer table_elems = table_size DIV esize;
        bits(table_size) table;

        if double_table then
            bits(VL) top = Z[(n + 1) MOD 32];
            bits(VL) bottom = Z[n];
            table = (top:bottom)<table_size-1:0>;
        else
            table = Z[n];

        for e = 0 to elements-1
            integer idx = UInt(Elem[indexes, e, esize]);
            Elem[result, e, esize] = if idx < table_elems then Elem[table, idx, esize] else Zeros();

        Z[d] = result;
