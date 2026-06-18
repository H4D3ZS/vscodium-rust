__instruction LDFF1B_Z.P.AI_S
    __encoding LDFF1B_Z.P.AI_S
        __instruction_set A64
        __field imm5 16 +: 5
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10000100 001xxxxx 111xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Zn);
            integer g = UInt(Pg);
            integer esize = 32;
            integer msize = 8;
            boolean unsigned = TRUE;
            integer offset = UInt(imm5);

    __encoding LDFF1B_Z.P.AI_D
        __instruction_set A64
        __field imm5 16 +: 5
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zt 0 +: 5
        __opcode '11000100 001xxxxx 111xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Zn);
            integer g = UInt(Pg);
            integer esize = 64;
            integer msize = 8;
            boolean unsigned = TRUE;
            integer offset = UInt(imm5);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) base = Z[n];
        bits(64) addr;
        bits(PL) mask = P[g];
        bits(VL) result;
        bits(VL) orig = Z[t];
        bits(msize) data;
        constant integer mbytes = msize DIV 8;
        boolean first = TRUE;
        boolean fault = FALSE;
        boolean faulted = FALSE;
        boolean unknown = FALSE;

        if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                addr = ZeroExtend(Elem[base, e, esize], 64) + offset * mbytes;
                if first then
                    // Mem[] will not return if a fault is detected for the first active element
                    data = Mem[addr, mbytes, AccType_NORMAL];
                    first = FALSE;
                else
                    // MemNF[] will return fault=TRUE if access is not performed for any reason
                    (data, fault) = MemNF[addr, mbytes, AccType_NONFAULT];
            else
                (data, fault) = (Zeros(msize), FALSE);

            // FFR elements set to FALSE following a supressed access/fault
            faulted = faulted || fault;
            if faulted then
                ElemFFR[e, esize] = '0';

            // Value becomes CONSTRAINED UNPREDICTABLE after an FFR element is FALSE
            unknown = unknown || ElemFFR[e, esize] == '0';
            if unknown then
                if !fault && ConstrainUnpredictableBool(Unpredictable_SVELDNFDATA) then
                    Elem[result, e, esize] = Extend(data, esize, unsigned);
                elsif ConstrainUnpredictableBool(Unpredictable_SVELDNFZERO) then
                    Elem[result, e, esize] = Zeros();
                else  // merge
                    Elem[result, e, esize] = Elem[orig, e, esize];
            else
                Elem[result, e, esize] = Extend(data, esize, unsigned);

        Z[t] = result;

__instruction LDFF1H_Z.P.AI_S
    __encoding LDFF1H_Z.P.AI_S
        __instruction_set A64
        __field imm5 16 +: 5
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10000100 101xxxxx 111xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Zn);
            integer g = UInt(Pg);
            integer esize = 32;
            integer msize = 16;
            boolean unsigned = TRUE;
            integer offset = UInt(imm5);

    __encoding LDFF1H_Z.P.AI_D
        __instruction_set A64
        __field imm5 16 +: 5
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zt 0 +: 5
        __opcode '11000100 101xxxxx 111xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Zn);
            integer g = UInt(Pg);
            integer esize = 64;
            integer msize = 16;
            boolean unsigned = TRUE;
            integer offset = UInt(imm5);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) base = Z[n];
        bits(64) addr;
        bits(PL) mask = P[g];
        bits(VL) result;
        bits(VL) orig = Z[t];
        bits(msize) data;
        constant integer mbytes = msize DIV 8;
        boolean first = TRUE;
        boolean fault = FALSE;
        boolean faulted = FALSE;
        boolean unknown = FALSE;

        if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                addr = ZeroExtend(Elem[base, e, esize], 64) + offset * mbytes;
                if first then
                    // Mem[] will not return if a fault is detected for the first active element
                    data = Mem[addr, mbytes, AccType_NORMAL];
                    first = FALSE;
                else
                    // MemNF[] will return fault=TRUE if access is not performed for any reason
                    (data, fault) = MemNF[addr, mbytes, AccType_NONFAULT];
            else
                (data, fault) = (Zeros(msize), FALSE);

            // FFR elements set to FALSE following a supressed access/fault
            faulted = faulted || fault;
            if faulted then
                ElemFFR[e, esize] = '0';

            // Value becomes CONSTRAINED UNPREDICTABLE after an FFR element is FALSE
            unknown = unknown || ElemFFR[e, esize] == '0';
            if unknown then
                if !fault && ConstrainUnpredictableBool(Unpredictable_SVELDNFDATA) then
                    Elem[result, e, esize] = Extend(data, esize, unsigned);
                elsif ConstrainUnpredictableBool(Unpredictable_SVELDNFZERO) then
                    Elem[result, e, esize] = Zeros();
                else  // merge
                    Elem[result, e, esize] = Elem[orig, e, esize];
            else
                Elem[result, e, esize] = Extend(data, esize, unsigned);

        Z[t] = result;

