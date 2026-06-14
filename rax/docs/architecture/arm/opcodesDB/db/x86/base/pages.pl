#BEGIN_HEADER
#
# Copyright (C) 2020 Mahdi Safsafi.
#
# https://github.com/MahdiSafsafi/opcodesDB
#
# See licence file 'LICENCE' for use and distribution rights.
#
#END_HEADER

use strict;
use warnings;

PAGE AAA => {
  title => 'AAA-ASCII Adjust After Addition',
};

PAGE AAD => {
  title => 'AAD-ASCII Adjust AX Before Division',
};

PAGE AAM => {
  title => 'AAM-ASCII Adjust AX After Multiply',
};

PAGE AAS => {
  title => 'AAS-ASCII Adjust AL After Subtraction',
};

PAGE ADC => {
  title => 'ADC-Add with Carry',
};

PAGE ADCX => {
  title => 'ADCX-Unsigned Integer Addition of Two Operands with Carry Flag',
};

PAGE ADD => {
  title => 'ADD-Add',
};

PAGE ADOX => {
  title => 'ADOX-Unsigned Integer Addition of Two Operands with Overflow Flag',
};

PAGE AESDEC => {
  title => 'AESDEC-Perform One Round of an AES Decryption Flow',
};

PAGE AESDECLAST => {
  title => 'AESDECLAST-Perform Last Round of an AES Decryption Flow',
};

PAGE AESENC => {
  title => 'AESENC-Perform One Round of an AES Encryption Flow',
};

PAGE AESENCLAST => {
  title => 'AESENCLAST-Perform Last Round of an AES Encryption Flow',
};

PAGE AESIMC => {
  title => 'AESIMC-Perform the AES InvMixColumn Transformation',
};

PAGE AESKEYGENASSIST => {
  title => 'AESKEYGENASSIST-AES Round Key Generation Assist',
};

PAGE AND => {
  title => 'AND-Logical AND',
};

PAGE ANDN => {
  title => 'ANDN-Logical AND NOT',
};

PAGE ARPL => {
  title => 'ARPL-Adjust RPL Field of Segment Selector',
};

PAGE BLSI => {
  title => 'BLSI-Extract Lowest Set Isolated Bit',
};

PAGE BLSMSK => {
  title => 'BLSMSK-Get Mask Up to Lowest Set Bit',
};

PAGE BLSR => {
  title => 'BLSR-Reset Lowest Set Bit',
};

PAGE BOUND => {
  title => 'BOUND-Check Array Index Against Bounds',
};

PAGE BSF => {
  title => 'BSF-Bit Scan Forward',
};

PAGE BSR => {
  title => 'BSR-Bit Scan Reverse',
};

PAGE BSWAP => {
  title => 'BSWAP-Byte Swap',
};

PAGE BT => {
  title => 'BT-Bit Test',
};

PAGE BTC => {
  title => 'BTC-Bit Test and Complement',
};

PAGE BTR => {
  title => 'BTR-Bit Test and Reset',
};

PAGE BTS => {
  title => 'BTS-Bit Test and Set',
};

PAGE BZHI => {
  title => 'BZHI-Zero High Bits Starting with Specified Bit Position',
};

PAGE CALL => {
  title => 'CALL-Call Procedure',
};

PAGE CBWD2WDQ => {
  title => 'CBW/CWDE/CDQE-Convert Byte to Word/Convert Word to Doubleword/Convert Doubleword to Quadword',
};

PAGE CLC => {
  title => 'CLC-Clear Carry Flag',
};

PAGE CLD => {
  title => 'CLD-Clear Direction Flag',
};

PAGE CLI => {
  title => 'CLI-Clear Interrupt Flag',
};

PAGE CLTS => {
  title => 'CLTS-Clear Task-Switched Flag in CR0',
};

PAGE CMC => {
  title => 'CMC-Complement Carry Flag',
};

PAGE CMOVcc => {
  title => 'CMOVcc-Conditional Move',
};

PAGE CMP => {
  title => 'CMP-Compare Two Operands',
};

PAGE CMPS => {
  title => 'CMPS/CMPSB/CMPSW/CMPSD/CMPSQ-Compare String Operands',
};

PAGE CMPXCHG => {
  title => 'CMPXCHG-Compare and Exchange',
};

PAGE CMPXCHGxx => {
  title => 'CMPXCHG8B/CMPXCHG16B-Compare and Exchange Bytes',
};

PAGE CPUID => {
  title => 'CPUID-CPU Identification',
};

PAGE CWD2DQ => {
  title => 'CWD/CDQ/CQO-Convert Word to Doubleword/Convert Doubleword to Quadword',
};

PAGE DAA => {
  title => 'DAA-Decimal Adjust AL after Addition',
};

PAGE DAS => {
  title => 'DAS-Decimal Adjust AL after Subtraction',
};

PAGE DEC => {
  title => 'DEC-Decrement by 1',
};

PAGE DIV => {
  title => 'DIV-Unsigned Divide',
};

