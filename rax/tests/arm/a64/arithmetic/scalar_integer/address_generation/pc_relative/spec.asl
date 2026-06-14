__instruction aarch64_integer_arithmetic_address_pc_rel
    __encoding aarch64_integer_arithmetic_address_pc_rel
        __instruction_set A64
        __field op 31 +: 1
        __field immlo 29 +: 2
        __field immhi 5 +: 19
        __field Rd 0 +: 5
        __opcode 'xxx10000 xxxxxxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            boolean page = (op == '1');
            bits(64) imm;

            if page then
                imm = SignExtend(immhi:immlo:Zeros(12), 64);
            else
                imm = SignExtend(immhi:immlo, 64);

    __execute
        bits(64) base = PC[];

        if page then
            base<11:0> = Zeros(12);

        X[d] = base + imm;
