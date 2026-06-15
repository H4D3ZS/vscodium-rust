use strict;
use warnings;
use Data::Dump qw/pp/;
use feature qw/say/;
use JSON::XS;

sub get_arch{'shared'}

require 'core.pl';
require 'node.pl';
require 'db/enums.pl';
my $enums       = enums();

my $environment = {
	enums   => $enums,
};

my $json = JSON::XS->new->pretty(1)->encode($environment);
open my $fh, '>', 'json/shared.json';
print $fh $json;
close $fh;