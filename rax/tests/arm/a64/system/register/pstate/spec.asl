__instruction aarch64_system_register_cpsr
    __encoding aarch64_system_register_cpsr
        __instruction_set A64
        __field op1 16 +: 3
        __field CRm 8 +: 4
        __field op2 5 +: 3
        __opcode '11010101 00000xxx 0100xxxx xxx11111'
        __guard TRUE
        __decode
            if op1 == '000' && op2 == '000' then SEE "CFINV";
            if op1 == '000' && op2 == '001' then SEE "XAFlag";
            if op1 == '000' && op2 == '010' then SEE "AXFlag";

            AArch64.CheckSystemAccess('00', op1, '0100', CRm, op2, '11111', '0');

            bits(4) operand = CRm;
            PSTATEField field;
            case op1:op2 of
                when '000 011'
                    if !HaveUAOExt() then
                        UNDEFINED;
                    field = PSTATEField_UAO;
                when '000 100'
                    if !HavePANExt() then
                        UNDEFINED;
                    field = PSTATEField_PAN;
                when '000 101' field = PSTATEField_SP;
                when '011 010'
                    if !HaveDITExt() then
                        UNDEFINED;
                    field = PSTATEField_DIT;
                when '011 110' field = PSTATEField_DAIFSet;
                when '011 111' field = PSTATEField_DAIFClr;
                when '011 001'
                    if !HaveSSBSExt() then
                        UNDEFINED;
                    field = PSTATEField_SSBS;
                otherwise      UNDEFINED;

            // Check that an AArch64 MSR/MRS access to the DAIF flags is permitted
            if op1 == '011' && PSTATE.EL == EL0 && (IsInHost() || SCTLR_EL1.UMA == '0') then
                AArch64.SystemAccessTrap(EL1, 0x18);    // Exception_SystemRegisterTrap

    __execute
        case field of
            when PSTATEField_SSBS
                PSTATE.SSBS = operand<0>;
            when PSTATEField_SP
                PSTATE.SP = operand<0>;
            when PSTATEField_DAIFSet
                PSTATE.D = PSTATE.D OR operand<3>;
                PSTATE.A = PSTATE.A OR operand<2>;
                PSTATE.I = PSTATE.I OR operand<1>;
                PSTATE.F = PSTATE.F OR operand<0>;
            when PSTATEField_DAIFClr
                PSTATE.D = PSTATE.D AND NOT(operand<3>);
                PSTATE.A = PSTATE.A AND NOT(operand<2>);
                PSTATE.I = PSTATE.I AND NOT(operand<1>);
                PSTATE.F = PSTATE.F AND NOT(operand<0>);
            when PSTATEField_PAN
                PSTATE.PAN = operand<0>;
            when PSTATEField_UAO
                PSTATE.UAO = operand<0>;
            when PSTATEField_DIT
                PSTATE.DIT = operand<0>;
