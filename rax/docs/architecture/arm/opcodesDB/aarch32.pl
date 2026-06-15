use strict;
use warnings;
use Data::Dump qw/pp/;
use feature qw/say/;
use JSON::XS;

sub get_arch{'aarch32'}

require 'core.pl';
require 'node.pl';
require 'arm.pl';


require "db/attrs.pl";
require "db/aarch32/nodes.pl";
require "db/aarch32/enums.pl";
require "db/aarch32/invalid.pl";
require "db/aarch32/registers.pl";
foreach my $key (qw/cond-table tables/) {
	require "db/aarch32/$key.pl";
}

foreach my $key (qw/basic fpsimd/) {
	require "db/aarch32/$key/groups.pl";
	require "db/aarch32/$key/iclasses.pl";
	require "db/aarch32/$key/pages.pl";
	require "db/aarch32/$key/classes.pl";
	require "db/aarch32/$key/encodings.pl";
	require "db/aarch32/$key/templates.pl";
}

my $records     = records();
my $tables      = tables();
my $enums       = enums();
my $registers   = registers();
my $environment = {
	arch    => 'aarch32',
	version => '3',
	enums   => $enums,
	records => $records,
	tables  => $tables,
	registers => $registers,
};


my $json = JSON::XS->new->pretty(1)->encode($environment);
open my $fh, '>', 'json/aarch32.json';
print $fh $json;
close $fh;

1;
