__instruction aarch64_vector_arithmetic_binary_element_mul_acc_high_sisd
    __encoding aarch64_vector_arithmetic_binary_element_mul_acc_high_sisd
        __instruction_set A64
        __field size 22 +: 2
        __field L 21 +: 1
        __field M 20 +: 1
        __field Rm 16 +: 4
        __field S 13 +: 1
        __field H 11 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01111111 xxxxxxxx 11x1x0xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveQRDMLAHExt() then UNDEFINED;

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

            boolean rounding = TRUE;
            boolean sub_op = (S  == '1');

    __encoding aarch64_vector_arithmetic_binary_element_mul_acc_high_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field size 22 +: 2
        __field L 21 +: 1
        __field M 20 +: 1
        __field Rm 16 +: 4
        __field S 13 +: 1
        __field H 11 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x101111 xxxxxxxx 11x1x0xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveQRDMLAHExt() then UNDEFINED;

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

            boolean rounding = TRUE;
            boolean sub_op = (S  == '1');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand1 = V[n];
        bits(idxdsize) operand2 = V[m];
        bits(datasize) operand3 = V[d];
        bits(datasize) result;
        integer rounding_const = if rounding then 1 << (esize - 1) else 0;
        integer element1;
        integer element2;
        integer element3;
        integer product;
        boolean sat;

        element2 = SInt(Elem[operand2, index, esize]);
        for e = 0 to elements-1
            element1 = SInt(Elem[operand1, e, esize]);
            element3 = SInt(Elem[operand3, e, esize]);
            if sub_op then
                accum = ((element3 << esize) - 2 * (element1 * element2) + rounding_const);
            else
                accum = ((element3 << esize) + 2 * (element1 * element2) + rounding_const);
            (Elem[result, e, esize], sat) = SignedSatQ(accum >> esize, esize);
            if sat then FPSR.QC = '1';

        V[d] = result;

__instruction aarch64_vector_arithmetic_binary_element_mul_acc_long
    __encoding aarch64_vector_arithmetic_binary_element_mul_acc_long
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field size 22 +: 2
        __field L 21 +: 1
        __field M 20 +: 1
        __field Rm 16 +: 4
        __field o2 14 +: 1
        __field H 11 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01111 xxxxxxxx 0x10x0xx xxxxxxxx'
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

            boolean unsigned = (U == '1');
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

        element2 = Int(Elem[operand2, index, esize], unsigned);
        for e = 0 to elements-1
            element1 = Int(Elem[operand1, e, esize], unsigned);
            product = (element1 * element2)<2*esize-1:0>;
            if sub_op then
                Elem[result, e, 2*esize] = Elem[operand3, e, 2*esize] - product;
            else
                Elem[result, e, 2*esize] = Elem[operand3, e, 2*esize] + product;

        V[d] = result;

__instruction aarch64_vector_arithmetic_binary_element_mul_long
    __encoding aarch64_vector_arithmetic_binary_element_mul_long
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field size 22 +: 2
        __field L 21 +: 1
        __field M 20 +: 1
        __field Rm 16 +: 4
        __field H 11 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01111 xxxxxxxx 1010x0xx xxxxxxxx'
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
            boolean unsigned = (U == '1');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize)   operand1 = Vpart[n, part];
        bits(idxdsize)   operand2 = V[m];
        bits(2*datasize) result;
        integer element1;
        integer element2;
        bits(2*esize) product;

        element2 = Int(Elem[operand2, index, esize], unsigned);
        for e = 0 to elements-1
            element1 = Int(Elem[operand1, e, esize], unsigned);
            product = (element1 * element2)<2*esize-1:0>;
            Elem[result, e, 2*esize] = product;

        V[d] = result;

__instruction aarch64_vector_arithmetic_binary_element_mul_acc_int
    __encoding aarch64_vector_arithmetic_binary_element_mul_acc_int
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
        __opcode '0x101111 xxxxxxxx 0x00x0xx xxxxxxxx'
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

            boolean sub_op = (o2 == '1');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand1 = V[n];
        bits(idxdsize) operand2 = V[m];
        bits(datasize) operand3 = V[d];
        bits(datasize) result;
        integer element1;
        integer element2;
        bits(esize) product;

        element2 = UInt(Elem[operand2, index, esize]);
        for e = 0 to elements-1
            element1 = UInt(Elem[operand1, e, esize]);
            product = (element1 * element2)<esize-1:0>;
            if sub_op then
                Elem[result, e, esize] = Elem[operand3, e, esize] - product;
            else
                Elem[result, e, esize] = Elem[operand3, e, esize] + product;
        V[d] = result;

__instruction aarch64_vector_arithmetic_binary_element_mul_acc_mul_norounding_i_lower
    __encoding aarch64_vector_arithmetic_binary_element_mul_acc_mul_norounding_i_lower
        __instruction_set A64
        __field Q 30 +: 1
        __field sz 22 +: 1
        __field L 21 +: 1
        __field M 20 +: 1
        __field Rm 16 +: 4
        __field S 14 +: 1
        __field H 11 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x001111 1xxxxxxx 0x00x0xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveFP16MulNoRoundingToFP32Ext() then UNDEFINED;
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt('0':Rm);    // Vm can only be in bottom 16 registers.
            if sz == '1' then UNDEFINED;
            integer index = UInt(H:L:M);

            integer esize = 32;
            integer datasize = if Q=='1' then 128 else 64;
            integer elements = datasize DIV esize;

            boolean sub_op = (S == '1');
            integer part = 0;

    __encoding aarch64_vector_arithmetic_binary_element_mul_acc_mul_norounding_i_upper
        __instruction_set A64
        __field Q 30 +: 1
        __field sz 22 +: 1
        __field L 21 +: 1
        __field M 20 +: 1
        __field Rm 16 +: 4
        __field S 14 +: 1
        __field H 11 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0x101111 1xxxxxxx 1x00x0xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveFP16MulNoRoundingToFP32Ext() then UNDEFINED;
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt('0':Rm);    // Vm can only be in bottom 16 registers.
            if sz == '1' then UNDEFINED;
            integer index = UInt(H:L:M);

            integer esize = 32;
            integer datasize = if Q=='1' then 128 else 64;
            integer elements = datasize DIV esize;

            boolean sub_op = (S == '1');
            integer part = 1;

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize DIV 2) operand1 = Vpart[n,part];
        bits(128) operand2 = V[m];
        bits(datasize) operand3 = V[d];
        bits(datasize) result;
        bits(esize DIV 2) element1;
        bits(esize DIV 2) element2 = Elem[operand2, index, esize DIV 2];

        for e = 0 to elements-1
            element1 = Elem[operand1, e, esize DIV 2];
            if sub_op then element1 = FPNeg(element1);
            Elem[result, e, esize] = FPMulAddH(Elem[operand3, e, esize], element1, element2, FPCR);
        V[d] = result;
