__instruction aarch64_integer_bitfield
    __encoding aarch64_integer_bitfield
        __instruction_set A64
        __field sf 31 +: 1
        __field opc 29 +: 2
        __field N 22 +: 1
        __field immr 16 +: 6
        __field imms 10 +: 6
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode 'xxx10011 0xxxxxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer datasize = if sf == '1' then 64 else 32;

            boolean inzero;
            boolean extend;
            integer R;
            integer S;
            bits(datasize) wmask;
            bits(datasize) tmask;

            case opc of
                when '00' inzero = TRUE;  extend = TRUE;    // SBFM
                when '01' inzero = FALSE; extend = FALSE;   // BFM
                when '10' inzero = TRUE;  extend = FALSE;   // UBFM
                when '11' UNDEFINED;

            if sf == '1' && N != '1' then UNDEFINED;
            if sf == '0' && (N != '0' || immr<5> != '0' || imms<5> != '0') then UNDEFINED;

            R = UInt(immr);
            S = UInt(imms);
            (wmask, tmask) = DecodeBitMasks(N, imms, immr, FALSE);

    __execute
        bits(datasize) dst = if inzero then Zeros() else X[d];
        bits(datasize) src = X[n];

        // perform bitfield move on low bits
        bits(datasize) bot = (dst AND NOT(wmask)) OR (ROR(src, R) AND wmask);

        // determine extension bits (sign, zero or dest register)
        bits(datasize) top = if extend then Replicate(src<S>) else dst;

        // combine extension bits and result bits
        X[d] = (top AND NOT(tmask)) OR (bot AND tmask);

__instruction aarch64_integer_ins_ext_extract_immediate
    __encoding aarch64_integer_ins_ext_extract_immediate
        __instruction_set A64
        __field sf 31 +: 1
        __field N 22 +: 1
        __field Rm 16 +: 5
        __field imms 10 +: 6
        __field Rn 5 +: 5
        __field Rd 0 +: 5
        __opcode 'x0010011 1x0xxxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer n = UInt(Rn);
            integer m = UInt(Rm);
            integer datasize = if sf == '1' then 64 else 32;
            integer lsb;

            if N != sf then UNDEFINED;
            if sf == '0' && imms<5> == '1' then UNDEFINED;
            lsb = UInt(imms);

    __execute
        bits(datasize) result;
        bits(datasize) operand1 = X[n];
        bits(datasize) operand2 = X[m];
        bits(2*datasize) concat = operand1:operand2;

        result = concat<lsb+datasize-1:lsb>;

        X[d] = result;

__instruction aarch64_integer_ins_ext_insert_movewide
    __encoding aarch64_integer_ins_ext_insert_movewide
        __instruction_set A64
        __field sf 31 +: 1
        __field opc 29 +: 2
        __field hw 21 +: 2
        __field imm16 5 +: 16
        __field Rd 0 +: 5
        __opcode 'xxx10010 1xxxxxxx xxxxxxxx xxxxxxxx'
        __guard TRUE
        __decode
            integer d = UInt(Rd);
            integer datasize = if sf == '1' then 64 else 32;
            bits(16) imm = imm16;
            integer pos;
            MoveWideOp opcode;

            case opc of
                when '00' opcode = MoveWideOp_N;
                when '10' opcode = MoveWideOp_Z;
                when '11' opcode = MoveWideOp_K;
                otherwise UNDEFINED;

            if sf == '0' && hw<1> == '1' then UNDEFINED;
            pos = UInt(hw:'0000');

    __execute
        bits(datasize) result;

        if opcode == MoveWideOp_K then
            result = X[d];
        else
            result = Zeros();

        result<pos+15:pos> = imm;
        if opcode == MoveWideOp_N then
            result = NOT(result);
        X[d] = result;
