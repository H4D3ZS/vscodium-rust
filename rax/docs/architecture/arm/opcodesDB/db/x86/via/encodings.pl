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
ENCODING MONTMUL => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xa6 P66=0 PF2=0 PF3=1 REG=0 RM=0',
  extensions => 'PADLOCK',
  categories => 'SYSTEM',
  metadata   => 'isa=PADLOCK_PMM',
  tags       => 'page=MONTMUL',
};


# XCRYPT_CBC.
ENCODING XCRYPT_CBC => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xa7 P66=0 PF2=0 PF3=1 REG=2 RM=0',
  extensions => 'PADLOCK',
  categories => 'SYSTEM',
  metadata   => 'isa=PADLOCK_ACE',
  tags       => 'page=XCRYPT_CBC',
};


# XCRYPT_CFB.
ENCODING XCRYPT_CFB => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xa7 P66=0 PF2=0 PF3=1 REG=4 RM=0',
  extensions => 'PADLOCK',
  categories => 'SYSTEM',
  metadata   => 'isa=PADLOCK_ACE',
  tags       => 'page=XCRYPT_CFB',
};


# XCRYPT_CTR.
ENCODING XCRYPT_CTR => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xa7 P66=0 PF2=0 PF3=1 REG=3 RM=0',
  extensions => 'PADLOCK',
  categories => 'SYSTEM',
  metadata   => 'isa=PADLOCK_ACE',
  tags       => 'page=XCRYPT_CTR',
};


# XCRYPT_ECB.
ENCODING XCRYPT_ECB => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xa7 P66=0 PF2=0 PF3=1 REG=1 RM=0',
  extensions => 'PADLOCK',
  categories => 'SYSTEM',
  metadata   => 'isa=PADLOCK_ACE',
  tags       => 'page=XCRYPT_ECB',
};


# XCRYPT_OFB.
ENCODING XCRYPT_OFB => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xa7 P66=0 PF2=0 PF3=1 REG=5 RM=0',
  extensions => 'PADLOCK',
  categories => 'SYSTEM',
  metadata   => 'isa=PADLOCK_ACE',
  tags       => 'page=XCRYPT_OFB',
};


# XSHA1.
ENCODING XSHA1 => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xa6 P66=0 PF2=0 PF3=1 REG=1 RM=0',
  extensions => 'PADLOCK',
  categories => 'SYSTEM',
  metadata   => 'isa=PADLOCK_PHE',
  tags       => 'page=XSHA1',
};


# XSHA256.
ENCODING XSHA256 => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xa6 P66=0 PF2=0 PF3=1 REG=2 RM=0',
  extensions => 'PADLOCK',
  categories => 'SYSTEM',
  metadata   => 'isa=PADLOCK_PHE',
  tags       => 'page=XSHA256',
};


# XSTORE.
ENCODING XSTORE => {
  diagram    => 'MAP=0f MOD=REG MR=1 OP=0xa7 P66=0 PF2=0 REG=0 RM=0',
  extensions => 'PADLOCK',
  categories => 'SYSTEM',
  metadata   => 'isa=PADLOCK_RNG',
  tags       => 'page=XSTORE',
};

