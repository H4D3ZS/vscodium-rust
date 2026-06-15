__instruction aarch64_vector_arithmetic_binary_element_mul_fp16_sisd
    __encoding aarch64_vector_arithmetic_binary_element_mul_fp16_sisd
        __instruction_set A64
        __field U 29 +: 1
        __field L 21 +: 1
        __field M 20 +: 1
        __field Rm 16 +: 4
        __field H 11 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01x11111 00xxxxxx 1001x0xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveFP16Ext() then UNDEFINED;

            integer idxdsize = if H == '1' then 128 else 64;
            integer index;
            index = UInt(H:L:M);

            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);

            integer esize = 16;
            integer datasize = esize;
            integer elements = 1;
            boolean mulx_op = (U == '1');

    __encoding aarch64_vector_arithmetic_binary_element_mul_fp_sisd
        __instruction_set A64
        __field U 29 +: 1
        __field sz 22 +: 1
        __field L 21 +: 1
        __field M 20 +: 1
        __field Rm 16 +: 4
        __field H 11 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01x11111 1xxxxxxx 1001x0xx xxxxxxxx'
        __guard TRUE
        __decode
            integer idxdsize = if H == '1' then 128 else 64;
            integer index;
            bit Rmhi = M;
            case sz:L of
                when '0x' index = UInt(H:L);
                when '10' index = UInt(H);
                when '11' UNDEFINED;

            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rmhi:Rm);

            integer esize = 32 << UInt(sz);
            integer datasize = esize;
            integer elements = 1;
            boolean mulx_op = (U == '1');

    __encoding aarch64_vector_arithmetic_binary_element_mul_fp16_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field L 21 +: 1
        __field M 20 +: 1
        __field Rm 16 +: 4
        __field H 11 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01111 00xxxxxx 1001x0xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveFP16Ext() then UNDEFINED;

            integer idxdsize = if H == '1' then 128 else 64;
            integer index;
            index = UInt(H:L:M);

            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);

            integer esize = 16;
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;
            boolean mulx_op = (U == '1');

    __encoding aarch64_vector_arithmetic_binary_element_mul_fp_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field sz 22 +: 1
        __field L 21 +: 1
        __field M 20 +: 1
        __field Rm 16 +: 4
        __field H 11 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01111 1xxxxxxx 1001x0xx xxxxxxxx'
        __guard TRUE
        __decode
            integer idxdsize = if H == '1' then 128 else 64;
            integer index;
            bit Rmhi = M;
            case sz:L of
                when '0x' index = UInt(H:L);
                when '10' index = UInt(H);
                when '11' UNDEFINED;

            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rmhi:Rm);

            if sz:Q == '10' then UNDEFINED;
            integer esize = 32 << UInt(sz);
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;
            boolean mulx_op = (U == '1');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand1 = V[n];
        bits(idxdsize) operand2 = V[m];
        bits(datasize) result;
        bits(esize) element1;
        bits(esize) element2 = Elem[operand2, index, esize];

        for e = 0 to elements-1
            element1 = Elem[operand1, e, esize];
            if mulx_op then
                Elem[result, e, esize] = FPMulX(element1, element2, FPCR);
            else
                Elem[result, e, esize] = FPMul(element1, element2, FPCR);

        V[d] = result;

__instruction aarch64_vector_arithmetic_binary_element_mul_double_sisd
    __encoding aarch64_vector_arithmetic_binary_element_mul_double_sisd
        __instruction_set A64
        __field size 22 +: 2
        __field L 21 +: 1
        __field M 20 +: 1
        __field Rm 16 +: 4
        __field H 11 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01011111 xxxxxxxx 1011x0xx xxxxxxxx'
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
            integer part = 0;

    __encoding aarch64_vector_arithmetic_binary_element_mul_double_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field size 22 +: 2
        __field L 21 +: 1
        __field M 20 +: 1
        __field Rm 16 +: 4
        __field H 11 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x001111 xxxxxxxx 1011x0xx xxxxxxxx'
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
            integer datasize = 64;
            integer part = UInt(Q);
            integer elements = datasize DIV esize;

    __execute
        CheckFPAdvSIMDEnabled64();

        bits(datasize)   operand1 = Vpart[n, part];
        bits(idxdsize)   operand2 = V[m];
        bits(2*datasize) result;
        integer element1;
        integer element2;
        bits(2*esize) product;
        boolean sat;

        element2 = SInt(Elem[operand2, index, esize]);
        for e = 0 to elements-1
            element1 = SInt(Elem[operand1, e, esize]);
            (product, sat) = SignedSatQ(2 * element1 * element2, 2*esize);
            Elem[result, e, 2*esize] = product;
            if sat then FPSR.QC = '1';

        V[d] = result;

__instruction aarch64_vector_arithmetic_binary_element_mul_acc_double_sisd
    __encoding aarch64_vector_arithmetic_binary_element_mul_acc_double_sisd
        __instruction_set A64
        __field size 22 +: 2
        __field L 21 +: 1
        __field M 20 +: 1
        __field Rm 16 +: 4
        __field o2 14 +: 1
        __field H 11 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01011111 xxxxxxxx 0x11x0xx xxxxxxxx'
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
            integer part = 0;

            boolean sub_op = (o2 == '1');

    __encoding aarch64_vector_arithmetic_binary_element_mul_acc_double_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field size 22 +: 2
        __field L 21 +: 1
        __field M 20 +: 1
        __field Rm 16 +: 4
        __field o2 14 +: 1
        __field H 11 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x001111 xxxxxxxx 0x11x0xx xxxxxxxx'
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
            integer datasize = 64;
            integer part = UInt(Q);
            integer elements = datasize DIV esize;

            boolean sub_op = (o2 == '1');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize)   operand1 = Vpart[n, part];
        bits(idxdsize)   operand2 = V[m];
        bits(2*datasize) operand3 = V[d];
        bits(2*datasize) result;
        integer element1;
        integer element2;
        bits(2*esize) product;
        integer accum;
        boolean sat1;
        boolean sat2;

        element2 = SInt(Elem[operand2, index, esize]);
        for e = 0 to elements-1
            element1 = SInt(Elem[operand1, e, esize]);
            (product, sat1) = SignedSatQ(2 * element1 * element2, 2*esize);
            if sub_op then
                accum = SInt(Elem[operand3, e, 2*esize]) - SInt(product);
            else
                accum = SInt(Elem[operand3, e, 2*esize]) + SInt(product);
            (Elem[result, e, 2*esize], sat2) = SignedSatQ(accum, 2*esize);
            if sat1 || sat2 then FPSR.QC = '1';

        V[d] = result;
