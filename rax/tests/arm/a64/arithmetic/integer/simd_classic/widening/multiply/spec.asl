__instruction aarch64_vector_arithmetic_binary_disparate_mul_accum
    __encoding aarch64_vector_arithmetic_binary_disparate_mul_accum
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field size 22 +: 2
        __field Rm 16 +: 5
        __field o1 13 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01110 xx1xxxxx 10x000xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);

            if size == '11' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer datasize = 64;
            integer part = UInt(Q);
            integer elements = datasize DIV esize;
            boolean sub_op = (o1 == '1');
            boolean unsigned = (U == '1');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize)   operand1 = Vpart[n, part];
        bits(datasize)   operand2 = Vpart[m, part];
        bits(2*datasize) operand3 = V[d];
        bits(2*datasize) result;
        integer element1;
        integer element2;
        bits(2*esize) product;
        bits(2*esize) accum;

        for e = 0 to elements-1
            element1 = Int(Elem[operand1, e, esize], unsigned);
            element2 = Int(Elem[operand2, e, esize], unsigned);
            product = (element1 * element2)<2*esize-1:0>;
            if sub_op then
                accum = Elem[operand3, e, 2*esize] - product;
            else
                accum = Elem[operand3, e, 2*esize] + product;
            Elem[result, e, 2*esize] = accum;

        V[d] = result;

__instruction aarch64_vector_arithmetic_binary_disparate_mul_dmacc_sisd
    __encoding aarch64_vector_arithmetic_binary_disparate_mul_dmacc_sisd
        __instruction_set A64
        __field size 22 +: 2
        __field Rm 16 +: 5
        __field o1 13 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01011110 xx1xxxxx 10x100xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);

            if size == '00' || size == '11' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer datasize = esize;
            integer elements = 1;
            integer part = 0;

            boolean sub_op = (o1 == '1');

    __encoding aarch64_vector_arithmetic_binary_disparate_mul_dmacc_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field size 22 +: 2
        __field Rm 16 +: 5
        __field o1 13 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x001110 xx1xxxxx 10x100xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);

            if size == '00' || size == '11' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer datasize = 64;
            integer part = UInt(Q);
            integer elements = datasize DIV esize;

            boolean sub_op = (o1 == '1');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize)   operand1 = Vpart[n, part];
        bits(datasize)   operand2 = Vpart[m, part];
        bits(2*datasize) operand3 = V[d];
        bits(2*datasize) result;
        integer element1;
        integer element2;
        bits(2*esize) product;
        integer accum;
        boolean sat1;
        boolean sat2;

        for e = 0 to elements-1
            element1 = SInt(Elem[operand1, e, esize]);
            element2 = SInt(Elem[operand2, e, esize]);
            (product, sat1) = SignedSatQ(2 * element1 * element2, 2*esize);
            if sub_op then
                accum = SInt(Elem[operand3, e, 2*esize]) - SInt(product);
            else
                accum = SInt(Elem[operand3, e, 2*esize]) + SInt(product);
            (Elem[result, e, 2*esize], sat2) = SignedSatQ(accum, 2*esize);
            if sat1 || sat2 then FPSR.QC = '1';

        V[d] = result;

__instruction aarch64_vector_arithmetic_binary_disparate_mul_double_sisd
    __encoding aarch64_vector_arithmetic_binary_disparate_mul_double_sisd
        __instruction_set A64
        __field size 22 +: 2
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01011110 xx1xxxxx 110100xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);

            if size == '00' || size == '11' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer datasize = esize;
            integer elements = 1;
            integer part = 0;

    __encoding aarch64_vector_arithmetic_binary_disparate_mul_double_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field size 22 +: 2
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x001110 xx1xxxxx 110100xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);

            if size == '00' || size == '11' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer datasize = 64;
            integer part = UInt(Q);
            integer elements = datasize DIV esize;

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize)   operand1 = Vpart[n, part];
        bits(datasize)   operand2 = Vpart[m, part];
        bits(2*datasize) result;
        integer element1;
        integer element2;
        bits(2*esize) product;
        boolean sat;

        for e = 0 to elements-1
            element1 = SInt(Elem[operand1, e, esize]);
            element2 = SInt(Elem[operand2, e, esize]);
            (product, sat) = SignedSatQ(2 * element1 * element2, 2*esize);
            Elem[result, e, 2*esize] = product;
            if sat then FPSR.QC = '1';

        V[d] = result;

__instruction aarch64_vector_arithmetic_binary_disparate_mul_poly
    __encoding aarch64_vector_arithmetic_binary_disparate_mul_poly
        __instruction_set A64
        __field Q 30 +: 1
        __field size 22 +: 2
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x001110 xx1xxxxx 111000xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);

            if size == '01' || size == '10' then UNDEFINED;
            if size == '11' && !HaveBit128PMULLExt() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer datasize = 64;
            integer part = UInt(Q);
            integer elements = datasize DIV esize;

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize)   operand1 = Vpart[n, part];
        bits(datasize)   operand2 = Vpart[m, part];
        bits(2*datasize) result;
        bits(esize) element1;
        bits(esize) element2;

        for e = 0 to elements-1
            element1 = Elem[operand1, e, esize];
            element2 = Elem[operand2, e, esize];
            Elem[result, e, 2*esize] = PolynomialMult(element1, element2);

        V[d] = result;

__instruction aarch64_vector_arithmetic_binary_disparate_mul_product
    __encoding aarch64_vector_arithmetic_binary_disparate_mul_product
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field size 22 +: 2
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01110 xx1xxxxx 110000xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);

            if size == '11' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer datasize = 64;
            integer part = UInt(Q);
            integer elements = datasize DIV esize;

            boolean unsigned = (U == '1');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize)   operand1 = Vpart[n, part];
        bits(datasize)   operand2 = Vpart[m, part];
        bits(2*datasize) result;
        integer element1;
        integer element2;

        for e = 0 to elements-1
            element1 = Int(Elem[operand1, e, esize], unsigned);
            element2 = Int(Elem[operand2, e, esize], unsigned);
            Elem[result, e, 2*esize] = (element1 * element2)<2*esize-1:0>;

        V[d] = result;
