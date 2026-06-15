# opcodesDB:
opcodesDB is a CPU low level environment representation (registers, flags, instructions, ...).  Data are listed in a packed dynamic structure which can be unpacked by a parser at any time.

This project is a fruit of many years of development and a lot of attempts ([Parsable-Instructions](https://github.com/MahdiSafsafi/Parsable-Instructions), [asmdb](https://github.com/MahdiSafsafi/asmdb)) to standardize CPU environment.

Currently, it supports the following architectures:
- x86    : with all its extensions : FPU, MMX, SSE, FMA, AVX, AVX512. also it supports xeon instructions(KNC, KNL, ...).
- aarch32: arm and thumb instructions(ARMv8+).
- aarch64: ARMv8+ and SVE extension.

# Whats new in version 3 ?
- added aarch32 support.
- added aarch64 support.
- added xeon instruction set.
- improved x86 instructions definition.

# How to use it ?
you just need to run **x86.pl**, **aarch32.pl**, **aarch64.pl** and for each architecture, opcodesDB will generate a very big json file that contains all the architecture environment(instructions, registers, enums, tables, ...). All generated files (**x86.json**, **aarch32.json**, **aarch64.json**) are stored in the json folder.
