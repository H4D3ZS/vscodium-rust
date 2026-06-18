use strict;
use warnings;

NODE SZ        => {terminal => 1, mapto => 'size', };
NODE ENCODEDIN => {terminal => 1, lowercase => 1};
NODE LDACC     => {terminal => 1, lowercase => 1};
NODE STACC     => {terminal => 1, lowercase => 1};

# for all other nodes, we lowercase them:
NODE '*'       => {lowercase => 1,};

1;
