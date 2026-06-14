#BEGIN_HEADER
#
# Copyright (C) 2020 Mahdi Safsafi.
#
# https://github.com/MahdiSafsafi/opcodesDB
#
# See licence file 'LICENCE' for use and distribution rights.
#
#END_HEADER

use strict;
use warnings;


# AAA-ASCII Adjust After Addition.
T['AAA REG:SUPP:rw=AL REG:SUPP:rw=AH', 'NONE', 'AAA', ''];

# AAD-ASCII Adjust AX Before Division.
T['AAD IMM:u8=$uimm8 REG:SUPP:rw=AL REG:SUPP:rw=AH', 'NONE', 'AAD', ''];

# AAM-ASCII Adjust AX After Multiply.
T['AAM IMM:u8=$uimm8 REG:SUPP:rw=AL REG:SUPP:w=AH', 'NONE', 'AAM', ''];

# AAS-ASCII Adjust AL After Subtraction.
T['AAS REG:SUPP:rw=AL REG:SUPP:rw=AH', 'NONE', 'AAS', ''];

# ADC-Add with Carry.
T['ADC REG:rw=AL       IMM:s8=$simm8 ', 'NONE   ', 'ADC_bsb   ', '      '];
T['ADC REG:rw=$GPR8m   REG:r=$GPR8r  ', 'MOD=REG', 'ADC_mbrb  ', '      '];
T['ADC MEM:rw:u8=$mem8 REG:r=$GPR8r  ', 'MOD=MEM', 'ADC_mbrb  ', '$xlock'];
T['ADC REG:rw=$GPR8m   IMM:s8=$simm8 ', 'MOD=REG', 'ADC_mbsb_1', '      '];
T['ADC MEM:rw:u8=$mem8 IMM:s8=$simm8 ', 'MOD=MEM', 'ADC_mbsb_1', '$xlock'];
T['ADC REG:rw=$GPR8m   IMM:s8=$simm8 ', 'MOD=REG', 'ADC_mbsb_2', '      '];
T['ADC MEM:rw:u8=$mem8 IMM:s8=$simm8 ', 'MOD=MEM', 'ADC_mbsb_2', '$xlock'];
T['ADC REG:rw=$GPRvm   REG:r=$GPRvr  ', 'MOD=REG', 'ADC_mvrv  ', '      '];
T['ADC MEM:rw:sx=$memv REG:r=$GPRvr  ', 'MOD=MEM', 'ADC_mvrv  ', '$xlock'];
T['ADC REG:rw=$GPRvm   IMM:s8=$simm8 ', 'MOD=REG', 'ADC_mvsb  ', '      '];
T['ADC MEM:rw:sx=$memv IMM:s8=$simm8 ', 'MOD=MEM', 'ADC_mvsb  ', '$xlock'];
T['ADC REG:rw=$GPRvm   IMM:sx=$simmz ', 'MOD=REG', 'ADC_mvsz  ', '      '];
T['ADC MEM:rw:sx=$memv IMM:sx=$simmz ', 'MOD=MEM', 'ADC_mvsz  ', '$xlock'];
T['ADC REG:rw=$GPR8r   REG:r=$GPR8m  ', 'MOD=REG', 'ADC_rbmb  ', '      '];
T['ADC REG:rw=$GPR8r   MEM:r:u8=$mem8', 'MOD=MEM', 'ADC_rbmb  ', '      '];
T['ADC REG:rw=$GPRvr   REG:r=$GPRvm  ', 'MOD=REG', 'ADC_rvmv  ', '      '];
T['ADC REG:rw=$GPRvr   MEM:r:sx=$memv', 'MOD=MEM', 'ADC_rvmv  ', '      '];
T['ADC REG:rw=$AXv     IMM:sx=$simmz ', 'NONE   ', 'ADC_vsz   ', '      '];

# ADCX-Unsigned Integer Addition of Two Operands with Carry Flag.
T['ADCX REG:rw:s32=$GPR32r REG:r:s32=$GPR32m', 'MOD=REG W=0', 'ADCX', ''];
T['ADCX REG:rw:s64=$GPR64r REG:r:s64=$GPR64m', 'MOD=REG W=1', 'ADCX', ''];
T['ADCX REG:rw:s32=$GPR32r MEM:r:s32=$mem32 ', 'MOD=MEM W=0', 'ADCX', ''];
T['ADCX REG:rw:s64=$GPR64r MEM:r:s64=$mem64 ', 'MOD=MEM W=1', 'ADCX', ''];

# ADD-Add.
T['ADD REG:rw=AL       IMM:s8=$simm8 ', 'NONE   ', 'ADD_bsb   ', '      '];
T['ADD REG:rw=$GPR8m   REG:r=$GPR8r  ', 'MOD=REG', 'ADD_mbrb  ', '      '];
T['ADD MEM:rw:u8=$mem8 REG:r=$GPR8r  ', 'MOD=MEM', 'ADD_mbrb  ', '$xlock'];
T['ADD REG:rw=$GPR8m   IMM:u8=$uimm8 ', 'MOD=REG', 'ADD_mbsb_1', '      '];
T['ADD MEM:rw:u8=$mem8 IMM:s8=$simm8 ', 'MOD=MEM', 'ADD_mbsb_1', '$xlock'];
T['ADD REG:rw=$GPR8m   IMM:s8=$simm8 ', 'MOD=REG', 'ADD_mbsb_2', '      '];
T['ADD MEM:rw:u8=$mem8 IMM:s8=$simm8 ', 'MOD=MEM', 'ADD_mbsb_2', '$xlock'];
T['ADD REG:rw=$GPRvm   REG:r=$GPRvr  ', 'MOD=REG', 'ADD_mvrv  ', '      '];
T['ADD MEM:rw:sx=$memv REG:r=$GPRvr  ', 'MOD=MEM', 'ADD_mvrv  ', '$xlock'];
T['ADD REG:rw=$GPRvm   IMM:s8=$simm8 ', 'MOD=REG', 'ADD_mvsb  ', '      '];
T['ADD MEM:rw:sx=$memv IMM:s8=$simm8 ', 'MOD=MEM', 'ADD_mvsb  ', '$xlock'];
T['ADD REG:rw=$GPRvm   IMM:sx=$simmz ', 'MOD=REG', 'ADD_mvsz  ', '      '];
T['ADD MEM:rw:sx=$memv IMM:sx=$simmz ', 'MOD=MEM', 'ADD_mvsz  ', '$xlock'];
T['ADD REG:rw=$GPR8r   REG:r=$GPR8m  ', 'MOD=REG', 'ADD_rbmb  ', '      '];
T['ADD REG:rw=$GPR8r   MEM:r:u8=$mem8', 'MOD=MEM', 'ADD_rbmb  ', '      '];
T['ADD REG:rw=$GPRvr   REG:r=$GPRvm  ', 'MOD=REG', 'ADD_rvmv  ', '      '];
T['ADD REG:rw=$GPRvr   MEM:r:sx=$memv', 'MOD=MEM', 'ADD_rvmv  ', '      '];
T['ADD REG:rw=$AXv     IMM:sx=$simmz ', 'NONE   ', 'ADD_vsz   ', '      '];

# ADOX-Unsigned Integer Addition of Two Operands with Overflow Flag.
T['ADOX REG:rw:s32=$GPR32r REG:r:s32=$GPR32m', 'MOD=REG W=0', 'ADOX', ''];
T['ADOX REG:rw:s64=$GPR64r REG:r:s64=$GPR64m', 'MOD=REG W=1', 'ADOX', ''];
T['ADOX REG:rw:s32=$GPR32r MEM:r:s32=$mem32 ', 'MOD=MEM W=0', 'ADOX', ''];
T['ADOX REG:rw:s64=$GPR64r MEM:r:s64=$mem64 ', 'MOD=MEM W=1', 'ADOX', ''];

