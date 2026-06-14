use strict;
use warnings;

NODE SZ        => {terminal => 1, mapto => 'size', };
NODE FROM      => {terminal => 1, lowercase => 1, };
NODE TO        => {terminal => 1, lowercase => 1, };

# for all other nodes, we lowercase them:
NODE '*'       => {lowercase => 1,};

1;
