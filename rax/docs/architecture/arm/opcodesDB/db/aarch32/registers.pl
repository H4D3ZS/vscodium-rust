use strict;
use warnings;

my @gpr32   = ('R0' .. 'R9', qw(R10 R11 R12 SP LR PC));
my @simd32  = ('S0' .. 'S9', 'S10' .. 'S31');
my @simd64  = ('D0' .. 'D9', 'D10' .. 'D31');
my @simd128 = ('Q0' .. 'Q9', 'Q10' .. 'Q31');

# --- system register ---
my @bnkdregs = qw(R8_usr R9_usr R10_usr R11_usr R12_usr SP_usr LR_usr 
                  R8_fiq R9_fiq R10_fiq R11_fiq R12_fiq SP_fiq LR_fiq 
				  LR_irq SP_irq LR_svc SP_svc LR_abt SP_abt LR_und SP_und LR_mon SP_mon
				  ELR_hyp SP_hyp SPSR_fiq SPSR_irq SPSR_svc SPSR_abt 
				  SPSR_und SPSR_mon SPSR_hyp);
				  
my @specregs = qw/CPSR APSR SPSR FPSID FPSCR MVFR2 MVFR1 MVFR0 FPEXC/;

my @systemregs = (qw(NONE), @bnkdregs, @specregs);


REGISTERS NONE    => ( [qw/NONE/]   ,  0  ,  'NONE', );
REGISTERS GPR32   => ( \@gpr32,     ,  32 ,  'I32' , );
REGISTERS SIMD32  => ( \@simd32,    ,  32 ,  'F32' , );
REGISTERS SIMD64  => ( \@simd64,    ,  64 ,  'F64' , );
REGISTERS SIMD128 => ( \@simd128,   ,  128,  'NONE', );
REGISTERS SYSREG  => (\@systemregs, ,  0  ,  'NONE', );

1;