__instruction LDFF1B_Z.P.BR_U8
    __encoding LDFF1B_Z.P.BR_U8
        __instruction_set A64
        __field Rm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10100100 000xxxxx 011xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer g = UInt(Pg);
            integer esize = 8;
            integer msize = 8;
            boolean unsigned = TRUE;

    __encoding LDFF1B_Z.P.BR_U16
        __instruction_set A64
        __field Rm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10100100 001xxxxx 011xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer g = UInt(Pg);
            integer esize = 16;
            integer msize = 8;
            boolean unsigned = TRUE;

    __encoding LDFF1B_Z.P.BR_U32
        __instruction_set A64
        __field Rm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10100100 010xxxxx 011xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer g = UInt(Pg);
            integer esize = 32;
            integer msize = 8;
            boolean unsigned = TRUE;

    __encoding LDFF1B_Z.P.BR_U64
        __instruction_set A64
        __field Rm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10100100 011xxxxx 011xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer g = UInt(Pg);
            integer esize = 64;
            integer msize = 8;
            boolean unsigned = TRUE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(64) base;
        bits(64) addr;
        bits(PL) mask = P[g];
        bits(VL) result;
        bits(VL) orig = Z[t];
        bits(msize) data;
        bits(64) offset = X[m];
        constant integer mbytes = msize DIV 8;
        boolean first = TRUE;
        boolean fault = FALSE;
        boolean faulted = FALSE;
        boolean unknown = FALSE;

        if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);

        if n == 31 then
            CheckSPAlignment();
            base = SP[];
        else
            base = X[n];

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                addr = base + UInt(offset) * mbytes;
                if first then
                    // Mem[] will not return if a fault is detected for the first active element
                    data = Mem[addr, mbytes, AccType_NORMAL];
                    first = FALSE;
                else
                    // MemNF[] will return fault=TRUE if access is not performed for any reason
                    (data, fault) = MemNF[addr, mbytes, AccType_CNOTFIRST];
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

            offset = offset + 1;

        Z[t] = result;

__instruction LDFF1H_Z.P.BR_U16
    __encoding LDFF1H_Z.P.BR_U16
        __instruction_set A64
        __field Rm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10100100 101xxxxx 011xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer g = UInt(Pg);
            integer esize = 16;
            integer msize = 16;
            boolean unsigned = TRUE;

    __encoding LDFF1H_Z.P.BR_U32
        __instruction_set A64
        __field Rm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10100100 110xxxxx 011xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer g = UInt(Pg);
            integer esize = 32;
            integer msize = 16;
            boolean unsigned = TRUE;

    __encoding LDFF1H_Z.P.BR_U64
        __instruction_set A64
        __field Rm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10100100 111xxxxx 011xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer g = UInt(Pg);
            integer esize = 64;
            integer msize = 16;
            boolean unsigned = TRUE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(64) base;
        bits(64) addr;
        bits(PL) mask = P[g];
        bits(VL) result;
        bits(VL) orig = Z[t];
        bits(msize) data;
        bits(64) offset = X[m];
        constant integer mbytes = msize DIV 8;
        boolean first = TRUE;
        boolean fault = FALSE;
        boolean faulted = FALSE;
        boolean unknown = FALSE;

        if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);

        if n == 31 then
            CheckSPAlignment();
            base = SP[];
        else
            base = X[n];

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                addr = base + UInt(offset) * mbytes;
                if first then
                    // Mem[] will not return if a fault is detected for the first active element
                    data = Mem[addr, mbytes, AccType_NORMAL];
                    first = FALSE;
                else
                    // MemNF[] will return fault=TRUE if access is not performed for any reason
                    (data, fault) = MemNF[addr, mbytes, AccType_CNOTFIRST];
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

            offset = offset + 1;

        Z[t] = result;

__instruction LDFF1W_Z.P.BR_U32
    __encoding LDFF1W_Z.P.BR_U32
        __instruction_set A64
        __field Rm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10100101 010xxxxx 011xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer g = UInt(Pg);
            integer esize = 32;
            integer msize = 32;
            boolean unsigned = TRUE;

    __encoding LDFF1W_Z.P.BR_U64
        __instruction_set A64
        __field Rm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10100101 011xxxxx 011xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer g = UInt(Pg);
            integer esize = 64;
            integer msize = 32;
            boolean unsigned = TRUE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(64) base;
        bits(64) addr;
        bits(PL) mask = P[g];
        bits(VL) result;
        bits(VL) orig = Z[t];
        bits(msize) data;
        bits(64) offset = X[m];
        constant integer mbytes = msize DIV 8;
        boolean first = TRUE;
        boolean fault = FALSE;
        boolean faulted = FALSE;
        boolean unknown = FALSE;

        if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);

        if n == 31 then
            CheckSPAlignment();
            base = SP[];
        else
            base = X[n];

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                addr = base + UInt(offset) * mbytes;
                if first then
                    // Mem[] will not return if a fault is detected for the first active element
                    data = Mem[addr, mbytes, AccType_NORMAL];
                    first = FALSE;
                else
                    // MemNF[] will return fault=TRUE if access is not performed for any reason
                    (data, fault) = MemNF[addr, mbytes, AccType_CNOTFIRST];
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

            offset = offset + 1;

        Z[t] = result;

