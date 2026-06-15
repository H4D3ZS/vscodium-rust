__instruction aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_sisd
    __encoding aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_sisd
        __instruction_set A64
        __field U 29 +: 1
        __field size 22 +: 2
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01x11110 xx1xxxxx 101101xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            if size == '11' || size == '00' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer datasize = esize;
            integer elements = 1;
            boolean rounding = (U == '1');

    __encoding aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field size 22 +: 2
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01110 xx1xxxxx 101101xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            if size == '11' || size == '00' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;
            boolean rounding = (U == '1');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand1 = V[n];
        bits(datasize) operand2 = V[m];
        bits(datasize) result;
        integer round_const = if rounding then 1 << (esize - 1) else 0;
        integer element1;
        integer element2;
        integer product;
        boolean sat;

        for e = 0 to elements-1
            element1 = SInt(Elem[operand1, e, esize]);
            element2 = SInt(Elem[operand2, e, esize]);
            product = (2 * element1 * element2) + round_const;
            (Elem[result, e, esize], sat) = SignedSatQ(product >> esize, esize);
            if sat then FPSR.QC = '1';

        V[d] = result;

__instruction aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd
    __encoding aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd
        __instruction_set A64
        __field size 22 +: 2
        __field Rm 16 +: 5
        __field S 11 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01111110 xx0xxxxx 1000x1xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveQRDMLAHExt() then UNDEFINED;

            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            if size == '11' || size == '00' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer datasize = esize;
            integer elements = 1;
            boolean rounding = TRUE;
            boolean sub_op = (S == '1');

    __encoding aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_accum_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field size 22 +: 2
        __field Rm 16 +: 5
        __field S 11 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x101110 xx0xxxxx 1000x1xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveQRDMLAHExt() then UNDEFINED;

            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            if size == '11' || size == '00' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;
            boolean rounding = TRUE;
            boolean sub_op = (S == '1');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand1 = V[n];
        bits(datasize) operand2 = V[m];
        bits(datasize) operand3 = V[d];
        bits(datasize) result;
        integer rounding_const = if rounding then 1 << (esize - 1) else 0;
        integer element1;
        integer element2;
        integer element3;
        integer product;
        boolean sat;

        for e = 0 to elements-1
            element1 = SInt(Elem[operand1, e, esize]);
            element2 = SInt(Elem[operand2, e, esize]);
            element3 = SInt(Elem[operand3, e, esize]);
            if sub_op then
                accum = ((element3 << esize) - 2 * (element1 * element2) + rounding_const);
            else
                accum = ((element3 << esize) + 2 * (element1 * element2) + rounding_const);
            (Elem[result, e, esize], sat) = SignedSatQ(accum >> esize, esize);
            if sat then FPSR.QC = '1';

        V[d] = result;
