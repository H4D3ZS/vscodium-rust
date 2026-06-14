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

PAGE invalid => {
  name       => 'INVALID',
  title      => 'invalid',
  brief      => 'invalid',
  docvars    => 'armarmheading=INVALID instr-class=NONE isa=A32 mnemonic=INVALID',
  metadata   => 'isa=ARM isaform=A32 alias=0 invalid=1 alphaindex=invalid',
};


GROUP invalid => {
  name       => 'INVALID',
  title      => 'invalid',
  diagram    => 'ig0=0bxxxxxxxxxxxxxxxx ig2=0bxxxxxxxxxxxxxxxx',
  docvars    => 'armarmheading=INVALID instr-class=NONE isa=A32',
  metadata   => 'isa=ARM isaform=A32 alias=0 invalid=1 alphaindex=invalid',
  tags       => 'group=invalid',
};


ICLASS invalid => {
  name       => 'INVALID',
  title      => 'invalid',
  diagram    => 'ig0=0bxxxxxxxxxxxxxxxx ig2=0bxxxxxxxxxxxxxxxx',
  docvars    => 'armarmheading=INVALID instr-class=NONE isa=A32',
  metadata   => 'isa=ARM isaform=A32 alias=0 invalid=1 alphaindex=invalid',
  tags       => 'group=invalid iclass=invalid page=invalid cclass=invalid',
};

CLASS invalid => {
  name       => 'INVALID',
  diagram    => 'ig0=0bxxxxxxxxxxxxxxxx ig2=0bxxxxxxxxxxxxxxxx',
  docvars    => 'armarmheading=INVALID instr-class=NONE isa=A32 mnemonic=INVALID',
  metadata   => 'isa=ARM isaform=A32 alias=0 invalid=1 alphaindex=invalid',
  tags       => 'group=invalid iclass=invalid page=invalid cclass=invalid',
};

ENCODING invalid => {
  name       => 'INVALID',
  diagram    => 'ig0=0bxxxxxxxxxxxxxxxx ig2=0bxxxxxxxxxxxxxxxx',
  docvars    => 'armarmheading=INVALID instr-class=NONE isa=A32 mnemonic=INVALID',
  metadata   => 'isa=ARM isaform=A32 alias=0 invalid=1 alphaindex=invalid',
  tags       => 'group=invalid iclass=invalid page=invalid cclass=invalid',
};

T['INVALID ', 'ig0=0bxxxxxxxxxxxxxxxx', 'invalid', ''];

1;