__instruction LDFF1D_Z.P.BR_U64
    __encoding LDFF1D_Z.P.BR_U64
        __instruction_set A64
        __field Rm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10100101 111xxxxx 011xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer g = UInt(Pg);
            integer esize = 64;
            integer msize = 64;
            boolean unsigned = TRUE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(64) base;
        bits(64) addr;
        bits(PL) mask = P[g];
        bits(VL) result;
        bits(VL) orig = Z[t];
        bits(msize) data;
        bits(64) offset = X[m];
        constant integer mbytes = msize DIV 8;
        boolean first = TRUE;
        boolean fault = FALSE;
        boolean faulted = FALSE;
        boolean unknown = FALSE;

        if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);

        if n == 31 then
            CheckSPAlignment();
            base = SP[];
        else
            base = X[n];

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                addr = base + UInt(offset) * mbytes;
                if first then
                    // Mem[] will not return if a fault is detected for the first active element
                    data = Mem[addr, mbytes, AccType_NORMAL];
                    first = FALSE;
                else
                    // MemNF[] will return fault=TRUE if access is not performed for any reason
                    (data, fault) = MemNF[addr, mbytes, AccType_CNOTFIRST];
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

            offset = offset + 1;

        Z[t] = result;

__instruction LDFF1SB_Z.P.BR_S16
    __encoding LDFF1SB_Z.P.BR_S16
        __instruction_set A64
        __field Rm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10100101 110xxxxx 011xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer g = UInt(Pg);
            integer esize = 16;
            integer msize = 8;
            boolean unsigned = FALSE;

    __encoding LDFF1SB_Z.P.BR_S32
        __instruction_set A64
        __field Rm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10100101 101xxxxx 011xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer g = UInt(Pg);
            integer esize = 32;
            integer msize = 8;
            boolean unsigned = FALSE;

    __encoding LDFF1SB_Z.P.BR_S64
        __instruction_set A64
        __field Rm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10100101 100xxxxx 011xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer g = UInt(Pg);
            integer esize = 64;
            integer msize = 8;
            boolean unsigned = FALSE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(64) base;
        bits(64) addr;
        bits(PL) mask = P[g];
        bits(VL) result;
        bits(VL) orig = Z[t];
        bits(msize) data;
        bits(64) offset = X[m];
        constant integer mbytes = msize DIV 8;
        boolean first = TRUE;
        boolean fault = FALSE;
        boolean faulted = FALSE;
        boolean unknown = FALSE;

        if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);

        if n == 31 then
            CheckSPAlignment();
            base = SP[];
        else
            base = X[n];

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                addr = base + UInt(offset) * mbytes;
                if first then
                    // Mem[] will not return if a fault is detected for the first active element
                    data = Mem[addr, mbytes, AccType_NORMAL];
                    first = FALSE;
                else
                    // MemNF[] will return fault=TRUE if access is not performed for any reason
                    (data, fault) = MemNF[addr, mbytes, AccType_CNOTFIRST];
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

            offset = offset + 1;

        Z[t] = result;

__instruction LDFF1SH_Z.P.BR_S32
    __encoding LDFF1SH_Z.P.BR_S32
        __instruction_set A64
        __field Rm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10100101 001xxxxx 011xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer g = UInt(Pg);
            integer esize = 32;
            integer msize = 16;
            boolean unsigned = FALSE;

    __encoding LDFF1SH_Z.P.BR_S64
        __instruction_set A64
        __field Rm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10100101 000xxxxx 011xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer g = UInt(Pg);
            integer esize = 64;
            integer msize = 16;
            boolean unsigned = FALSE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(64) base;
        bits(64) addr;
        bits(PL) mask = P[g];
        bits(VL) result;
        bits(VL) orig = Z[t];
        bits(msize) data;
        bits(64) offset = X[m];
        constant integer mbytes = msize DIV 8;
        boolean first = TRUE;
        boolean fault = FALSE;
        boolean faulted = FALSE;
        boolean unknown = FALSE;

        if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);

        if n == 31 then
            CheckSPAlignment();
            base = SP[];
        else
            base = X[n];

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                addr = base + UInt(offset) * mbytes;
                if first then
                    // Mem[] will not return if a fault is detected for the first active element
                    data = Mem[addr, mbytes, AccType_NORMAL];
                    first = FALSE;
                else
                    // MemNF[] will return fault=TRUE if access is not performed for any reason
                    (data, fault) = MemNF[addr, mbytes, AccType_CNOTFIRST];
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

            offset = offset + 1;

        Z[t] = result;

__instruction LDFF1SW_Z.P.BR_S64
    __encoding LDFF1SW_Z.P.BR_S64
        __instruction_set A64
        __field Rm 16 +: 5
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10100100 100xxxxx 011xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer g = UInt(Pg);
            integer esize = 64;
            integer msize = 32;
            boolean unsigned = FALSE;

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(64) base;
        bits(64) addr;
        bits(PL) mask = P[g];
        bits(VL) result;
        bits(VL) orig = Z[t];
        bits(msize) data;
        bits(64) offset = X[m];
        constant integer mbytes = msize DIV 8;
        boolean first = TRUE;
        boolean fault = FALSE;
        boolean faulted = FALSE;
        boolean unknown = FALSE;

        if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);

        if n == 31 then
            CheckSPAlignment();
            base = SP[];
        else
            base = X[n];

        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
                addr = base + UInt(offset) * mbytes;
                if first then
                    // Mem[] will not return if a fault is detected for the first active element
                    data = Mem[addr, mbytes, AccType_NORMAL];
                    first = FALSE;
                else
                    // MemNF[] will return fault=TRUE if access is not performed for any reason
                    (data, fault) = MemNF[addr, mbytes, AccType_CNOTFIRST];
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

            offset = offset + 1;

        Z[t] = result;
