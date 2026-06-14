__instruction aarch64_vector_transfer_vector_table
    __encoding aarch64_vector_transfer_vector_table
        __instruction_set A64
        __field Q 30 +: 1
        __field Rm 16 +: 5
        __field len 13 +: 2
        __field op 12 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x001110 000xxxxx 0xxx00xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);

            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV 8;
            integer regs = UInt(len) + 1;
            boolean is_tbl = (op == '0');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) indices = V[m];
        bits(128*regs) table = Zeros();
        bits(datasize) result;
        integer index;

        // Create table from registers
        for i = 0 to regs - 1
            table<128*i+127:128*i> = V[n];
            n = (n + 1) MOD 32;

        result = if is_tbl then Zeros() else V[d];
        for i = 0 to elements - 1
            index = UInt(Elem[indices, i, 8]);
            if index < 16 * regs then
                Elem[result, i, 8] = Elem[table, index, 8];

        V[d] = result;
