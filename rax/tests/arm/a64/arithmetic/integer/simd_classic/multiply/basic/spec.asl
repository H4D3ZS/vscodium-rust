__instruction aarch64_vector_arithmetic_binary_uniform_mul_int_product
    __encoding aarch64_vector_arithmetic_binary_uniform_mul_int_product
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field size 22 +: 2
        __field Rm 16 +: 5
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01110 xx1xxxxx 100111xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            if U == '1' && size != '00' then UNDEFINED;
            if size == '11' then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;

            boolean poly = (U == '1');

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand1 = V[n];
        bits(datasize) operand2 = V[m];
        bits(datasize) result;
        bits(esize) element1;
        bits(esize) element2;
        bits(esize) product;

        for e = 0 to elements-1
            element1 = Elem[operand1, e, esize];
            element2 = Elem[operand2, e, esize];
            if poly then
                product = PolynomialMult(element1, element2)<esize-1:0>;
            else
                product = (UInt(element1) * UInt(element2))<esize-1:0>;
            Elem[result, e, esize] = product;

        V[d] = result;