__instruction LDFF1D_Z.P.AI_D
    __encoding LDFF1D_Z.P.AI_D
        __instruction_set A64
        __field imm5 16 +: 5
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zt 0 +: 5
        __opcode '11000101 101xxxxx 111xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Zn);
            integer g = UInt(Pg);
            integer esize = 64;
            integer msize = 64;
            boolean unsigned = TRUE;
            integer offset = UInt(imm5);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) base = Z[n];
        bits(64) addr;
        bits(PL) mask = P[g];
        bits(VL) result;
        bits(VL) orig = Z[t];
        bits(msize) data;
        constant integer mbytes = msize DIV 8;
        boolean first = TRUE;
        boolean fault = FALSE;
        boolean faulted = FALSE;
        boolean unknown = FALSE;

        if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                addr = ZeroExtend(Elem[base, e, esize], 64) + offset * mbytes;
                if first then
                    // Mem[] will not return if a fault is detected for the first active element
                    data = Mem[addr, mbytes, AccType_NORMAL];
                    first = FALSE;
                else
                    // MemNF[] will return fault=TRUE if access is not performed for any reason
                    (data, fault) = MemNF[addr, mbytes, AccType_NONFAULT];
            else
                (data, fault) = (Zeros(msize), FALSE);

            // FFR elements set to FALSE following a supressed access/fault
            faulted = faulted || fault;
            if faulted then
                ElemFFR[e, esize] = '0';

            // Value becomes CONSTRAINED UNPREDICTABLE after an FFR element is FALSE
            unknown = unknown || ElemFFR[e, esize] == '0';
            if unknown then
                if !fault && ConstrainUnpredictableBool(Unpredictable_SVELDNFDATA) then
                    Elem[result, e, esize] = Extend(data, esize, unsigned);
                elsif ConstrainUnpredictableBool(Unpredictable_SVELDNFZERO) then
                    Elem[result, e, esize] = Zeros();
                else  // merge
                    Elem[result, e, esize] = Elem[orig, e, esize];
            else
                Elem[result, e, esize] = Extend(data, esize, unsigned);

        Z[t] = result;

__instruction LDFF1SB_Z.P.AI_S
    __encoding LDFF1SB_Z.P.AI_S
        __instruction_set A64
        __field imm5 16 +: 5
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10000100 001xxxxx 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Zn);
            integer g = UInt(Pg);
            integer esize = 32;
            integer msize = 8;
            boolean unsigned = FALSE;
            integer offset = UInt(imm5);

    __encoding LDFF1SB_Z.P.AI_D
        __instruction_set A64
        __field imm5 16 +: 5
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zt 0 +: 5
        __opcode '11000100 001xxxxx 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Zn);
            integer g = UInt(Pg);
            integer esize = 64;
            integer msize = 8;
            boolean unsigned = FALSE;
            integer offset = UInt(imm5);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) base = Z[n];
        bits(64) addr;
        bits(PL) mask = P[g];
        bits(VL) result;
        bits(VL) orig = Z[t];
        bits(msize) data;
        constant integer mbytes = msize DIV 8;
        boolean first = TRUE;
        boolean fault = FALSE;
        boolean faulted = FALSE;
        boolean unknown = FALSE;

        if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                addr = ZeroExtend(Elem[base, e, esize], 64) + offset * mbytes;
                if first then
                    // Mem[] will not return if a fault is detected for the first active element
                    data = Mem[addr, mbytes, AccType_NORMAL];
                    first = FALSE;
                else
                    // MemNF[] will return fault=TRUE if access is not performed for any reason
                    (data, fault) = MemNF[addr, mbytes, AccType_NONFAULT];
            else
                (data, fault) = (Zeros(msize), FALSE);

            // FFR elements set to FALSE following a supressed access/fault
            faulted = faulted || fault;
            if faulted then
                ElemFFR[e, esize] = '0';

            // Value becomes CONSTRAINED UNPREDICTABLE after an FFR element is FALSE
            unknown = unknown || ElemFFR[e, esize] == '0';
            if unknown then
                if !fault && ConstrainUnpredictableBool(Unpredictable_SVELDNFDATA) then
                    Elem[result, e, esize] = Extend(data, esize, unsigned);
                elsif ConstrainUnpredictableBool(Unpredictable_SVELDNFZERO) then
                    Elem[result, e, esize] = Zeros();
                else  // merge
                    Elem[result, e, esize] = Elem[orig, e, esize];
            else
                Elem[result, e, esize] = Extend(data, esize, unsigned);

        Z[t] = result;

