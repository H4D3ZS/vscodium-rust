use strict;
use warnings;
use Data::Dump qw/pp/;
use feature qw/say/;

use strict;
use warnings;

my @gpr8     = qw/AL CL DL BL AH CH DH BH SPL BPL SIL DIL R8B R9B R10B R11B R12B R13B R14B R15B/;
my @gpr16    = qw/AX CX DX BX SP BP SI DI R8W R9W R10W R11W R12W R13W R14W R15W/;
my @gpr32    = qw/EAX ECX EDX EBX ESP EBP ESI EDI R8D R9D R10D R11D R12D R13D R14D R15D/;
my @gpr64    = qw/RAX RCX RDX RBX RSP RBP RSI RDI R8 R9 R10 R11 R12 R13 R14 R15/;
my @fpreg    = qw/ST0 ST1 ST2 ST3 ST4 ST5 ST6 ST7/;
my @mmxreg   = qw/MM0 MM1 MM2 MM3 MM4 MM5 MM6 MM7/;
my @segreg   = qw/ES CS SS DS FS GS/;
my @xmmreg   = ( 'XMM0' .. 'XMM9', 'XMM10' .. 'XMM31' );
my @ymmreg   = ( 'YMM0' .. 'YMM9', 'YMM10' .. 'YMM31' );
my @zmmreg   = ( 'ZMM0' .. 'ZMM9', 'ZMM10' .. 'ZMM31' );
my @maskreg  = ( 'K0' .. 'K7' );
my @testreg  = ( 'TR0' .. 'TR7' );
my @cntrlreg = ( 'CR0' .. 'CR9', 'CR10' .. 'CR15' );
my @dbgreg   = ( 'DR0' .. 'DR9', 'DR10' .. 'DR15' );
my @tblreg   = qw/GDTR LDTR IDTR TR/;
my @bndreg   = qw/BND0 BND1 BND2 BND3/;

# for some architecture such aarch32, IP register(AKA PC) is concidered as a general-purpose-register,
# however, on x86 there is a protection layer arround the register. so here, we concider it as a system-register.
my @sysreg16 = qw/FLAGS IP/;
my @sysreg32 = qw/EFLAGS PKRU MXCSR EIP/;
my @sysreg64 = qw/RFLAGS XCR0 RIP/;


# --------------------------------------------------------------------------------------------------------


REGISTERS NONE    => ( [qw/NONE/],  0,   'NONE', );
REGISTERS GPR8    => ( \@gpr8,      8,   'I8',   );
REGISTERS GPR16   => ( \@gpr16,     16,  'I16',  );
REGISTERS GPR32   => ( \@gpr32,     32,  'I32',  );
REGISTERS GPR64   => ( \@gpr64,     64,  'I64',  );
REGISTERS SEGREG  => ( \@segreg,    16,  'I16',  );
REGISTERS FPREG   => ( \@fpreg,     64,  'F64',  );
REGISTERS MMXREG  => ( \@mmxreg,    64,  'I64',  );
REGISTERS TBLREG  => ( \@tblreg,    16,  'I16',  );
REGISTERS XMMREG  => ( \@xmmreg,    128, 'NONE', );
REGISTERS YMMREG  => ( \@ymmreg,    256, 'NONE', );
REGISTERS ZMMREG  => ( \@zmmreg,    512, 'NONE', );
REGISTERS MASKREG => ( \@maskreg,   64,  'NONE', );
REGISTERS BNDREG  => ( \@bndreg,    128, 'NONE', );

# fixme: scale with platform pointer size: 
REGISTERS CREG => ( \@cntrlreg, 64, 'NONE', );
REGISTERS DREG => ( \@dbgreg,   64, 'NONE', );

# system registers (AKA special registers):
REGISTERS SYSREG16 => ( \@sysreg16, 16, 'NONE', );
REGISTERS SYSREG32 => ( \@sysreg32, 32, 'NONE', );
REGISTERS SYSREG64 => ( \@sysreg64, 64, 'NONE', );



1;