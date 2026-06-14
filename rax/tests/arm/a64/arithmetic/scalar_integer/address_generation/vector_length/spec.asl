__instruction ADDVL_R.RI__
    __encoding ADDVL_R.RI__
        __instruction_set A64
        __field Rn 16 +: 5
        __field imm6 5 +: 6
        __field Rd 0 +: 5
        __opcode '00000100 001xxxxx 01010xxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer n = UInt(Rn);
            integer d = UInt(Rd);
            integer imm = SInt(imm6);

    __execute
        CheckSVEEnabled();
        bits(64) operand1 = if n == 31 then SP[] else X[n];
        bits(64) result = operand1 + (imm * (VL DIV 8));

        if d == 31 then
            SP[] = result;
        else
            X[d] = result;

__instruction ADDPL_R.RI__
    __encoding ADDPL_R.RI__
        __instruction_set A64
        __field Rn 16 +: 5
        __field imm6 5 +: 6
        __field Rd 0 +: 5
        __opcode '00000100 011xxxxx 01010xxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer n = UInt(Rn);
            integer d = UInt(Rd);
            integer imm = SInt(imm6);

    __execute
        CheckSVEEnabled();
        bits(64) operand1 = if n == 31 then SP[] else X[n];
        bits(64) result = operand1 + (imm * (PL DIV 8));

        if d == 31 then
            SP[] = result;
        else
            X[d] = result;

__instruction RDVL_R.I__
    __encoding RDVL_R.I__
        __instruction_set A64
        __field imm6 5 +: 6
        __field Rd 0 +: 5
        __opcode '00000100 10111111 01010xxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer d = UInt(Rd);
            integer imm = SInt(imm6);

    __execute
        CheckSVEEnabled();
        integer len = imm * (VL DIV 8);
        X[d] = len<63:0>;
