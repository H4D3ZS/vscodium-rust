use strict;
use warnings;

# --- registers ---

my @gpr32   = ('W0' .. 'W9', 'W10' .. 'W30', qw(WZR WSP));
my @gpr64   = ('X0' .. 'X9', 'X10' .. 'X30', qw(XZR SP PC));

my @simd8    = ('B0' .. 'B9', 'B10' .. 'B31');
my @simd16   = ('H0' .. 'H9', 'H10' .. 'H31');
my @simd32   = ('S0' .. 'S9', 'S10' .. 'S31');
my @simd64   = ('D0' .. 'D9', 'D10' .. 'D31');
my @simd128  = ('Q0' .. 'Q9', 'Q10' .. 'Q31');

my @vecreg   = ('V0' .. 'V9', 'V10' .. 'V31');
my @svereg   = ('Z0' .. 'Z9', 'Z10' .. 'Z31');
my @prdreg   = ('P0' .. 'P9', 'P10' .. 'P15');

REGISTERS  NONE    => ( [qw/NONE/],  0,   'NONE', );
REGISTERS  GPR32   => ( \@gpr32,     32,  'I32',  );
REGISTERS  GPR64   => ( \@gpr64,     64,  'I64',  );
REGISTERS  SIMD8   => ( \@simd8,     8,   'I8',   );
REGISTERS  SIMD16  => ( \@simd16,    16,  'F16',  );
REGISTERS  SIMD32  => ( \@simd32,    32,  'F32',  );
REGISTERS  SIMD64  => ( \@simd64,    64,  'F64',  );
REGISTERS  SIMD128 => ( \@simd128,   128, 'NONE', );
REGISTERS  VECREG  => ( \@vecreg,    128, 'NONE', );
REGISTERS  PRDREG  => ( \@prdreg,    0,   'NONE', );
REGISTERS  SVEREG  => ( \@svereg,    0,   'NONE', );

1;