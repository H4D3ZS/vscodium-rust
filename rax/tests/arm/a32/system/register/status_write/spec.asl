__instruction aarch32_MSR_r_AS
    __encoding aarch32_MSR_r_A1_AS
        __instruction_set A32
        __field cond 28 +: 4
        __field R 22 +: 1
        __field mask 16 +: 4
        __field Rn 0 +: 4
        __opcode 'xxxx0001 0x10xxxx xxxxxx0x 0000xxxx'
        __guard cond != '1111'
        __decode
            n = UInt(Rn);  write_spsr = (R == '1');
            if mask == '0000' then UNPREDICTABLE;
            if n == 15 then UNPREDICTABLE;

    __encoding aarch32_MSR_r_T1_AS
        __instruction_set T32
        __field R 20 +: 1
        __field Rn 16 +: 4
        __field mask 8 +: 4
        __opcode '11110011 100xxxxx 10x0xxxx xx0xxxxx'
        __guard TRUE
        __decode
            n = UInt(Rn);  write_spsr = (R == '1');
            if mask == '0000' then UNPREDICTABLE;
            if n == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        if write_spsr then
            if PSTATE.M IN {M32_User,M32_System} then
                UNPREDICTABLE;
            else
                SPSRWriteByInstr(R[n], mask);
        else
            // Attempts to change to an illegal mode will invoke the Illegal Execution state mechanism
            CPSRWriteByInstr(R[n], mask);

__instruction aarch32_MSR_i_AS
    __encoding aarch32_MSR_i_A1_AS
        __instruction_set A32
        __field cond 28 +: 4
        __field R 22 +: 1
        __field mask 16 +: 4
        __field imm12 0 +: 12
        __opcode 'xxxx0011 0x10xxxx xxxxxxxx xxxxxxxx'
        __guard cond != '1111'
        __decode
            if mask == '0000' && R == '0' then SEE "Related encodings";
            imm32 = A32ExpandImm(imm12);  write_spsr = (R == '1');
            if mask == '0000' then UNPREDICTABLE;

    __execute __conditional
        if write_spsr then
            if PSTATE.M IN {M32_User,M32_System} then
                UNPREDICTABLE;
            else
                SPSRWriteByInstr(imm32, mask);
        else
            // Attempts to change to an illegal mode will invoke the Illegal Execution state mechanism
            CPSRWriteByInstr(imm32, mask);

__instruction aarch32_MSR_br_AS
    __encoding aarch32_MSR_br_A1_AS
        __instruction_set A32
        __field cond 28 +: 4
        __field R 22 +: 1
        __field M1 16 +: 4
        __field M 8 +: 1
        __field Rn 0 +: 4
        __opcode 'xxxx0001 0x10xxxx xxxxxx1x 0000xxxx'
        __guard cond != '1111'
        __decode
            n = UInt(Rn);  write_spsr = (R == '1');
            if n == 15 then UNPREDICTABLE;
            SYSm = M:M1;

    __encoding aarch32_MSR_br_T1_AS
        __instruction_set T32
        __field R 20 +: 1
        __field Rn 16 +: 4
        __field M1 8 +: 4
        __field M 4 +: 1
        __opcode '11110011 100xxxxx 10x0xxxx xx1xxxxx'
        __guard TRUE
        __decode
            n = UInt(Rn);  write_spsr = (R == '1');
            if n == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13
            SYSm = M:M1;

    __execute __conditional
        if PSTATE.EL == EL0 then
            UNPREDICTABLE;
        else
            mode = PSTATE.M;
            if write_spsr then
                SPSRaccessValid(SYSm, mode);             // Check for UNPREDICTABLE cases
                case SYSm of
                    when '01110'  SPSR_fiq = R[n];
                    when '10000'  SPSR_irq = R[n];
                    when '10010'  SPSR_svc = R[n];
                    when '10100'  SPSR_abt = R[n];
                    when '10110'  SPSR_und = R[n];
                    when '11100'
                        if !ELUsingAArch32(EL3) then AArch64.MonitorModeTrap();
                        SPSR_mon = R[n];
                    when '11110'  SPSR_hyp = R[n];
            else
                BankedRegisterAccessValid(SYSm, mode);  // Check for UNPREDICTABLE cases
                case SYSm of
                    when '00xxx'                       // Access the User mode registers
                        m = UInt(SYSm<2:0>) + 8;
                        Rmode[m,M32_User] = R[n];
                    when '01xxx'                       // Access the FIQ mode registers
                        m = UInt(SYSm<2:0>) + 8;
                        Rmode[m,M32_FIQ] = R[n];
                    when '1000x'                       // Access the IRQ mode registers
                        m = 14 - UInt(SYSm<0>);        // LR when SYSm<0> == 0, otherwise SP
                        Rmode[m,M32_IRQ] = R[n];
                    when '1001x'                       // Access the Supervisor mode registers
                        m = 14 - UInt(SYSm<0>);        // LR when SYSm<0> == 0, otherwise SP
                        Rmode[m,M32_Svc] = R[n];
                    when '1010x'                       // Access the Abort mode registers
                        m = 14 - UInt(SYSm<0>);        // LR when SYSm<0> == 0, otherwise SP
                        Rmode[m,M32_Abort] = R[n];
                    when '1011x'                       // Access the Undefined mode registers
                        m = 14 - UInt(SYSm<0>);        // LR when SYSm<0> == 0, otherwise SP
                        Rmode[m,M32_Undef] = R[n];
                    when '1110x'                       // Access Monitor registers
                        if !ELUsingAArch32(EL3) then AArch64.MonitorModeTrap();
                        m = 14 - UInt(SYSm<0>);        // LR when SYSm<0> == 0, otherwise SP
                        Rmode[m,M32_Monitor] = R[n];
                    when '11110'                       // Access ELR_hyp register
                        ELR_hyp = R[n];
                    when '11111'                       // Access SP_hyp register
                        Rmode[13,M32_Hyp] = R[n];
