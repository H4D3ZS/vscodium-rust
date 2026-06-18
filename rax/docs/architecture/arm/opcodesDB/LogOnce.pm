package LogOnce;
use strict;
use warnings;
use Data::Dump qw/pp/;
use feature qw/say/;

sub new {
	my ($class) = @_;
	my $self = { table => {}, };
	bless $self, $class;
	return $self;
}

sub log {
	my ( $self, $what, $comment ) = @_;
	my $table = $self->{table};
	$table->{$what}++ and return 0;
	if ( defined $comment ) {
		printf "%s # %s\n", $what, $comment;
	}
	else {
		printf "%s\n", $what;
	}
}

sub clear {
	my ($self) = @_;
	$self->{table} = {};
}
1;
