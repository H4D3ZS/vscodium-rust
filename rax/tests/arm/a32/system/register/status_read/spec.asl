__instruction aarch32_MRS_AS
    __encoding aarch32_MRS_A1_AS
        __instruction_set A32
        __field cond 28 +: 4
        __field R 22 +: 1
        __field Rd 12 +: 4
        __opcode 'xxxx0001 0x00xxxx xxxxxx0x 0000xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  read_spsr = (R == '1');
            if d == 15 then UNPREDICTABLE;

    __encoding aarch32_MRS_T1_AS
        __instruction_set T32
        __field R 20 +: 1
        __field Rd 8 +: 4
        __opcode '11110011 111xxxxx 10x0xxxx xx0xxxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  read_spsr = (R == '1');
            if d == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13

    __execute __conditional
        if read_spsr then
            if PSTATE.M IN {M32_User,M32_System} then
                UNPREDICTABLE;
            else
                R[d] = SPSR[];
        else
            // CPSR has same bit assignments as SPSR, but with the IT, J, SS, IL, and T bits masked out.
            bits(32) mask = '11111000 00001111 00000011 11011111';
            if HavePANExt() then
                mask<22> = '1';
            psr_val = GetPSRFromPSTATE() AND mask;
            if PSTATE.EL == EL0 then
                // If accessed from User mode return UNKNOWN values for E, A, I, F bits, bits<9:6>,
                // and for the M field, bits<4:0>
                psr_val<22> = bits(1) UNKNOWN;
                psr_val<9:6> = bits(4) UNKNOWN;
                psr_val<4:0> = bits(5) UNKNOWN;
            R[d] = psr_val;

__instruction aarch32_MRS_br_AS
    __encoding aarch32_MRS_br_A1_AS
        __instruction_set A32
        __field cond 28 +: 4
        __field R 22 +: 1
        __field M1 16 +: 4
        __field Rd 12 +: 4
        __field M 8 +: 1
        __opcode 'xxxx0001 0x00xxxx xxxxxx1x 0000xxxx'
        __guard cond != '1111'
        __decode
            d = UInt(Rd);  read_spsr = (R == '1');
            if d == 15 then UNPREDICTABLE;
            SYSm = M:M1;

    __encoding aarch32_MRS_br_T1_AS
        __instruction_set T32
        __field R 20 +: 1
        __field M1 16 +: 4
        __field Rd 8 +: 4
        __field M 4 +: 1
        __opcode '11110011 111xxxxx 10x0xxxx xx1xxxxx'
        __guard TRUE
        __decode
            d = UInt(Rd);  read_spsr = (R == '1');
            if d == 15 then UNPREDICTABLE; // Armv8-A removes UNPREDICTABLE for R13
            SYSm = M:M1;

    __execute __conditional
        if PSTATE.EL == EL0 then
           UNPREDICTABLE;
        else
            mode = PSTATE.M;
            if read_spsr then
                SPSRaccessValid(SYSm, mode);           // Check for UNPREDICTABLE cases
                case SYSm of
                    when '01110'  R[d] = SPSR_fiq;
                    when '10000'  R[d] = SPSR_irq;
                    when '10010'  R[d] = SPSR_svc;
                    when '10100'  R[d] = SPSR_abt;
                    when '10110'  R[d] = SPSR_und;
                    when '11100'
                        if !ELUsingAArch32(EL3) then AArch64.MonitorModeTrap();
                        R[d] = SPSR_mon;
                    when '11110'  R[d] = SPSR_hyp;
            else
                BankedRegisterAccessValid(SYSm, mode); // Check for UNPREDICTABLE cases
                case SYSm of
                    when '00xxx'                       // Access the User mode registers
                        m = UInt(SYSm<2:0>) + 8;
                        R[d] = Rmode[m,M32_User];
                    when '01xxx'                       // Access the FIQ mode registers
                        m = UInt(SYSm<2:0>) + 8;
                        R[d] = Rmode[m,M32_FIQ];
                    when '1000x'                       // Access the IRQ mode registers
                        m = 14 - UInt(SYSm<0>);        // LR when SYSm<0> == 0, otherwise SP
                        R[d] = Rmode[m,M32_IRQ];
                    when '1001x'                       // Access the Supervisor mode registers
                        m = 14 - UInt(SYSm<0>);        // LR when SYSm<0> == 0, otherwise SP
                        R[d] = Rmode[m,M32_Svc];
                    when '1010x'                       // Access the Abort mode registers
                        m = 14 - UInt(SYSm<0>);        // LR when SYSm<0> == 0, otherwise SP
                        R[d] = Rmode[m,M32_Abort];
                    when '1011x'                       // Access the Undefined mode registers
                        m = 14 - UInt(SYSm<0>);        // LR when SYSm<0> == 0, otherwise SP
                        R[d] = Rmode[m,M32_Undef];
                    when '1110x'                       // Access Monitor registers
                        if !ELUsingAArch32(EL3) then AArch64.MonitorModeTrap();
                        m = 14 - UInt(SYSm<0>);        // LR when SYSm<0> == 0, otherwise SP
                        R[d] = Rmode[m,M32_Monitor];
                    when '11110'                       // Access ELR_hyp register
                        R[d] = ELR_hyp;
                    when '11111'                       // Access SP_hyp register
                        R[d] = Rmode[13,M32_Hyp];
