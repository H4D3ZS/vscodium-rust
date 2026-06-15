# ======================================= x86 backend =======================================

use strict;
use warnings;
use feature qw/say/;
use Data::Dump qw/pp/;
use Carp qw/carp croak/;
use JSON::XS;
sub get_arch { 'x86' }

my @widths = ();

sub WIDTH(@) {
	my ($name) = shift;
	push @widths, { name => $name, width16 => shift, width32 => shift, width64 => shift };
}

sub SWIZZLE($) {
	my ($hash) = @_;
	$hash->{type} = 'swizzle_tv';
	$hash->{ordered} = $hash->{array} = 1;
	$hash;
}

require 'core.pl';
require 'node.pl';
require "db/attrs.pl";
require "db/x86/nodes.pl";
require 'db/x86/macros.pl';

require 'db/x86/width.pl';
require 'db/x86/registers.pl';
require 'db/x86/enums.pl';
foreach my $key (qw/base float system fpsimd simd advsimd amd via knc/) {
	-f "db/x86/$key/tables.pl" and require "db/x86/$key/tables.pl";
	require "db/x86/$key/pages.pl";
	require "db/x86/$key/encodings.pl";
	require "db/x86/$key/templates.pl";
	-f "db/x86/$key/skiped.pl" and do {
		warn sprintf "skiped.pl found in '%s' folder", $key;
		require "db/x86/$key/skiped.pl";
	  }
}

sub TE { { error => 1 } }

sub parse_diagram {
	local ($_) = @_;
	my @fields = ();
	while (/\G(\s*(\w+)\s*)/gc) {
		my $name = $2;
		/\G(=\s*)/gc or return undef;
		/\G(\w+)/gc  or return undef;
		my $value = $1;
		push @fields, { name => $name, value => $value };
	}
	return { fields => \@fields };
}

sub parse_metadata {
	local ($_) = @_;
	$_ = expand_macros($_);
	return parse_docvars($_);
}

sub T($) {
	my ($array) = @_;
	local $_ = shift @$array;
	my $bitdiffs = shift @$array;
	my $encoding = shift @$array;
	my $metadata = shift @$array;
	$metadata = parse_metadata($metadata);
	my $seen = seen();
	$encoding =~ s/\s+//g;
	my $syntax = {
		text => $_,
		mnem => '',
	};
	$syntax->{text} =~ s/(^\s*|\s+$)//g;
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
			my $diagram = parse_diagram($bitdiffs);
			$result->{bitdiffs} = { fields => $diagram->{fields} };
		}
	}
	else {
		carp sprintf "unable to find encoding '%s'", $encoding;
		return TE;
	}

	return $result;
}

my $records     = records();
my $tables      = tables();
my $enums       = enums();
my $registers   = registers();
my $environment = {
	arch      => 'x86',
	version   => '3',
	enums     => $enums,
	records   => $records,
	tables    => $tables,
	widths    => \@widths,
	registers => $registers,
};

my $json = JSON::XS->new->pretty(1)->encode($environment);
open my $fh, '>', 'json/x86.json';
print $fh $json;
close $fh;

1;
