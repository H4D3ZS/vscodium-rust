use strict;
use warnings;
use Data::Dump qw/pp/;
use feature qw/say/;
use Carp qw/carp croak/;
use Storable qw/dclone/;
use List::Util qw/min max uniq/;
use LogOnce;
my $log        = LogOnce->new();
my $records    = [];
my $tables     = [];
my $enums      = [];
my $defs       = {};
my $seen       = {};
my $attributes = {};
my $nodeinfo   = {};
my $arch       = get_arch();
my $registers  = [];
sub attributes { $attributes }
sub nodeinfo   { $nodeinfo }

sub registers { $registers }

sub REGISTERS(@) {
	my ( $type, $array, $size, $dt ) = @_;
	my $last = scalar @$array - 1;
	for my $i ( 0 .. $last ) {
		my $name = $array->[$i];
		push @$registers, {
			encoding => $i,
			reg      => $name,
			kind     => $type,
			size     => $size,
			datatype => $dt,
			is_first => 0 | !$i,
			is_last  => 0 | ( $i == $last ),
			is_even  => 0 | ( 0 == ( $i % 2 ) ),
		};
	}
}

sub NODE(@) {
	my $name  = shift;
	my $value = shift;
	$nodeinfo->{$name} = $value;
	$value;
}

sub ATTR(@) {
	my $name  = shift;
	my $value = shift;
	$attributes->{$name} = $value;
}

sub seen {
	my ($what) = @_;
	defined $what and $seen = $what;
	$seen;
}

sub defs {
	my ($what) = @_;
	defined $what and $defs = $what;
	$defs;
}

sub records {
	my ($what) = @_;
	defined $what and $records = $what;
	$records;
}

sub tables {
	my ($what) = @_;
	defined $what and $tables = $what;
	$tables;
}

sub enums {
	my ($what) = @_;
	defined $what and $enums = $what;
	$enums;
}

sub STRINGIFY($) {
	my ($hash) = @_;
	$hash->{stringify} = 1;
	$hash;
}

sub ORDERED($) {
	my ($hash) = @_;
	$hash->{ordered} = 1;
	$hash;
}

sub OPTIONAL($) {
	my ($hash) = @_;
	$hash->{optional} = 1;
	$hash;
}

sub DEPRECATED($) {
	my ($hash) = @_;
	$hash->{deprecated} = 1;
	$hash;
}

sub NOLONG($) {

	# alias for DEPRECATED.
	my ($hash) = @_;
	$hash->{deprecated} = 1;
	$hash;
}

sub UNPREDICTABLE($) {
	my ($hash) = @_;
	$hash->{unpredictable} = 1;
	$hash;
}

sub UNDEFINED($) {
	my ($hash) = @_;
	$hash->{undefined} = 1;
	$hash;
}

sub THROUGH($) {
	my ($name) = @_;
	return { table => $name };
}

sub FALL($) {
	my ($hash) = @_;
	$hash;
}

sub SKIP($) {
	my ($hash) = @_;
	$hash->{skip} = 1;
	$hash;
}

sub DEF(@) {
	my ( $link, $data ) = @_;
	$defs->{$link} = $data;
	return $data;
}

sub dereference_binary_values {
	my ($items) = @_;
	my @items = ();
	foreach my $item (@$items) {
		unless ( exists $item->{encoding} ) {
			push @items, $item;
			next;
		}
		my $value = $item->{encoding};
		if ( $value =~ /^0b[01]+$/ ) {
			$item->{encoding} = oct $value;
			push @items, $item;
		}
		elsif ( $value =~ /^0b([01x]+)$/ ) {
			my ( $min, $max ) = ( $value, $value );
			$min =~ s/x/0/g;
			$max =~ s/x/1/g;
			$min = oct $min;
			$max = oct $max;
			for my $i ( $min .. $max ) {
				( $i & $min ) == $min or next;
				my $new = dclone $item;
				$new->{encoding} = $i;
				push @items, $new;
			}
		}
		else {
			push @items, $item;
		}
	}
	return \@items;
}

sub TABLE(@) {
	my ( $link, $what ) = @_;
	$what = dereference_binary_values($what);
	push @$tables,
	  my $tbl = {
		id    => $link,
		items => $what
	  };
	$seen->{$link} = $tbl;
	$tbl;
}

sub ARRAY($) {
	my ($what) = @_;
	$what->{array} = 1;
	$what;
}

sub ENUM(@) {
	my ($link) = shift;
	my @items = ();
	push @items, { name => $_ } foreach (@_);
	my $enum = {
		id    => $link,
		items => \@items,
	};
	push @$enums, $enum;
	return $enum;
}

sub parse_docvars {
	local ($_) = @_;
	my %docvars = ();
	$docvars{$2} = $3 while (/\G(\s*([^=]+)\s*=\s*(\S+)\s*)/gc);
	return \%docvars;
}

sub parse_array {
	local ($_) = @_;
	my @array = split(/\s*\|\s*/);
	return \@array;
}

my %key2parser = (
	docvars    => \&parse_docvars,
	docvars2   => \&parse_docvars,
	metadata   => \&parse_docvars,
	tags       => \&parse_docvars,
	iflags     => \&parse_docvars,
	pstate     => \&parse_docvars,
	diagram    => \&parse_diagram,
	exceptions => \&parse_array,
	extensions => \&parse_array,
	categories => \&parse_array,
);

sub RECORD(@) {
	my ( $link, $hash ) = @_;
	$hash->{id} = $link;
	$seen->{$link} = $hash;
	push @$records, $hash;
	foreach my $key ( keys %$hash ) {
		my $value = $hash->{$key};
		if ( exists $key2parser{$key} ) {
			my $parser = $key2parser{$key};
			$value = $parser->($value);
			$hash->{$key} = $value;
		}
	}
	return $hash;
}

sub PAGE(@) {
	my $result = RECORD(@_);
	$result->{rectype} = 'PAGE';
	$result;
}

sub GROUP(@) {
	my $result = RECORD(@_);
	$result->{rectype} = 'GROUP';
	$result;
}

sub ICLASS(@) {
	my $result = RECORD(@_);
	$result->{rectype} = 'ICLASS';
	$result;
}

sub CLASS(@) {
	my $result = RECORD(@_);
	$result->{rectype} = 'CLASS';
	$result;
}

sub ENCODING(@) {
	my $result = RECORD(@_);
	$result->{rectype} = 'ENCODING';
	$result;
}
1;
