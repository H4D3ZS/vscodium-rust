use strict;
use warnings;
use Carp qw/carp croak confess/;
# TODO: move specefic attributes (non common) to target folder.

# common attributes for aarch32/aarch64/x86:
# ------------------------------------------

ATTR  SUPP => { suppressed  => 1 };
ATTR  OPT  => { optional    => 1 };
ATTR  rw   => { read        => 1, write => 1 };
ATTR  r    => { read        => 1 };
ATTR  w    => { write       => 1 };
ATTR  cr   => { read        => 1, conditional_reading => 1 };
ATTR  cw   => { write       => 1, conditional_writing => 1 };
ATTR  crw  => { read        => 1, write => 1, conditional_reading => 1 };
ATTR  rcw  => { read        => 1, write => 1, conditional_writing => 1 };

ATTR  Z  => { zeroing     => 1 };
ATTR  M  => { merging     => 1 };


# unkown attribute:
# we emit a warn and we assume that attribute is a boolean type.
ATTR "*" => sub {
	my ($hash,$attr) = @_;
	carp sprintf "unkown attribute '%s'", $attr;
	$hash->{$attr} = 1; # as true.
};

# common attributes for aarch32/aarch64:
# -------------------------------------

ATTR even => {even => 1};  # register muse be even-numbred.
ATTR pair => {pair => 1};  # register muse be pair => reg=previous_reg+1.

# OFF: we don't need it. was introduced to make instruction looks good
#      when there are instructions in the same group that use PRE/POST attribute.
ATTR OFF   => {}; 
ATTR PRE   => { pre_indexed  => 1 };
ATTR PST   => { post_indexed => 1 };


# attributes for aarch64:
# -----------------------

# noxzr: constraint if xzr was used.
# nowzr: constraint if wzr was used.
# we alias them to a single field 'nozr'.
ATTR noxzr => {nozr => 1}; 
ATTR nowzr => {nozr => 1}; 

ATTR sp => {sp => 1};  # use of sp is permitted.

# i0: is used only by PRFx instruction. 
# FIXME: correct datatype/memory size for PRFx instruction.
ATTR i0 => {};


# attributes for aarch32:
# -----------------------

# pc attributes:
ATTR upc  => {upc  => 1};  # unpredictable if pc was used.
ATTR dpc  => {dpc  => 1};  # deprecated    if pc was used.
ATTR nopc => {nopc => 1};  # constraint    if pc was used.

ATTR nosp => {nosp => 1};  # constraint    if sp was used.

ATTR iwb  => {iwb => 1};   # interworking branch.
ATTR erb  => {erb => 1};   # exception return. 

ATTR ei => {empty_index => 1}; # {regs}[].

# attributes for x86:
# -------------------
ATTR	sae  => { supress_all_exception => 1 };
ATTR	bcst => { broadcasting          => 1 };
ATTR	swiz => { swizzle               => 1 };
ATTR	rndc => { round_control         => 1 };

# when we know the data type but we dont know the size:
ATTR sx => { datatype =>  'SX' };
ATTR ux => { datatype =>  'UX' };
ATTR ix  => { datatype => 'IX'  };


# ---------------------------------------------------------------------------------------------------------------------

# datatype attributes:
foreach my $dt (qw/f16 bf16 f32 f64 f80 bcd80 i8 i16 i32 i64 i128 i256 
				   s8 s16 s32 s64 u8 u16 u32 u64 u128 u256 p8 p64/) {
	ATTR lc $dt  => { datatype => uc $dt };
	# for aarch32:
	ATTR uc $dt  => { datatype => uc $dt, dt_required => 1};
}



# aliases:
my %aliases=(
	read  => 'r',
	write => 'w',	
	SP => 'sp',
);

my $attributes = attributes();
foreach my $key(keys %aliases){
	my $org = $aliases{$key};
	ATTR ($key , $attributes->{$org});
}

1;