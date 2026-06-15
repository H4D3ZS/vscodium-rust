use strict;
use warnings;
use feature qw/say/;
use Data::Dump qw/pp/;
use Storable qw/dclone/;
use Carp qw/carp croak confess/;

my $definitions = defs();
my $attributes  = attributes();
my $nodeinfo    = nodeinfo();

sub get_node_info {
	my ($name) = @_;
	foreach my $key ( $name, qw/*/ ) {
		exists $nodeinfo->{$key} and return $nodeinfo->{$key};
	}
	return undef;
}

sub get_node_definition {
	my ($link) = @_;
	if ( exists $definitions->{$link} ) {
		return $definitions->{$link};
	}
	else {
		carp sprintf "undefined '\$%s'", $link;
	}
}

sub read_attrs {
	my ($ref) = @_;
	my @attrs = ();
	for ($$ref) {
		push @attrs, $2 while (/\G(\s*:\s*(\w+)\s*)/gc);
	}
	return \@attrs;
}

sub read_left {
	my ($ref) = @_;
	my $name  = '';
	my $attrs = [];
	for ($$ref) {
		/\G(\s*(\w+)\s*)/gc or last;
		$name = $2;
		if (/\G(:)/) {
			$attrs = read_attrs( \$_ );
			$attrs or last;
		}
		return ( $name, $attrs );
	}
	return ( undef, undef );
}

sub read_hash {
	my ($ref) = @_;
	my $array = read_nodes( \$_ );
	my $hash  = {};
	foreach my $item (@$array) {
		my $type = $item->{type};
		$hash->{$type} = $item;
	}
	return $hash;
}

sub read_array {
	my ($ref) = @_;
	my $array = read_nodes( \$_ );
	$array or return undef;
	my $hash = { items => $array, };
	return $hash;
}

sub read_right {
	my ($ref) = @_;
	for ($$ref) {
		/\G(\s+)/gc;
		if (/\G(\{)/gc) {
			my $value = read_hash( \$_ );
			/\G(\s*\})/gc or return undef;
			return $value;
		}
		elsif (/\G(\[)/gc) {
			my $value = read_array( \$_ );
			/\G(\s*\])/gc or return undef;
			return $value;
		}
		elsif (/\G(\$(\w+))/gc) {
			my $value = get_node_definition($2);
			if ( ref($value) =~ /HASH/ ) {
				$value = dclone $value;
			}
			elsif ( ref($value) =~ /ARRAY/ ) {
				$value = dclone $value;
			}
			elsif ( !ref($value) ) {
				$value = { value => $value };
			}
			return $value;
		}
		m/\G('([^']*)')/gc and return { text => $2 };
		if (/\G([^\{\}\[\]\s']+)/gc) {
			my $what = $1;
			$what =~ /^[\-\+]?\d+\.\d+$/ and return { fpvalue => $what };
			return { value => $what };
		}
	}
	return undef;
}

sub read_node {
	my ($ref) = @_;
	for ($$ref) {
		/\G(\s+)/gc;
		my ( $left, $attrs ) = read_left( \$_ );
		$left or return undef;
		/\G(\s+)/gc;
		/\G(=)/gc or return undef;
		my $right = read_right( \$_ );
		defined $right or return undef;
		$right->{type} = $left;
		@$attrs and apply_attrs( $right, $attrs );
		return fix_node($right);
	}
	return undef;
}

sub read_sequence {
	my ( $ref, $pattern ) = @_;
	my @array  = ();
	my $result = $pattern->($ref);
	while ($result) {
		push @array, $result;
		$result = $pattern->($ref);
	}
	return \@array;
}

sub read_nodes {
	my ($ref) = @_;
	my $array = read_sequence( $ref, \&read_node );
	return $array;
}

sub expand_macros {
	local ($_) = @_;
	my $MAX_ALLOWED_DEPTH = 100;
	my $depth = 0;
  again:
	$depth++;
	my $again = 0;
	my @symbols = /\$(\w+)/g;
	foreach my $symbol (@symbols) {
		if ( exists $definitions->{$symbol} ) {
			my $value = $definitions->{$symbol};
			if ( !ref($value) ) {
				s/\$\Q$symbol\E/$value/;
				$again++;
			}
		}
	}
	if($again){
		$depth > $MAX_ALLOWED_DEPTH and die "abused use of macros.";
		goto again;
	}
	return $_;
}

sub parse_node($) {
	local ($_) = @_;
	$_ = expand_macros($_);
	my $ref   = \$_;
	my $nodes = read_nodes($ref);
	for ($$ref) {
		/\G(\s+)/gc;
		/\G(.+)/ and return undef;
	}
	return $nodes;
}

sub apply_attrs {
	my ( $hash, $attrs ) = @_;
	foreach my $attr (@$attrs) {
		my $definition = $attributes->{$attr} // $attributes->{'*'};
		if ($definition) {
			if ( ref($definition) =~ /HASH/ ) {
				$definition = dclone $definition;
				foreach my $key ( keys %$definition ) {
					my $value = $definition->{$key};
					$hash->{$key} = $value;
				}
			}
			elsif ( $definition =~ /CODE/ ) {
				$definition->( $hash, $attr );
			}
			else {
				$hash->{$attr} = $definition;
			}
		}
		else {
			warn sprintf "unkown used attribute '%s'", $attr;
		}
	}
}

sub get_right_value {
	my ($right) = @_;
	foreach my $key (qw/value fpvalue text/) {
		exists $right->{$key} and return $right->{$key};
	}
	return undef;
}

sub fix_node {
	my ($node) = @_;
	my @keys = keys %$node;
	foreach my $key (@keys) {
		my $right = $node->{$key};
		my $info  = get_node_info($key);
		$info or next;
		$info->{terminal} and $right = get_right_value($right);
		if ( $info->{mapto} ) {
			delete $node->{$key};
			$key = $info->{mapto};
		}
		elsif ( $info->{lowercase} ) {
			delete $node->{$key};
			$key = lc $key;
		}
		$info->{handler} and	$right = $info->{handler}->($right);
		$node->{$key} = $right;
	}
	
	return $node;
}

1;
