__instruction LDNF1SB_Z.P.BI_S16
    __encoding LDNF1SB_Z.P.BI_S16
        __instruction_set A64
        __field imm4 16 +: 4
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10100101 1101xxxx 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer g = UInt(Pg);
            integer esize = 16;
            integer msize = 8;
            boolean unsigned = FALSE;
            integer offset = SInt(imm4);

    __encoding LDNF1SB_Z.P.BI_S32
        __instruction_set A64
        __field imm4 16 +: 4
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10100101 1011xxxx 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer g = UInt(Pg);
            integer esize = 32;
            integer msize = 8;
            boolean unsigned = FALSE;
            integer offset = SInt(imm4);

    __encoding LDNF1SB_Z.P.BI_S64
        __instruction_set A64
        __field imm4 16 +: 4
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10100101 1001xxxx 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer g = UInt(Pg);
            integer esize = 64;
            integer msize = 8;
            boolean unsigned = FALSE;
            integer offset = SInt(imm4);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(64) base;
        bits(64) addr;
        bits(PL) mask = P[g];
        bits(VL) result;
        bits(VL) orig = Z[t];
        bits(msize) data;
        constant integer mbytes = msize DIV 8;
        boolean fault = FALSE;
        boolean faulted = FALSE;
        boolean unknown = FALSE;

        if n == 31 then
            CheckSPAlignment();
            if HaveMTEExt() then SetNotTagCheckedInstruction(TRUE);
            base = SP[];
        else
            if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);
            base = X[n];

        addr = base + offset * elements * mbytes;
        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
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

            addr = addr + mbytes;

        Z[t] = result;

__instruction LDNF1SH_Z.P.BI_S32
    __encoding LDNF1SH_Z.P.BI_S32
        __instruction_set A64
        __field imm4 16 +: 4
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10100101 0011xxxx 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer g = UInt(Pg);
            integer esize = 32;
            integer msize = 16;
            boolean unsigned = FALSE;
            integer offset = SInt(imm4);

    __encoding LDNF1SH_Z.P.BI_S64
        __instruction_set A64
        __field imm4 16 +: 4
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10100101 0001xxxx 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer g = UInt(Pg);
            integer esize = 64;
            integer msize = 16;
            boolean unsigned = FALSE;
            integer offset = SInt(imm4);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(64) base;
        bits(64) addr;
        bits(PL) mask = P[g];
        bits(VL) result;
        bits(VL) orig = Z[t];
        bits(msize) data;
        constant integer mbytes = msize DIV 8;
        boolean fault = FALSE;
        boolean faulted = FALSE;
        boolean unknown = FALSE;

        if n == 31 then
            CheckSPAlignment();
            if HaveMTEExt() then SetNotTagCheckedInstruction(TRUE);
            base = SP[];
        else
            if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);
            base = X[n];

        addr = base + offset * elements * mbytes;
        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
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

            addr = addr + mbytes;

        Z[t] = result;

__instruction LDNF1SW_Z.P.BI_S64
    __encoding LDNF1SW_Z.P.BI_S64
        __instruction_set A64
        __field imm4 16 +: 4
        __field Pg 10 +: 3
        __field Rn 5 +: 5
        __field Zt 0 +: 5
        __opcode '10100100 1001xxxx 101xxxxx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer t = UInt(Zt);
            integer n = UInt(Rn);
            integer g = UInt(Pg);
            integer esize = 64;
            integer msize = 32;
            boolean unsigned = FALSE;
            integer offset = SInt(imm4);

    __execute
        CheckSVEEnabled();
        integer elements = VL DIV esize;
        bits(64) base;
        bits(64) addr;
        bits(PL) mask = P[g];
        bits(VL) result;
        bits(VL) orig = Z[t];
        bits(msize) data;
        constant integer mbytes = msize DIV 8;
        boolean fault = FALSE;
        boolean faulted = FALSE;
        boolean unknown = FALSE;

        if n == 31 then
            CheckSPAlignment();
            if HaveMTEExt() then SetNotTagCheckedInstruction(TRUE);
            base = SP[];
        else
            if HaveMTEExt() then SetNotTagCheckedInstruction(FALSE);
            base = X[n];

        addr = base + offset * elements * mbytes;
        for e = 0 to elements-1
            if ElemP[mask, e, esize] == '1' then
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

            addr = addr + mbytes;

        Z[t] = result;
