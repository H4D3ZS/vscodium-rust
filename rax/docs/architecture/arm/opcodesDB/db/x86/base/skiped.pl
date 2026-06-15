use strict;
use warnings;

# NOP-No Operation.
# ----------------

# with CLDEMOTE=1 a nop instruction is valid only 
# when there is a mandatory prefix (either 66, f2, f3).
T['NOP MEM:r:sx=$memv REG:r=$GPRvr    ', 'CLDEMOTE=1 MOD=MEM P66=1 FP2=0 PF3=0 ', 'NOP_mvrv_5 ', ''];
T['NOP MEM:r:sx=$memv REG:r=$GPRvr    ', 'CLDEMOTE=1 MOD=MEM P66=0 PF2=1 PF3=0 ', 'NOP_mvrv_5 ', ''];
T['NOP MEM:r:sx=$memv REG:r=$GPRvr    ', 'CLDEMOTE=1 MOD=MEM P66=0 PF2=0 PF3=1 ', 'NOP_mvrv_5 ', ''];
