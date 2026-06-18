use strict;
use warnings;
use Data::Dump qw/pp/;
use feature qw/say/;
use Carp qw/carp croak/;

# table value macros:
sub TV_ANY {
	my ( $type, $symbol, $encoding ) = ( shift, shift, shift );
	my $representable = 1;
	my $tv = { encoding => $encoding };
	if ( $symbol =~ /^(UNPREDICTABLE|UNDEFINED|ABSENT|OMITTED)$/ ) {
		$tv->{ lc $symbol } = 1;
	}
	else {
		$symbol=~/uimm\d+/i and $representable = 0;
		$tv->{ lc $type } = $symbol;
	}

	my $attrs = shift;
	if ($attrs) {
		my @attrs = split( /\|/, $attrs );
		foreach (@attrs) {
			if (/^(HASALIAS|ALIAS)$/) {
				$tv->{ lc $_ } = 1;
			}
		}
	}
	$tv->{is_representable} = $representable;
	return $tv;
}

sub TV_COND        { TV_ANY( 'COND',        @_ ) }
sub TV_SHIFT       { TV_ANY( 'SHIFT',       @_ ) }
sub TV_IMM         { TV_ANY( 'VALUE',       @_ ) }
sub TV_FPIMM       { TV_ANY( 'FPVALUE',     @_ ) }
sub TV_PRD         { TV_ANY( 'PRD',         @_ ) }
sub TV_ALIGN       { TV_ANY( 'VALUE',       @_ ) }
sub TV_ENDIAN      { TV_ANY( 'ENDIAN',      @_ ) }
sub TV_INC         { TV_ANY( 'value',       @_ ) }
sub TV_OPTION      { TV_ANY( 'OPTION',      @_ ) }
sub TV_BARRIER     { TV_ANY( 'BARRIER',     @_ ) }
sub TV_BNKDREG     { TV_ANY( 'BNKDREG',     @_ ) }
sub TV_SPECREG     { TV_ANY( 'SPECREG',     @_ ) }
sub TV_CSPACE      { TV_ANY( 'CSPACE',      @_ ) }
sub TV_PSPACE      { TV_ANY( 'PSPACE',      @_ ) }
sub TV_AT_OP       { TV_ANY( 'AT_OP',       @_ ) }
sub TV_DC_OP       { TV_ANY( 'DC_OP',       @_ ) }
sub TV_IC_OP       { TV_ANY( 'IC_OP',       @_ ) }
sub TV_TLBI_OP     { TV_ANY( 'TLBI_OP',     @_ ) }
sub TV_BTI_OP      { TV_ANY( 'BTI_OP',      @_ ) }
sub TV_PATTERN     { TV_ANY( 'PATTERN',     @_ ) }
sub TV_PSTATEFIELD { TV_ANY( 'PSTATEFIELD', @_ ) }
sub TV_PRF_OP { TV_ANY( 'PRF_OP', @_ ) }


sub TV_SHIFTER {
	my $value    = shift;
	my $encoding = shift;
	my $tv       = {};
	if ( $value =~ /^\s*(\w+)\s*#?(\d+)\s*$/ ) {
		$tv = { 'shift' => $1, amount => $2, encoding => $encoding };
	}
	else {
		carp sprintf "unable to parse '%s' as shifter.", $value;
	}
	return $tv;
}

sub TV_EXTEND {
	my $tv = TV_ANY( 'EXTEND', @_ );
	if ( exists( $tv->{extend} ) && $tv->{extend} =~ s/^(\w+)\|(\w+)/$1/ ) {
		$tv->{extend2} = $2;
	}
	return $tv;
}

# node macros:
sub ANY {
	my $result = {
		symbol    => shift,
		encodedin => shift,
	};
	my $attrs = undef;
	while (@_) {
		my $what = shift;
		defined $what or last;
		if ( $what eq 'default' ) {
			$result->{'default_to'} = shift;
		}
		elsif ( $what eq 'table' ) {
			$result->{table} = shift;
		}
		elsif ( $what eq 'scale' ) {
			$result->{scale} = shift;
		}
		elsif ( $what eq 'signpos' ) {
			$result->{signpos} = shift;
		}
		elsif ( $what eq 'range' ) {
			$result->{range} = { min => shift, max => shift };
		}
		elsif ( $what =~ /^arg\d+$/ ) {
			$result->{$what} = shift;
		}
		elsif ( !$attrs ) {
			$attrs = $what;
		}
		else {
			carp "unable to handle '%s'", $what;
		}
	}
	if ($attrs) {
		my @attrs = split( /\|/, $attrs );
		apply_attrs( $result, \@attrs );
	}
	return $result;
}

