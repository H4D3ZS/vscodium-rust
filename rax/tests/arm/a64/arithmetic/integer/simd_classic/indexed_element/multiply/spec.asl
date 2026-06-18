__instruction aarch64_vector_arithmetic_binary_element_mul_int
    __encoding aarch64_vector_arithmetic_binary_element_mul_int
        __instruction_set A64
        __field Q 30 +: 1
        __field size 22 +: 2
        __field L 21 +: 1
        __field M 20 +: 1
        __field Rm 16 +: 4
        __field H 11 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x001111 xxxxxxxx 1000x0xx xxxxxxxx'
        __guard TRUE
        __decode
            integer idxdsize = if H == '1' then 128 else 64;
            integer index;
            bit Rmhi;
            case size of
                when '01' index = UInt(H:L:M); Rmhi = '0';
                when '10' index = UInt(H:L);   Rmhi = M;
                otherwise UNDEFINED;

            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rmhi:Rm);

            integer esize = 8 << UInt(size);
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand1 = V[n];
        bits(idxdsize) operand2 = V[m];
        bits(datasize) result;
        integer element1;
        integer element2;
        bits(esize) product;

        element2 = UInt(Elem[operand2, index, esize]);
        for e = 0 to elements-1
            element1 = UInt(Elem[operand1, e, esize]);
            product = (element1 * element2)<esize-1:0>;
            Elem[result, e, esize] = product;

        V[d] = result;

__instruction aarch64_vector_arithmetic_binary_element_mul_high_sisd
    __encoding aarch64_vector_arithmetic_binary_element_mul_high_sisd
        __instruction_set A64
        __field size 22 +: 2
        __field L 21 +: 1
        __field M 20 +: 1
        __field Rm 16 +: 4
        __field op 12 +: 1
        __field H 11 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01011111 xxxxxxxx 110xx0xx xxxxxxxx'
        __guard TRUE
        __decode
            integer idxdsize = if H == '1' then 128 else 64;
            integer index;
            bit Rmhi;
            case size of
                when '01' index = UInt(H:L:M); Rmhi = '0';
                when '10' index = UInt(H:L);   Rmhi = M;
                otherwise UNDEFINED;

            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rmhi:Rm);

            integer esize = 8 << UInt(size);
            integer datasize = esize;
            integer elements = 1;

            boolean round = (op == '1');

    __encoding aarch64_vector_arithmetic_binary_element_mul_high_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field size 22 +: 2
        __field L 21 +: 1
        __field M 20 +: 1
        __field Rm 16 +: 4
        __field op 12 +: 1
        __field H 11 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x001111 xxxxxxxx 110xx0xx xxxxxxxx'
        __guard TRUE
        __decode
            integer idxdsize = if H == '1' then 128 else 64;
            integer index;
            bit Rmhi;
            case size of
                when '01' index = UInt(H:L:M); Rmhi = '0';
                when '10' index = UInt(H:L);   Rmhi = M;
                otherwise UNDEFINED;

            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rmhi:Rm);

            integer esize = 8 << UInt(size);
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;

            boolean round = (op == '1');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand1 = V[n];
        bits(idxdsize) operand2 = V[m];
        bits(datasize) result;
        integer round_const = if round then 1 << (esize - 1) else 0;
        integer element1;
        integer element2;
        integer product;
        boolean sat;

        element2 = SInt(Elem[operand2, index, esize]);
        for e = 0 to elements-1
            element1 = SInt(Elem[operand1, e, esize]);
            product = (2 * element1 * element2) + round_const;
            // The following only saturates if element1 and element2 equal -(2^(esize-1))
            (Elem[result, e, esize], sat) = SignedSatQ(product >> esize, esize);
            if sat then FPSR.QC = '1';

        V[d] = result;