PAGE ENTER => {
  title => 'ENTER-Make Stack Frame for Procedure Parameters',
};

PAGE HLT => {
  title => 'HLT-Halt',
};

PAGE IDIV => {
  title => 'IDIV-Signed Divide',
};

PAGE IMUL => {
  title => 'IMUL-Signed Multiply',
};

PAGE IN => {
  title => 'IN-Input from Port',
};

PAGE INC => {
  title => 'INC-Increment by 1',
};

PAGE INS => {
  title => 'INS/INSB/INSW/INSD-Input from Port to String',
};

PAGE INT => {
  title => 'INT-n/INTO/INT3/INT1-Call to Interrupt Procedure',
};

PAGE INVD => {
  title => 'INVD-Invalidate Internal Caches',
};

PAGE INVLPG => {
  title => 'INVLPG-Invalidate TLB Entries',
};

PAGE IRET => {
  title => 'IRET/IRETD/IRETQ-Interrupt Return',
};

PAGE JMP => {
  title => 'JMP-Jump',
};

PAGE Jcc => {
  title => 'Jcc-Jump if Condition Is Met',
};

PAGE LAHF => {
  title => 'LAHF-Load Status Flags into AH Register',
};

PAGE LAR => {
  title => 'LAR-Load Access Rights Byte',
};

PAGE LEA => {
  title => 'LEA-Load Effective Address',
};

PAGE LEAVE => {
  title => 'LEAVE-High Level Procedure Exit',
};

PAGE LLDT => {
  title => 'LLDT-Load Local Descriptor Table Register',
};

PAGE LMSW => {
  title => 'LMSW-Load Machine Status Word',
};

PAGE LODS => {
  title => 'LODS/LODSB/LODSW/LODSD/LODSQ-Load String',
};

PAGE LOOP => {
  title => 'LOOP/LOOPcc-Loop According to ECX Counter',
};

PAGE LSL => {
  title => 'LSL-Load Segment Limit',
};

PAGE LTR => {
  title => 'LTR-Load Task Register',
};

PAGE LZCNT => {
  title => 'LZCNT-Count the Number of Leading Zero Bits',
};

PAGE LxDT => {
  title => 'LGDT/LIDT-Load Global/Interrupt Descriptor Table Register',
};

PAGE LxS => {
  title => 'LDS/LES/LFS/LGS/LSS-Load Far Pointer',
};

PAGE MOV => {
  title => 'MOV-Move',
};

PAGE MOVBE => {
  title => 'MOVBE-Move Data After Swapping Bytes',
};

PAGE MOVDIR64B => {
  title => 'MOVDIR64B-Move 64 Bytes as Direct Store',
};

PAGE MOVDIRI => {
  title => 'MOVDIRI-Move Doubleword as Direct Store',
};

PAGE MOVS => {
  title => 'MOVS/MOVSB/MOVSW/MOVSD/MOVSQ-Move Data from String to String',
};

PAGE MOVSX => {
  title => 'MOVSX/MOVSXD-Move with Sign-Extension',
};

PAGE MOVZX => {
  title => 'MOVZX-Move with Zero-Extend',
};

PAGE MOVcntrl => {
  title => 'MOV-Move to/from Control Registers',
};

PAGE MOVdbg => {
  title => 'MOV-Move to/from Debug Registers',
};

PAGE MUL => {
  title => 'MUL-Unsigned Multiply',
};

PAGE MULX => {
  title => 'MULX-Unsigned Multiply Without Affecting Flags',
};

PAGE NEG => {
  title => 'NEG-Two"s Complement Negation',
};

PAGE NOP => {
  title => 'NOP-No Operation',
};

PAGE NOT => {
  title => 'NOT-One"s Complement Negation',
};

PAGE OR => {
  title => 'OR-Logical Inclusive OR',
};

PAGE OUT => {
  title => 'OUT-Output to Port',
};

PAGE OUTS => {
  title => 'OUTS/OUTSB/OUTSW/OUTSD-Output String to Port',
};

PAGE PAUSE => {
  title => 'PAUSE-Spin Loop Hint',
};

PAGE PCLMULQDQ => {
  title => 'PCLMULQDQ-Carry-Less Multiplication Quadword',
};

PAGE PCONFIG => {
  title => 'PCONFIG',
};

PAGE PDEP => {
  title => 'PDEP-Parallel Bits Deposit',
};

PAGE PEXT => {
  title => 'PEXT-Parallel Bits Extract',
};

PAGE POP => {
  title => 'POP-Pop a Value from the Stack',
};

PAGE POPA => {
  title => 'POPA/POPAD-Pop All General-Purpose Registers',
};

PAGE POPF => {
  title => 'POPF/POPFD/POPFQ-Pop Stack into EFLAGS Register',
};

PAGE PUSH => {
  title => 'PUSH-Push Word, Doubleword or Quadword Onto the Stack',
};

PAGE PUSHA => {
  title => 'PUSHA/PUSHAD-Push All General-Purpose Registers',
};

