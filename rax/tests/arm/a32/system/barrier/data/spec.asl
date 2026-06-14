__instruction aarch32_DSB_A
    __encoding aarch32_DSB_A1_A
        __instruction_set A32
        __field option 0 +: 4
        __opcode '11110101 0111xxxx xxxxxxxx 0100xxxx'
        __guard TRUE
        __decode
            // No additional decoding required

    __encoding aarch32_DSB_T1_A
        __instruction_set T32
        __field option 0 +: 4
        __opcode '11110011 1011xxxx 10x0xxxx 0100xxxx'
        __guard TRUE
        __decode
            // No additional decoding required

    __execute __conditional
        case option of
            when '0001'  domain = MBReqDomain_OuterShareable;  types = MBReqTypes_Reads;
            when '0010'  domain = MBReqDomain_OuterShareable;  types = MBReqTypes_Writes;
            when '0011'  domain = MBReqDomain_OuterShareable;  types = MBReqTypes_All;
            when '0101'  domain = MBReqDomain_Nonshareable;    types = MBReqTypes_Reads;
            when '0110'  domain = MBReqDomain_Nonshareable;    types = MBReqTypes_Writes;
            when '0111'  domain = MBReqDomain_Nonshareable;    types = MBReqTypes_All;
            when '1001'  domain = MBReqDomain_InnerShareable;  types = MBReqTypes_Reads;
            when '1010'  domain = MBReqDomain_InnerShareable;  types = MBReqTypes_Writes;
            when '1011'  domain = MBReqDomain_InnerShareable;  types = MBReqTypes_All;
            when '1101'  domain = MBReqDomain_FullSystem;      types = MBReqTypes_Reads;
            when '1110'  domain = MBReqDomain_FullSystem;      types = MBReqTypes_Writes;
            otherwise
                if       option == '0000' then SEE "SSBB";
                elsif    option == '0100' then SEE "PSSBB";
                else     domain = MBReqDomain_FullSystem;      types = MBReqTypes_All;

        if EL2Enabled() && PSTATE.EL IN {EL0,EL1} then
            if HCR.BSU == '11' then
                domain = MBReqDomain_FullSystem;
            if HCR.BSU == '10' && domain != MBReqDomain_FullSystem then
                domain = MBReqDomain_OuterShareable;
            if HCR.BSU == '01' && domain == MBReqDomain_Nonshareable then
                domain = MBReqDomain_InnerShareable;

        DataSynchronizationBarrier(domain, types);

__instruction aarch32_DMB_A
    __encoding aarch32_DMB_A1_A
        __instruction_set A32
        __field option 0 +: 4
        __opcode '11110101 0111xxxx xxxxxxxx 0101xxxx'
        __guard TRUE
        __decode
            // No additional decoding required

    __encoding aarch32_DMB_T1_A
        __instruction_set T32
        __field option 0 +: 4
        __opcode '11110011 1011xxxx 10x0xxxx 0101xxxx'
        __guard TRUE
        __decode
            // No additional decoding required

    __execute __conditional
        case option of
            when '0001'  domain = MBReqDomain_OuterShareable;  types = MBReqTypes_Reads;
            when '0010'  domain = MBReqDomain_OuterShareable;  types = MBReqTypes_Writes;
            when '0011'  domain = MBReqDomain_OuterShareable;  types = MBReqTypes_All;
            when '0101'  domain = MBReqDomain_Nonshareable;    types = MBReqTypes_Reads;
            when '0110'  domain = MBReqDomain_Nonshareable;    types = MBReqTypes_Writes;
            when '0111'  domain = MBReqDomain_Nonshareable;    types = MBReqTypes_All;
            when '1001'  domain = MBReqDomain_InnerShareable;  types = MBReqTypes_Reads;
            when '1010'  domain = MBReqDomain_InnerShareable;  types = MBReqTypes_Writes;
            when '1011'  domain = MBReqDomain_InnerShareable;  types = MBReqTypes_All;
            when '1101'  domain = MBReqDomain_FullSystem;      types = MBReqTypes_Reads;
            when '1110'  domain = MBReqDomain_FullSystem;      types = MBReqTypes_Writes;
            otherwise    domain = MBReqDomain_FullSystem;      types = MBReqTypes_All;

        if EL2Enabled() && PSTATE.EL IN {EL0,EL1} then
            if HCR.BSU == '11' then
                domain = MBReqDomain_FullSystem;
            if HCR.BSU == '10' && domain != MBReqDomain_FullSystem then
                domain = MBReqDomain_OuterShareable;
            if HCR.BSU == '01' && domain == MBReqDomain_Nonshareable then
                domain = MBReqDomain_InnerShareable;

        DataMemoryBarrier(domain, types);
