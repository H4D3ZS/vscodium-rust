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


# MONTMUL.
T['MONTMUL REG:SUPP:rw=$AXa REG:SUPP:rw=$CXa REG:SUPP:w=EDX REG:SUPP:r=$SIa MEM:SUPP:r={SEG:r=ES BASE:r=$SIa SZ=SZ_V}', 'NONE', 'MONTMUL', 'rep=1'];

# XCRYPT_CBC.
T['XCRYPT_CBC REG:SUPP:rcw=$AXa REG:SUPP:rcw=$CXa REG:SUPP:rcw=$SIa MEM:SUPP:rcw:sx={SEG:r=ES BASE:r=$AXa SZ=SZ_V} MEM:SUPP:cr={SEG:r=ES BASE:r=$DXa SZ=SZ_V} MEM:SUPP:cr:sx={SEG:r=ES BASE:r=$BXa SZ=SZ_V} MEM:SUPP:cr:sx={SEG:r=ES BASE:r=$SIa SZ=SZ_V} MEM:SUPP:cw:sx={SEG:r=ES BASE:r=$DIa SZ=SZ_V}', 'NONE', 'XCRYPT_CBC', 'rep=1'];

# XCRYPT_CFB.
T['XCRYPT_CFB REG:SUPP:rcw=$AXa REG:SUPP:rcw=$CXa REG:SUPP:rcw=$SIa MEM:SUPP:rcw:sx={SEG:r=ES BASE:r=$AXa SZ=SZ_V} MEM:SUPP:cr={SEG:r=ES BASE:r=$DXa SZ=SZ_V} MEM:SUPP:cr:sx={SEG:r=ES BASE:r=$BXa SZ=SZ_V} MEM:SUPP:cr:sx={SEG:r=ES BASE:r=$SIa SZ=SZ_V} MEM:SUPP:cw:sx={SEG:r=ES BASE:r=$DIa SZ=SZ_V}', 'NONE', 'XCRYPT_CFB', 'rep=1'];

# XCRYPT_CTR.
T['XCRYPT_CTR REG:SUPP:rcw=$AXa REG:SUPP:rcw=$CXa REG:SUPP:rcw=$SIa REG:SUPP:rcw=$DIa MEM:SUPP:rcw:sx={SEG:r=ES BASE:r=$AXa SZ=SZ_V} MEM:SUPP:cr={SEG:r=ES BASE:r=$DXa SZ=SZ_V} MEM:SUPP:cr:sx={SEG:r=ES BASE:r=$BXa SZ=SZ_V} MEM:SUPP:cr:sx={SEG:r=ES BASE:r=$SIa SZ=SZ_V} MEM:SUPP:cw:sx={SEG:r=ES BASE:r=$DIa SZ=SZ_V}', 'NONE', 'XCRYPT_CTR', 'rep=1'];

# XCRYPT_ECB.
T['XCRYPT_ECB REG:SUPP:rcw=$CXa REG:SUPP:rcw=$SIa REG:SUPP:rcw=$DIa MEM:SUPP:cr={SEG:r=ES BASE:r=$DXa SZ=SZ_V} MEM:SUPP:cr:sx={SEG:r=ES BASE:r=$BXa SZ=SZ_V} MEM:SUPP:cr:sx={SEG:r=ES BASE:r=$SIa SZ=SZ_V} MEM:SUPP:cw:sx={SEG:r=ES BASE:r=$DIa SZ=SZ_V}', 'NONE', 'XCRYPT_ECB', 'rep=1'];

# XCRYPT_OFB.
T['XCRYPT_OFB REG:SUPP:rcw=$AXa REG:SUPP:rcw=$CXa REG:SUPP:rcw=$SIa REG:SUPP:rcw=$DIa MEM:SUPP:rcw:sx={SEG:r=ES BASE:r=$AXa SZ=SZ_V} MEM:SUPP:cr={SEG:r=ES BASE:r=$DXa SZ=SZ_V} MEM:SUPP:cr:sx={SEG:r=ES BASE:r=$BXa SZ=SZ_V} MEM:SUPP:cr:sx={SEG:r=ES BASE:r=$SIa SZ=SZ_V} MEM:SUPP:cw:sx={SEG:r=ES BASE:r=$DIa SZ=SZ_V}', 'NONE', 'XCRYPT_OFB', 'rep=1'];

# XSHA1.
T['XSHA1 REG:SUPP:rcw=$AXa REG:SUPP:r=$CXa REG:SUPP:rcw=$SIa REG:SUPP:cr=$DIa MEM:SUPP:cr:sx={SEG:r=ES BASE:r=$SIa SZ=SZ_V} MEM:SUPP:cw:sx={SEG:r=ES BASE:r=$DIa SZ=SZ_V}', 'NONE', 'XSHA1', 'rep=1'];

# XSHA256.
T['XSHA256 REG:SUPP:rcw=$AXa REG:SUPP:r=$CXa REG:SUPP:rcw=$SIa REG:SUPP:cr=$DIa MEM:SUPP:cr:sx={SEG:r=ES BASE:r=$SIa SZ=SZ_V} MEM:SUPP:cw:sx={SEG:r=ES BASE:r=$DIa SZ=SZ_V}', 'NONE', 'XSHA256', 'rep=1'];

# XSTORE.
T['XSTORE REG:SUPP:w=EAX  REG:SUPP:r=EDX    REG:SUPP:rw=$DIa MEM:SUPP:w:s8={SEG:r=ES BASE:r=$DIa SZ=8}                                           ', 'PF3=0', 'XSTORE', '     '];
T['XSTORE REG:SUPP:cw=EAX REG:SUPP:rcw=$CXa REG:SUPP:rcw=EDX REG:SUPP:rcw=$DIa                         MEM:SUPP:cw:s8={SEG:r=ES BASE:r=$DIa SZ=8}', 'PF3=1', 'XSTORE', 'rep=1'];
