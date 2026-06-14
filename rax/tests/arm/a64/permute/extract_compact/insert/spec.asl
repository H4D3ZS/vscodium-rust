__instruction INSR_Z.V__
    __encoding INSR_Z.V__
        __instruction_set A64
        __field size 22 +: 2
        __field Vm 5 +: 5
        __field Zdn 0 +: 5
        __opcode '00000101 xx110100 001110xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer dn = UInt(Zdn);
            integer m = UInt(Vm);

    __execute
        CheckSVEEnabled();
        bits(VL) dest = Z[dn];
        bits(esize) src = V[m];
        Z[dn] = dest<VL-esize-1:0> : src;

__instruction INSR_Z.R__
    __encoding INSR_Z.R__
        __instruction_set A64
        __field size 22 +: 2
        __field Rm 5 +: 5
        __field Zdn 0 +: 5
        __opcode '00000101 xx100100 001110xx xxxxxxxx'
        __guard TRUE
        __decode
            if !HaveSVE() then UNDEFINED;
            integer esize = 8 << UInt(size);
            integer dn = UInt(Zdn);
            integer m = UInt(Rm);

    __execute
        CheckSVEEnabled();
        bits(VL) dest = Z[dn];
        bits(esize) src = X[m];
        Z[dn] = dest<VL-esize-1:0> : src;
