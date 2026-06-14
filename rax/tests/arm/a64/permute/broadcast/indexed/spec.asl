__instruction DUP_Z.Zi__
    __encoding DUP_Z.Zi__
        __instruction_set A64
        __field imm2 22 +: 2
        __field tsz 16 +: 5
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '00000101 xx1xxxxx 001000xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            bits(7) imm = imm2:tsz;
            case tsz of
                when '00000' UNDEFINED;
                when '10000' esize = 128; index = UInt(imm<6:5>);
                when 'x1000' esize = 64;  index = UInt(imm<6:4>);
                when 'xx100' esize = 32;  index = UInt(imm<6:3>);
                when 'xxx10' esize = 16;  index = UInt(imm<6:2>);
                when 'xxxx1' esize = 8;   index = UInt(imm<6:1>);
            integer n = UInt(Zn);
            integer d = UInt(Zd);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) operand1 = Z[n];
        bits(VL) result;
        bits(esize) element;

        if index >= elements then
            element = Zeros();
        else
            element = Elem[operand1, index, esize];
        result = Replicate(element);

        Z[d] = result;