# AESDEC-Perform One Round of an AES Decryption Flow.
T['AESDEC VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'AESDEC', ''];
T['AESDEC VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'AESDEC', ''];

# AESDECLAST-Perform Last Round of an AES Decryption Flow.
T['AESDECLAST VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'AESDECLAST', ''];
T['AESDECLAST VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'AESDECLAST', ''];

# AESENC-Perform One Round of an AES Encryption Flow.
T['AESENC VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'AESENC', ''];
T['AESENC VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'AESENC', ''];

# AESENCLAST-Perform Last Round of an AES Encryption Flow.
T['AESENCLAST VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'AESENCLAST', ''];
T['AESENCLAST VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'AESENCLAST', ''];

# AESIMC-Perform the AES InvMixColumn Transformation.
T['AESIMC VREG:w:s32=$XMMr VREG:r:s32=$XMMm ', 'MOD=REG', 'AESIMC', ''];
T['AESIMC VREG:w:s32=$XMMr MEM:r:s32=$mem128', 'MOD=MEM', 'AESIMC', ''];

# AESKEYGENASSIST-AES Round Key Generation Assist.
T['AESKEYGENASSIST VREG:w:s32=$XMMr VREG:r:s32=$XMMm  IMM:u8=$uimm8', 'MOD=REG', 'AESKEYGENASSIST', ''];
T['AESKEYGENASSIST VREG:w:s32=$XMMr MEM:r:s32=$mem128 IMM:u8=$uimm8', 'MOD=MEM', 'AESKEYGENASSIST', ''];

# AND-Logical AND.
T['AND REG:rw=AL       IMM:s8=$simm8 ', 'NONE   ', 'AND_bsb   ', '      '];
T['AND REG:rw=$GPR8m   REG:r=$GPR8r  ', 'MOD=REG', 'AND_mbrb  ', '      '];
T['AND MEM:rw:u8=$mem8 REG:r=$GPR8r  ', 'MOD=MEM', 'AND_mbrb  ', '$xlock'];
T['AND REG:rw=$GPR8m   IMM:u8=$uimm8 ', 'MOD=REG', 'AND_mbub_1', '      '];
T['AND MEM:rw:u8=$mem8 IMM:u8=$uimm8 ', 'MOD=MEM', 'AND_mbub_1', '$xlock'];
T['AND REG:rw=$GPR8m   IMM:u8=$uimm8 ', 'MOD=REG', 'AND_mbub_2', '      '];
T['AND MEM:rw:u8=$mem8 IMM:u8=$uimm8 ', 'MOD=MEM', 'AND_mbub_2', '$xlock'];
T['AND REG:rw=$GPRvm   REG:r=$GPRvr  ', 'MOD=REG', 'AND_mvrv  ', '      '];
T['AND MEM:rw:sx=$memv REG:r=$GPRvr  ', 'MOD=MEM', 'AND_mvrv  ', '$xlock'];
T['AND REG:rw=$GPRvm   IMM:s8=$simm8 ', 'MOD=REG', 'AND_mvsb  ', '      '];
T['AND MEM:rw:sx=$memv IMM:s8=$simm8 ', 'MOD=MEM', 'AND_mvsb  ', '$xlock'];
T['AND REG:rw=$GPRvm   IMM:sx=$simmz ', 'MOD=REG', 'AND_mvsz  ', '      '];
T['AND MEM:rw:sx=$memv IMM:sx=$simmz ', 'MOD=MEM', 'AND_mvsz  ', '$xlock'];
T['AND REG:rw=$GPR8r   REG:r=$GPR8m  ', 'MOD=REG', 'AND_rbmb  ', '      '];
T['AND REG:rw=$GPR8r   MEM:r:u8=$mem8', 'MOD=MEM', 'AND_rbmb  ', '      '];
T['AND REG:rw=$GPRvr   REG:r=$GPRvm  ', 'MOD=REG', 'AND_rvmv  ', '      '];
T['AND REG:rw=$GPRvr   MEM:r:sx=$memv', 'MOD=MEM', 'AND_rvmv  ', '      '];
T['AND REG:rw=$AXv     IMM:sx=$simmz ', 'NONE   ', 'AND_vsz   ', '      '];

# ANDN-Logical AND NOT.
T['ANDN REG:w:s32=$GPR32r REG:r:s32=$GPR32n REG:r:s32=$GPR32m', 'MOD=REG MODE=64 W=0', 'ANDN', ''];
T['ANDN REG:w:s64=$GPR64r REG:r:s64=$GPR64n REG:r:s64=$GPR64m', 'MOD=REG MODE=64 W=1', 'ANDN', ''];
T['ANDN REG:w:s32=$GPR32r REG:r:s32=$GPR32n MEM:r:s32=$mem32 ', 'MOD=MEM MODE=64 W=0', 'ANDN', ''];
T['ANDN REG:w:s64=$GPR64r REG:r:s64=$GPR64n MEM:r:s64=$mem64 ', 'MOD=MEM MODE=64 W=1', 'ANDN', ''];
T['ANDN REG:w:s32=$GPR32r REG:r:s32=$GPR32n REG:r:s32=$GPR32m', 'MOD=REG MODE=NO64  ', 'ANDN', ''];
T['ANDN REG:w:s32=$GPR32r REG:r:s32=$GPR32n MEM:r:s32=$mem32 ', 'MOD=MEM MODE=NO64  ', 'ANDN', ''];

# ARPL-Adjust RPL Field of Segment Selector.
T['ARPL REG:rw=$GPR16m    REG:r=$GPR16r', 'MOD=REG', 'ARPL', ''];
T['ARPL MEM:rw:s16=$mem16 REG:r=$GPR16r', 'MOD=MEM', 'ARPL', ''];

# BEXTR-Bit Field Extract.
T['BEXTR REG:w:s32=$GPR32r REG:r:s32=$GPR32m REG:r:s32=$GPR32n', 'MOD=REG MODE=64 W=0', 'BEXTR_rxmxnx', ''];
T['BEXTR REG:w:s64=$GPR64r REG:r:s64=$GPR64m REG:r:s64=$GPR64n', 'MOD=REG MODE=64 W=1', 'BEXTR_rxmxnx', ''];
T['BEXTR REG:w:s32=$GPR32r MEM:r:s32=$mem32  REG:r:s32=$GPR32n', 'MOD=MEM MODE=64 W=0', 'BEXTR_rxmxnx', ''];
T['BEXTR REG:w:s64=$GPR64r MEM:r:s64=$mem64  REG:r:s64=$GPR64n', 'MOD=MEM MODE=64 W=1', 'BEXTR_rxmxnx', ''];
T['BEXTR REG:w:s32=$GPR32r REG:r:s32=$GPR32m REG:r:s32=$GPR32n', 'MOD=REG MODE=NO64  ', 'BEXTR_rxmxnx', ''];
T['BEXTR REG:w:s32=$GPR32r MEM:r:s32=$mem32  REG:r:s32=$GPR32n', 'MOD=MEM MODE=NO64  ', 'BEXTR_rxmxnx', ''];

# BLSI-Extract Lowest Set Isolated Bit.
T['BLSI REG:w:s32=$GPR32n REG:r:s32=$GPR32m', 'MOD=REG MODE=64 W=0', 'BLSI', ''];
T['BLSI REG:w:s64=$GPR64n REG:r:s64=$GPR64m', 'MOD=REG MODE=64 W=1', 'BLSI', ''];
T['BLSI REG:w:s32=$GPR32n MEM:r:s32=$mem32 ', 'MOD=MEM MODE=64 W=0', 'BLSI', ''];
T['BLSI REG:w:s64=$GPR64n MEM:r:s64=$mem64 ', 'MOD=MEM MODE=64 W=1', 'BLSI', ''];
T['BLSI REG:w:s32=$GPR32n REG:r:s32=$GPR32m', 'MOD=REG MODE=NO64  ', 'BLSI', ''];
T['BLSI REG:w:s32=$GPR32n MEM:r:s32=$mem32 ', 'MOD=MEM MODE=NO64  ', 'BLSI', ''];

# BLSMSK-Get Mask Up to Lowest Set Bit.
T['BLSMSK REG:w:s32=$GPR32n REG:r:s32=$GPR32m', 'MOD=REG MODE=64 W=0', 'BLSMSK', ''];
T['BLSMSK REG:w:s64=$GPR64n REG:r:s64=$GPR64m', 'MOD=REG MODE=64 W=1', 'BLSMSK', ''];
T['BLSMSK REG:w:s32=$GPR32n MEM:r:s32=$mem32 ', 'MOD=MEM MODE=64 W=0', 'BLSMSK', ''];
T['BLSMSK REG:w:s64=$GPR64n MEM:r:s64=$mem64 ', 'MOD=MEM MODE=64 W=1', 'BLSMSK', ''];
T['BLSMSK REG:w:s32=$GPR32n REG:r:s32=$GPR32m', 'MOD=REG MODE=NO64  ', 'BLSMSK', ''];
T['BLSMSK REG:w:s32=$GPR32n MEM:r:s32=$mem32 ', 'MOD=MEM MODE=NO64  ', 'BLSMSK', ''];

# BLSR-Reset Lowest Set Bit.
T['BLSR REG:w:s32=$GPR32n REG:r:s32=$GPR32m', 'MOD=REG MODE=64 W=0', 'BLSR', ''];
T['BLSR REG:w:s64=$GPR64n REG:r:s64=$GPR64m', 'MOD=REG MODE=64 W=1', 'BLSR', ''];
T['BLSR REG:w:s32=$GPR32n MEM:r:s32=$mem32 ', 'MOD=MEM MODE=64 W=0', 'BLSR', ''];
T['BLSR REG:w:s64=$GPR64n MEM:r:s64=$mem64 ', 'MOD=MEM MODE=64 W=1', 'BLSR', ''];
T['BLSR REG:w:s32=$GPR32n REG:r:s32=$GPR32m', 'MOD=REG MODE=NO64  ', 'BLSR', ''];
T['BLSR REG:w:s32=$GPR32n MEM:r:s32=$mem32 ', 'MOD=MEM MODE=NO64  ', 'BLSR', ''];

# BOUND-Check Array Index Against Bounds.
T['BOUND REG:r=$GPRvr MEM:r:s16=$mem32', 'OSZ=16', 'BOUND', ''];
T['BOUND REG:r=$GPRvr MEM:r:s32=$mem64', 'OSZ=32', 'BOUND', ''];

# BSF-Bit Scan Forward.
T['BSF REG:cw=$GPRvr REG:r=$GPRvm  ', 'BMI=0 MOD=REG                  ', 'BSF', ''];
T['BSF REG:cw=$GPRvr MEM:r:sx=$memv', 'BMI=0 MOD=MEM                  ', 'BSF', ''];
T['BSF REG:cw=$GPRvr REG:r=$GPRvm  ', 'BMI=1 MOD=REG P66=0 PF2=0 PF3=0', 'BSF', ''];
T['BSF REG:cw=$GPRvr MEM:r:sx=$memv', 'BMI=1 MOD=MEM P66=0 PF2=0 PF3=0', 'BSF', ''];

# BSR-Bit Scan Reverse.
T['BSR REG:cw=$GPRvr REG:r=$GPRvm  ', 'BMI=0 MOD=REG                  ', 'BSR', ''];
T['BSR REG:cw=$GPRvr MEM:r:sx=$memv', 'BMI=0 MOD=MEM                  ', 'BSR', ''];
T['BSR REG:cw=$GPRvr REG:r=$GPRvm  ', 'BMI=1 MOD=REG P66=0 PF2=0 PF3=0', 'BSR', ''];
T['BSR REG:cw=$GPRvr MEM:r:sx=$memv', 'BMI=1 MOD=MEM P66=0 PF2=0 PF3=0', 'BSR', ''];

# BSWAP-Byte Swap.
T['BSWAP REG:rw=$GPRvo', 'NONE', 'BSWAP_ov_1', ''];
T['BSWAP REG:rw=$GPRvo', 'NONE', 'BSWAP_ov_2', ''];
T['BSWAP REG:rw=$GPRvo', 'NONE', 'BSWAP_ov_3', ''];
T['BSWAP REG:rw=$GPRvo', 'NONE', 'BSWAP_ov_4', ''];
T['BSWAP REG:rw=$GPRvo', 'NONE', 'BSWAP_ov_5', ''];
T['BSWAP REG:rw=$GPRvo', 'NONE', 'BSWAP_ov_6', ''];
T['BSWAP REG:rw=$GPRvo', 'NONE', 'BSWAP_ov_7', ''];
T['BSWAP REG:rw=$GPRvo', 'NONE', 'BSWAP_ov_8', ''];

# BT-Bit Test.
T['BT REG:r=$GPRvm   REG:r=$GPRvr ', 'MOD=REG', 'BT_mvrv', ''];
T['BT MEM:r:sx=$memv REG:r=$GPRvr ', 'MOD=MEM', 'BT_mvrv', ''];
T['BT REG:r=$GPRvm   IMM:u8=$uimm8', 'MOD=REG', 'BT_mvub', ''];
T['BT MEM:r:sx=$memv IMM:u8=$uimm8', 'MOD=MEM', 'BT_mvub', ''];

# BTC-Bit Test and Complement.
T['BTC REG:rw=$GPRvm   REG:r=$GPRvr ', 'MOD=REG', 'BTC_mvrv', '      '];
T['BTC MEM:rw:sx=$memv REG:r=$GPRvr ', 'MOD=MEM', 'BTC_mvrv', '$xlock'];
T['BTC REG:rw=$GPRvm   IMM:u8=$uimm8', 'MOD=REG', 'BTC_mvub', '      '];
T['BTC MEM:rw:sx=$memv IMM:u8=$uimm8', 'MOD=MEM', 'BTC_mvub', '$xlock'];

# BTR-Bit Test and Reset.
T['BTR REG:rw=$GPRvm   REG:r=$GPRvr ', 'MOD=REG', 'BTR_mvrv', '      '];
T['BTR MEM:rw:sx=$memv REG:r=$GPRvr ', 'MOD=MEM', 'BTR_mvrv', '$xlock'];
T['BTR REG:rw=$GPRvm   IMM:u8=$uimm8', 'MOD=REG', 'BTR_mvub', '      '];
T['BTR MEM:rw:sx=$memv IMM:u8=$uimm8', 'MOD=MEM', 'BTR_mvub', '$xlock'];

# BTS-Bit Test and Set.
T['BTS REG:rw=$GPRvm   REG:r=$GPRvr ', 'MOD=REG', 'BTS_mvrv', '      '];
T['BTS MEM:rw:sx=$memv REG:r=$GPRvr ', 'MOD=MEM', 'BTS_mvrv', '$xlock'];
T['BTS REG:rw=$GPRvm   IMM:u8=$uimm8', 'MOD=REG', 'BTS_mvub', '      '];
T['BTS MEM:rw:sx=$memv IMM:u8=$uimm8', 'MOD=MEM', 'BTS_mvub', '$xlock'];

# BZHI-Zero High Bits Starting with Specified Bit Position.
T['BZHI REG:w:s32=$GPR32r REG:r:s32=$GPR32m REG:r:s32=$GPR32n', 'MOD=REG MODE=64 W=0', 'BZHI', ''];
T['BZHI REG:w:s64=$GPR64r REG:r:s64=$GPR64m REG:r:s64=$GPR64n', 'MOD=REG MODE=64 W=1', 'BZHI', ''];
T['BZHI REG:w:s32=$GPR32r MEM:r:s32=$mem32  REG:r:s32=$GPR32n', 'MOD=MEM MODE=64 W=0', 'BZHI', ''];
T['BZHI REG:w:s64=$GPR64r MEM:r:s64=$mem64  REG:r:s64=$GPR64n', 'MOD=MEM MODE=64 W=1', 'BZHI', ''];
T['BZHI REG:w:s32=$GPR32r REG:r:s32=$GPR32m REG:r:s32=$GPR32n', 'MOD=REG MODE=NO64  ', 'BZHI', ''];
T['BZHI REG:w:s32=$GPR32r MEM:r:s32=$mem32  REG:r:s32=$GPR32n', 'MOD=MEM MODE=NO64  ', 'BZHI', ''];

# CALL-Call Procedure.
T['CALL PTR:r=$ptrdpp  REG:SUPP:w=EIP   REG:SUPP:rw=$SPa MEM:SUPP:w:sx={BASE:r=$SPa SZ=SZ_F}', 'NONE               ', 'CALL     ', '       '];
T['CALL MEM:r=$memdpr  REG:SUPP:w=$IPa  REG:SUPP:rw=$SPa MEM:SUPP:w:sx={BASE:r=$SPa SZ=SZ_F}', 'NONE               ', 'CALL_mdpr', '       '];
T['CALL REG:r=$GPRvm   REG:SUPP:rw=$IPa REG:SUPP:rw=$SPa MEM:SUPP:w:sx={BASE:r=$SPa SZ=SZ_V}', 'MOD=REG F64=1      ', 'CALL_mv  ', 'bound=1'];
T['CALL MEM:r:sx=$memv REG:SUPP:rw=$IPa REG:SUPP:rw=$SPa MEM:SUPP:w:sx={BASE:r=$SPa SZ=SZ_V}', 'MOD=MEM F64=1      ', 'CALL_mv  ', 'bound=1'];
T['CALL REL:s32=$rel32 REG:SUPP:rw=RIP  REG:SUPP:rw=$SPa MEM:SUPP:w:sx={BASE:r=$SPa SZ=SZ_V}', 'AMD=0 MODE=64 F64=1', 'CALL_sx  ', 'bound=1'];
T['CALL REL:sx=$relz   REG:SUPP:rw=RIP  REG:SUPP:rw=$SPa MEM:SUPP:w:sx={BASE:r=$SPa SZ=SZ_V}', 'AMD=1 MODE=64 D64=1', 'CALL_sx  ', 'bound=1'];
T['CALL REL:sx=$relz   REG:SUPP:rw=EIP  REG:SUPP:rw=$SPa MEM:SUPP:w:sx={BASE:r=$SPa SZ=SZ_V}', 'MODE=NO64          ', 'CALL_sx  ', 'bound=1'];

# CBW/CWDE/CDQE-Convert Byte to Word/Convert Word to Doubleword/Convert Doubleword to Quadword.
T['CBW  REG:SUPP:w=AX  REG:SUPP:r=AL ', 'NONE', 'CBW ', ''];
T['CDQE REG:SUPP:w=RAX REG:SUPP:r=EAX', 'NONE', 'CDQE', ''];
T['CWDE REG:SUPP:w=EAX REG:SUPP:r=AX ', 'NONE', 'CWDE', ''];

# CLC-Clear Carry Flag.
T['CLC', 'NONE', 'CLC', ''];

# CLD-Clear Direction Flag.
T['CLD', 'NONE', 'CLD', ''];

# CLI-Clear Interrupt Flag.
T['CLI', 'NONE', 'CLI', ''];

# CLTS-Clear Task-Switched Flag in CR0.
T['CLTS', 'NONE', 'CLTS', ''];

# CMC-Complement Carry Flag.
T['CMC', 'NONE', 'CMC', ''];

# CMOVcc-Conditional Move.
T['CMOVB   REG:cw=$GPRvr REG:r=$GPRvm  ', 'MOD=REG', 'CMOVB  ', ''];
T['CMOVB   REG:cw=$GPRvr MEM:r:sx=$memv', 'MOD=MEM', 'CMOVB  ', ''];
T['CMOVBE  REG:cw=$GPRvr REG:r=$GPRvm  ', 'MOD=REG', 'CMOVBE ', ''];
T['CMOVBE  REG:cw=$GPRvr MEM:r:sx=$memv', 'MOD=MEM', 'CMOVBE ', ''];
T['CMOVL   REG:cw=$GPRvr REG:r=$GPRvm  ', 'MOD=REG', 'CMOVL  ', ''];
T['CMOVL   REG:cw=$GPRvr MEM:r:sx=$memv', 'MOD=MEM', 'CMOVL  ', ''];
T['CMOVLE  REG:cw=$GPRvr REG:r=$GPRvm  ', 'MOD=REG', 'CMOVLE ', ''];
T['CMOVLE  REG:cw=$GPRvr MEM:r:sx=$memv', 'MOD=MEM', 'CMOVLE ', ''];
T['CMOVNB  REG:cw=$GPRvr REG:r=$GPRvm  ', 'MOD=REG', 'CMOVNB ', ''];
T['CMOVNB  REG:cw=$GPRvr MEM:r:sx=$memv', 'MOD=MEM', 'CMOVNB ', ''];
T['CMOVNBE REG:cw=$GPRvr REG:r=$GPRvm  ', 'MOD=REG', 'CMOVNBE', ''];
T['CMOVNBE REG:cw=$GPRvr MEM:r:sx=$memv', 'MOD=MEM', 'CMOVNBE', ''];
T['CMOVNL  REG:cw=$GPRvr REG:r=$GPRvm  ', 'MOD=REG', 'CMOVNL ', ''];
T['CMOVNL  REG:cw=$GPRvr MEM:r:sx=$memv', 'MOD=MEM', 'CMOVNL ', ''];
T['CMOVNLE REG:cw=$GPRvr REG:r=$GPRvm  ', 'MOD=REG', 'CMOVNLE', ''];
T['CMOVNLE REG:cw=$GPRvr MEM:r:sx=$memv', 'MOD=MEM', 'CMOVNLE', ''];
T['CMOVNO  REG:cw=$GPRvr REG:r=$GPRvm  ', 'MOD=REG', 'CMOVNO ', ''];
T['CMOVNO  REG:cw=$GPRvr MEM:r:sx=$memv', 'MOD=MEM', 'CMOVNO ', ''];
T['CMOVNP  REG:cw=$GPRvr REG:r=$GPRvm  ', 'MOD=REG', 'CMOVNP ', ''];
T['CMOVNP  REG:cw=$GPRvr MEM:r:sx=$memv', 'MOD=MEM', 'CMOVNP ', ''];
T['CMOVNS  REG:cw=$GPRvr REG:r=$GPRvm  ', 'MOD=REG', 'CMOVNS ', ''];
T['CMOVNS  REG:cw=$GPRvr MEM:r:sx=$memv', 'MOD=MEM', 'CMOVNS ', ''];
T['CMOVNZ  REG:cw=$GPRvr REG:r=$GPRvm  ', 'MOD=REG', 'CMOVNZ ', ''];
T['CMOVNZ  REG:cw=$GPRvr MEM:r:sx=$memv', 'MOD=MEM', 'CMOVNZ ', ''];
T['CMOVO   REG:cw=$GPRvr REG:r=$GPRvm  ', 'MOD=REG', 'CMOVO  ', ''];
T['CMOVO   REG:cw=$GPRvr MEM:r:sx=$memv', 'MOD=MEM', 'CMOVO  ', ''];
T['CMOVP   REG:cw=$GPRvr REG:r=$GPRvm  ', 'MOD=REG', 'CMOVP  ', ''];
T['CMOVP   REG:cw=$GPRvr MEM:r:sx=$memv', 'MOD=MEM', 'CMOVP  ', ''];
T['CMOVS   REG:cw=$GPRvr REG:r=$GPRvm  ', 'MOD=REG', 'CMOVS  ', ''];
T['CMOVS   REG:cw=$GPRvr MEM:r:sx=$memv', 'MOD=MEM', 'CMOVS  ', ''];
T['CMOVZ   REG:cw=$GPRvr REG:r=$GPRvm  ', 'MOD=REG', 'CMOVZ  ', ''];
T['CMOVZ   REG:cw=$GPRvr MEM:r:sx=$memv', 'MOD=MEM', 'CMOVZ  ', ''];

# CMP-Compare Two Operands.
T['CMP REG:r=AL       IMM:s8=$simm8 ', 'NONE   ', 'CMP_bsb   ', ''];
T['CMP REG:r=$GPR8m   REG:r=$GPR8r  ', 'MOD=REG', 'CMP_mbrb  ', ''];
T['CMP MEM:r:u8=$mem8 REG:r=$GPR8r  ', 'MOD=MEM', 'CMP_mbrb  ', ''];
T['CMP REG:r=$GPR8m   IMM:s8=$simm8 ', 'MOD=REG', 'CMP_mbsb_1', ''];
T['CMP MEM:r:u8=$mem8 IMM:s8=$simm8 ', 'MOD=MEM', 'CMP_mbsb_1', ''];
T['CMP REG:r=$GPR8m   IMM:s8=$simm8 ', 'MOD=REG', 'CMP_mbsb_2', ''];
T['CMP MEM:r:u8=$mem8 IMM:s8=$simm8 ', 'MOD=MEM', 'CMP_mbsb_2', ''];
T['CMP REG:r=$GPRvm   REG:r=$GPRvr  ', 'MOD=REG', 'CMP_mvrv  ', ''];
T['CMP MEM:r:sx=$memv REG:r=$GPRvr  ', 'MOD=MEM', 'CMP_mvrv  ', ''];
T['CMP REG:r=$GPRvm   IMM:s8=$simm8 ', 'MOD=REG', 'CMP_mvsb  ', ''];
T['CMP MEM:r:sx=$memv IMM:s8=$simm8 ', 'MOD=MEM', 'CMP_mvsb  ', ''];
T['CMP REG:r=$GPRvm   IMM:sx=$simmz ', 'MOD=REG', 'CMP_mvsz  ', ''];
T['CMP MEM:r:sx=$memv IMM:sx=$simmz ', 'MOD=MEM', 'CMP_mvsz  ', ''];
T['CMP REG:r=$GPR8r   REG:r=$GPR8m  ', 'MOD=REG', 'CMP_rbmb  ', ''];
T['CMP REG:r=$GPR8r   MEM:r:u8=$mem8', 'MOD=MEM', 'CMP_rbmb  ', ''];
T['CMP REG:r=$GPRvr   REG:r=$GPRvm  ', 'MOD=REG', 'CMP_rvmv  ', ''];
T['CMP REG:r=$GPRvr   MEM:r:sx=$memv', 'MOD=MEM', 'CMP_rvmv  ', ''];
T['CMP REG:r=$AXv     IMM:sx=$simmz ', 'NONE   ', 'CMP_vsz   ', ''];

# CMPS/CMPSB/CMPSW/CMPSD/CMPSQ-Compare String Operands.
T['CMPSB MEM:SUPP:r:u8={SEG:r=DS BASE:r=$SIa SZ=8}    MEM:SUPP:r:u8={SEG:r=ES BASE:r=$DIa SZ=8}                     ', 'NONE       ', 'CMPSB_1', 'repnz=1 repz=1'];
T['CMPSB MEM:SUPP:cr:u8={SEG:r=DS BASE:r=$SIa SZ=8}   MEM:SUPP:cr:u8={SEG:r=ES BASE:r=$DIa SZ=8}   REG:SUPP:rcw=$CXa', 'PF2=0 PF3=1', 'CMPSB_2', 'repnz=1 repz=1'];
T['CMPSB MEM:SUPP:cr:u8={SEG:r=DS BASE:r=$SIa SZ=8}   MEM:SUPP:cr:u8={SEG:r=ES BASE:r=$DIa SZ=8}   REG:SUPP:rcw=$CXa', 'PF2=1 PF3=0', 'CMPSB_2', 'repnz=1 repz=1'];
T['CMPSD MEM:SUPP:r:s32={SEG:r=DS BASE:r=$SIa SZ=32}  MEM:SUPP:r:s32={SEG:r=ES BASE:r=$DIa SZ=32}                   ', 'NONE       ', 'CMPSD_1', 'repnz=1 repz=1'];
T['CMPSD MEM:SUPP:cr:s32={SEG:r=DS BASE:r=$SIa SZ=32} MEM:SUPP:cr:s32={SEG:r=ES BASE:r=$DIa SZ=32} REG:SUPP:rcw=$CXa', 'PF2=0 PF3=1', 'CMPSD_2', 'repnz=1 repz=1'];
T['CMPSD MEM:SUPP:cr:s32={SEG:r=DS BASE:r=$SIa SZ=32} MEM:SUPP:cr:s32={SEG:r=ES BASE:r=$DIa SZ=32} REG:SUPP:rcw=$CXa', 'PF2=1 PF3=0', 'CMPSD_2', 'repnz=1 repz=1'];
T['CMPSQ MEM:SUPP:r:s64={SEG:r=DS BASE:r=$SIa SZ=64}  MEM:SUPP:r:s64={SEG:r=ES BASE:r=$DIa SZ=64}                   ', 'NONE       ', 'CMPSQ_1', 'repnz=1 repz=1'];
T['CMPSQ MEM:SUPP:cr:s64={SEG:r=DS BASE:r=$SIa SZ=64} MEM:SUPP:cr:s64={SEG:r=ES BASE:r=$DIa SZ=64} REG:SUPP:rcw=$CXa', 'PF2=0 PF3=1', 'CMPSQ_2', 'repnz=1 repz=1'];
T['CMPSQ MEM:SUPP:cr:s64={SEG:r=DS BASE:r=$SIa SZ=64} MEM:SUPP:cr:s64={SEG:r=ES BASE:r=$DIa SZ=64} REG:SUPP:rcw=$CXa', 'PF2=1 PF3=0', 'CMPSQ_2', 'repnz=1 repz=1'];
T['CMPSW MEM:SUPP:r:s16={SEG:r=DS BASE:r=$SIa SZ=16}  MEM:SUPP:r:s16={SEG:r=ES BASE:r=$DIa SZ=16}                   ', 'NONE       ', 'CMPSW_1', 'repnz=1 repz=1'];
T['CMPSW MEM:SUPP:cr:s16={SEG:r=DS BASE:r=$SIa SZ=16} MEM:SUPP:cr:s16={SEG:r=ES BASE:r=$DIa SZ=16} REG:SUPP:rcw=$CXa', 'PF2=0 PF3=1', 'CMPSW_2', 'repnz=1 repz=1'];
T['CMPSW MEM:SUPP:cr:s16={SEG:r=DS BASE:r=$SIa SZ=16} MEM:SUPP:cr:s16={SEG:r=ES BASE:r=$DIa SZ=16} REG:SUPP:rcw=$CXa', 'PF2=1 PF3=0', 'CMPSW_2', 'repnz=1 repz=1'];

# CMPXCHG-Compare and Exchange.
T['CMPXCHG REG:rcw=$GPR8m   REG:r=$GPR8r REG:SUPP:rcw=AL  ', 'MOD=REG', 'CMPXCHG_mbrb', '      '];
T['CMPXCHG MEM:rcw:u8=$mem8 REG:r=$GPR8r REG:SUPP:rcw=AL  ', 'MOD=MEM', 'CMPXCHG_mbrb', '$xlock'];
T['CMPXCHG REG:rcw=$GPRvm   REG:r=$GPRvr REG:SUPP:rcw=$AXv', 'MOD=REG', 'CMPXCHG_mvrv', '      '];
T['CMPXCHG MEM:rcw:sx=$memv REG:r=$GPRvr REG:SUPP:rcw=$AXv', 'MOD=MEM', 'CMPXCHG_mvrv', '$xlock'];

# CMPXCHG8B/CMPXCHG16B-Compare and Exchange Bytes.
T['CMPXCHG16B MEM:rcw:s32=$mem128 REG:SUPP:rcw=RDX REG:SUPP:rcw=RAX REG:SUPP:r=RCX REG:SUPP:r=RBX', 'NONE       ', 'CMPXCHG16B', 'lock=1'];
T['CMPXCHG8B  MEM:rcw:s64=$mem64  REG:SUPP:rcw=EDX REG:SUPP:rcw=EAX REG:SUPP:r=ECX REG:SUPP:r=EBX', 'MODE=64 W=0', 'CMPXCHG8B ', '$xlock'];
T['CMPXCHG8B  MEM:rcw:s64=$mem64  REG:SUPP:rcw=EDX REG:SUPP:rcw=EAX REG:SUPP:r=ECX REG:SUPP:r=EBX', 'MODE=NO64  ', 'CMPXCHG8B ', '$xlock'];

# CPUID-CPU Identification.
T['CPUID REG:SUPP:rw=EAX REG:SUPP:w=EBX REG:SUPP:crw=ECX REG:SUPP:w=EDX', 'NONE', 'CPUID', ''];

# CWD/CDQ/CQO-Convert Word to Doubleword/Convert Doubleword to Quadword.
T['CDQ REG:SUPP:w=EDX REG:SUPP:r=EAX', 'NONE', 'CDQ', ''];
T['CQO REG:SUPP:w=RDX REG:SUPP:r=RAX', 'NONE', 'CQO', ''];
T['CWD REG:SUPP:w=DX  REG:SUPP:r=AX ', 'NONE', 'CWD', ''];

# DAA-Decimal Adjust AL after Addition.
T['DAA REG:SUPP:rw=AL', 'NONE', 'DAA', ''];

# DAS-Decimal Adjust AL after Subtraction.
T['DAS REG:SUPP:rw=AL', 'NONE', 'DAS', ''];

# DEC-Decrement by 1.
T['DEC REG:rw=$GPR8m  ', 'MOD=REG', 'DEC_mb  ', '      '];
T['DEC MEM:rw:u8=$mem8', 'MOD=MEM', 'DEC_mb  ', '$xlock'];
T['DEC REG:rw=$GPRvm  ', 'MOD=REG', 'DEC_mv  ', '      '];
T['DEC MEM:rw:sx=$memv', 'MOD=MEM', 'DEC_mv  ', '$xlock'];
T['DEC REG:rw=$GPRvo  ', 'NONE   ', 'DEC_ov_1', '      '];
T['DEC REG:rw=$GPRvo  ', 'NONE   ', 'DEC_ov_2', '      '];
T['DEC REG:rw=$GPRvo  ', 'NONE   ', 'DEC_ov_3', '      '];
T['DEC REG:rw=$GPRvo  ', 'NONE   ', 'DEC_ov_4', '      '];
T['DEC REG:rw=$GPRvo  ', 'NONE   ', 'DEC_ov_5', '      '];
T['DEC REG:rw=$GPRvo  ', 'NONE   ', 'DEC_ov_6', '      '];
T['DEC REG:rw=$GPRvo  ', 'NONE   ', 'DEC_ov_7', '      '];
T['DEC REG:rw=$GPRvo  ', 'NONE   ', 'DEC_ov_8', '      '];

# DIV-Unsigned Divide.
T['DIV REG:r=$GPR8m   REG:SUPP:rw=AX                   ', 'MOD=REG', 'DIV_mb', ''];
T['DIV MEM:r:u8=$mem8 REG:SUPP:rw=AX                   ', 'MOD=MEM', 'DIV_mb', ''];
T['DIV REG:r=$GPRvm   REG:SUPP:rw=$AXv REG:SUPP:rw=$DXv', 'MOD=REG', 'DIV_mv', ''];
T['DIV MEM:r:sx=$memv REG:SUPP:rw=$AXv REG:SUPP:rw=$DXv', 'MOD=MEM', 'DIV_mv', ''];

# ENTER-Make Stack Frame for Procedure Parameters.
T['ENTER IMM:s16=$simm16 IMM:u8=$uimm8 REG:SUPP:rw=$BPv REG:SUPP:rw=$SPa MEM:SUPP:w:sx={BASE:r=$SPa SZ=SZ_V}', 'D64=1', 'ENTER', ''];

# HLT-Halt.
T['HLT', 'NONE', 'HLT', ''];

# IDIV-Signed Divide.
T['IDIV REG:r=$GPR8m   REG:SUPP:rw=AX                   ', 'MOD=REG', 'IDIV_mb', ''];
T['IDIV MEM:r:u8=$mem8 REG:SUPP:rw=AX                   ', 'MOD=MEM', 'IDIV_mb', ''];
T['IDIV REG:r=$GPRvm   REG:SUPP:rw=$AXv REG:SUPP:rw=$DXv', 'MOD=REG', 'IDIV_mv', ''];
T['IDIV MEM:r:sx=$memv REG:SUPP:rw=$AXv REG:SUPP:rw=$DXv', 'MOD=MEM', 'IDIV_mv', ''];

# IMUL-Signed Multiply.
T['IMUL REG:r=$GPR8m   REG:SUPP:r=AL    REG:SUPP:w=AX  ', 'MOD=REG', 'IMUL_mb    ', ''];
T['IMUL MEM:r:u8=$mem8 REG:SUPP:r=AL    REG:SUPP:w=AX  ', 'MOD=MEM', 'IMUL_mb    ', ''];
T['IMUL REG:r=$GPRvm   REG:SUPP:rw=$AXv REG:SUPP:w=$DXv', 'MOD=REG', 'IMUL_mv    ', ''];
T['IMUL MEM:r:sx=$memv REG:SUPP:rw=$AXv REG:SUPP:w=$DXv', 'MOD=MEM', 'IMUL_mv    ', ''];
T['IMUL REG:rw=$GPRvr  REG:r=$GPRvm                    ', 'MOD=REG', 'IMUL_rvmv  ', ''];
T['IMUL REG:rw=$GPRvr  MEM:r:sx=$memv                  ', 'MOD=MEM', 'IMUL_rvmv  ', ''];
T['IMUL REG:w=$GPRvr   REG:r=$GPRvm     IMM:s8=$simm8  ', 'MOD=REG', 'IMUL_rvmvsb', ''];
T['IMUL REG:w=$GPRvr   MEM:r:sx=$memv   IMM:s8=$simm8  ', 'MOD=MEM', 'IMUL_rvmvsb', ''];
T['IMUL REG:w=$GPRvr   REG:r=$GPRvm     IMM:sx=$simmz  ', 'MOD=REG', 'IMUL_rvmvsz', ''];
T['IMUL REG:w=$GPRvr   MEM:r:sx=$memv   IMM:sx=$simmz  ', 'MOD=MEM', 'IMUL_rvmvsz', ''];

# IN-Input from Port.
T['IN REG:w=AL  IMM:u8=$uimm8', 'NONE  ', 'IN_bub', ''];
T['IN REG:w=AL  REG:r=DX     ', 'NONE  ', 'IN_bw ', ''];
T['IN REG:w=AX  REG:r=DX     ', 'OSZ=16', 'IN_x  ', ''];
T['IN REG:w=EAX REG:r=DX     ', 'OSZ=32', 'IN_x  ', ''];
T['IN REG:w=EAX REG:r=DX     ', 'OSZ=64', 'IN_x  ', ''];
T['IN REG:w=AX  IMM:u8=$uimm8', 'OSZ=16', 'IN_xub', ''];
T['IN REG:w=EAX IMM:u8=$uimm8', 'OSZ=32', 'IN_xub', ''];
T['IN REG:w=EAX IMM:u8=$uimm8', 'OSZ=64', 'IN_xub', ''];

# INC-Increment by 1.
T['INC REG:rw=$GPR8m  ', 'MOD=REG', 'INC_mb  ', '      '];
T['INC MEM:rw:u8=$mem8', 'MOD=MEM', 'INC_mb  ', '$xlock'];
T['INC REG:rw=$GPRvm  ', 'MOD=REG', 'INC_mv  ', '      '];
T['INC MEM:rw:sx=$memv', 'MOD=MEM', 'INC_mv  ', '$xlock'];
T['INC REG:rw=$GPRvo  ', 'NONE   ', 'INC_ov_1', '      '];
T['INC REG:rw=$GPRvo  ', 'NONE   ', 'INC_ov_2', '      '];
T['INC REG:rw=$GPRvo  ', 'NONE   ', 'INC_ov_3', '      '];
T['INC REG:rw=$GPRvo  ', 'NONE   ', 'INC_ov_4', '      '];
T['INC REG:rw=$GPRvo  ', 'NONE   ', 'INC_ov_5', '      '];
T['INC REG:rw=$GPRvo  ', 'NONE   ', 'INC_ov_6', '      '];
T['INC REG:rw=$GPRvo  ', 'NONE   ', 'INC_ov_7', '      '];
T['INC REG:rw=$GPRvo  ', 'NONE   ', 'INC_ov_8', '      '];

# INS/INSB/INSW/INSD-Input from Port to String.
T['INSB MEM:SUPP:w:u8={SEG:r=ES BASE:r=$DIa SZ=8}    REG:SUPP:r=DX                  ', 'PF2=0 PF3=0             ', 'INSB', 'rep=1 repnz=1'];
T['INSB MEM:SUPP:cw:u8={SEG:r=ES BASE:r=$DIa SZ=8}   REG:SUPP:r=DX REG:SUPP:rcw=$CXa', 'P66=0 PF2=0 PF3=1       ', 'INSB', 'rep=1 repnz=1'];
T['INSB MEM:SUPP:cw:u8={SEG:r=ES BASE:r=$DIa SZ=8}   REG:SUPP:r=DX REG:SUPP:rcw=$CXa', 'P66=0 PF2=1 PF3=0       ', 'INSB', 'rep=1 repnz=1'];
T['INSD MEM:SUPP:w:s32={SEG:r=ES BASE:r=$DIa SZ=32}  REG:SUPP:r=DX                  ', 'OSZ=32 PF2=0 PF3=0      ', 'INSD', 'rep=1 repnz=1'];
T['INSD MEM:SUPP:w:s32={SEG:r=ES BASE:r=$DIa SZ=32}  REG:SUPP:r=DX                  ', 'OSZ=64 PF2=0 PF3=0      ', 'INSD', 'rep=1 repnz=1'];
T['INSD MEM:SUPP:cw:s32={SEG:r=ES BASE:r=$DIa SZ=32} REG:SUPP:r=DX REG:SUPP:rcw=$CXa', 'OSZ=32 P66=0 PF2=0 PF3=1', 'INSD', 'rep=1 repnz=1'];
T['INSD MEM:SUPP:cw:s32={SEG:r=ES BASE:r=$DIa SZ=32} REG:SUPP:r=DX REG:SUPP:rcw=$CXa', 'OSZ=64 P66=0 PF2=0 PF3=1', 'INSD', 'rep=1 repnz=1'];
T['INSD MEM:SUPP:cw:s32={SEG:r=ES BASE:r=$DIa SZ=32} REG:SUPP:r=DX REG:SUPP:rcw=$CXa', 'OSZ=32 P66=0 PF2=1 PF3=0', 'INSD', 'rep=1 repnz=1'];
T['INSD MEM:SUPP:cw:s32={SEG:r=ES BASE:r=$DIa SZ=32} REG:SUPP:r=DX REG:SUPP:rcw=$CXa', 'OSZ=64 P66=0 PF2=1 PF3=0', 'INSD', 'rep=1 repnz=1'];
T['INSW MEM:SUPP:w:s16={SEG:r=ES BASE:r=$DIa SZ=16}  REG:SUPP:r=DX                  ', 'PF2=0 PF3=0             ', 'INSW', 'rep=1 repnz=1'];
T['INSW MEM:SUPP:cw:s16={SEG:r=ES BASE:r=$DIa SZ=16} REG:SUPP:r=DX REG:SUPP:rcw=$CXa', 'P66=0 PF2=0 PF3=1       ', 'INSW', 'rep=1 repnz=1'];
T['INSW MEM:SUPP:cw:s16={SEG:r=ES BASE:r=$DIa SZ=16} REG:SUPP:r=DX REG:SUPP:rcw=$CXa', 'P66=0 PF2=1 PF3=0       ', 'INSW', 'rep=1 repnz=1'];

# INT-n/INTO/INT3/INT1-Call to Interrupt Procedure.
T['INT  IMM:u8=$uimm8   REG:SUPP:w=$IPa', 'NONE', 'INT ', ''];
T['INT1 REG:SUPP:w=$IPa                ', 'NONE', 'INT1', ''];
T['INT3 REG:SUPP:w=$IPa                ', 'NONE', 'INT3', ''];
T['INTO REG:SUPP:w=EIP                 ', 'NONE', 'INTO', ''];

# INVD-Invalidate Internal Caches.
T['INVD', 'NONE', 'INVD', ''];

# INVLPG-Invalidate TLB Entries.
T['INVLPG MEM:r:u8=$mem8', 'NONE', 'INVLPG', ''];

# IRET/IRETD/IRETQ-Interrupt Return.
T['IRET  REG:SUPP:w=$IPa REG:SUPP:rw=$SPa MEM:SUPP:r:sx={BASE:r=$SPa SZ=SZ_RST}', 'NONE', 'IRET ', ''];
T['IRETD REG:SUPP:w=$IPa REG:SUPP:rw=$SPa MEM:SUPP:r:sx={BASE:r=$SPa SZ=SZ_RST}', 'NONE', 'IRETD', ''];
T['IRETQ REG:SUPP:w=RIP  REG:SUPP:rw=$SPa MEM:SUPP:r:sx={BASE:r=$SPa SZ=SZ_RST}', 'NONE', 'IRETQ', ''];

# JMP-Jump.
T['JMP PTR:r=$ptrdpp  REG:SUPP:w=EIP ', 'NONE               ', 'JMP     ', '       '];
T['JMP MEM:r=$memdpr  REG:SUPP:w=$IPa', 'NONE               ', 'JMP_mdpr', '       '];
T['JMP REG:r=$GPRvm   REG:SUPP:w=$IPa', 'MOD=REG F64=1      ', 'JMP_mv  ', 'bound=1'];
T['JMP MEM:r:sx=$memv REG:SUPP:w=$IPa', 'MOD=MEM F64=1      ', 'JMP_mv  ', 'bound=1'];
T['JMP REL:s8=$rel8   REG:SUPP:rw=RIP', 'MODE=64 F64=1      ', 'JMP_s8  ', '       '];
T['JMP REL:s8=$rel8   REG:SUPP:rw=EIP', 'MODE=NO64          ', 'JMP_s8  ', '       '];
T['JMP REL:s32=$rel32 REG:SUPP:rw=RIP', 'AMD=0 MODE=64 F64=1', 'JMP_sx  ', 'bound=1'];
T['JMP REL:sx=$relz   REG:SUPP:rw=RIP', 'AMD=1 MODE=64 D64=1', 'JMP_sx  ', 'bound=1'];
T['JMP REL:sx=$relz   REG:SUPP:rw=EIP', 'MODE=NO64          ', 'JMP_sx  ', 'bound=1'];

# Jcc-Jump if Condition Is Met.
T['JBE   REL:s8=$rel8   REG:SUPP:rw=RIP                ', 'MODE=64 F64=1  ', 'JBE_s8 ', 'bhint=1 bound=1'];
T['JBE   REL:s8=$rel8   REG:SUPP:rw=EIP                ', 'MODE=NO64      ', 'JBE_s8 ', 'bhint=1 bound=1'];
T['JBE   REL:s32=$rel32 REG:SUPP:rw=RIP                ', 'AMD=0 F64=1    ', 'JBE_sx ', 'bhint=1 bound=1'];
T['JBE   REL:sx=$relz   REG:SUPP:rw=RIP                ', 'AMD=1 D64=1    ', 'JBE_sx ', 'bhint=1 bound=1'];
T['JBE   REL:sx=$relz   REG:SUPP:rw=EIP                ', 'NONE           ', 'JBE_sz ', 'bhint=1 bound=1'];
T['JB    REL:s8=$rel8   REG:SUPP:rw=RIP                ', 'MODE=64 F64=1  ', 'JB_s8  ', 'bhint=1 bound=1'];
T['JB    REL:s8=$rel8   REG:SUPP:rw=EIP                ', 'MODE=NO64      ', 'JB_s8  ', 'bhint=1 bound=1'];
T['JB    REL:s32=$rel32 REG:SUPP:rw=RIP                ', 'AMD=0 F64=1    ', 'JB_sx  ', 'bhint=1 bound=1'];
T['JB    REL:sx=$relz   REG:SUPP:rw=RIP                ', 'AMD=1 D64=1    ', 'JB_sx  ', 'bhint=1 bound=1'];
T['JB    REL:sx=$relz   REG:SUPP:rw=EIP                ', 'NONE           ', 'JB_sz  ', 'bhint=1 bound=1'];
T['JCXZ  REL:s8=$rel8   REG:SUPP:r=CX   REG:SUPP:rw=IP ', 'MODE=64        ', 'JCXZ   ', '               '];
T['JCXZ  REL:s8=$rel8   REG:SUPP:r=CX   REG:SUPP:rw=IP ', 'MODE=NO64      ', 'JCXZ   ', '               '];
T['JECXZ REL:s8=$rel8   REG:SUPP:r=ECX  REG:SUPP:rw=RIP', 'MODE=64 F64=1  ', 'JECXZ  ', '               '];
T['JECXZ REL:s8=$rel8   REG:SUPP:r=ECX  REG:SUPP:rw=EIP', 'MODE=NO64      ', 'JECXZ  ', '               '];
T['JLE   REL:s8=$rel8   REG:SUPP:rw=RIP                ', 'MODE=64 F64=1  ', 'JLE_s8 ', 'bhint=1 bound=1'];
T['JLE   REL:s8=$rel8   REG:SUPP:rw=EIP                ', 'MODE=NO64      ', 'JLE_s8 ', 'bhint=1 bound=1'];
T['JLE   REL:s32=$rel32 REG:SUPP:rw=RIP                ', 'AMD=0 F64=1    ', 'JLE_sx ', 'bhint=1 bound=1'];
T['JLE   REL:sx=$relz   REG:SUPP:rw=RIP                ', 'AMD=1 D64=1    ', 'JLE_sx ', 'bhint=1 bound=1'];
T['JLE   REL:sx=$relz   REG:SUPP:rw=EIP                ', 'NONE           ', 'JLE_sz ', 'bhint=1 bound=1'];
T['JL    REL:s8=$rel8   REG:SUPP:rw=RIP                ', 'MODE=64 F64=1  ', 'JL_s8  ', 'bhint=1 bound=1'];
T['JL    REL:s8=$rel8   REG:SUPP:rw=EIP                ', 'MODE=NO64      ', 'JL_s8  ', 'bhint=1 bound=1'];
T['JL    REL:s32=$rel32 REG:SUPP:rw=RIP                ', 'AMD=0 F64=1    ', 'JL_sx  ', 'bhint=1 bound=1'];
T['JL    REL:sx=$relz   REG:SUPP:rw=RIP                ', 'AMD=1 D64=1    ', 'JL_sx  ', 'bhint=1 bound=1'];
T['JL    REL:sx=$relz   REG:SUPP:rw=EIP                ', 'NONE           ', 'JL_sz  ', 'bhint=1 bound=1'];
T['JNBE  REL:s8=$rel8   REG:SUPP:rw=RIP                ', 'MODE=64 F64=1  ', 'JNBE_s8', 'bhint=1 bound=1'];
T['JNBE  REL:s8=$rel8   REG:SUPP:rw=EIP                ', 'MODE=NO64      ', 'JNBE_s8', 'bhint=1 bound=1'];
T['JNBE  REL:s32=$rel32 REG:SUPP:rw=RIP                ', 'AMD=0 F64=1    ', 'JNBE_sx', 'bhint=1 bound=1'];
T['JNBE  REL:sx=$relz   REG:SUPP:rw=RIP                ', 'AMD=1 D64=1    ', 'JNBE_sx', 'bhint=1 bound=1'];
T['JNBE  REL:sx=$relz   REG:SUPP:rw=EIP                ', 'NONE           ', 'JNBE_sz', 'bhint=1 bound=1'];
T['JNB   REL:s8=$rel8   REG:SUPP:rw=RIP                ', 'MODE=64 F64=1  ', 'JNB_s8 ', 'bhint=1 bound=1'];
T['JNB   REL:s8=$rel8   REG:SUPP:rw=EIP                ', 'MODE=NO64      ', 'JNB_s8 ', 'bhint=1 bound=1'];
T['JNB   REL:s32=$rel32 REG:SUPP:rw=RIP                ', 'AMD=0 F64=1    ', 'JNB_sx ', 'bhint=1 bound=1'];
T['JNB   REL:sx=$relz   REG:SUPP:rw=RIP                ', 'AMD=1 D64=1    ', 'JNB_sx ', 'bhint=1 bound=1'];
T['JNB   REL:sx=$relz   REG:SUPP:rw=EIP                ', 'NONE           ', 'JNB_sz ', 'bhint=1 bound=1'];
T['JNLE  REL:s8=$rel8   REG:SUPP:rw=RIP                ', 'MODE=64 F64=1  ', 'JNLE_s8', 'bhint=1 bound=1'];
T['JNLE  REL:s8=$rel8   REG:SUPP:rw=EIP                ', 'MODE=NO64      ', 'JNLE_s8', 'bhint=1 bound=1'];
T['JNLE  REL:s32=$rel32 REG:SUPP:rw=RIP                ', 'AMD=0 F64=1    ', 'JNLE_sx', 'bhint=1 bound=1'];
T['JNLE  REL:sx=$relz   REG:SUPP:rw=RIP                ', 'AMD=1 D64=1    ', 'JNLE_sx', 'bhint=1 bound=1'];
T['JNLE  REL:sx=$relz   REG:SUPP:rw=EIP                ', 'NONE           ', 'JNLE_sz', 'bhint=1 bound=1'];
T['JNL   REL:s8=$rel8   REG:SUPP:rw=RIP                ', 'MODE=64 F64=1  ', 'JNL_s8 ', 'bhint=1 bound=1'];
T['JNL   REL:s8=$rel8   REG:SUPP:rw=EIP                ', 'MODE=NO64      ', 'JNL_s8 ', 'bhint=1 bound=1'];
T['JNL   REL:s32=$rel32 REG:SUPP:rw=RIP                ', 'AMD=0 F64=1    ', 'JNL_sx ', 'bhint=1 bound=1'];
T['JNL   REL:sx=$relz   REG:SUPP:rw=RIP                ', 'AMD=1 D64=1    ', 'JNL_sx ', 'bhint=1 bound=1'];
T['JNL   REL:sx=$relz   REG:SUPP:rw=EIP                ', 'NONE           ', 'JNL_sz ', 'bhint=1 bound=1'];
T['JNO   REL:s8=$rel8   REG:SUPP:rw=RIP                ', 'MODE=64 F64=1  ', 'JNO_s8 ', 'bhint=1 bound=1'];
T['JNO   REL:s8=$rel8   REG:SUPP:rw=EIP                ', 'MODE=NO64      ', 'JNO_s8 ', 'bhint=1 bound=1'];
T['JNO   REL:s32=$rel32 REG:SUPP:rw=RIP                ', 'AMD=0 F64=1    ', 'JNO_sx ', 'bhint=1 bound=1'];
T['JNO   REL:sx=$relz   REG:SUPP:rw=RIP                ', 'AMD=1 D64=1    ', 'JNO_sx ', 'bhint=1 bound=1'];
T['JNO   REL:sx=$relz   REG:SUPP:rw=EIP                ', 'NONE           ', 'JNO_sz ', 'bhint=1 bound=1'];
T['JNP   REL:s8=$rel8   REG:SUPP:rw=RIP                ', 'MODE=64 F64=1  ', 'JNP_s8 ', 'bhint=1 bound=1'];
T['JNP   REL:s8=$rel8   REG:SUPP:rw=EIP                ', 'MODE=NO64      ', 'JNP_s8 ', 'bhint=1 bound=1'];
T['JNP   REL:s32=$rel32 REG:SUPP:rw=RIP                ', 'AMD=0 F64=1    ', 'JNP_sx ', 'bhint=1 bound=1'];
T['JNP   REL:sx=$relz   REG:SUPP:rw=RIP                ', 'AMD=1 D64=1    ', 'JNP_sx ', 'bhint=1 bound=1'];
T['JNP   REL:sx=$relz   REG:SUPP:rw=EIP                ', 'NONE           ', 'JNP_sz ', 'bhint=1 bound=1'];
T['JNS   REL:s8=$rel8   REG:SUPP:rw=RIP                ', 'MODE=64 F64=1  ', 'JNS_s8 ', 'bhint=1 bound=1'];
T['JNS   REL:s8=$rel8   REG:SUPP:rw=EIP                ', 'MODE=NO64      ', 'JNS_s8 ', 'bhint=1 bound=1'];
T['JNS   REL:s32=$rel32 REG:SUPP:rw=RIP                ', 'AMD=0 F64=1    ', 'JNS_sx ', 'bhint=1 bound=1'];
T['JNS   REL:sx=$relz   REG:SUPP:rw=RIP                ', 'AMD=1 D64=1    ', 'JNS_sx ', 'bhint=1 bound=1'];
T['JNS   REL:sx=$relz   REG:SUPP:rw=EIP                ', 'NONE           ', 'JNS_sz ', 'bhint=1 bound=1'];
T['JNZ   REL:s8=$rel8   REG:SUPP:rw=RIP                ', 'MODE=64 F64=1  ', 'JNZ_s8 ', 'bhint=1 bound=1'];
T['JNZ   REL:s8=$rel8   REG:SUPP:rw=EIP                ', 'MODE=NO64      ', 'JNZ_s8 ', 'bhint=1 bound=1'];
T['JNZ   REL:s32=$rel32 REG:SUPP:rw=RIP                ', 'AMD=0 F64=1    ', 'JNZ_sx ', 'bhint=1 bound=1'];
T['JNZ   REL:sx=$relz   REG:SUPP:rw=RIP                ', 'AMD=1 D64=1    ', 'JNZ_sx ', 'bhint=1 bound=1'];
T['JNZ   REL:sx=$relz   REG:SUPP:rw=EIP                ', 'NONE           ', 'JNZ_sz ', 'bhint=1 bound=1'];
T['JO    REL:s8=$rel8   REG:SUPP:rw=RIP                ', 'MODE=64 F64=1  ', 'JO_s8  ', 'bhint=1 bound=1'];
T['JO    REL:s8=$rel8   REG:SUPP:rw=EIP                ', 'MODE=NO64      ', 'JO_s8  ', 'bhint=1 bound=1'];
T['JO    REL:s32=$rel32 REG:SUPP:rw=RIP                ', 'AMD=0 F64=1    ', 'JO_sx  ', 'bhint=1 bound=1'];
T['JO    REL:sx=$relz   REG:SUPP:rw=RIP                ', 'AMD=1 D64=1    ', 'JO_sx  ', 'bhint=1 bound=1'];
T['JO    REL:sx=$relz   REG:SUPP:rw=EIP                ', 'NONE           ', 'JO_sz  ', 'bhint=1 bound=1'];
T['JP    REL:s8=$rel8   REG:SUPP:rw=RIP                ', 'MODE=64 F64=1  ', 'JP_s8  ', 'bhint=1 bound=1'];
T['JP    REL:s8=$rel8   REG:SUPP:rw=EIP                ', 'MODE=NO64      ', 'JP_s8  ', 'bhint=1 bound=1'];
T['JP    REL:s32=$rel32 REG:SUPP:rw=RIP                ', 'AMD=0 F64=1    ', 'JP_sx  ', 'bhint=1 bound=1'];
T['JP    REL:sx=$relz   REG:SUPP:rw=RIP                ', 'AMD=1 D64=1    ', 'JP_sx  ', 'bhint=1 bound=1'];
T['JP    REL:sx=$relz   REG:SUPP:rw=EIP                ', 'NONE           ', 'JP_sz  ', 'bhint=1 bound=1'];
T['JRCXZ REL:s8=$rel8   REG:SUPP:r=RCX  REG:SUPP:rw=RIP', 'MODE=64 F64=1  ', 'JRCXZ  ', '               '];
T['JRCXZ REL:s8=$rel8   REG:SUPP:r=RCX  REG:SUPP:rw=RIP', 'MODE=NO64 F64=1', 'JRCXZ  ', '               '];
T['JS    REL:s8=$rel8   REG:SUPP:rw=RIP                ', 'MODE=64 F64=1  ', 'JS_s8  ', 'bhint=1 bound=1'];
T['JS    REL:s8=$rel8   REG:SUPP:rw=EIP                ', 'MODE=NO64      ', 'JS_s8  ', 'bhint=1 bound=1'];
T['JS    REL:s32=$rel32 REG:SUPP:rw=RIP                ', 'AMD=0 F64=1    ', 'JS_sx  ', 'bhint=1 bound=1'];
T['JS    REL:sx=$relz   REG:SUPP:rw=RIP                ', 'AMD=1 D64=1    ', 'JS_sx  ', 'bhint=1 bound=1'];
T['JS    REL:sx=$relz   REG:SUPP:rw=EIP                ', 'NONE           ', 'JS_sz  ', 'bhint=1 bound=1'];
T['JZ    REL:s8=$rel8   REG:SUPP:rw=RIP                ', 'MODE=64 F64=1  ', 'JZ_s8  ', 'bhint=1 bound=1'];
T['JZ    REL:s8=$rel8   REG:SUPP:rw=EIP                ', 'MODE=NO64      ', 'JZ_s8  ', 'bhint=1 bound=1'];
T['JZ    REL:s32=$rel32 REG:SUPP:rw=RIP                ', 'AMD=0 F64=1    ', 'JZ_sx  ', 'bhint=1 bound=1'];
T['JZ    REL:sx=$relz   REG:SUPP:rw=RIP                ', 'AMD=1 D64=1    ', 'JZ_sx  ', 'bhint=1 bound=1'];
T['JZ    REL:sx=$relz   REG:SUPP:rw=EIP                ', 'NONE           ', 'JZ_sz  ', 'bhint=1 bound=1'];

# LAHF-Load Status Flags into AH Register.
T['LAHF REG:SUPP:w=AH', 'NONE', 'LAHF', ''];

# LAR-Load Access Rights Byte.
T['LAR REG:cw=$GPRvr REG:r=$GPRvm    ', 'MOD=REG', 'LAR', ''];
T['LAR REG:cw=$GPRvr MEM:r:s16=$mem16', 'MOD=MEM', 'LAR', ''];

# LEA-Load Effective Address.
T['LEA REG:w=$GPRvr AGEN:r=$agen', 'NONE', 'LEA', ''];

# LEAVE-High Level Procedure Exit.
T['LEAVE MEM:SUPP:r:sx={SEG:r=SS BASE:r=$BPa SZ=SZ_V} REG:SUPP:rw=$BPv REG:SUPP:rw=$SPv', 'D64=1', 'LEAVE', ''];

# LLDT-Load Local Descriptor Table Register.
T['LLDT REG:r=$GPR16m    REG:SUPP:w=LDTR', 'MOD=REG', 'LLDT', ''];
T['LLDT MEM:r:s16=$mem16 REG:SUPP:w=LDTR', 'MOD=MEM', 'LLDT', ''];

# LMSW-Load Machine Status Word.
T['LMSW REG:r=$GPR16m    REG:SUPP:w=CR0', 'MOD=REG', 'LMSW', ''];
T['LMSW MEM:r:s16=$mem16 REG:SUPP:w=CR0', 'MOD=MEM', 'LMSW', ''];

# LODS/LODSB/LODSW/LODSD/LODSQ-Load String.
T['LODSB REG:SUPP:w=AL   MEM:SUPP:r:u8={SEG:r=DS BASE:r=$SIa SZ=8}                     ', 'PF2=0 PF3=0      ', 'LODSB', 'rep=1 repnz=1'];
T['LODSB REG:SUPP:cw=AL  MEM:SUPP:cr:u8={SEG:r=DS BASE:r=$SIa SZ=8}   REG:SUPP:rcw=$CXa', 'P66=0 PF2=0 PF3=1', 'LODSB', 'rep=1 repnz=1'];
T['LODSB REG:SUPP:cw=AL  MEM:SUPP:cr:u8={SEG:r=DS BASE:r=$SIa SZ=8}   REG:SUPP:rcw=$CXa', 'P66=0 PF2=1 PF3=0', 'LODSB', 'rep=1 repnz=1'];
T['LODSD REG:SUPP:w=EAX  MEM:SUPP:r:s32={SEG:r=DS BASE:r=$SIa SZ=32}                   ', 'PF2=0 PF3=0      ', 'LODSD', 'rep=1 repnz=1'];
T['LODSD REG:SUPP:cw=EAX MEM:SUPP:cr:s32={SEG:r=DS BASE:r=$SIa SZ=32} REG:SUPP:rcw=$CXa', 'P66=0 PF2=0 PF3=1', 'LODSD', 'rep=1 repnz=1'];
T['LODSD REG:SUPP:cw=EAX MEM:SUPP:cr:s32={SEG:r=DS BASE:r=$SIa SZ=32} REG:SUPP:rcw=$CXa', 'P66=0 PF2=1 PF3=0', 'LODSD', 'rep=1 repnz=1'];
T['LODSQ REG:SUPP:w=RAX  MEM:SUPP:r:s64={SEG:r=DS BASE:r=$SIa SZ=64}                   ', 'PF2=0 PF3=0      ', 'LODSQ', 'rep=1 repnz=1'];
T['LODSQ REG:SUPP:cw=RAX MEM:SUPP:cr:s64={SEG:r=DS BASE:r=$SIa SZ=64} REG:SUPP:rcw=$CXa', 'P66=0 PF2=0 PF3=1', 'LODSQ', 'rep=1 repnz=1'];
T['LODSQ REG:SUPP:cw=RAX MEM:SUPP:cr:s64={SEG:r=DS BASE:r=$SIa SZ=64} REG:SUPP:rcw=$CXa', 'P66=0 PF2=1 PF3=0', 'LODSQ', 'rep=1 repnz=1'];
T['LODSW REG:SUPP:w=AX   MEM:SUPP:r:s16={SEG:r=DS BASE:r=$SIa SZ=16}                   ', 'PF2=0 PF3=0      ', 'LODSW', 'rep=1 repnz=1'];
T['LODSW REG:SUPP:cw=AX  MEM:SUPP:cr:s16={SEG:r=DS BASE:r=$SIa SZ=16} REG:SUPP:rcw=$CXa', 'P66=0 PF2=0 PF3=1', 'LODSW', 'rep=1 repnz=1'];
T['LODSW REG:SUPP:cw=AX  MEM:SUPP:cr:s16={SEG:r=DS BASE:r=$SIa SZ=16} REG:SUPP:rcw=$CXa', 'P66=0 PF2=1 PF3=0', 'LODSW', 'rep=1 repnz=1'];

# LOOP/LOOPcc-Loop According to ECX Counter.
T['LOOP   REL:s8=$rel8 REG:SUPP:rw=$CXa REG:SUPP:rw=$IPa', 'F64=1', 'LOOP  ', ''];
T['LOOPE  REL:s8=$rel8 REG:SUPP:rw=$CXa REG:SUPP:rw=$IPa', 'F64=1', 'LOOPE ', ''];
T['LOOPNE REL:s8=$rel8 REG:SUPP:rw=$CXa REG:SUPP:rw=$IPa', 'F64=1', 'LOOPNE', ''];

# LSL-Load Segment Limit.
T['LSL REG:rw=$GPRvr REG:r=$GPRzm    ', 'MOD=REG', 'LSL', ''];
T['LSL REG:rw=$GPRvr MEM:r:s16=$mem16', 'MOD=MEM', 'LSL', ''];

# LTR-Load Task Register.
T['LTR REG:r=$GPR16m    REG:SUPP:w=TR', 'MOD=REG', 'LTR', ''];
T['LTR MEM:r:s16=$mem16 REG:SUPP:w=TR', 'MOD=MEM', 'LTR', ''];

# LZCNT-Count the Number of Leading Zero Bits.
T['LZCNT REG:w:sx=$GPRvr REG:r:sx=$GPRvm', 'MOD=REG', 'LZCNT_rvmv', ''];
T['LZCNT REG:w:sx=$GPRvr MEM:r:sx=$memv ', 'MOD=MEM', 'LZCNT_rvmv', ''];

# LGDT/LIDT-Load Global/Interrupt Descriptor Table Register.
T['LGDT MEM:r=$mem80  REG:SUPP:w=GDTR', 'MODE=64 F64=1', 'LGDT', ''];
T['LGDT MEM:r=$memppr REG:SUPP:w=GDTR', 'MODE=NO64    ', 'LGDT', ''];
T['LIDT MEM:r=$mem80  REG:SUPP:w=IDTR', 'MODE=64 F64=1', 'LIDT', ''];
T['LIDT MEM:r=$memppr REG:SUPP:w=IDTR', 'MODE=NO64    ', 'LIDT', ''];

# LDS/LES/LFS/LGS/LSS-Load Far Pointer.
T['LDS REG:w=$GPRzr MEM:r=$memdpp REG:SUPP:w=DS', 'NONE', 'LDS', ''];
T['LES REG:w=$GPRzr MEM:r=$memdpp REG:SUPP:w=ES', 'NONE', 'LES', ''];
T['LFS REG:w=$GPRvr MEM:r=$memdpr REG:SUPP:w=FS', 'NONE', 'LFS', ''];
T['LGS REG:w=$GPRvr MEM:r=$memdpr REG:SUPP:w=GS', 'NONE', 'LGS', ''];
T['LSS REG:w=$GPRvr MEM:r=$memdpr REG:SUPP:w=SS', 'NONE', 'LSS', ''];

# MOV-Move.
T['MOV REG:w=AL           MOFFS:r:s8=$moffs8', 'NONE   ', 'MOV_bs8   ', '                '];
T['MOV REG:w=$GPR8m       REG:r=$GPR8r      ', 'MOD=REG', 'MOV_mbrb  ', '                '];
T['MOV MEM:w:u8=$mem8     REG:r=$GPR8r      ', 'MOD=MEM', 'MOV_mbrb  ', 'hle=1 xrelease=1'];
T['MOV REG:w=$GPR8m       IMM:u8=$uimm8     ', 'MOD=REG', 'MOV_mbub  ', '                '];
T['MOV MEM:w:u8=$mem8     IMM:u8=$uimm8     ', 'MOD=MEM', 'MOV_mbub  ', 'hle=1 xrelease=1'];
T['MOV REG:w=$GPRvm       REG:r=$GPRvr      ', 'MOD=REG', 'MOV_mvrv  ', '                '];
T['MOV MEM:w:sx=$memv     REG:r=$GPRvr      ', 'MOD=MEM', 'MOV_mvrv  ', 'hle=1 xrelease=1'];
T['MOV REG:w=$GPRvm       IMM:sx=$simmz     ', 'MOD=REG', 'MOV_mvsz  ', '                '];
T['MOV MEM:w:sx=$memv     IMM:sx=$simmz     ', 'MOD=MEM', 'MOV_mvsz  ', 'hle=1 xrelease=1'];
T['MOV REG:w=$GPRvm       REG:r=$SRr        ', 'MOD=REG', 'MOV_mxrw  ', '                '];
T['MOV MEM:w:s16=$mem16   REG:r=$SRr        ', 'MOD=MEM', 'MOV_mxrw  ', '                '];
T['MOV REG:w=$GPR8o       IMM:u8=$uimm8     ', 'NONE   ', 'MOV_obub_1', '                '];
T['MOV REG:w=$GPR8o       IMM:u8=$uimm8     ', 'NONE   ', 'MOV_obub_2', '                '];
T['MOV REG:w=$GPR8o       IMM:u8=$uimm8     ', 'NONE   ', 'MOV_obub_3', '                '];
T['MOV REG:w=$GPR8o       IMM:u8=$uimm8     ', 'NONE   ', 'MOV_obub_4', '                '];
T['MOV REG:w=$GPR8o       IMM:u8=$uimm8     ', 'NONE   ', 'MOV_obub_5', '                '];
T['MOV REG:w=$GPR8o       IMM:u8=$uimm8     ', 'NONE   ', 'MOV_obub_6', '                '];
T['MOV REG:w=$GPR8o       IMM:u8=$uimm8     ', 'NONE   ', 'MOV_obub_7', '                '];
T['MOV REG:w=$GPR8o       IMM:u8=$uimm8     ', 'NONE   ', 'MOV_obub_8', '                '];
T['MOV REG:w=$GPRvo       IMM:sx=$simmv     ', 'NONE   ', 'MOV_ovuv_1', '                '];
T['MOV REG:w=$GPRvo       IMM:sx=$simmv     ', 'NONE   ', 'MOV_ovuv_2', '                '];
T['MOV REG:w=$GPRvo       IMM:sx=$simmv     ', 'NONE   ', 'MOV_ovuv_3', '                '];
T['MOV REG:w=$GPRvo       IMM:sx=$simmv     ', 'NONE   ', 'MOV_ovuv_4', '                '];
T['MOV REG:w=$GPRvo       IMM:sx=$simmv     ', 'NONE   ', 'MOV_ovuv_5', '                '];
T['MOV REG:w=$GPRvo       IMM:sx=$simmv     ', 'NONE   ', 'MOV_ovuv_6', '                '];
T['MOV REG:w=$GPRvo       IMM:sx=$simmv     ', 'NONE   ', 'MOV_ovuv_7', '                '];
T['MOV REG:w=$GPRvo       IMM:sx=$simmv     ', 'NONE   ', 'MOV_ovuv_8', '                '];
T['MOV REG:w=$GPR8r       REG:r=$GPR8m      ', 'MOD=REG', 'MOV_rbmb  ', '                '];
T['MOV REG:w=$GPR8r       MEM:r:u8=$mem8    ', 'MOD=MEM', 'MOV_rbmb  ', '                '];
T['MOV REG:w=$GPRvr       REG:r=$GPRvm      ', 'MOD=REG', 'MOV_rvmv  ', '                '];
T['MOV REG:w=$GPRvr       MEM:r:sx=$memv    ', 'MOD=MEM', 'MOV_rvmv  ', '                '];
T['MOV REG:w=$SRr         REG:r=$GPR16m     ', 'MOD=REG', 'MOV_rwmw  ', '                '];
T['MOV REG:w=$SRr         MEM:r:s16=$mem16  ', 'MOD=MEM', 'MOV_rwmw  ', '                '];
T['MOV MOFFS:w:s8=$moffs8 REG:r=AL          ', 'NONE   ', 'MOV_s8b   ', '                '];
T['MOV MOFFS:w:sx=$moffsv REG:r=$AXv        ', 'NONE   ', 'MOV_svv   ', '                '];
T['MOV REG:w=$AXv         MOFFS:r:sx=$moffsv', 'NONE   ', 'MOV_vsv   ', '                '];

# MOVBE-Move Data After Swapping Bytes.
T['MOVBE MEM:w:sx=$memv REG:r=$GPRvr  ', 'NONE', 'MOVBE_mvrv', ''];
T['MOVBE REG:w=$GPRvr   MEM:r:sx=$memv', 'NONE', 'MOVBE_rvmv', ''];

# MOVDIR64B-Move 64 Bytes as Direct Store.
T['MOVDIR64B REG:r=$GPRar MEM:r:u32=$mem512 MEM:SUPP:w:s32={BASE:r=$GPRar SZ=512}         ', 'MODE=64  ', 'MOVDIR64B', ''];
T['MOVDIR64B REG:r=$GPRar MEM:r:u32=$mem512 MEM:SUPP:w:s32={SEG:r=ES BASE:r=$GPRar SZ=512}', 'MODE=NO64', 'MOVDIR64B', ''];

# MOVDIRI-Move Doubleword as Direct Store.
T['MOVDIRI MEM:w:u32=$mem32 REG:r:u32=$GPR32r', 'W=0', 'MOVDIRI', ''];
T['MOVDIRI MEM:w:u64=$mem64 REG:r:u64=$GPR64r', 'W=1', 'MOVDIRI', ''];

# MOVS/MOVSB/MOVSW/MOVSD/MOVSQ-Move Data from String to String.
T['MOVSB MEM:SUPP:w:u8={SEG:r=ES BASE:r=$DIa SZ=8}    MEM:SUPP:r:u8={SEG:r=DS BASE:r=$SIa SZ=8}                     ', 'PF2=0 PF3=0      ', 'MOVSB', 'rep=1 repnz=1'];
T['MOVSB MEM:SUPP:cw:u8={SEG:r=ES BASE:r=$DIa SZ=8}   MEM:SUPP:cr:u8={SEG:r=DS BASE:r=$SIa SZ=8}   REG:SUPP:rcw=$CXa', 'P66=0 PF2=0 PF3=1', 'MOVSB', 'rep=1 repnz=1'];
T['MOVSB MEM:SUPP:cw:u8={SEG:r=ES BASE:r=$DIa SZ=8}   MEM:SUPP:cr:u8={SEG:r=DS BASE:r=$SIa SZ=8}   REG:SUPP:rcw=$CXa', 'P66=0 PF2=1 PF3=0', 'MOVSB', 'rep=1 repnz=1'];
T['MOVSD MEM:SUPP:w:s32={SEG:r=ES BASE:r=$DIa SZ=32}  MEM:SUPP:r:s32={SEG:r=DS BASE:r=$SIa SZ=32}                   ', 'PF2=0 PF3=0      ', 'MOVSD', 'rep=1 repnz=1'];
T['MOVSD MEM:SUPP:cw:s32={SEG:r=ES BASE:r=$DIa SZ=32} MEM:SUPP:cr:s32={SEG:r=DS BASE:r=$SIa SZ=32} REG:SUPP:rcw=$CXa', 'P66=0 PF2=0 PF3=1', 'MOVSD', 'rep=1 repnz=1'];
T['MOVSD MEM:SUPP:cw:s32={SEG:r=ES BASE:r=$DIa SZ=32} MEM:SUPP:cr:s32={SEG:r=DS BASE:r=$SIa SZ=32} REG:SUPP:rcw=$CXa', 'P66=0 PF2=1 PF3=0', 'MOVSD', 'rep=1 repnz=1'];
T['MOVSQ MEM:SUPP:w:s64={SEG:r=ES BASE:r=$DIa SZ=64}  MEM:SUPP:r:s64={SEG:r=DS BASE:r=$SIa SZ=64}                   ', 'PF2=0 PF3=0      ', 'MOVSQ', 'rep=1 repnz=1'];
T['MOVSQ MEM:SUPP:cw:s64={SEG:r=ES BASE:r=$DIa SZ=64} MEM:SUPP:cr:s64={SEG:r=DS BASE:r=$SIa SZ=64} REG:SUPP:rcw=$CXa', 'P66=0 PF2=0 PF3=1', 'MOVSQ', 'rep=1 repnz=1'];
T['MOVSQ MEM:SUPP:cw:s64={SEG:r=ES BASE:r=$DIa SZ=64} MEM:SUPP:cr:s64={SEG:r=DS BASE:r=$SIa SZ=64} REG:SUPP:rcw=$CXa', 'P66=0 PF2=1 PF3=0', 'MOVSQ', 'rep=1 repnz=1'];
T['MOVSW MEM:SUPP:w:s16={SEG:r=ES BASE:r=$DIa SZ=16}  MEM:SUPP:r:s16={SEG:r=DS BASE:r=$SIa SZ=16}                   ', 'PF2=0 PF3=0      ', 'MOVSW', 'rep=1 repnz=1'];
T['MOVSW MEM:SUPP:cw:s16={SEG:r=ES BASE:r=$DIa SZ=16} MEM:SUPP:cr:s16={SEG:r=DS BASE:r=$SIa SZ=16} REG:SUPP:rcw=$CXa', 'P66=0 PF2=0 PF3=1', 'MOVSW', 'rep=1 repnz=1'];
T['MOVSW MEM:SUPP:cw:s16={SEG:r=ES BASE:r=$DIa SZ=16} MEM:SUPP:cr:s16={SEG:r=DS BASE:r=$SIa SZ=16} REG:SUPP:rcw=$CXa', 'P66=0 PF2=1 PF3=0', 'MOVSW', 'rep=1 repnz=1'];

# MOVSX/MOVSXD-Move with Sign-Extension.
T['MOVSXD REG:w=$GPRvr REG:r=$GPRzm    ', 'MOD=REG', 'MOVSXD    ', ''];
T['MOVSXD REG:w=$GPRvr MEM:r:s32=$mem32', 'MOD=MEM', 'MOVSXD    ', ''];
T['MOVSX  REG:w=$GPRvr REG:r=$GPR8m    ', 'MOD=REG', 'MOVSX_rvmb', ''];
T['MOVSX  REG:w=$GPRvr MEM:r:u8=$mem8  ', 'MOD=MEM', 'MOVSX_rvmb', ''];
T['MOVSX  REG:w=$GPRvr REG:r=$GPR16m   ', 'MOD=REG', 'MOVSX_rvmw', ''];
T['MOVSX  REG:w=$GPRvr MEM:r:s16=$mem16', 'MOD=MEM', 'MOVSX_rvmw', ''];

# MOVZX-Move with Zero-Extend.
T['MOVZX REG:w=$GPRvr REG:r=$GPR8m    ', 'MOD=REG', 'MOVZX_rvmb', ''];
T['MOVZX REG:w=$GPRvr MEM:r:u8=$mem8  ', 'MOD=MEM', 'MOVZX_rvmb', ''];
T['MOVZX REG:w=$GPRvr REG:r=$GPR16m   ', 'MOD=REG', 'MOVZX_rvmw', ''];
T['MOVZX REG:w=$GPRvr MEM:r:s16=$mem16', 'MOD=MEM', 'MOVZX_rvmw', ''];

# MOV-Move to/from Control Registers.
T['MOV REG:w=$GPR64m REG:r=$CRr   ', 'MODE=64  ', 'MOV_mxrq_1', ''];
T['MOV REG:w=$GPR32m REG:r=$CRr   ', 'MODE=NO64', 'MOV_mxrq_1', ''];
T['MOV REG:w=$CRr    REG:r=$GPR64m', 'MODE=64  ', 'MOV_rqmx_1', ''];
T['MOV REG:w=$CRr    REG:r=$GPR32m', 'MODE=NO64', 'MOV_rqmx_1', ''];

# MOV-Move to/from Debug Registers.
T['MOV REG:w=$GPR64m REG:r=$DRr   ', 'MODE=64  ', 'MOV_mxrq_2', ''];
T['MOV REG:w=$GPR32m REG:r=$DRr   ', 'MODE=NO64', 'MOV_mxrq_2', ''];
T['MOV REG:w=$DRr    REG:r=$GPR64m', 'MODE=64  ', 'MOV_rqmx_2', ''];
T['MOV REG:w=$DRr    REG:r=$GPR32m', 'MODE=NO64', 'MOV_rqmx_2', ''];

# MUL-Unsigned Multiply.
T['MUL REG:r=$GPR8m   REG:SUPP:r=AL    REG:SUPP:w=AX  ', 'MOD=REG', 'MUL_mb', ''];
T['MUL MEM:r:u8=$mem8 REG:SUPP:r=AL    REG:SUPP:w=AX  ', 'MOD=MEM', 'MUL_mb', ''];
T['MUL REG:r=$GPRvm   REG:SUPP:rw=$AXv REG:SUPP:w=$DXv', 'MOD=REG', 'MUL_mv', ''];
T['MUL MEM:r:sx=$memv REG:SUPP:rw=$AXv REG:SUPP:w=$DXv', 'MOD=MEM', 'MUL_mv', ''];

# MULX-Unsigned Multiply Without Affecting Flags.
T['MULX REG:w:s32=$GPR32r REG:w:s32=$GPR32n REG:r:s32=$GPR32m REG:SUPP:r=EDX', 'MOD=REG MODE=64 W=0', 'MULX', ''];
T['MULX REG:w:s64=$GPR64r REG:w:s64=$GPR64n REG:r:s64=$GPR64m REG:SUPP:r=RDX', 'MOD=REG MODE=64 W=1', 'MULX', ''];
T['MULX REG:w:s32=$GPR32r REG:w:s32=$GPR32n MEM:r:s32=$mem32  REG:SUPP:r=EDX', 'MOD=MEM MODE=64 W=0', 'MULX', ''];
T['MULX REG:w:s64=$GPR64r REG:w:s64=$GPR64n MEM:r:s64=$mem64  REG:SUPP:r=RDX', 'MOD=MEM MODE=64 W=1', 'MULX', ''];
T['MULX REG:w:s32=$GPR32r REG:w:s32=$GPR32n REG:r:s32=$GPR32m REG:SUPP:r=EDX', 'MOD=REG MODE=NO64  ', 'MULX', ''];
T['MULX REG:w:s32=$GPR32r REG:w:s32=$GPR32n MEM:r:s32=$mem32  REG:SUPP:r=EDX', 'MOD=MEM MODE=NO64  ', 'MULX', ''];

# NEG-Two's Complement Negation.
T['NEG REG:rw=$GPR8m  ', 'MOD=REG', 'NEG_mb', '      '];
T['NEG MEM:rw:u8=$mem8', 'MOD=MEM', 'NEG_mb', '$xlock'];
T['NEG REG:rw=$GPRvm  ', 'MOD=REG', 'NEG_mv', '      '];
T['NEG MEM:rw:sx=$memv', 'MOD=MEM', 'NEG_mv', '$xlock'];

# NOP-No Operation.
T['NOP                              ', 'NONE                     ', 'NOP        ', ''];
T['NOP REG:r=$GPRvm                 ', 'NONE                     ', 'NOP_mv_1   ', ''];
T['NOP REG:r=$GPRvm   MEM:r:sx=$memv', 'NONE                     ', 'NOP_mv_10  ', ''];
T['NOP REG:r=$GPRvm                 ', 'NONE                     ', 'NOP_mv_2   ', ''];
T['NOP REG:r=$GPRvm                 ', 'NONE                     ', 'NOP_mv_3   ', ''];
T['NOP REG:r=$GPRvm                 ', 'NONE                     ', 'NOP_mv_4   ', ''];
T['NOP REG:r=$GPRvm                 ', 'MOD=REG                  ', 'NOP_mv_5   ', ''];
T['NOP MEM:r:sx=$memv               ', 'MOD=MEM                  ', 'NOP_mv_5   ', ''];
T['NOP REG:r=$GPRvm                 ', 'MOD=REG                  ', 'NOP_mv_6   ', ''];
T['NOP MEM:r:sx=$memv               ', 'MOD=MEM                  ', 'NOP_mv_6   ', ''];
T['NOP REG:r=$GPRvm                 ', 'MOD=REG                  ', 'NOP_mv_7   ', ''];
T['NOP MEM:r:sx=$memv               ', 'MOD=MEM                  ', 'NOP_mv_7   ', ''];
T['NOP REG:r=$GPRvm                 ', 'MOD=REG                  ', 'NOP_mv_8   ', ''];
T['NOP MEM:r:sx=$memv               ', 'MOD=MEM                  ', 'NOP_mv_8   ', ''];
T['NOP REG:r=$GPRvm   MEM:r:sx=$memv', 'NONE                     ', 'NOP_mv_9   ', ''];
T['NOP REG:r=$GPRvm   REG:r=$GPRvr  ', 'NONE                     ', 'NOP_mvrv_1 ', ''];
T['NOP REG:r=$GPRvm   REG:r=$GPRvr  ', 'NONE                     ', 'NOP_mvrv_10', ''];
T['NOP REG:r=$GPRvm   REG:r=$GPRvr  ', 'NONE                     ', 'NOP_mvrv_11', ''];
T['NOP REG:r=$GPRvm   REG:r=$GPRvr  ', 'NONE                     ', 'NOP_mvrv_12', ''];
T['NOP REG:r=$GPRvm   REG:r=$GPRvr  ', 'NONE                     ', 'NOP_mvrv_13', ''];
T['NOP REG:r=$GPRvm   REG:r=$GPRvr  ', 'RM=0                     ', 'NOP_mvrv_14', ''];
T['NOP REG:r=$GPRvm   REG:r=$GPRvr  ', 'RM=1                     ', 'NOP_mvrv_14', ''];
T['NOP REG:r=$GPRvm   REG:r=$GPRvr  ', 'CET=0 RM=2               ', 'NOP_mvrv_14', ''];
T['NOP REG:r=$GPRvm   REG:r=$GPRvr  ', 'CET=0 RM=3               ', 'NOP_mvrv_14', ''];
T['NOP REG:r=$GPRvm   REG:r=$GPRvr  ', 'RM=4                     ', 'NOP_mvrv_14', ''];
T['NOP REG:r=$GPRvm   REG:r=$GPRvr  ', 'RM=5                     ', 'NOP_mvrv_14', ''];
T['NOP REG:r=$GPRvm   REG:r=$GPRvr  ', 'RM=6                     ', 'NOP_mvrv_14', ''];
T['NOP REG:r=$GPRvm   REG:r=$GPRvr  ', 'RM=7                     ', 'NOP_mvrv_14', ''];
T['NOP REG:r=$GPRvm   REG:r=$GPRvr  ', 'MODE=64 W=1              ', 'NOP_mvrv_15', ''];
T['NOP REG:r=$GPRvm   REG:r=$GPRvr  ', 'MODE=NO64 W=0            ', 'NOP_mvrv_15', ''];
T['NOP REG:r=$GPRvm   REG:r=$GPRvr  ', 'MOD=REG                  ', 'NOP_mvrv_16', ''];
T['NOP MEM:r:sx=$memv REG:r=$GPRvr  ', 'MOD=MEM                  ', 'NOP_mvrv_16', ''];
T['NOP REG:r=$GPRvm   REG:r=$GPRvr  ', 'MOD=REG                  ', 'NOP_mvrv_17', ''];
T['NOP MEM:r:sx=$memv REG:r=$GPRvr  ', 'MOD=MEM                  ', 'NOP_mvrv_17', ''];
T['NOP REG:r=$GPRvm   REG:r=$GPRvr  ', 'MOD=REG                  ', 'NOP_mvrv_2 ', ''];
T['NOP MEM:r:sx=$memv REG:r=$GPRvr  ', 'MOD=MEM                  ', 'NOP_mvrv_2 ', ''];
T['NOP REG:r=$GPRvm   REG:r=$GPRvr  ', 'MPX=0                    ', 'NOP_mvrv_3 ', ''];
T['NOP REG:r=$GPRvm   REG:r=$GPRvr  ', 'MPX=1 P66=0 PF2=0 PF3=0  ', 'NOP_mvrv_3 ', ''];
T['NOP REG:r=$GPRvm   REG:r=$GPRvr  ', 'MPX=0                    ', 'NOP_mvrv_4 ', ''];
T['NOP REG:r=$GPRvm   REG:r=$GPRvr  ', 'MPX=1 P66=0 PF2=0 PF3=0  ', 'NOP_mvrv_4 ', ''];
T['NOP REG:r=$GPRvm   REG:r=$GPRvr  ', 'MPX=1 P66=0 PF2=0 PF3=1  ', 'NOP_mvrv_4 ', ''];
T['NOP REG:r=$GPRvm   REG:r=$GPRvr  ', 'MOD=REG                  ', 'NOP_mvrv_5 ', ''];
T['NOP MEM:r:sx=$memv REG:r=$GPRvr  ', 'CLDEMOTE=0 MOD=MEM       ', 'NOP_mvrv_5 ', ''];
T['NOP REG:r=$GPRvm   REG:r=$GPRvr  ', 'MOD=REG                  ', 'NOP_mvrv_6 ', ''];
T['NOP MEM:r:sx=$memv REG:r=$GPRvr  ', 'MOD=MEM                  ', 'NOP_mvrv_6 ', ''];
T['NOP MEM:r:sx=$memv REG:r=$GPRvr  ', 'MOD=MEM                  ', 'NOP_mvrv_7 ', ''];
T['NOP REG:r=$GPRvm   REG:r=$GPRvr  ', 'MOD=REG P66=0 PF2=0 PF3=0', 'NOP_mvrv_7 ', ''];
T['NOP REG:r=$GPRvm   REG:r=$GPRvr  ', 'MOD=REG P66=1 PF2=0 PF3=0', 'NOP_mvrv_7 ', ''];
T['NOP REG:r=$GPRvm   REG:r=$GPRvr  ', 'MOD=REG PF2=1 PF3=0      ', 'NOP_mvrv_7 ', ''];
T['NOP REG:r=$GPRvm   REG:r=$GPRvr  ', 'NONE                     ', 'NOP_mvrv_8 ', ''];
T['NOP REG:r=$GPRvm   REG:r=$GPRvr  ', 'NONE                     ', 'NOP_mvrv_9 ', ''];

# NOT-One's Complement Negation.
T['NOT REG:rw=$GPR8m  ', 'MOD=REG', 'NOT_mb', '      '];
T['NOT MEM:rw:u8=$mem8', 'MOD=MEM', 'NOT_mb', '$xlock'];
T['NOT REG:rw=$GPRvm  ', 'MOD=REG', 'NOT_mv', '      '];
T['NOT MEM:rw:sx=$memv', 'MOD=MEM', 'NOT_mv', '$xlock'];

# OR-Logical Inclusive OR.
T['OR REG:rw=AL       IMM:u8=$uimm8 ', 'NONE   ', 'OR_bub   ', '      '];
T['OR REG:rw=$GPR8m   REG:r=$GPR8r  ', 'MOD=REG', 'OR_mbrb  ', '      '];
T['OR MEM:rw:u8=$mem8 REG:r=$GPR8r  ', 'MOD=MEM', 'OR_mbrb  ', '$xlock'];
T['OR REG:rw=$GPR8m   IMM:s8=$simm8 ', 'MOD=REG', 'OR_mbsb_1', '      '];
T['OR MEM:rw:u8=$mem8 IMM:s8=$simm8 ', 'MOD=MEM', 'OR_mbsb_1', '$xlock'];
T['OR REG:rw=$GPR8m   IMM:s8=$simm8 ', 'MOD=REG', 'OR_mbsb_2', '      '];
T['OR MEM:rw:u8=$mem8 IMM:s8=$simm8 ', 'MOD=MEM', 'OR_mbsb_2', '$xlock'];
T['OR REG:rw=$GPRvm   REG:r=$GPRvr  ', 'MOD=REG', 'OR_mvrv  ', '      '];
T['OR MEM:rw:sx=$memv REG:r=$GPRvr  ', 'MOD=MEM', 'OR_mvrv  ', '$xlock'];
T['OR REG:rw=$GPRvm   IMM:s8=$simm8 ', 'MOD=REG', 'OR_mvsb  ', '      '];
T['OR MEM:rw:sx=$memv IMM:s8=$simm8 ', 'MOD=MEM', 'OR_mvsb  ', '$xlock'];
T['OR REG:rw=$GPRvm   IMM:sx=$simmz ', 'MOD=REG', 'OR_mvsz  ', '      '];
T['OR MEM:rw:sx=$memv IMM:sx=$simmz ', 'MOD=MEM', 'OR_mvsz  ', '$xlock'];
T['OR REG:rw=$GPR8r   REG:r=$GPR8m  ', 'MOD=REG', 'OR_rbmb  ', '      '];
T['OR REG:rw=$GPR8r   MEM:r:u8=$mem8', 'MOD=MEM', 'OR_rbmb  ', '      '];
T['OR REG:rw=$GPRvr   REG:r=$GPRvm  ', 'MOD=REG', 'OR_rvmv  ', '      '];
T['OR REG:rw=$GPRvr   MEM:r:sx=$memv', 'MOD=MEM', 'OR_rvmv  ', '      '];
T['OR REG:rw=$AXv     IMM:sx=$simmz ', 'NONE   ', 'OR_vsz   ', '      '];

# OUT-Output to Port.
T['OUT IMM:u8=$uimm8 REG:r=AL ', 'NONE  ', 'OUT_ubb', ''];
T['OUT IMM:u8=$uimm8 REG:r=AX ', 'OSZ=16', 'OUT_ubx', ''];
T['OUT IMM:u8=$uimm8 REG:r=EAX', 'OSZ=32', 'OUT_ubx', ''];
T['OUT IMM:u8=$uimm8 REG:r=EAX', 'OSZ=64', 'OUT_ubx', ''];
T['OUT REG:r=DX      REG:r=AX ', 'OSZ=16', 'OUT_w  ', ''];
T['OUT REG:r=DX      REG:r=EAX', 'OSZ=32', 'OUT_w  ', ''];
T['OUT REG:r=DX      REG:r=EAX', 'OSZ=64', 'OUT_w  ', ''];
T['OUT REG:r=DX      REG:r=AL ', 'NONE  ', 'OUT_wb ', ''];

# OUTS/OUTSB/OUTSW/OUTSD-Output String to Port.
T['OUTSB REG:SUPP:r=DX MEM:SUPP:r:u8={SEG:r=DS BASE:r=$SIa SZ=8}                     ', 'PF2=0 PF3=0             ', 'OUTSB', 'rep=1 repnz=1'];
T['OUTSB REG:SUPP:r=DX MEM:SUPP:cr:u8={SEG:r=DS BASE:r=$SIa SZ=8}   REG:SUPP:rcw=$CXa', 'P66=0 PF2=0 PF3=1       ', 'OUTSB', 'rep=1 repnz=1'];
T['OUTSB REG:SUPP:r=DX MEM:SUPP:cr:u8={SEG:r=DS BASE:r=$SIa SZ=8}   REG:SUPP:rcw=$CXa', 'P66=0 PF2=1 PF3=0       ', 'OUTSB', 'rep=1 repnz=1'];
T['OUTSD REG:SUPP:r=DX MEM:SUPP:r:s32={SEG:r=DS BASE:r=$SIa SZ=32}                   ', 'OSZ=32 PF2=0 PF3=0      ', 'OUTSD', 'rep=1 repnz=1'];
T['OUTSD REG:SUPP:r=DX MEM:SUPP:r:s32={SEG:r=DS BASE:r=$SIa SZ=32}                   ', 'OSZ=64 PF2=0 PF3=0      ', 'OUTSD', 'rep=1 repnz=1'];
T['OUTSD REG:SUPP:r=DX MEM:SUPP:cr:s32={SEG:r=DS BASE:r=$SIa SZ=32} REG:SUPP:rcw=$CXa', 'OSZ=32 P66=0 PF2=0 PF3=1', 'OUTSD', 'rep=1 repnz=1'];
T['OUTSD REG:SUPP:r=DX MEM:SUPP:cr:s32={SEG:r=DS BASE:r=$SIa SZ=32} REG:SUPP:rcw=$CXa', 'OSZ=64 P66=0 PF2=0 PF3=1', 'OUTSD', 'rep=1 repnz=1'];
T['OUTSD REG:SUPP:r=DX MEM:SUPP:cr:s32={SEG:r=DS BASE:r=$SIa SZ=32} REG:SUPP:rcw=$CXa', 'OSZ=32 P66=0 PF2=1 PF3=0', 'OUTSD', 'rep=1 repnz=1'];
T['OUTSD REG:SUPP:r=DX MEM:SUPP:cr:s32={SEG:r=DS BASE:r=$SIa SZ=32} REG:SUPP:rcw=$CXa', 'OSZ=64 P66=0 PF2=1 PF3=0', 'OUTSD', 'rep=1 repnz=1'];
T['OUTSW REG:SUPP:r=DX MEM:SUPP:r:s16={SEG:r=DS BASE:r=$SIa SZ=16}                   ', 'PF2=0 PF3=0             ', 'OUTSW', 'rep=1 repnz=1'];
T['OUTSW REG:SUPP:r=DX MEM:SUPP:cr:s16={SEG:r=DS BASE:r=$SIa SZ=16} REG:SUPP:rcw=$CXa', 'P66=0 PF2=0 PF3=1       ', 'OUTSW', 'rep=1 repnz=1'];
T['OUTSW REG:SUPP:r=DX MEM:SUPP:cr:s16={SEG:r=DS BASE:r=$SIa SZ=16} REG:SUPP:rcw=$CXa', 'P66=0 PF2=1 PF3=0       ', 'OUTSW', 'rep=1 repnz=1'];

# PAUSE-Spin Loop Hint.
T['PAUSE', 'NONE', 'PAUSE', ''];

# PCLMULQDQ-Carry-Less Multiplication Quadword.
T['PCLMULQDQ  VREG:rw:s32=$XMMr VREG:r:s32=$XMMm  IMM:u8=$uimm8                  ', 'MOD=REG', 'PCLMULQDQ            ', ''];
T['PCLMULQDQ  VREG:rw:s32=$XMMr MEM:r:s32=$mem128 IMM:u8=$uimm8                  ', 'MOD=MEM', 'PCLMULQDQ            ', ''];
T['VPCLMULQDQ VREG:w:u128=$YMMr VREG:r:u64=$YMMn  VREG:r:u64=$YMMm  IMM:u8=$uimm8', 'MOD=REG', 'VPCLMULQDQ_rynymyub_1', ''];
T['VPCLMULQDQ VREG:w:u128=$YMMr VREG:r:u64=$YMMn  MEM:r:u64=$mem256 IMM:u8=$uimm8', 'MOD=MEM', 'VPCLMULQDQ_rynymyub_1', ''];

# PCONFIG.
T['PCONFIG REG:SUPP:rw:u32=EAX REG:SUPP:crw:u32=EBX REG:SUPP:crw:u32=ECX REG:SUPP:crw:u32=EDX', 'NONE', 'PCONFIG', ''];

# PDEP-Parallel Bits Deposit.
T['PDEP REG:w:s32=$GPR32r REG:r:s32=$GPR32n REG:r:s32=$GPR32m', 'MOD=REG MODE=64 W=0', 'PDEP', ''];
T['PDEP REG:w:s64=$GPR64r REG:r:s64=$GPR64n REG:r:s64=$GPR64m', 'MOD=REG MODE=64 W=1', 'PDEP', ''];
T['PDEP REG:w:s32=$GPR32r REG:r:s32=$GPR32n MEM:r:s32=$mem32 ', 'MOD=MEM MODE=64 W=0', 'PDEP', ''];
T['PDEP REG:w:s64=$GPR64r REG:r:s64=$GPR64n MEM:r:s64=$mem64 ', 'MOD=MEM MODE=64 W=1', 'PDEP', ''];
T['PDEP REG:w:s32=$GPR32r REG:r:s32=$GPR32n REG:r:s32=$GPR32m', 'MOD=REG MODE=NO64  ', 'PDEP', ''];
T['PDEP REG:w:s32=$GPR32r REG:r:s32=$GPR32n MEM:r:s32=$mem32 ', 'MOD=MEM MODE=NO64  ', 'PDEP', ''];

# PEXT-Parallel Bits Extract.
T['PEXT REG:w:s32=$GPR32r REG:r:s32=$GPR32n REG:r:s32=$GPR32m', 'MOD=REG MODE=64 W=0', 'PEXT', ''];
T['PEXT REG:w:s64=$GPR64r REG:r:s64=$GPR64n REG:r:s64=$GPR64m', 'MOD=REG MODE=64 W=1', 'PEXT', ''];
T['PEXT REG:w:s32=$GPR32r REG:r:s32=$GPR32n MEM:r:s32=$mem32 ', 'MOD=MEM MODE=64 W=0', 'PEXT', ''];
T['PEXT REG:w:s64=$GPR64r REG:r:s64=$GPR64n MEM:r:s64=$mem64 ', 'MOD=MEM MODE=64 W=1', 'PEXT', ''];
T['PEXT REG:w:s32=$GPR32r REG:r:s32=$GPR32n REG:r:s32=$GPR32m', 'MOD=REG MODE=NO64  ', 'PEXT', ''];
T['PEXT REG:w:s32=$GPR32r REG:r:s32=$GPR32n MEM:r:s32=$mem32 ', 'MOD=MEM MODE=NO64  ', 'PEXT', ''];

# POP-Pop a Value from the Stack.
T['POP REG:w=$GPRvm   REG:SUPP:rw=$SPa MEM:SUPP:r:sx={BASE:r=$SPa SZ=SZ_V}', 'MOD=REG D64=1', 'POP_mv  ', ''];
T['POP MEM:w:sx=$memv REG:SUPP:rw=$SPa MEM:SUPP:r:sx={BASE:r=$SPa SZ=SZ_V}', 'MOD=MEM D64=1', 'POP_mv  ', ''];
T['POP REG:w=$GPRvo   REG:SUPP:rw=$SPa MEM:SUPP:r:sx={BASE:r=$SPa SZ=SZ_V}', 'D64=1        ', 'POP_ov_1', ''];
T['POP REG:w=$GPRvo   REG:SUPP:rw=$SPa MEM:SUPP:r:sx={BASE:r=$SPa SZ=SZ_V}', 'D64=1        ', 'POP_ov_2', ''];
T['POP REG:w=$GPRvo   REG:SUPP:rw=$SPa MEM:SUPP:r:sx={BASE:r=$SPa SZ=SZ_V}', 'D64=1        ', 'POP_ov_3', ''];
T['POP REG:w=$GPRvo   REG:SUPP:rw=$SPa MEM:SUPP:r:sx={BASE:r=$SPa SZ=SZ_V}', 'D64=1        ', 'POP_ov_4', ''];
T['POP REG:w=$GPRvo   REG:SUPP:rw=$SPa MEM:SUPP:r:sx={BASE:r=$SPa SZ=SZ_V}', 'D64=1        ', 'POP_ov_5', ''];
T['POP REG:w=$GPRvo   REG:SUPP:rw=$SPa MEM:SUPP:r:sx={BASE:r=$SPa SZ=SZ_V}', 'D64=1        ', 'POP_ov_6', ''];
T['POP REG:w=$GPRvo   REG:SUPP:rw=$SPa MEM:SUPP:r:sx={BASE:r=$SPa SZ=SZ_V}', 'D64=1        ', 'POP_ov_7', ''];
T['POP REG:w=$GPRvo   REG:SUPP:rw=$SPa MEM:SUPP:r:sx={BASE:r=$SPa SZ=SZ_V}', 'D64=1        ', 'POP_ov_8', ''];
T['POP REG:w=ES       REG:SUPP:rw=$SPa MEM:SUPP:r:sx={BASE:r=$SPa SZ=SZ_V}', 'NONE         ', 'POP_w_1 ', ''];
T['POP REG:w=SS       REG:SUPP:rw=$SPa MEM:SUPP:r:sx={BASE:r=$SPa SZ=SZ_V}', 'NONE         ', 'POP_w_2 ', ''];
T['POP REG:w=DS       REG:SUPP:rw=$SPa MEM:SUPP:r:sx={BASE:r=$SPa SZ=SZ_V}', 'NONE         ', 'POP_w_3 ', ''];
T['POP REG:w=FS       REG:SUPP:rw=$SPa MEM:SUPP:r:sx={BASE:r=$SPa SZ=SZ_V}', 'D64=1        ', 'POP_w_4 ', ''];
T['POP REG:w=GS       REG:SUPP:rw=$SPa MEM:SUPP:r:sx={BASE:r=$SPa SZ=SZ_V}', 'D64=1        ', 'POP_w_5 ', ''];

# POPA/POPAD-Pop All General-Purpose Registers.
T['POPA  REG:SUPP:w=AX  REG:SUPP:w=CX  REG:SUPP:w=DX  REG:SUPP:w=BX  REG:SUPP:w=BP  REG:SUPP:w=SI  REG:SUPP:w=DI  REG:SUPP:rw=$SPa MEM:SUPP:r:sx={BASE:r=$SPa SZ=SZ_X}', 'NONE', 'POPA ', ''];
T['POPAD REG:SUPP:w=EAX REG:SUPP:w=ECX REG:SUPP:w=EDX REG:SUPP:w=EBX REG:SUPP:w=EBP REG:SUPP:w=ESI REG:SUPP:w=EDI REG:SUPP:rw=$SPa MEM:SUPP:r:sx={BASE:r=$SPa SZ=SZ_X}', 'NONE', 'POPAD', ''];

# POPF/POPFD/POPFQ-Pop Stack into EFLAGS Register.
T['POPF  REG:SUPP:rw=$SPa MEM:SUPP:r:s16={BASE:r=$SPa SZ=16}', 'MODE=64 D64=1  ', 'POPF ', ''];
T['POPF  REG:SUPP:rw=$SPa MEM:SUPP:r:s16={BASE:r=$SPa SZ=16}', 'MODE=NO64 D64=1', 'POPF ', ''];
T['POPFD REG:SUPP:rw=$SPa MEM:SUPP:r:s32={BASE:r=$SPa SZ=32}', 'D64=1          ', 'POPFD', ''];
T['POPFQ REG:SUPP:rw=$SPa MEM:SUPP:r:s64={BASE:r=$SPa SZ=64}', 'OSZ=32 D64=1   ', 'POPFQ', ''];
T['POPFQ REG:SUPP:rw=$SPa MEM:SUPP:r:s64={BASE:r=$SPa SZ=64}', 'OSZ=64 D64=1   ', 'POPFQ', ''];

# PUSH-Push Word, Doubleword or Quadword Onto the Stack.
T['PUSH REG:r=$GPRvm   REG:SUPP:rw=$SPa MEM:SUPP:w:sx={BASE:r=$SPa SZ=SZ_V}', 'MOD=REG D64=1', 'PUSH_mv  ', ''];
T['PUSH MEM:r:sx=$memv REG:SUPP:rw=$SPa MEM:SUPP:w:sx={BASE:r=$SPa SZ=SZ_V}', 'MOD=MEM D64=1', 'PUSH_mv  ', ''];
T['PUSH REG:r=$GPRvo   REG:SUPP:rw=$SPa MEM:SUPP:w:sx={BASE:r=$SPa SZ=SZ_V}', 'D64=1        ', 'PUSH_ov_1', ''];
T['PUSH REG:r=$GPRvo   REG:SUPP:rw=$SPa MEM:SUPP:w:sx={BASE:r=$SPa SZ=SZ_V}', 'D64=1        ', 'PUSH_ov_2', ''];
T['PUSH REG:r=$GPRvo   REG:SUPP:rw=$SPa MEM:SUPP:w:sx={BASE:r=$SPa SZ=SZ_V}', 'D64=1        ', 'PUSH_ov_3', ''];
T['PUSH REG:r=$GPRvo   REG:SUPP:rw=$SPa MEM:SUPP:w:sx={BASE:r=$SPa SZ=SZ_V}', 'D64=1        ', 'PUSH_ov_4', ''];
T['PUSH REG:r=$GPRvo   REG:SUPP:rw=$SPa MEM:SUPP:w:sx={BASE:r=$SPa SZ=SZ_V}', 'D64=1        ', 'PUSH_ov_5', ''];
T['PUSH REG:r=$GPRvo   REG:SUPP:rw=$SPa MEM:SUPP:w:sx={BASE:r=$SPa SZ=SZ_V}', 'D64=1        ', 'PUSH_ov_6', ''];
T['PUSH REG:r=$GPRvo   REG:SUPP:rw=$SPa MEM:SUPP:w:sx={BASE:r=$SPa SZ=SZ_V}', 'D64=1        ', 'PUSH_ov_7', ''];
T['PUSH REG:r=$GPRvo   REG:SUPP:rw=$SPa MEM:SUPP:w:sx={BASE:r=$SPa SZ=SZ_V}', 'D64=1        ', 'PUSH_ov_8', ''];
T['PUSH IMM:s8=$simm8  REG:SUPP:rw=$SPa MEM:SUPP:w:sx={BASE:r=$SPa SZ=SZ_V}', 'D64=1        ', 'PUSH_sb  ', ''];
T['PUSH IMM:sx=$simmz  REG:SUPP:rw=$SPa MEM:SUPP:w:sx={BASE:r=$SPa SZ=SZ_V}', 'D64=1        ', 'PUSH_sz  ', ''];
T['PUSH REG:r=ES       REG:SUPP:rw=$SPa MEM:SUPP:w:sx={BASE:r=$SPa SZ=SZ_V}', 'NONE         ', 'PUSH_w_1 ', ''];
T['PUSH REG:r=CS       REG:SUPP:rw=$SPa MEM:SUPP:w:sx={BASE:r=$SPa SZ=SZ_V}', 'NONE         ', 'PUSH_w_2 ', ''];
T['PUSH REG:r=SS       REG:SUPP:rw=$SPa MEM:SUPP:w:sx={BASE:r=$SPa SZ=SZ_V}', 'NONE         ', 'PUSH_w_3 ', ''];
T['PUSH REG:r=DS       REG:SUPP:rw=$SPa MEM:SUPP:w:sx={BASE:r=$SPa SZ=SZ_V}', 'NONE         ', 'PUSH_w_4 ', ''];
T['PUSH REG:r=FS       REG:SUPP:rw=$SPa MEM:SUPP:w:sx={BASE:r=$SPa SZ=SZ_V}', 'D64=1        ', 'PUSH_w_5 ', ''];
T['PUSH REG:r=GS       REG:SUPP:rw=$SPa MEM:SUPP:w:sx={BASE:r=$SPa SZ=SZ_V}', 'D64=1        ', 'PUSH_w_6 ', ''];

# PUSHA/PUSHAD-Push All General-Purpose Registers.
T['PUSHA  REG:SUPP:r=AX  REG:SUPP:r=CX  REG:SUPP:r=DX  REG:SUPP:r=BX  REG:SUPP:r=SP  REG:SUPP:r=BP  REG:SUPP:r=SI  REG:SUPP:r=DI  REG:SUPP:rw=$SPa MEM:SUPP:w:sx={BASE:r=$SPa SZ=SZ_X}', 'NONE', 'PUSHA ', ''];
T['PUSHAD REG:SUPP:r=EAX REG:SUPP:r=ECX REG:SUPP:r=EDX REG:SUPP:r=EBX REG:SUPP:r=ESP REG:SUPP:r=EBP REG:SUPP:r=ESI REG:SUPP:r=EDI REG:SUPP:rw=$SPa MEM:SUPP:w:sx={BASE:r=$SPa SZ=SZ_X}', 'NONE', 'PUSHAD', ''];

# PUSHF/PUSHFD/PUSHFQ-Push EFLAGS Register onto the Stack.
T['PUSHF  REG:SUPP:rw=$SPa MEM:SUPP:w:s16={BASE:r=$SPa SZ=16}', 'MODE=64 D64=1  ', 'PUSHF ', ''];
T['PUSHF  REG:SUPP:rw=$SPa MEM:SUPP:w:s16={BASE:r=$SPa SZ=16}', 'MODE=NO64 D64=1', 'PUSHF ', ''];
T['PUSHFD REG:SUPP:rw=$SPa MEM:SUPP:w:s32={BASE:r=$SPa SZ=32}', 'D64=1          ', 'PUSHFD', ''];
T['PUSHFQ REG:SUPP:rw=$SPa MEM:SUPP:w:s64={BASE:r=$SPa SZ=64}', 'OSZ=32 D64=1   ', 'PUSHFQ', ''];
T['PUSHFQ REG:SUPP:rw=$SPa MEM:SUPP:w:s64={BASE:r=$SPa SZ=64}', 'OSZ=64 D64=1   ', 'PUSHFQ', ''];

# RDMSR-Read from Model Specific Register.
T['RDMSR REG:SUPP:w=EAX REG:SUPP:w=EDX REG:SUPP:r=ECX', 'NONE', 'RDMSR', ''];

# RDPMC-Read Performance-Monitoring Counters.
T['RDPMC REG:SUPP:w=EAX REG:SUPP:w=EDX REG:SUPP:r=ECX', 'NONE', 'RDPMC', ''];

# RDTSC-Read Time-Stamp Counter.
T['RDTSC REG:SUPP:w=EAX REG:SUPP:w=EDX', 'NONE', 'RDTSC', ''];

# RET-Return from Procedure.
T['RET REG:SUPP:w=$IPa REG:SUPP:rw=$SPa MEM:SUPP:r:sx={BASE:r=$SPa SZ=SZ_V}                                    ', 'F64=1', 'RET_1   ', 'bound=1'];
T['RET REG:SUPP:w=$IPa REG:SUPP:rw=$SPa MEM:SUPP:r:sx={BASE:r=$SPa SZ=SZ_F}                                    ', 'NONE ', 'RET_2   ', '       '];
T['RET IMM:s16=$simm16 REG:SUPP:w=$IPa  REG:SUPP:rw=$SPa                    MEM:SUPP:r:sx={BASE:r=$SPa SZ=SZ_V}', 'F64=1', 'RET_uw_1', 'bound=1'];
T['RET IMM:s16=$simm16 REG:SUPP:w=$IPa  REG:SUPP:rw=$SPa                    MEM:SUPP:r:sx={BASE:r=$SPa SZ=SZ_F}', 'NONE ', 'RET_uw_2', '       '];

# RORX-Rotate Right Logical Without Affecting Flags.
T['RORX REG:w:s32=$GPR32r REG:r:s32=$GPR32m ROTATE:r:u8=$ror', 'MOD=REG MODE=64 W=0', 'RORX', ''];
T['RORX REG:w:s64=$GPR64r REG:r:s64=$GPR64m ROTATE:r:u8=$ror', 'MOD=REG MODE=64 W=1', 'RORX', ''];
T['RORX REG:w:s32=$GPR32r MEM:r:s32=$mem32  ROTATE:r:u8=$ror', 'MOD=MEM MODE=64 W=0', 'RORX', ''];
T['RORX REG:w:s64=$GPR64r MEM:r:s64=$mem64  ROTATE:r:u8=$ror', 'MOD=MEM MODE=64 W=1', 'RORX', ''];
T['RORX REG:w:s32=$GPR32r REG:r:s32=$GPR32m ROTATE:r:u8=$ror', 'MOD=REG MODE=NO64  ', 'RORX', ''];
T['RORX REG:w:s32=$GPR32r MEM:r:s32=$mem32  ROTATE:r:u8=$ror', 'MOD=MEM MODE=NO64  ', 'RORX', ''];

# RCL/RCR/ROL/ROR-Rotate.
T['RCL REG:rw=$GPR8m   IMM:u8=1        ', 'MOD=REG', 'RCL_mb  ', ''];
T['RCL MEM:rw:u8=$mem8 IMM:u8=1        ', 'MOD=MEM', 'RCL_mb  ', ''];
T['RCL REG:rw=$GPR8m   REG:r=CL        ', 'MOD=REG', 'RCL_mbb ', ''];
T['RCL MEM:rw:u8=$mem8 REG:r=CL        ', 'MOD=MEM', 'RCL_mbb ', ''];
T['RCL REG:rw=$GPR8m   ROTATE:r:u8=$rol', 'MOD=REG', 'RCL_mbub', ''];
T['RCL MEM:rw:u8=$mem8 ROTATE:r:u8=$rol', 'MOD=MEM', 'RCL_mbub', ''];
T['RCL REG:rw=$GPRvm   IMM:u8=1        ', 'MOD=REG', 'RCL_mv  ', ''];
T['RCL MEM:rw:sx=$memv IMM:u8=1        ', 'MOD=MEM', 'RCL_mv  ', ''];
T['RCL REG:rw=$GPRvm   REG:r=CL        ', 'MOD=REG', 'RCL_mvb ', ''];
T['RCL MEM:rw:sx=$memv REG:r=CL        ', 'MOD=MEM', 'RCL_mvb ', ''];
T['RCL REG:rw=$GPRvm   ROTATE:r:u8=$rol', 'MOD=REG', 'RCL_mvub', ''];
T['RCL MEM:rw:sx=$memv ROTATE:r:u8=$rol', 'MOD=MEM', 'RCL_mvub', ''];
T['RCR REG:rw=$GPR8m   IMM:u8=1        ', 'MOD=REG', 'RCR_mb  ', ''];
T['RCR MEM:rw:u8=$mem8 IMM:u8=1        ', 'MOD=MEM', 'RCR_mb  ', ''];
T['RCR REG:rw=$GPR8m   REG:r=CL        ', 'MOD=REG', 'RCR_mbb ', ''];
T['RCR MEM:rw:u8=$mem8 REG:r=CL        ', 'MOD=MEM', 'RCR_mbb ', ''];
T['RCR REG:rw=$GPR8m   ROTATE:r:u8=$ror', 'MOD=REG', 'RCR_mbub', ''];
T['RCR MEM:rw:u8=$mem8 ROTATE:r:u8=$ror', 'MOD=MEM', 'RCR_mbub', ''];
T['RCR REG:rw=$GPRvm   IMM:u8=1        ', 'MOD=REG', 'RCR_mv  ', ''];
T['RCR MEM:rw:sx=$memv IMM:u8=1        ', 'MOD=MEM', 'RCR_mv  ', ''];
T['RCR REG:rw=$GPRvm   REG:r=CL        ', 'MOD=REG', 'RCR_mvb ', ''];
T['RCR MEM:rw:sx=$memv REG:r=CL        ', 'MOD=MEM', 'RCR_mvb ', ''];
T['RCR REG:rw=$GPRvm   ROTATE:r:u8=$ror', 'MOD=REG', 'RCR_mvub', ''];
T['RCR MEM:rw:sx=$memv ROTATE:r:u8=$ror', 'MOD=MEM', 'RCR_mvub', ''];
T['ROL REG:rw=$GPR8m   IMM:u8=1        ', 'MOD=REG', 'ROL_mb  ', ''];
T['ROL MEM:rw:u8=$mem8 IMM:u8=1        ', 'MOD=MEM', 'ROL_mb  ', ''];
T['ROL REG:rw=$GPR8m   REG:r=CL        ', 'MOD=REG', 'ROL_mbb ', ''];
T['ROL MEM:rw:u8=$mem8 REG:r=CL        ', 'MOD=MEM', 'ROL_mbb ', ''];
T['ROL REG:rw=$GPR8m   ROTATE:r:u8=$rol', 'MOD=REG', 'ROL_mbub', ''];
T['ROL MEM:rw:u8=$mem8 ROTATE:r:u8=$rol', 'MOD=MEM', 'ROL_mbub', ''];
T['ROL REG:rw=$GPRvm   IMM:u8=1        ', 'MOD=REG', 'ROL_mv  ', ''];
T['ROL MEM:rw:sx=$memv IMM:u8=1        ', 'MOD=MEM', 'ROL_mv  ', ''];
T['ROL REG:rw=$GPRvm   REG:r=CL        ', 'MOD=REG', 'ROL_mvb ', ''];
T['ROL MEM:rw:sx=$memv REG:r=CL        ', 'MOD=MEM', 'ROL_mvb ', ''];
T['ROL REG:rw=$GPRvm   ROTATE:r:u8=$rol', 'MOD=REG', 'ROL_mvub', ''];
T['ROL MEM:rw:sx=$memv ROTATE:r:u8=$rol', 'MOD=MEM', 'ROL_mvub', ''];
T['ROR REG:rw=$GPR8m   IMM:u8=1        ', 'MOD=REG', 'ROR_mb  ', ''];
T['ROR MEM:rw:u8=$mem8 IMM:u8=1        ', 'MOD=MEM', 'ROR_mb  ', ''];
T['ROR REG:rw=$GPR8m   REG:r=CL        ', 'MOD=REG', 'ROR_mbb ', ''];
T['ROR MEM:rw:u8=$mem8 REG:r=CL        ', 'MOD=MEM', 'ROR_mbb ', ''];
T['ROR REG:rw=$GPR8m   ROTATE:r:u8=$ror', 'MOD=REG', 'ROR_mbub', ''];
T['ROR MEM:rw:u8=$mem8 ROTATE:r:u8=$ror', 'MOD=MEM', 'ROR_mbub', ''];
T['ROR REG:rw=$GPRvm   IMM:u8=1        ', 'MOD=REG', 'ROR_mv  ', ''];
T['ROR MEM:rw:sx=$memv IMM:u8=1        ', 'MOD=MEM', 'ROR_mv  ', ''];
T['ROR REG:rw=$GPRvm   REG:r=CL        ', 'MOD=REG', 'ROR_mvb ', ''];
T['ROR MEM:rw:sx=$memv REG:r=CL        ', 'MOD=MEM', 'ROR_mvb ', ''];
T['ROR REG:rw=$GPRvm   ROTATE:r:u8=$ror', 'MOD=REG', 'ROR_mvub', ''];
T['ROR MEM:rw:sx=$memv ROTATE:r:u8=$ror', 'MOD=MEM', 'ROR_mvub', ''];

# RSM-Resume from System Management Mode.
T['RSM REG:SUPP:w=$IPa', 'NONE', 'RSM', ''];

# SAHF-Store AH into Flags.
T['SAHF REG:SUPP:r=AH', 'NONE', 'SAHF', ''];

# SALC.
T['SALC REG:SUPP:w=AL', 'NONE', 'SALC', ''];

# SBB-Integer Subtraction with Borrow.
T['SBB REG:rw=AL       IMM:s8=$simm8 ', 'NONE   ', 'SBB_bsb   ', '      '];
T['SBB REG:rw=$GPR8m   REG:r=$GPR8r  ', 'MOD=REG', 'SBB_mbrb  ', '      '];
T['SBB MEM:rw:u8=$mem8 REG:r=$GPR8r  ', 'MOD=MEM', 'SBB_mbrb  ', '$xlock'];
T['SBB REG:rw=$GPR8m   IMM:s8=$simm8 ', 'MOD=REG', 'SBB_mbsb_1', '      '];
T['SBB MEM:rw:u8=$mem8 IMM:s8=$simm8 ', 'MOD=MEM', 'SBB_mbsb_1', '$xlock'];
T['SBB REG:rw=$GPR8m   IMM:s8=$simm8 ', 'MOD=REG', 'SBB_mbsb_2', '      '];
T['SBB MEM:rw:u8=$mem8 IMM:s8=$simm8 ', 'MOD=MEM', 'SBB_mbsb_2', '$xlock'];
T['SBB REG:rw=$GPRvm   REG:r=$GPRvr  ', 'MOD=REG', 'SBB_mvrv  ', '      '];
T['SBB MEM:rw:sx=$memv REG:r=$GPRvr  ', 'MOD=MEM', 'SBB_mvrv  ', '$xlock'];
T['SBB REG:rw=$GPRvm   IMM:s8=$simm8 ', 'MOD=REG', 'SBB_mvsb  ', '      '];
T['SBB MEM:rw:sx=$memv IMM:s8=$simm8 ', 'MOD=MEM', 'SBB_mvsb  ', '$xlock'];
T['SBB REG:rw=$GPRvm   IMM:sx=$simmz ', 'MOD=REG', 'SBB_mvsz  ', '      '];
T['SBB MEM:rw:sx=$memv IMM:sx=$simmz ', 'MOD=MEM', 'SBB_mvsz  ', '$xlock'];
T['SBB REG:rw=$GPR8r   REG:r=$GPR8m  ', 'MOD=REG', 'SBB_rbmb  ', '      '];
T['SBB REG:rw=$GPR8r   MEM:r:u8=$mem8', 'MOD=MEM', 'SBB_rbmb  ', '      '];
T['SBB REG:rw=$GPRvr   REG:r=$GPRvm  ', 'MOD=REG', 'SBB_rvmv  ', '      '];
T['SBB REG:rw=$GPRvr   MEM:r:sx=$memv', 'MOD=MEM', 'SBB_rvmv  ', '      '];
T['SBB REG:rw=$AXv     IMM:sx=$simmz ', 'NONE   ', 'SBB_vsz   ', '      '];

# SCAS/SCASB/SCASW/SCASD-Scan String.
T['SCASB REG:SUPP:r=AL  MEM:SUPP:r:u8={SEG:r=ES BASE:r=$DIa SZ=8}                     ', 'NONE       ', 'SCASB_1', 'repnz=1 repz=1'];
T['SCASB REG:SUPP:r=AL  MEM:SUPP:cr:u8={SEG:r=ES BASE:r=$DIa SZ=8}   REG:SUPP:rcw=$CXa', 'PF2=0 PF3=1', 'SCASB_2', 'repnz=1 repz=1'];
T['SCASB REG:SUPP:r=AL  MEM:SUPP:cr:u8={SEG:r=ES BASE:r=$DIa SZ=8}   REG:SUPP:rcw=$CXa', 'PF2=1 PF3=0', 'SCASB_2', 'repnz=1 repz=1'];
T['SCASD REG:SUPP:r=EAX MEM:SUPP:r:s32={SEG:r=ES BASE:r=$DIa SZ=32}                   ', 'NONE       ', 'SCASD_1', 'repnz=1 repz=1'];
T['SCASD REG:SUPP:r=EAX MEM:SUPP:cr:s32={SEG:r=ES BASE:r=$DIa SZ=32} REG:SUPP:rcw=$CXa', 'PF2=0 PF3=1', 'SCASD_2', 'repnz=1 repz=1'];
T['SCASD REG:SUPP:r=EAX MEM:SUPP:cr:s32={SEG:r=ES BASE:r=$DIa SZ=32} REG:SUPP:rcw=$CXa', 'PF2=1 PF3=0', 'SCASD_2', 'repnz=1 repz=1'];
T['SCASQ REG:SUPP:r=RAX MEM:SUPP:r:s64={SEG:r=ES BASE:r=$DIa SZ=64}                   ', 'NONE       ', 'SCASQ_1', 'repnz=1 repz=1'];
T['SCASQ REG:SUPP:r=RAX MEM:SUPP:cr:s64={SEG:r=ES BASE:r=$DIa SZ=64} REG:SUPP:rcw=$CXa', 'PF2=0 PF3=1', 'SCASQ_2', 'repnz=1 repz=1'];
T['SCASQ REG:SUPP:r=RAX MEM:SUPP:cr:s64={SEG:r=ES BASE:r=$DIa SZ=64} REG:SUPP:rcw=$CXa', 'PF2=1 PF3=0', 'SCASQ_2', 'repnz=1 repz=1'];
T['SCASW REG:SUPP:r=AX  MEM:SUPP:r:s16={SEG:r=ES BASE:r=$DIa SZ=16}                   ', 'NONE       ', 'SCASW_1', 'repnz=1 repz=1'];
T['SCASW REG:SUPP:r=AX  MEM:SUPP:cr:s16={SEG:r=ES BASE:r=$DIa SZ=16} REG:SUPP:rcw=$CXa', 'PF2=0 PF3=1', 'SCASW_2', 'repnz=1 repz=1'];
T['SCASW REG:SUPP:r=AX  MEM:SUPP:cr:s16={SEG:r=ES BASE:r=$DIa SZ=16} REG:SUPP:rcw=$CXa', 'PF2=1 PF3=0', 'SCASW_2', 'repnz=1 repz=1'];

# SETcc-Set Byte on Condition.
T['SETB   REG:w=$GPR8m  ', 'MOD=REG', 'SETB  ', ''];
T['SETB   MEM:w:u8=$mem8', 'MOD=MEM', 'SETB  ', ''];
T['SETBE  REG:w=$GPR8m  ', 'MOD=REG', 'SETBE ', ''];
T['SETBE  MEM:w:u8=$mem8', 'MOD=MEM', 'SETBE ', ''];
T['SETL   REG:w=$GPR8m  ', 'MOD=REG', 'SETL  ', ''];
T['SETL   MEM:w:u8=$mem8', 'MOD=MEM', 'SETL  ', ''];
T['SETLE  REG:w=$GPR8m  ', 'MOD=REG', 'SETLE ', ''];
T['SETLE  MEM:w:u8=$mem8', 'MOD=MEM', 'SETLE ', ''];
T['SETNB  REG:w=$GPR8m  ', 'MOD=REG', 'SETNB ', ''];
T['SETNB  MEM:w:u8=$mem8', 'MOD=MEM', 'SETNB ', ''];
T['SETNBE REG:w=$GPR8m  ', 'MOD=REG', 'SETNBE', ''];
T['SETNBE MEM:w:u8=$mem8', 'MOD=MEM', 'SETNBE', ''];
T['SETNL  REG:w=$GPR8m  ', 'MOD=REG', 'SETNL ', ''];
T['SETNL  MEM:w:u8=$mem8', 'MOD=MEM', 'SETNL ', ''];
T['SETNLE REG:w=$GPR8m  ', 'MOD=REG', 'SETNLE', ''];
T['SETNLE MEM:w:u8=$mem8', 'MOD=MEM', 'SETNLE', ''];
T['SETNO  REG:w=$GPR8m  ', 'MOD=REG', 'SETNO ', ''];
T['SETNO  MEM:w:u8=$mem8', 'MOD=MEM', 'SETNO ', ''];
T['SETNP  REG:w=$GPR8m  ', 'MOD=REG', 'SETNP ', ''];
T['SETNP  MEM:w:u8=$mem8', 'MOD=MEM', 'SETNP ', ''];
T['SETNS  REG:w=$GPR8m  ', 'MOD=REG', 'SETNS ', ''];
T['SETNS  MEM:w:u8=$mem8', 'MOD=MEM', 'SETNS ', ''];
T['SETNZ  REG:w=$GPR8m  ', 'MOD=REG', 'SETNZ ', ''];
T['SETNZ  MEM:w:u8=$mem8', 'MOD=MEM', 'SETNZ ', ''];
T['SETO   REG:w=$GPR8m  ', 'MOD=REG', 'SETO  ', ''];
T['SETO   MEM:w:u8=$mem8', 'MOD=MEM', 'SETO  ', ''];
T['SETP   REG:w=$GPR8m  ', 'MOD=REG', 'SETP  ', ''];
T['SETP   MEM:w:u8=$mem8', 'MOD=MEM', 'SETP  ', ''];
T['SETS   REG:w=$GPR8m  ', 'MOD=REG', 'SETS  ', ''];
T['SETS   MEM:w:u8=$mem8', 'MOD=MEM', 'SETS  ', ''];
T['SETZ   REG:w=$GPR8m  ', 'MOD=REG', 'SETZ  ', ''];
T['SETZ   MEM:w:u8=$mem8', 'MOD=MEM', 'SETZ  ', ''];

# SGDT-Store Global Descriptor Table Register.
T['SGDT MEM:w=$mem80  REG:SUPP:r=GDTR', 'MODE=64 F64=1', 'SGDT', ''];
T['SGDT MEM:w=$memppr REG:SUPP:r=GDTR', 'MODE=NO64    ', 'SGDT', ''];

# SHA1MSG1-Perform an Intermediate Calculation for the Next Four SHA1 Message Dwords.
T['SHA1MSG1 VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'NONE', 'SHA1MSG1_romo_1', ''];
T['SHA1MSG1 VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'NONE', 'SHA1MSG1_romo_2', ''];

# SHA1MSG2-Perform a Final Calculation for the Next Four SHA1 Message Dwords.
T['SHA1MSG2 VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'NONE', 'SHA1MSG2_romo_1', ''];
T['SHA1MSG2 VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'NONE', 'SHA1MSG2_romo_2', ''];

# SHA1NEXTE-Calculate SHA1 State Variable E after Four Rounds.
T['SHA1NEXTE VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'NONE', 'SHA1NEXTE_romo_1', ''];
T['SHA1NEXTE VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'NONE', 'SHA1NEXTE_romo_2', ''];

# SHA1RNDS4-Perform Four Rounds of SHA1 Operation.
T['SHA1RNDS4 VREG:rw:s32=$XMMr VREG:r:s32=$XMMm  IMM:u8=$uimm8', 'NONE', 'SHA1RNDS4_romoub_1', ''];
T['SHA1RNDS4 VREG:rw:s32=$XMMr MEM:r:s32=$mem128 IMM:u8=$uimm8', 'NONE', 'SHA1RNDS4_romoub_2', ''];

# SHA256MSG1-Perform an Intermediate Calculation for the Next Four SHA256 Message Dwords.
T['SHA256MSG1 VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'NONE', 'SHA256MSG1_romo_1', ''];
T['SHA256MSG1 VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'NONE', 'SHA256MSG1_romo_2', ''];

# SHA256MSG2-Perform a Final Calculation for the Next Four SHA256 Message Dwords.
T['SHA256MSG2 VREG:rw:s32=$XMMr VREG:r:s32=$XMMm ', 'NONE', 'SHA256MSG2_romo_1', ''];
T['SHA256MSG2 VREG:rw:s32=$XMMr MEM:r:s32=$mem128', 'NONE', 'SHA256MSG2_romo_2', ''];

# SHA256RNDS2-Perform Two Rounds of SHA256 Operation.
T['SHA256RNDS2 VREG:rw:s32=$XMMr VREG:r:s32=$XMMm  VREG:SUPP:r:u8=XMM0', 'NONE', 'SHA256RNDS2_romo_1', ''];
T['SHA256RNDS2 VREG:rw:s32=$XMMr MEM:r:s32=$mem128 VREG:SUPP:r:u8=XMM0', 'NONE', 'SHA256RNDS2_romo_2', ''];

# SAL/SAR/SHL/SHR-Shift.
T['SAR REG:rw=$GPR8m   IMM:u8=1       ', 'MOD=REG', 'SAR_mb    ', ''];
T['SAR MEM:rw:u8=$mem8 IMM:u8=1       ', 'MOD=MEM', 'SAR_mb    ', ''];
T['SAR REG:rw=$GPR8m   REG:r=CL       ', 'MOD=REG', 'SAR_mbb   ', ''];
T['SAR MEM:rw:u8=$mem8 REG:r=CL       ', 'MOD=MEM', 'SAR_mbb   ', ''];
T['SAR REG:rw=$GPR8m   SHIFT:r:u8=$shr', 'MOD=REG', 'SAR_mbub  ', ''];
T['SAR MEM:rw:u8=$mem8 SHIFT:r:u8=$shr', 'MOD=MEM', 'SAR_mbub  ', ''];
T['SAR REG:rw=$GPRvm   IMM:u8=1       ', 'MOD=REG', 'SAR_mv    ', ''];
T['SAR MEM:rw:sx=$memv IMM:u8=1       ', 'MOD=MEM', 'SAR_mv    ', ''];
T['SAR REG:rw=$GPRvm   REG:r=CL       ', 'MOD=REG', 'SAR_mvb   ', ''];
T['SAR MEM:rw:sx=$memv REG:r=CL       ', 'MOD=MEM', 'SAR_mvb   ', ''];
T['SAR REG:rw=$GPRvm   SHIFT:r:u8=$shr', 'MOD=REG', 'SAR_mvub  ', ''];
T['SAR MEM:rw:sx=$memv SHIFT:r:u8=$shr', 'MOD=MEM', 'SAR_mvub  ', ''];
T['SHL REG:rw=$GPR8m   IMM:u8=1       ', 'MOD=REG', 'SHL_mb_1  ', ''];
T['SHL MEM:rw:u8=$mem8 IMM:u8=1       ', 'MOD=MEM', 'SHL_mb_1  ', ''];
T['SHL REG:rw=$GPR8m   IMM:u8=1       ', 'MOD=REG', 'SHL_mb_2  ', ''];
T['SHL MEM:rw:u8=$mem8 IMM:u8=1       ', 'MOD=MEM', 'SHL_mb_2  ', ''];
T['SHL REG:rw=$GPR8m   REG:r=CL       ', 'MOD=REG', 'SHL_mbb_1 ', ''];
T['SHL MEM:rw:u8=$mem8 REG:r=CL       ', 'MOD=MEM', 'SHL_mbb_1 ', ''];
T['SHL REG:rw=$GPR8m   REG:r=CL       ', 'MOD=REG', 'SHL_mbb_2 ', ''];
T['SHL MEM:rw:u8=$mem8 REG:r=CL       ', 'MOD=MEM', 'SHL_mbb_2 ', ''];
T['SHL REG:rw=$GPR8m   SHIFT:r:u8=$shl', 'MOD=REG', 'SHL_mbub_1', ''];
T['SHL MEM:rw:u8=$mem8 SHIFT:r:u8=$shl', 'MOD=MEM', 'SHL_mbub_1', ''];
T['SHL REG:rw=$GPR8m   SHIFT:r:u8=$shl', 'MOD=REG', 'SHL_mbub_2', ''];
T['SHL MEM:rw:u8=$mem8 SHIFT:r:u8=$shl', 'MOD=MEM', 'SHL_mbub_2', ''];
T['SHL REG:rw=$GPRvm   IMM:u8=1       ', 'MOD=REG', 'SHL_mv_1  ', ''];
T['SHL MEM:rw:sx=$memv IMM:u8=1       ', 'MOD=MEM', 'SHL_mv_1  ', ''];
T['SHL REG:rw=$GPRvm   IMM:u8=1       ', 'MOD=REG', 'SHL_mv_2  ', ''];
T['SHL MEM:rw:sx=$memv IMM:u8=1       ', 'MOD=MEM', 'SHL_mv_2  ', ''];
T['SHL REG:rw=$GPRvm   REG:r=CL       ', 'MOD=REG', 'SHL_mvb_1 ', ''];
T['SHL MEM:rw:sx=$memv REG:r=CL       ', 'MOD=MEM', 'SHL_mvb_1 ', ''];
T['SHL REG:rw=$GPRvm   REG:r=CL       ', 'MOD=REG', 'SHL_mvb_2 ', ''];
T['SHL MEM:rw:sx=$memv REG:r=CL       ', 'MOD=MEM', 'SHL_mvb_2 ', ''];
T['SHL REG:rw=$GPRvm   SHIFT:r:u8=$shl', 'MOD=REG', 'SHL_mvub_1', ''];
T['SHL MEM:rw:sx=$memv SHIFT:r:u8=$shl', 'MOD=MEM', 'SHL_mvub_1', ''];
T['SHL REG:rw=$GPRvm   SHIFT:r:u8=$shl', 'MOD=REG', 'SHL_mvub_2', ''];
T['SHL MEM:rw:sx=$memv SHIFT:r:u8=$shl', 'MOD=MEM', 'SHL_mvub_2', ''];
T['SHR REG:rw=$GPR8m   IMM:u8=1       ', 'MOD=REG', 'SHR_mb    ', ''];
T['SHR MEM:rw:u8=$mem8 IMM:u8=1       ', 'MOD=MEM', 'SHR_mb    ', ''];
T['SHR REG:rw=$GPR8m   REG:r=CL       ', 'MOD=REG', 'SHR_mbb   ', ''];
T['SHR MEM:rw:u8=$mem8 REG:r=CL       ', 'MOD=MEM', 'SHR_mbb   ', ''];
T['SHR REG:rw=$GPR8m   SHIFT:r:u8=$shr', 'MOD=REG', 'SHR_mbub  ', ''];
T['SHR MEM:rw:u8=$mem8 SHIFT:r:u8=$shr', 'MOD=MEM', 'SHR_mbub  ', ''];
T['SHR REG:rw=$GPRvm   IMM:u8=1       ', 'MOD=REG', 'SHR_mv    ', ''];
T['SHR MEM:rw:sx=$memv IMM:u8=1       ', 'MOD=MEM', 'SHR_mv    ', ''];
T['SHR REG:rw=$GPRvm   REG:r=CL       ', 'MOD=REG', 'SHR_mvb   ', ''];
T['SHR MEM:rw:sx=$memv REG:r=CL       ', 'MOD=MEM', 'SHR_mvb   ', ''];
T['SHR REG:rw=$GPRvm   SHIFT:r:u8=$shr', 'MOD=REG', 'SHR_mvub  ', ''];
T['SHR MEM:rw:sx=$memv SHIFT:r:u8=$shr', 'MOD=MEM', 'SHR_mvub  ', ''];

# SARX/SHLX/SHRX-Shift Without Affecting Flags.
T['SHLX REG:w:s32=$GPR32r REG:r:s32=$GPR32m REG:r:s32=$GPR32n', 'MOD=REG MODE=64 W=0            ', 'SHLX', ''];
T['SHLX REG:w:s64=$GPR64r REG:r:s64=$GPR64m REG:r:s64=$GPR64n', 'MOD=REG MODE=64 W=1            ', 'SHLX', ''];
T['SHLX REG:w:s32=$GPR32r MEM:r:s32=$mem32  REG:r:s32=$GPR32n', 'MOD=MEM MODE=64 W=0            ', 'SHLX', ''];
T['SHLX REG:w:s64=$GPR64r MEM:r:s64=$mem64  REG:r:s64=$GPR64n', 'MOD=MEM MODE=64 W=1            ', 'SHLX', ''];
T['SHLX REG:w:s32=$GPR32r REG:r:s32=$GPR32m REG:r:s32=$GPR32n', 'MOD=REG MODE=NO64              ', 'SHLX', ''];
T['SHLX REG:w:s32=$GPR32r MEM:r:s32=$mem32  REG:r:s32=$GPR32n', 'MOD=MEM MODE=NO64              ', 'SHLX', ''];
T['SARX REG:w:s32=$GPR32r REG:r:s32=$GPR32m REG:r:s32=$GPR32n', 'MOD=REG MODE=64 PF2=0 PF3=1 W=0', 'SxRX', ''];
T['SARX REG:w:s64=$GPR64r REG:r:s64=$GPR64m REG:r:s64=$GPR64n', 'MOD=REG MODE=64 PF2=0 PF3=1 W=1', 'SxRX', ''];
T['SARX REG:w:s32=$GPR32r MEM:r:s32=$mem32  REG:r:s32=$GPR32n', 'MOD=MEM MODE=64 PF2=0 PF3=1 W=0', 'SxRX', ''];
T['SARX REG:w:s64=$GPR64r MEM:r:s64=$mem64  REG:r:s64=$GPR64n', 'MOD=MEM MODE=64 PF2=0 PF3=1 W=1', 'SxRX', ''];
T['SARX REG:w:s32=$GPR32r REG:r:s32=$GPR32m REG:r:s32=$GPR32n', 'MOD=REG MODE=NO64 PF2=0 PF3=1  ', 'SxRX', ''];
T['SARX REG:w:s32=$GPR32r MEM:r:s32=$mem32  REG:r:s32=$GPR32n', 'MOD=MEM MODE=NO64 PF2=0 PF3=1  ', 'SxRX', ''];
T['SHRX REG:w:s32=$GPR32r REG:r:s32=$GPR32m REG:r:s32=$GPR32n', 'MOD=REG MODE=64 PF2=1 PF3=0 W=0', 'SxRX', ''];
T['SHRX REG:w:s64=$GPR64r REG:r:s64=$GPR64m REG:r:s64=$GPR64n', 'MOD=REG MODE=64 PF2=1 PF3=0 W=1', 'SxRX', ''];
T['SHRX REG:w:s32=$GPR32r MEM:r:s32=$mem32  REG:r:s32=$GPR32n', 'MOD=MEM MODE=64 PF2=1 PF3=0 W=0', 'SxRX', ''];
T['SHRX REG:w:s64=$GPR64r MEM:r:s64=$mem64  REG:r:s64=$GPR64n', 'MOD=MEM MODE=64 PF2=1 PF3=0 W=1', 'SxRX', ''];
T['SHRX REG:w:s32=$GPR32r REG:r:s32=$GPR32m REG:r:s32=$GPR32n', 'MOD=REG MODE=NO64 PF2=1 PF3=0  ', 'SxRX', ''];
T['SHRX REG:w:s32=$GPR32r MEM:r:s32=$mem32  REG:r:s32=$GPR32n', 'MOD=MEM MODE=NO64 PF2=1 PF3=0  ', 'SxRX', ''];

# SHLD-Double Precision Shift Left.
T['SHLD REG:rcw=$GPRvm   REG:r=$GPRvr REG:r=CL       ', 'MOD=REG', 'SHLD_mvrvb ', ''];
T['SHLD MEM:rcw:sx=$memv REG:r=$GPRvr REG:r=CL       ', 'MOD=MEM', 'SHLD_mvrvb ', ''];
T['SHLD REG:rcw=$GPRvm   REG:r=$GPRvr SHIFT:r:u8=$shl', 'MOD=REG', 'SHLD_mvrvub', ''];
T['SHLD MEM:rcw:sx=$memv REG:r=$GPRvr SHIFT:r:u8=$shl', 'MOD=MEM', 'SHLD_mvrvub', ''];

# SHRD-Double Precision Shift Right.
T['SHRD REG:rcw=$GPRvm   REG:r=$GPRvr REG:r=CL       ', 'MOD=REG', 'SHRD_mvrvb ', ''];
T['SHRD MEM:rcw:sx=$memv REG:r=$GPRvr REG:r=CL       ', 'MOD=MEM', 'SHRD_mvrvb ', ''];
T['SHRD REG:rcw=$GPRvm   REG:r=$GPRvr SHIFT:r:u8=$shr', 'MOD=REG', 'SHRD_mvrvub', ''];
T['SHRD MEM:rcw:sx=$memv REG:r=$GPRvr SHIFT:r:u8=$shr', 'MOD=MEM', 'SHRD_mvrvub', ''];

# SIDT-Store Interrupt Descriptor Table Register.
T['SIDT MEM:w=$mem80  REG:SUPP:r=IDTR', 'MODE=64 F64=1', 'SIDT', ''];
T['SIDT MEM:w=$memppr REG:SUPP:r=IDTR', 'MODE=NO64    ', 'SIDT', ''];

# SLDT-Store Local Descriptor Table Register.
T['SLDT REG:w=$GPRvm     REG:SUPP:r=LDTR', 'MOD=REG', 'SLDT', ''];
T['SLDT MEM:w:s16=$mem16 REG:SUPP:r=LDTR', 'MOD=MEM', 'SLDT', ''];

# SMSW-Store Machine Status Word.
T['SMSW REG:w=$GPRvm     REG:SUPP:r=CR0', 'MOD=REG', 'SMSW', ''];
T['SMSW MEM:w:s16=$mem16 REG:SUPP:r=CR0', 'MOD=MEM', 'SMSW', ''];

# STC-Set Carry Flag.
T['STC', 'NONE', 'STC', ''];

# STD-Set Direction Flag.
T['STD', 'NONE', 'STD', ''];

# STI-Set Interrupt Flag.
T['STI', 'NONE', 'STI', ''];

# STOS/STOSB/STOSW/STOSD/STOSQ-Store String.
T['STOSB MEM:SUPP:w:u8={SEG:r=ES BASE:r=$DIa SZ=8}    REG:SUPP:r=AL                   ', 'PF2=0 PF3=0      ', 'STOSB', 'rep=1 repnz=1'];
T['STOSB MEM:SUPP:cw:u8={SEG:r=ES BASE:r=$DIa SZ=8}   REG:SUPP:r=AL  REG:SUPP:rcw=$CXa', 'P66=0 PF2=0 PF3=1', 'STOSB', 'rep=1 repnz=1'];
T['STOSB MEM:SUPP:cw:u8={SEG:r=ES BASE:r=$DIa SZ=8}   REG:SUPP:r=AL  REG:SUPP:rcw=$CXa', 'P66=0 PF2=1 PF3=0', 'STOSB', 'rep=1 repnz=1'];
T['STOSD MEM:SUPP:w:s32={SEG:r=ES BASE:r=$DIa SZ=32}  REG:SUPP:r=EAX                  ', 'PF2=0 PF3=0      ', 'STOSD', 'rep=1 repnz=1'];
T['STOSD MEM:SUPP:cw:s32={SEG:r=ES BASE:r=$DIa SZ=32} REG:SUPP:r=EAX REG:SUPP:rcw=$CXa', 'P66=0 PF2=0 PF3=1', 'STOSD', 'rep=1 repnz=1'];
T['STOSD MEM:SUPP:cw:s32={SEG:r=ES BASE:r=$DIa SZ=32} REG:SUPP:r=EAX REG:SUPP:rcw=$CXa', 'P66=0 PF2=1 PF3=0', 'STOSD', 'rep=1 repnz=1'];
T['STOSQ MEM:SUPP:w:s64={SEG:r=ES BASE:r=$DIa SZ=64}  REG:SUPP:r=RAX                  ', 'PF2=0 PF3=0      ', 'STOSQ', 'rep=1 repnz=1'];
T['STOSQ MEM:SUPP:cw:s64={SEG:r=ES BASE:r=$DIa SZ=64} REG:SUPP:r=RAX REG:SUPP:rcw=$CXa', 'P66=0 PF2=0 PF3=1', 'STOSQ', 'rep=1 repnz=1'];
T['STOSQ MEM:SUPP:cw:s64={SEG:r=ES BASE:r=$DIa SZ=64} REG:SUPP:r=RAX REG:SUPP:rcw=$CXa', 'P66=0 PF2=1 PF3=0', 'STOSQ', 'rep=1 repnz=1'];
T['STOSW MEM:SUPP:w:s16={SEG:r=ES BASE:r=$DIa SZ=16}  REG:SUPP:r=AX                   ', 'PF2=0 PF3=0      ', 'STOSW', 'rep=1 repnz=1'];
T['STOSW MEM:SUPP:cw:s16={SEG:r=ES BASE:r=$DIa SZ=16} REG:SUPP:r=AX  REG:SUPP:rcw=$CXa', 'P66=0 PF2=0 PF3=1', 'STOSW', 'rep=1 repnz=1'];
T['STOSW MEM:SUPP:cw:s16={SEG:r=ES BASE:r=$DIa SZ=16} REG:SUPP:r=AX  REG:SUPP:rcw=$CXa', 'P66=0 PF2=1 PF3=0', 'STOSW', 'rep=1 repnz=1'];

# STR-Store Task Register.
T['STR REG:w=$GPRvm     REG:SUPP:r=TR', 'MOD=REG', 'STR', ''];
T['STR MEM:w:s16=$mem16 REG:SUPP:r=TR', 'MOD=MEM', 'STR', ''];

# SUB-Subtract.
T['SUB REG:rw=AL       IMM:s8=$simm8 ', 'NONE   ', 'SUB_bsb   ', '      '];
T['SUB REG:rw=$GPR8m   REG:r=$GPR8r  ', 'MOD=REG', 'SUB_mbrb  ', '      '];
T['SUB MEM:rw:u8=$mem8 REG:r=$GPR8r  ', 'MOD=MEM', 'SUB_mbrb  ', '$xlock'];
T['SUB REG:rw=$GPR8m   IMM:s8=$simm8 ', 'MOD=REG', 'SUB_mbsb_1', '      '];
T['SUB MEM:rw:u8=$mem8 IMM:s8=$simm8 ', 'MOD=MEM', 'SUB_mbsb_1', '$xlock'];
T['SUB REG:rw=$GPR8m   IMM:s8=$simm8 ', 'MOD=REG', 'SUB_mbsb_2', '      '];
T['SUB MEM:rw:u8=$mem8 IMM:s8=$simm8 ', 'MOD=MEM', 'SUB_mbsb_2', '$xlock'];
T['SUB REG:rw=$GPRvm   REG:r=$GPRvr  ', 'MOD=REG', 'SUB_mvrv  ', '      '];
T['SUB MEM:rw:sx=$memv REG:r=$GPRvr  ', 'MOD=MEM', 'SUB_mvrv  ', '$xlock'];
T['SUB REG:rw=$GPRvm   IMM:s8=$simm8 ', 'MOD=REG', 'SUB_mvsb  ', '      '];
T['SUB MEM:rw:sx=$memv IMM:s8=$simm8 ', 'MOD=MEM', 'SUB_mvsb  ', '$xlock'];
T['SUB REG:rw=$GPRvm   IMM:sx=$simmz ', 'MOD=REG', 'SUB_mvsz  ', '      '];
T['SUB MEM:rw:sx=$memv IMM:sx=$simmz ', 'MOD=MEM', 'SUB_mvsz  ', '$xlock'];
T['SUB REG:rw=$GPR8r   REG:r=$GPR8m  ', 'MOD=REG', 'SUB_rbmb  ', '      '];
T['SUB REG:rw=$GPR8r   MEM:r:u8=$mem8', 'MOD=MEM', 'SUB_rbmb  ', '      '];
T['SUB REG:rw=$GPRvr   REG:r=$GPRvm  ', 'MOD=REG', 'SUB_rvmv  ', '      '];
T['SUB REG:rw=$GPRvr   MEM:r:sx=$memv', 'MOD=MEM', 'SUB_rvmv  ', '      '];
T['SUB REG:rw=$AXv     IMM:sx=$simmz ', 'NONE   ', 'SUB_vsz   ', '      '];

# SWAPGS-Swap GS Base Register.
T['SWAPGS', 'NONE', 'SWAPGS', ''];

# SYSCALL-Fast System Call.
T['SYSCALL REG:SUPP:w=RIP REG:SUPP:w=RCX REG:SUPP:w=R11 REG:SUPP:w=CS REG:SUPP:w=SS', 'F64=1', 'SYSCALL_1', ''];

# SYSENTER-Fast System Call.
T['SYSENTER REG:SUPP:w=RIP REG:SUPP:w=RSP', 'MODE=64  ', 'SYSENTER', ''];
T['SYSENTER REG:SUPP:w=EIP REG:SUPP:w=ESP', 'MODE=NO64', 'SYSENTER', ''];

# SYSEXIT-Fast Return from Fast System Call.
T['SYSEXIT REG:SUPP:w=RIP REG:SUPP:w=RSP REG:SUPP:r=RCX REG:SUPP:r=RDX', 'MODE=64  ', 'SYSEXIT', ''];
T['SYSEXIT REG:SUPP:w=EIP REG:SUPP:w=ESP REG:SUPP:r=ECX REG:SUPP:r=EDX', 'MODE=NO64', 'SYSEXIT', ''];

# SYSRET-Return From Fast System Call.
T['SYSRET REG:SUPP:w=RIP REG:SUPP:r=RCX REG:SUPP:r=R11 REG:SUPP:w=CS REG:SUPP:w=SS', 'OSZ=64', 'SYSRET_1', ''];
T['SYSRET REG:SUPP:w=EIP REG:SUPP:r=ECX REG:SUPP:r=R11 REG:SUPP:w=CS REG:SUPP:w=SS', 'OSZ=Z ', 'SYSRET_1', ''];

# TEST-Logical Compare.
T['TEST REG:r=AL       IMM:s8=$simm8', 'NONE   ', 'TEST_bsb   ', ''];
T['TEST REG:r=$GPR8m   REG:r=$GPR8r ', 'MOD=REG', 'TEST_mbrb  ', ''];
T['TEST MEM:r:u8=$mem8 REG:r=$GPR8r ', 'MOD=MEM', 'TEST_mbrb  ', ''];
T['TEST REG:r=$GPR8m   IMM:s8=$simm8', 'MOD=REG', 'TEST_mbsb_1', ''];
T['TEST MEM:r:u8=$mem8 IMM:s8=$simm8', 'MOD=MEM', 'TEST_mbsb_1', ''];
T['TEST REG:r=$GPR8m   IMM:s8=$simm8', 'MOD=REG', 'TEST_mbsb_2', ''];
T['TEST MEM:r:u8=$mem8 IMM:s8=$simm8', 'MOD=MEM', 'TEST_mbsb_2', ''];
T['TEST REG:r=$GPRvm   REG:r=$GPRvr ', 'MOD=REG', 'TEST_mvrv  ', ''];
T['TEST MEM:r:sx=$memv REG:r=$GPRvr ', 'MOD=MEM', 'TEST_mvrv  ', ''];
T['TEST REG:r=$GPRvm   IMM:sx=$simmz', 'MOD=REG', 'TEST_mvsz_1', ''];
T['TEST MEM:r:sx=$memv IMM:sx=$simmz', 'MOD=MEM', 'TEST_mvsz_1', ''];
T['TEST REG:r=$GPRvm   IMM:sx=$simmz', 'MOD=REG', 'TEST_mvsz_2', ''];
T['TEST MEM:r:sx=$memv IMM:sx=$simmz', 'MOD=MEM', 'TEST_mvsz_2', ''];
T['TEST REG:r=$AXv     IMM:sx=$simmz', 'NONE   ', 'TEST_vsz   ', ''];

# TZCNT-Count the Number of Trailing Zero Bits.
T['TZCNT REG:w=$GPRvr REG:r=$GPRvm  ', 'MOD=REG', 'TZCNT_rvmv', ''];
T['TZCNT REG:w=$GPRvr MEM:r:sx=$memv', 'MOD=MEM', 'TZCNT_rvmv', ''];

# UD-Undefined Instruction.
T['UD0 REG:r=$GPR32r REG:r=$GPR32m   ', 'MOD=REG', 'UD0', ''];
T['UD0 REG:r=$GPR32r MEM:r:s32=$mem32', 'MOD=MEM', 'UD0', ''];
T['UD1 REG:r=$GPR32r REG:r=$GPR32m   ', 'MOD=REG', 'UD1', ''];
T['UD1 REG:r=$GPR32r MEM:r:s32=$mem32', 'MOD=MEM', 'UD1', ''];
T['UD2                               ', 'NONE   ', 'UD2', ''];

# VCVTPH2PS-Convert 16-bit FP values to Single-Precision FP values.
T['VCVTPH2PS VREG:w:f32=$XMMr VREG:r:f16=$XMMm ', 'MOD=REG VL=128', 'VCVTPH2PS_rxmx', ''];
T['VCVTPH2PS VREG:w:f32=$YMMr VREG:r:f16=$XMMm ', 'MOD=REG VL=256', 'VCVTPH2PS_rxmx', ''];
T['VCVTPH2PS VREG:w:f32=$XMMr MEM:r:f16=$mem64 ', 'MOD=MEM VL=128', 'VCVTPH2PS_rxmx', ''];
T['VCVTPH2PS VREG:w:f32=$YMMr MEM:r:f16=$mem128', 'MOD=MEM VL=256', 'VCVTPH2PS_rxmx', ''];

# VCVTPS2PH-Convert Single-Precision FP value to 16-bit FP value.
T['VCVTPS2PH VREG:w:f16=$XMMm  VREG:r:f32=$XMMr RNDC:r:u8=$rndc3', 'MOD=REG VL=128', 'VCVTPS2PH_mxrxub', ''];
T['VCVTPS2PH VREG:w:f16=$XMMm  VREG:r:f32=$YMMr RNDC:r:u8=$rndc3', 'MOD=REG VL=256', 'VCVTPS2PH_mxrxub', ''];
T['VCVTPS2PH MEM:w:f16=$mem64  VREG:r:f32=$XMMr RNDC:r:u8=$rndc3', 'MOD=MEM VL=128', 'VCVTPS2PH_mxrxub', ''];
T['VCVTPS2PH MEM:w:f16=$mem128 VREG:r:f32=$YMMr RNDC:r:u8=$rndc3', 'MOD=MEM VL=256', 'VCVTPS2PH_mxrxub', ''];

# VERR/VERW-Verify a Segment for Reading or Writing.
T['VERR REG:r=$GPR16m   ', 'MOD=REG', 'VERR', ''];
T['VERR MEM:r:s16=$mem16', 'MOD=MEM', 'VERR', ''];
T['VERW REG:r=$GPR16m   ', 'MOD=REG', 'VERW', ''];
T['VERW MEM:r:s16=$mem16', 'MOD=MEM', 'VERW', ''];

# WBINVD-Write Back and Invalidate Cache.
T['WBINVD', 'NONE', 'WBINVD', ''];

# WRMSR-Write to Model Specific Register.
T['WRMSR REG:SUPP:r=EAX REG:SUPP:r=EDX REG:SUPP:r=ECX', 'NONE', 'WRMSR', ''];

# XADD-Exchange and Add.
T['XADD REG:rw=$GPR8m   REG:rw=$GPR8r', 'MOD=REG', 'XADD_mbrb', '      '];
T['XADD MEM:rw:u8=$mem8 REG:rw=$GPR8r', 'MOD=MEM', 'XADD_mbrb', '$xlock'];
T['XADD REG:rw=$GPRvm   REG:rw=$GPRvr', 'MOD=REG', 'XADD_mvrv', '      '];
T['XADD MEM:rw:sx=$memv REG:rw=$GPRvr', 'MOD=MEM', 'XADD_mvrv', '$xlock'];

# XCHG-Exchange Register/Memory with Register.
T['XCHG REG:rw=$GPR8m   REG:rw=$GPR8r', 'MOD=REG', 'XCHG_mbrb ', '            '];
T['XCHG MEM:rw:u8=$mem8 REG:rw=$GPR8r', 'MOD=MEM', 'XCHG_mbrb ', 'hle=1 $xlock'];
T['XCHG REG:rw=$GPRvm   REG:rw=$GPRvr', 'MOD=REG', 'XCHG_mvrv ', '            '];
T['XCHG MEM:rw:sx=$memv REG:rw=$GPRvr', 'MOD=MEM', 'XCHG_mvrv ', 'hle=1 $xlock'];
T['XCHG REG:rw=$GPRvo   REG:rw=$AXv  ', 'NONE   ', 'XCHG_ovv_1', '            '];
T['XCHG REG:rw=$GPRvo   REG:rw=$AXv  ', 'NONE   ', 'XCHG_ovv_2', '            '];
T['XCHG REG:rw=$GPRvo   REG:rw=$AXv  ', 'NONE   ', 'XCHG_ovv_3', '            '];
T['XCHG REG:rw=$GPRvo   REG:rw=$AXv  ', 'NONE   ', 'XCHG_ovv_4', '            '];
T['XCHG REG:rw=$GPRvo   REG:rw=$AXv  ', 'NONE   ', 'XCHG_ovv_5', '            '];
T['XCHG REG:rw=$GPRvo   REG:rw=$AXv  ', 'NONE   ', 'XCHG_ovv_6', '            '];
T['XCHG REG:rw=$GPRvo   REG:rw=$AXv  ', 'NONE   ', 'XCHG_ovv_7', '            '];
T['XCHG REG:rw=$GPRvo   REG:rw=$AXv  ', 'NONE   ', 'XCHG_ovv_8', '            '];

# XLAT/XLATB-Table Look-up Translation.
T['XLAT MEM:SUPP:r:u8={SEG:r=DS BASE:r=$BXa SZ=8} REG:SUPP:w=AL', 'NONE', 'XLAT', ''];

# XOR-Logical Exclusive OR.
T['XOR REG:rw=AL       IMM:u8=$uimm8 ', 'NONE   ', 'XOR_bub   ', '      '];
T['XOR REG:rw=$GPR8m   REG:r=$GPR8r  ', 'MOD=REG', 'XOR_mbrb  ', '      '];
T['XOR MEM:rw:u8=$mem8 REG:r=$GPR8r  ', 'MOD=MEM', 'XOR_mbrb  ', '$xlock'];
T['XOR REG:rw=$GPR8m   IMM:u8=$uimm8 ', 'MOD=REG', 'XOR_mbub_1', '      '];
T['XOR MEM:rw:u8=$mem8 IMM:u8=$uimm8 ', 'MOD=MEM', 'XOR_mbub_1', '$xlock'];
T['XOR REG:rw=$GPR8m   IMM:u8=$uimm8 ', 'MOD=REG', 'XOR_mbub_2', '      '];
T['XOR MEM:rw:u8=$mem8 IMM:u8=$uimm8 ', 'MOD=MEM', 'XOR_mbub_2', '$xlock'];
T['XOR REG:rw=$GPRvm   REG:r=$GPRvr  ', 'MOD=REG', 'XOR_mvrv  ', '      '];
T['XOR MEM:rw:sx=$memv REG:r=$GPRvr  ', 'MOD=MEM', 'XOR_mvrv  ', '$xlock'];
T['XOR REG:rw=$GPRvm   IMM:s8=$simm8 ', 'MOD=REG', 'XOR_mvsb  ', '      '];
T['XOR MEM:rw:sx=$memv IMM:s8=$simm8 ', 'MOD=MEM', 'XOR_mvsb  ', '$xlock'];
T['XOR REG:rw=$GPRvm   IMM:sx=$simmz ', 'MOD=REG', 'XOR_mvsz  ', '      '];
T['XOR MEM:rw:sx=$memv IMM:sx=$simmz ', 'MOD=MEM', 'XOR_mvsz  ', '$xlock'];
T['XOR REG:rw=$GPR8r   REG:r=$GPR8m  ', 'MOD=REG', 'XOR_rbmb  ', '      '];
T['XOR REG:rw=$GPR8r   MEM:r:u8=$mem8', 'MOD=MEM', 'XOR_rbmb  ', '      '];
T['XOR REG:rw=$GPRvr   REG:r=$GPRvm  ', 'MOD=REG', 'XOR_rvmv  ', '      '];
T['XOR REG:rw=$GPRvr   MEM:r:sx=$memv', 'MOD=MEM', 'XOR_rvmv  ', '      '];
T['XOR REG:rw=$AXv     IMM:sx=$simmz ', 'NONE   ', 'XOR_vsz   ', '      '];
