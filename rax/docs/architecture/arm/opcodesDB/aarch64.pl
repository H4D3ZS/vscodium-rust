use strict;
use warnings;
use Data::Dump qw/pp/;
use feature qw/say/;
use JSON::XS;

sub get_arch { 'aarch64' }

require 'core.pl';
require 'node.pl';
require 'arm.pl';

require "db/attrs.pl";
require "db/aarch64/nodes.pl";
require "db/aarch64/enums.pl";
require "db/aarch64/invalid.pl";
require "db/aarch64/registers.pl";

foreach my $key (qw/ tables/) {
	require "db/aarch64/$key.pl";
}

foreach my $key (qw/basic fpsimd sve/) {
	require "db/aarch64/$key/groups.pl";
	require "db/aarch64/$key/iclasses.pl";
	require "db/aarch64/$key/pages.pl";
	require "db/aarch64/$key/classes.pl";
	require "db/aarch64/$key/encodings.pl";
	require "db/aarch64/$key/templates.pl";
}

my $records     = records();
my $tables      = tables();
my $enums       = enums();
my $registers   = registers();
my $environment = {
	arch      => 'aarch64',
	version   => '3',
	enums     => $enums,
	records   => $records,
	tables    => $tables,
	registers => $registers,
};

my $json = JSON::XS->new->pretty(1)->encode($environment);
open my $fh, '>', 'json/aarch64.json';
print $fh $json;
close $fh;

1;
