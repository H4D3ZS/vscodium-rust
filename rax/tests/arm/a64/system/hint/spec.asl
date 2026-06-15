__instruction aarch64_system_hints
    __encoding aarch64_system_hints
        __instruction_set A64
        __field CRm 8 +: 4
        __field op2 5 +: 3
        __opcode '11010101 00000011 0010xxxx xxx11111'
        __guard TRUE
        __decode
            SystemHintOp op;

            case CRm:op2 of
                when '0000 000' op = SystemHintOp_NOP;
                when '0000 001' op = SystemHintOp_YIELD;
                when '0000 010' op = SystemHintOp_WFE;
                when '0000 011' op = SystemHintOp_WFI;
                when '0000 100' op = SystemHintOp_SEV;
                when '0000 101' op = SystemHintOp_SEVL;
                when '0000 111'
                    SEE "XPACLRI";
                when '0001 xxx'
                    SEE "PACIA1716, PACIB1716, AUTIA1716, AUTIB1716";
                when '0010 000'
                    if !HaveRASExt() then EndOfInstruction();                  // Instruction executes as NOP
                    op = SystemHintOp_ESB;
                when '0010 001'
                    if !HaveStatisticalProfiling() then EndOfInstruction();    // Instruction executes as NOP
                    op = SystemHintOp_PSB;
                when '0010 010'
                    if !HaveSelfHostedTrace() then EndOfInstruction();         // Instruction executes as NOP
                    op = SystemHintOp_TSB;
                when '0010 100'
                    op = SystemHintOp_CSDB;
                when '0011 xxx'
                    SEE "PACIAZ, PACIASP, PACIBZ, PACIBSP, AUTIAZ, AUTIASP, AUTIBZ, AUTIBSP";
                when '0100 xx0'
                    op = SystemHintOp_BTI;
                    // Check branch target compatibility
                    // between BTI instruction and PSTATE.BTYPE
                    BTypeCompatible = BTypeCompatible_BTI(op2<2:1>);
                otherwise  EndOfInstruction();                                 // Instruction executes as NOP

    __execute
        case op of
            when SystemHintOp_YIELD
                Hint_Yield();

            when SystemHintOp_WFE
                if IsEventRegisterSet() then
                    ClearEventRegister();
                else
                    if PSTATE.EL == EL0 then
                        // Check for traps described by the OS which may be EL1 or EL2.
                        AArch64.CheckForWFxTrap(EL1, TRUE);
                    if EL2Enabled() && PSTATE.EL IN {EL0,EL1} && !IsInHost() then
                        // Check for traps described by the Hypervisor.
                        AArch64.CheckForWFxTrap(EL2, TRUE);
                    if HaveEL(EL3) && PSTATE.EL != EL3 then
                        // Check for traps described by the Secure Monitor.
                        AArch64.CheckForWFxTrap(EL3, TRUE);
                    WaitForEvent();

            when SystemHintOp_WFI
                if !InterruptPending() then
                    if PSTATE.EL == EL0 then
                        // Check for traps described by the OS which may be EL1 or EL2.
                        AArch64.CheckForWFxTrap(EL1, FALSE);
                    if EL2Enabled() && PSTATE.EL IN {EL0,EL1} && !IsInHost() then
                        // Check for traps described by the Hypervisor.
                        AArch64.CheckForWFxTrap(EL2, FALSE);
                    if HaveEL(EL3) && PSTATE.EL != EL3 then
                        // Check for traps described by the Secure Monitor.
                        AArch64.CheckForWFxTrap(EL3, FALSE);
                    WaitForInterrupt();

            when SystemHintOp_SEV
                SendEvent();

            when SystemHintOp_SEVL
                SendEventLocal();

            when SystemHintOp_ESB
                SynchronizeErrors();
                AArch64.ESBOperation();
                if EL2Enabled() && PSTATE.EL IN {EL0,EL1} then AArch64.vESBOperation();
                TakeUnmaskedSErrorInterrupts();

            when SystemHintOp_PSB
                ProfilingSynchronizationBarrier();

            when SystemHintOp_TSB
                TraceSynchronizationBarrier();

            when SystemHintOp_CSDB
                ConsumptionOfSpeculativeDataBarrier();

            when SystemHintOp_BTI
                BTypeNext = '00';

            otherwise // do nothing