sub REG {
	my $result = ANY(@_);
	$result;
}

sub GPR32 {
	my $result = REG(@_);
	$result->{kind} = 'GPR32';
	$result;
}

sub GPR64 {
	my $result = REG(@_);
	$result->{kind} = 'GPR64';
	$result;
}

sub SIMD8 {
	my $result = REG(@_);
	$result->{kind} = 'SIMD8';
	$result;
}

sub SIMD16 {
	my $result = REG(@_);
	$result->{kind} = 'SIMD16';
	$result;
}

sub SIMD32 {
	my $result = REG(@_);
	$result->{kind} = 'SIMD32';
	$result;
}

sub SIMD64 {
	my $result = REG(@_);
	$result->{kind} = 'SIMD64';
	$result;
}

sub SIMD128 {
	my $result = REG(@_);
	$result->{kind} = 'SIMD128';
	$result;
}

sub VECREG {
	my $result = REG(@_);
	$result->{kind} = 'VECREG';
	$result;
}

sub PRDREG {
	my $result = REG(@_);
	$result->{kind} = 'PRDREG';
	$result;
}

sub SVEREG {
	my $result = REG(@_);
	$result->{kind} = 'SVEREG';
	$result;
}

sub CSPACE {
	my $result = ANY(@_);
	$result;
}

sub XYZ {
	my $result = ANY(@_);
	$result;
}

sub PSPACE {
	my $result = ANY(@_);
	$result;
}

sub IMM {
	my $result = ANY(@_);
	$result;
}

sub FPIMM {
	my $result = ANY(@_);
	$result;
}

sub ENDIAN {
	my $result = ANY(@_);
	$result;
}

sub PRF_OP {
	my $result = ANY(@_);
	$result;
}

sub PATTERN {
	my $result = ANY(@_);
	$result;
}

sub PRD {
	my $result = ANY(@_);
	$result;
}

sub IFLAGS {
	my $result = ANY(@_);
	$result;
}

sub LABEL {
	my $result = ANY(@_);
	$result;
}

sub SHIFT {
	my $result = ANY(@_);
	$result;
}

sub EXTEND {
	my $result = ANY(@_);
	$result;
}

sub SHIFTER {
	my $result = ANY(@_);
	$result;
}

sub SLIST {
	my $result = ANY(@_);
	$result;
}

sub GLIST {
	my $result = ANY(@_);
	$result;
}

sub WBACK {
	my $result = ANY(@_);
	$result;
}

sub CREG {
	my $result = ANY(@_);
	$result;
}

sub ADD {
	my $result = ANY(@_);
	$result;
}

sub COND {
	my $result = ANY(@_);
	$result;
}

sub ALIGN {
	my $result = ANY(@_);
	$result;
}

sub BTI_OP {
	my $result = ANY(@_);
	$result;
}

sub PSTATEFIELD {
	my $result = ANY(@_);
	$result;
}

sub AT_OP {
	my $result = ANY(@_);
	$result;
}

sub DC_OP {
	my $result = ANY(@_);
	$result;
}

sub IC_OP {
	my $result = ANY(@_);
	$result;
}

sub TLBI_OP {
	my $result = ANY(@_);
	$result;
}

sub SYSREG {
	my $result = ANY(@_);
	$result;
}

sub INC {
	my $result = ANY(@_);
	$result;
}

sub COUNT {
	my $result = ANY(@_);
	$result;
}

sub BARRIER {
	my $result = ANY(@_);
	$result;
}

sub BNKDREG {
	my $result = ANY(@_);
	$result;
}

sub SPECREG {
	my $result = ANY(@_);
	$result;
}

# hint
sub HINT(@) {

}

# template error
sub TE { { error => 1 } }

