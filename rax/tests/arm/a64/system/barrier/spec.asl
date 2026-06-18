__instruction aarch64_system_barriers
    __encoding aarch64_system_barriers
        __instruction_set A64
        __field CRm 8 +: 4
        __field opc 5 +: 2
        __opcode '11010101 00000011 0011xxxx 1xx11111'
        __guard TRUE
        __decode
            MemBarrierOp op;
            MBReqDomain domain;
            MBReqTypes types;

            case opc of
                when '00' op = MemBarrierOp_DSB;
                when '01' op = MemBarrierOp_DMB;
                when '10' op = MemBarrierOp_ISB;
                otherwise
                    if HaveSBExt() && CRm<3:0> == '0000' then
                        op = MemBarrierOp_SB;
                    else
                        UNDEFINED;

            case CRm<3:2> of
                when '00' domain = MBReqDomain_OuterShareable;
                when '01' domain = MBReqDomain_Nonshareable;
                when '10' domain = MBReqDomain_InnerShareable;
                when '11' domain = MBReqDomain_FullSystem;

            case CRm<1:0> of
                when '01' types = MBReqTypes_Reads;
                when '10' types = MBReqTypes_Writes;
                when '11' types = MBReqTypes_All;
                otherwise
                    if CRm<3:2> == '01' then
                        op = MemBarrierOp_PSSBB;
                    elsif CRm<3:2> == '00' && opc == '00' then
                        op = MemBarrierOp_SSBB;
                    elsif HaveSBExt() && CRm<3:2> == '00' && opc == '11' then
                        op = MemBarrierOp_SB;
                    else
                        types  = MBReqTypes_All;
                        domain = MBReqDomain_FullSystem;

    __execute
        case op of
            when MemBarrierOp_DSB
                DataSynchronizationBarrier(domain, types);
            when MemBarrierOp_DMB
                DataMemoryBarrier(domain, types);
            when MemBarrierOp_ISB
                InstructionSynchronizationBarrier();
            when MemBarrierOp_SSBB
                SpeculativeStoreBypassBarrierToVA();
            when MemBarrierOp_PSSBB
                SpeculativeStoreBypassBarrierToPA();
            when MemBarrierOp_SB
                SpeculationBarrier();
