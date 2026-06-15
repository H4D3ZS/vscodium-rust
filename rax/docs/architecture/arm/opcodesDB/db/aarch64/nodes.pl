use strict;
use warnings;

# handler for custom size format eg: v32x16, 32x2
NODE SZ   => {terminal => 1, mapto => 'size', 
	handler => sub{
		local($_)=@_;
		my $first = '';
		s/^([vp]\d+x?)// and  $first = $1;
		again: s/(\d+)x(\d+)/$1*$2/e and goto again;
		uc $first.$_;
	}
};

NODE ENCODEDIN => {terminal => 1, lowercase => 1};
NODE LDACC     => {terminal => 1, lowercase => 1};
NODE STACC     => {terminal => 1, lowercase => 1};

# for all other nodes, we lowercase them:
NODE '*'       => {lowercase => 1,};

1;
