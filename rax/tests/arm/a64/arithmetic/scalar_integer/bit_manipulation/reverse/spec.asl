__instruction aarch64_integer_arithmetic_rev
    __encoding aarch64_integer_arithmetic_rev
        __instruction_set A64
        __field sf 31 +: 1
        __field opc 10 +: 2
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode 'x1011010 11000000 0000xxxx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            integer datasize = if sf == '1' then 64 else 32;

            integer container_size;
            case opc of
                when '00'
                    Unreachable();
                when '01'
                    container_size = 16;
                when '10'
                    container_size = 32;
                when '11'
                    if sf == '0' then UNDEFINED;
                    container_size = 64;

    __execute
        bits(datasize) operand = X[n];
        bits(datasize) result;

        integer containers = datasize DIV container_size;
        integer elements_per_container = container_size DIV 8;
        integer index = 0;
        integer rev_index;
        for c = 0 to containers-1
            rev_index = index + ((elements_per_container - 1) * 8);
            for e = 0 to elements_per_container-1
                result<rev_index + 7:rev_index> = operand<index + 7:index>;
                index = index + 8;
                rev_index = rev_index - 8;

        X[d] = result;

__instruction aarch64_integer_arithmetic_rbit
    __encoding aarch64_integer_arithmetic_rbit
        __instruction_set A64
        __field sf 31 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode 'x1011010 11000000 000000xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            integer datasize = if sf == '1' then 64 else 32;

    __execute
        bits(datasize) operand = X[n];
        bits(datasize) result;

        for i = 0 to datasize-1
            result<datasize-1-i> = operand<i>;

        X[d] = result;
