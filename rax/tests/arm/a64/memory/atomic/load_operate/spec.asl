__instruction aarch64_memory_atomicops_ld
    __encoding aarch64_memory_atomicops_ld
        __instruction_set A64
        __field size 30 +: 2
        __field A 23 +: 1
        __field R 22 +: 1
        __field Rs 16 +: 5
        __field opc 12 +: 3
        __field Rn 5 +: 5
        __field Rt 0 +: 5
        __opcode 'xx111000 xx1xxxxx 0xxx00xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveAtomicExt() then UNDEFINED;
            integer t = UInt(Rt);
            integer n = UInt(Rn);
            integer s = UInt(Rs);

            integer datasize = 8 << UInt(size);
            integer regsize = if datasize == 64 then 64 else 32;
            AccType ldacctype = if A == '1' && Rt != '11111' then AccType_ORDEREDATOMICRW else AccType_ATOMICRW;
            AccType stacctype = if R == '1' then AccType_ORDEREDATOMICRW else AccType_ATOMICRW;
            MemAtomicOp op;
            case opc of
                when '000' op = MemAtomicOp_ADD;
                when '001' op = MemAtomicOp_BIC;
                when '010' op = MemAtomicOp_EOR;
                when '011' op = MemAtomicOp_ORR;
                when '100' op = MemAtomicOp_SMAX;
                when '101' op = MemAtomicOp_SMIN;
                when '110' op = MemAtomicOp_UMAX;
                when '111' op = MemAtomicOp_UMIN;
            boolean tag_checked = n != 31;

    __execute
        bits(64) address;
        bits(datasize) value;
        bits(datasize) data;
        bits(datasize) result;

        if HaveMTEExt() then
            SetNotTagCheckedInstruction(!tag_checked);

        value = X[s];
        if n == 31 then
            CheckSPAlignment();
            address = SP[];
        else
            address = X[n];

        // All observers in the shareability domain observe the
        // following load and store atomically.
        data =  Mem[address, datasize DIV 8, ldacctype];

        case op of
            when MemAtomicOp_ADD   result = data + value;
            when MemAtomicOp_BIC   result = data AND NOT(value);
            when MemAtomicOp_EOR   result = data EOR value;
            when MemAtomicOp_ORR   result = data OR value;
            when MemAtomicOp_SMAX  result = if SInt(data) > SInt(value) then data else value;
            when MemAtomicOp_SMIN  result = if SInt(data) > SInt(value) then value else data;
            when MemAtomicOp_UMAX  result = if UInt(data) > UInt(value) then data else value;
            when MemAtomicOp_UMIN  result = if UInt(data) > UInt(value) then value else data;

        Mem[address, datasize DIV 8, stacctype] = result;

        if t != 31 then
            X[t] = ZeroExtend(data, regsize);
