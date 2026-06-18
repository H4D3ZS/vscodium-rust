__instruction aarch64_vector_shift_left_sat_sisd
    __encoding aarch64_vector_shift_left_sat_sisd
        __instruction_set A64
        __field U 29 +: 1
        __field immh 19 +: 4
        __field immb 16 +: 3
        __field op 12 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '01x11111 0xxxxxxx 011x01xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if immh == '0000' then UNDEFINED;
            integer esize = 8 << HighestSetBit(immh);
            integer datasize = esize;
            integer elements = 1;

            integer shift = UInt(immh:immb) - esize;

            boolean src_unsigned;
            boolean dst_unsigned;
            case op:U of
                when '00' UNDEFINED;
                when '01' src_unsigned = FALSE; dst_unsigned = TRUE;
                when '10' src_unsigned = FALSE; dst_unsigned = FALSE;
                when '11' src_unsigned = TRUE;  dst_unsigned = TRUE;

    __encoding aarch64_vector_shift_left_sat_simd
        __instruction_set A64
        __field Q 30 +: 1
        __field U 29 +: 1
        __field immh 19 +: 4
        __field immb 16 +: 3
        __field op 12 +: 1
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode '0xx01111 0xxxxxxx 011x01xx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);

            if immh == '0000' then SEE(asimdimm);
            if immh<3>:Q == '10' then UNDEFINED;
            integer esize = 8 << HighestSetBit(immh);
            integer datasize = if Q == '1' then 128 else 64;
            integer elements = datasize DIV esize;

            integer shift = UInt(immh:immb) - esize;

            boolean src_unsigned;
            boolean dst_unsigned;
            case op:U of
                when '00' UNDEFINED;
                when '01' src_unsigned = FALSE; dst_unsigned = TRUE;
                when '10' src_unsigned = FALSE; dst_unsigned = FALSE;
                when '11' src_unsigned = TRUE;  dst_unsigned = TRUE;

    __execute
        CheckFPAdvSIMDEnabled64();
        bits(datasize) operand  = V[n];
        bits(datasize) result;
        integer element;
        boolean sat;

        for e = 0 to elements-1
            element = Int(Elem[operand, e, esize], src_unsigned) << shift;
            (Elem[result, e, esize], sat) = SatQ(element, esize, dst_unsigned);
            if sat then FPSR.QC = '1';

        V[d] = result;
