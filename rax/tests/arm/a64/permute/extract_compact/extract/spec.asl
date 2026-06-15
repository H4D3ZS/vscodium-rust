__instruction EXT_Z.ZI_Des
    __encoding EXT_Z.ZI_Des
        __instruction_set A64
        __field imm8h 16 +: 5
        __field imm8l 10 +: 3
        __field Zm 5 +: 5
        __field Zdn 0 +: 5
        __opcode '00000101 001xxxxx 000xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8;
            integer dst = UInt(Zdn);
            integer s1 = dst;
            integer s2 = UInt(Zm);
            integer position = UInt(imm8h:imm8l);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) operand1 = Z[s1];
        bits(VL) operand2 = Z[s2];
        bits(VL) result;

        if position >= elements then
            position = 0;

        position = position << 3;
        bits(VL*2) concat = operand2 : operand1;
        result = concat<position+VL-1:position>;

        Z[dst] = result;
