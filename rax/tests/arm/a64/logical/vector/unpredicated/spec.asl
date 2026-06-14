__instruction AND_Z.ZZ__
    __encoding AND_Z.ZZ__
        __instruction_set A64
        __field Zm 16 +: 5
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '00000100 001xxxxx 001100xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer n = UInt(Zn);
            integer m = UInt(Zm);
            integer d = UInt(Zd);

    __execute
        CheckSVEEnabled();
        bits(VL) operand1 = Z[n];
        bits(VL) operand2 = Z[m];

        Z[d] = operand1 AND operand2;

__instruction ORR_Z.ZZ__
    __encoding ORR_Z.ZZ__
        __instruction_set A64
        __field Zm 16 +: 5
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '00000100 011xxxxx 001100xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer n = UInt(Zn);
            integer m = UInt(Zm);
            integer d = UInt(Zd);

    __execute
        CheckSVEEnabled();
        bits(VL) operand1 = Z[n];
        bits(VL) operand2 = Z[m];

        Z[d] = operand1 OR operand2;

__instruction EOR_Z.ZZ__
    __encoding EOR_Z.ZZ__
        __instruction_set A64
        __field Zm 16 +: 5
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '00000100 101xxxxx 001100xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer n = UInt(Zn);
            integer m = UInt(Zm);
            integer d = UInt(Zd);

    __execute
        CheckSVEEnabled();
        bits(VL) operand1 = Z[n];
        bits(VL) operand2 = Z[m];

        Z[d] = operand1 EOR operand2;

__instruction BIC_Z.ZZ__
    __encoding BIC_Z.ZZ__
        __instruction_set A64
        __field Zm 16 +: 5
        __field Zn 5 +: 5
        __field Zd 0 +: 5
        __opcode '00000100 111xxxxx 001100xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer n = UInt(Zn);
            integer m = UInt(Zm);
            integer d = UInt(Zd);

    __execute
        CheckSVEEnabled();
        bits(VL) operand1 = Z[n];
        bits(VL) operand2 = Z[m];

        Z[d] = operand1 AND (NOT operand2);