PAGE PUSHF => {
  title => 'PUSHF/PUSHFD/PUSHFQ-Push EFLAGS Register onto the Stack',
};

PAGE RDMSR => {
  title => 'RDMSR-Read from Model Specific Register',
};

PAGE RDPMC => {
  title => 'RDPMC-Read Performance-Monitoring Counters',
};

PAGE RDTSC => {
  title => 'RDTSC-Read Time-Stamp Counter',
};

PAGE RET => {
  title => 'RET-Return from Procedure',
};

PAGE RORX => {
  title => 'RORX-Rotate Right Logical Without Affecting Flags',
};

PAGE ROTATE => {
  title => 'RCL/RCR/ROL/ROR-Rotate',
};

PAGE RSM => {
  title => 'RSM-Resume from System Management Mode',
};

PAGE SAHF => {
  title => 'SAHF-Store AH into Flags',
};

PAGE SALC => {
  title => 'SALC',
};

PAGE SBB => {
  title => 'SBB-Integer Subtraction with Borrow',
};

PAGE SCAS => {
  title => 'SCAS/SCASB/SCASW/SCASD-Scan String',
};

PAGE SETcc => {
  title => 'SETcc-Set Byte on Condition',
};

PAGE SGDT => {
  title => 'SGDT-Store Global Descriptor Table Register',
};

PAGE SHA1MSG1 => {
  title => 'SHA1MSG1-Perform an Intermediate Calculation for the Next Four SHA1 Message Dwords',
};

PAGE SHA1MSG2 => {
  title => 'SHA1MSG2-Perform a Final Calculation for the Next Four SHA1 Message Dwords',
};

PAGE SHA1NEXTE => {
  title => 'SHA1NEXTE-Calculate SHA1 State Variable E after Four Rounds',
};

PAGE SHA1RNDS4 => {
  title => 'SHA1RNDS4-Perform Four Rounds of SHA1 Operation',
};

PAGE SHA256MSG1 => {
  title => 'SHA256MSG1-Perform an Intermediate Calculation for the Next Four SHA256 Message Dwords',
};

PAGE SHA256MSG2 => {
  title => 'SHA256MSG2-Perform a Final Calculation for the Next Four SHA256 Message Dwords',
};

PAGE SHA256RNDS2 => {
  title => 'SHA256RNDS2-Perform Two Rounds of SHA256 Operation',
};

PAGE SHIFT => {
  title => 'SAL/SAR/SHL/SHR-Shift',
};

PAGE SHIFTX => {
  title => 'SARX/SHLX/SHRX-Shift Without Affecting Flags',
};

PAGE SHLD => {
  title => 'SHLD-Double Precision Shift Left',
};

PAGE SHRD => {
  title => 'SHRD-Double Precision Shift Right',
};

PAGE SIDT => {
  title => 'SIDT-Store Interrupt Descriptor Table Register',
};

PAGE SLDT => {
  title => 'SLDT-Store Local Descriptor Table Register',
};

PAGE SMSW => {
  title => 'SMSW-Store Machine Status Word',
};

PAGE STC => {
  title => 'STC-Set Carry Flag',
};

PAGE STD => {
  title => 'STD-Set Direction Flag',
};

PAGE STI => {
  title => 'STI-Set Interrupt Flag',
};

PAGE STOS => {
  title => 'STOS/STOSB/STOSW/STOSD/STOSQ-Store String',
};

PAGE STR => {
  title => 'STR-Store Task Register',
};

PAGE SUB => {
  title => 'SUB-Subtract',
};

PAGE SWAPGS => {
  title => 'SWAPGS-Swap GS Base Register',
};

PAGE SYSENTER => {
  title => 'SYSENTER-Fast System Call',
};

PAGE SYSEXIT => {
  title => 'SYSEXIT-Fast Return from Fast System Call',
};

PAGE TEST => {
  title => 'TEST-Logical Compare',
};

PAGE TZCNT => {
  title => 'TZCNT-Count the Number of Trailing Zero Bits',
};

PAGE UD => {
  title => 'UD-Undefined Instruction',
};

PAGE VCVTPH2PS => {
  title => 'VCVTPH2PS-Convert 16-bit FP values to Single-Precision FP values',
};

PAGE VCVTPS2PH => {
  title => 'VCVTPS2PH-Convert Single-Precision FP value to 16-bit FP value',
};

PAGE VERx => {
  title => 'VERR/VERW-Verify a Segment for Reading or Writing',
};

PAGE WBINVD => {
  title => 'WBINVD-Write Back and Invalidate Cache',
};

PAGE WRMSR => {
  title => 'WRMSR-Write to Model Specific Register',
};

PAGE XADD => {
  title => 'XADD-Exchange and Add',
};

PAGE XCHG => {
  title => 'XCHG-Exchange Register/Memory with Register',
};

PAGE XLAT => {
  title => 'XLAT/XLATB-Table Look-up Translation',
};

PAGE XOR => {
  title => 'XOR-Logical Exclusive OR',
};