__instruction LDFF1SH_Z.P.AI_S
    __encoding LDFF1SH_Z.P.AI_S
        __instruction_set A64
        __field imm5 16 +: 5
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10000100 101xxxxx 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Zn);
            integer g = UInt(Pg);
            integer esize = 32;
            integer msize = 16;
            boolean unsigned = FALSE;
            integer offset = UInt(imm5);

    __encoding LDFF1SH_Z.P.AI_D
        __instruction_set A64
        __field imm5 16 +: 5
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zt 0 +: 5
        __opcode '11000100 101xxxxx 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Zn);
            integer g = UInt(Pg);
            integer esize = 64;
            integer msize = 16;
            boolean unsigned = FALSE;
            integer offset = UInt(imm5);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) base = Z[n];
        bits(64) addr;
        bits(PL) mask = P[g];
        bits(VL) result;
        bits(VL) orig = Z[t];
        bits(msize) data;
        constant integer mbytes = msize DIV 8;
        boolean first = TRUE;
        boolean fault = FALSE;
        boolean faulted = FALSE;
        boolean unknown = FALSE;

        if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                addr = ZeroExtend(Elem[base, e, esize], 64) + offset * mbytes;
                if first then
                    // Mem[] will not return if a fault is detected for the first active element
                    data = Mem[addr, mbytes, AccType_NORMAL];
                    first = FALSE;
                else
                    // MemNF[] will return fault=TRUE if access is not performed for any reason
                    (data, fault) = MemNF[addr, mbytes, AccType_NONFAULT];
            else
                (data, fault) = (Zeros(msize), FALSE);

            // FFR elements set to FALSE following a supressed access/fault
            faulted = faulted || fault;
            if faulted then
                ElemFFR[e, esize] = '0';

            // Value becomes CONSTRAINED UNPREDICTABLE after an FFR element is FALSE
            unknown = unknown || ElemFFR[e, esize] == '0';
            if unknown then
                if !fault && ConstrainUnpredictableBool(Unpredictable_SVELDNFDATA) then
                    Elem[result, e, esize] = Extend(data, esize, unsigned);
                elsif ConstrainUnpredictableBool(Unpredictable_SVELDNFZERO) then
                    Elem[result, e, esize] = Zeros();
                else  // merge
                    Elem[result, e, esize] = Elem[orig, e, esize];
            else
                Elem[result, e, esize] = Extend(data, esize, unsigned);

        Z[t] = result;

__instruction LDFF1SW_Z.P.AI_D
    __encoding LDFF1SW_Z.P.AI_D
        __instruction_set A64
        __field imm5 16 +: 5
        __field Pg 10 +: 3
        __field Zn 5 +: 5
        __field Zt 0 +: 5
        __opcode '11000101 001xxxxx 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Zn);
            integer g = UInt(Pg);
            integer esize = 64;
            integer msize = 32;
            boolean unsigned = FALSE;
            integer offset = UInt(imm5);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(VL) base = Z[n];
        bits(64) addr;
        bits(PL) mask = P[g];
        bits(VL) result;
        bits(VL) orig = Z[t];
        bits(msize) data;
        constant integer mbytes = msize DIV 8;
        boolean first = TRUE;
        boolean fault = FALSE;
        boolean faulted = FALSE;
        boolean unknown = FALSE;

        if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                addr = ZeroExtend(Elem[base, e, esize], 64) + offset * mbytes;
                if first then
                    // Mem[] will not return if a fault is detected for the first active element
                    data = Mem[addr, mbytes, AccType_NORMAL];
                    first = FALSE;
                else
                    // MemNF[] will return fault=TRUE if access is not performed for any reason
                    (data, fault) = MemNF[addr, mbytes, AccType_NONFAULT];
            else
                (data, fault) = (Zeros(msize), FALSE);

            // FFR elements set to FALSE following a supressed access/fault
            faulted = faulted || fault;
            if faulted then
                ElemFFR[e, esize] = '0';

            // Value becomes CONSTRAINED UNPREDICTABLE after an FFR element is FALSE
            unknown = unknown || ElemFFR[e, esize] == '0';
            if unknown then
                if !fault && ConstrainUnpredictableBool(Unpredictable_SVELDNFDATA) then
                    Elem[result, e, esize] = Extend(data, esize, unsigned);
                elsif ConstrainUnpredictableBool(Unpredictable_SVELDNFZERO) then
                    Elem[result, e, esize] = Zeros();
                else  // merge
                    Elem[result, e, esize] = Elem[orig, e, esize];
            else
                Elem[result, e, esize] = Extend(data, esize, unsigned);

        Z[t] = result;