sub parse_diagram {
	local ($_) = @_;
	my @fields = ();
	while (/\G(\s*(\w+)\s*)/gc) {
		my $name = $2;
		/\G(\[\d+\])/gc;
		/\G(\[\d+:\d+\])/gc;

		my $value = undef;
		my @attrs = ();
		push @attrs, $2 while (/\G(:\s*(\w+)\s*)/gc);
		unless (/\G(\s*=\s*)/gc) {
			carp sprintf "unable to parse diagram '%s'", $_;
			return undef;
		}
		if (/\G(0b[01xzs]+)/igc) {
			$value = $1;
		}
		else {
			carp sprintf "unable to parse diagram '%s'", $_;
			return undef;
		}
		push @fields,
		  my $field = {
			name  => $name,
			value => $value,
		  };
		foreach (@attrs) {
			if (/^u$/i) {
				$field->{usename} = 1;
			}
			else {
				$field->{$_} = 1;
			}
		}
	}
	unless ( scalar @fields ) {
		carp sprintf "unable to parse diagram '%s'", $_;
	}
	my ( $opcode, $mask, $pos ) = ( 0, 0, 0 );
	foreach my $field ( reverse @fields ) {
		my $value = $field->{value};
		$value =~ s/^0b//;
		$value = scalar reverse $value;
		$value =~ s/z/0/ig;
		$value =~ s/s/1/ig;
		$field->{size} = length $value;
		$field->{"pos"} = $pos;
		while ( $value =~ /\G([01x])/igc ) {
			my $char = $1;
			if ( $char eq '0' || $char eq '1' ) {
				$mask   |= 1 << $pos;
				$opcode |= ( $char + 0 ) << $pos;
			}
			$pos++;
		}
	}
	( $pos == 32 || $pos == 16 ) or return undef;
	my $diagram = {
		opcode => $opcode,
		mask   => $mask,
		size   => $pos,
		fields => \@fields,
	};
	return $diagram;
}

sub parse_bitdiffs {
	local ($_) = @_;
	my @fields = ();
	while (/\G(\s*(\w+)\s*=(0b[01x]+)\s*)/gc) {
		push @fields, { name => $2, value => $3 };
	}
	return \@fields;
}

sub get_bitdiffs {
	my ( $diagram, $diffs ) = @_;
	my %fields = ();
	$fields{ $_->{name} } = $_ foreach ( @{ $diagram->{fields} } );
	my ( $opcode, $mask ) = ( 0, 0 );
	foreach my $fld (@$diffs) {
		my $name  = $fld->{name};
		my $value = $fld->{value};
		my $field = $fields{$name};
		unless ($field) {
			carp sprintf "unable to find parent field '%s'", $name;
			return undef;
		}
		my $pos = $field->{pos};
		$value =~ s/^0b//;
		$value = scalar reverse $value;
		while ( $value =~ /\G([01x])/gc ) {
			my $char = $1;
			if ( $char =~ /^[01]$/ ) {
				$opcode |= $char << $pos;
				$mask   |= 1 << $pos;
			}
			$pos++;
		}
	}
	return {
		opcode => $opcode,
		mask   => $mask,
		fields => $diffs,
	};
}

sub parse_metadata {
	local ($_) = @_;
	$_ = expand_macros($_);
	my $metadata = parse_docvars($_);
	foreach my $key (qw/extensions/) {
		exists $metadata->{$key} or next;
		my $value = $metadata->{$key};
		my @array = split /\|/, $value;
		$metadata->{$key} = \@array;
	}
	if ( $metadata->{NZCV} ) {
		my $nzcv   = $metadata->{NZCV};
		my @chars  = $nzcv =~ /(.)/g;
		my $iflags = {
			n => $chars[0],
			z => $chars[1],
			c => $chars[2],
			v => $chars[3],
		};
		delete $metadata->{NZCV};
		$metadata->{iflags} = $iflags;
	}
	return $metadata;
}

sub T($) {
	my ($array) = @_;
	local $_ = shift @$array;
	my $bitdiffs = shift @$array;
	my $encoding = shift @$array;
	my $metadata = shift @$array;
	my $seen     = seen();
	$encoding =~ s/\s+//g;
	$metadata = parse_metadata($metadata);

	my $syntax = {
		text => $_,
		mnem => '',
	};
	my $result = {
		syntax   => $syntax,
		bitdiffs => undef,
		metadata => $metadata,
	};

	if (s/^\s*(\w+)\s*//) {
		$syntax->{mnem} = $1;
	}
	else {
		carp sprintf "unable to get mnem from '%s'", $_;
		return TE;
	}
	my $ast = parse_node($_);
	$ast or return TE;
	$syntax->{ast} = $ast;
	if ( exists $seen->{$encoding} ) {
		my $record = $seen->{$encoding};
		push @{ $record->{templates} }, $result;
		if ( $bitdiffs !~ /^\s*NONE\s*$/i ) {
			my $fields = parse_bitdiffs($bitdiffs);
			$bitdiffs = get_bitdiffs( $record->{diagram}, $fields );
			$result->{bitdiffs} = $bitdiffs;
		}
	}
	else {
		carp sprintf "unable to find encoding '%s'", $encoding;
		return TE;
	}

	return $result;
}
1;
