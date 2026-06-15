__instruction aarch32_VREV16_A
    __encoding aarch32_VREV16_T1A1_A
        __instruction_set A32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field op 7 +: 2
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11110011 1x11xx00 xxxx0000 1xx0xxxx'
        __guard TRUE
        __decode
            if UInt(op)+UInt(size) >= 3 then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;

            esize = 8 << UInt(size);
            integer container_size;
            case op of
                when '10' container_size = 16;
                when '01' container_size = 32;
                when '00' container_size = 64;
            integer containers = 64 DIV container_size;
            integer elements_per_container = container_size DIV esize;

            d = UInt(D:Vd); m = UInt(M:Vm); regs = if Q == '0' then 1 else 2;

    __encoding aarch32_VREV16_T1A1_A
        __instruction_set T32
        __field D 22 +: 1
        __field size 18 +: 2
        __field Vd 12 +: 4
        __field op 7 +: 2
        __field Q 6 +: 1
        __field M 5 +: 1
        __field Vm 0 +: 4
        __opcode '11111111 1x11xx00 xxxx0000 1xx0xxxx'
        __guard TRUE
        __decode
            if UInt(op)+UInt(size) >= 3 then UNDEFINED;
            if Q == '1' && (Vd<0> == '1' || Vm<0> == '1') then UNDEFINED;

            esize = 8 << UInt(size);
            integer container_size;
            case op of
                when '10' container_size = 16;
                when '01' container_size = 32;
                when '00' container_size = 64;
            integer containers = 64 DIV container_size;
            integer elements_per_container = container_size DIV esize;

            d = UInt(D:Vd); m = UInt(M:Vm); regs = if Q == '0' then 1 else 2;

    __execute __conditional
        CheckAdvSIMDEnabled();

        bits(64) result;
        integer element;
        integer rev_element;
        for r = 0 to regs-1
            element = 0;
            for c = 0 to containers-1
                rev_element = element + elements_per_container - 1;
                for e = 0 to elements_per_container-1
                    Elem[result, rev_element, esize] = Elem[D[m+r], element, esize];
                    element = element + 1;
                    rev_element = rev_element - 1;
            D[d+r] = result;
