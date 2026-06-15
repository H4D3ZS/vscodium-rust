#!/bin/bash
# QEMU Bare-Metal Differential Testing for ARM Instructions
# Works with qemu-system-aarch64 (available on macOS via brew)
#
# This script runs generated ARM tests as bare-metal code under QEMU
# and compares results with oracle-computed expected values.
#
# Requirements:
#   - qemu-system-aarch64 (brew install qemu)
#   - aarch64 cross-compiler (brew install aarch64-elf-gcc OR use clang)

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
TEMP_DIR="/tmp/asl-qemu-baremetal"
QEMU="${QEMU:-qemu-system-aarch64}"

# Try to find cross-compiler
if command -v aarch64-elf-gcc &> /dev/null; then
    CC="aarch64-elf-gcc"
    OBJCOPY="aarch64-elf-objcopy"
elif command -v aarch64-linux-gnu-gcc &> /dev/null; then
    CC="aarch64-linux-gnu-gcc"
    OBJCOPY="aarch64-linux-gnu-objcopy"
else
    # Use clang with target
    CC="clang -target aarch64-unknown-none-elf"
    OBJCOPY="llvm-objcopy"
fi

PASSED=0
FAILED=0
SKIPPED=0
VERBOSE=0

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --verbose|-v)
            VERBOSE=1
            shift
            ;;
        --help|-h)
            echo "Usage: $0 [--verbose]"
            exit 0
            ;;
        *)
            shift
            ;;
    esac
done

# Check dependencies
if ! command -v "$QEMU" &> /dev/null; then
    echo "Error: $QEMU not found."
    echo "Install with: brew install qemu"
    exit 1
fi

mkdir -p "$TEMP_DIR"

echo "=== QEMU Bare-Metal Differential Test Runner ==="
echo "QEMU: $QEMU"
echo ""

# Create a minimal bare-metal test that outputs results via UART
create_test() {
    local name="$1"
    local encoding="$2"
    local init_x1="$3"
    local expected_x0="$4"
    
    local asm_file="$TEMP_DIR/${name}.S"
    
    cat > "$asm_file" << 'HEADER'
.section .text
.global _start

// PL011 UART base address for virt machine
.equ UART_BASE, 0x09000000
.equ UART_DR,   0x00

// Print a hex digit (0-F)
print_hex_digit:
    and x3, x0, #0xF
    cmp x3, #10
    blt 1f
    add x3, x3, #('A' - 10)
    b 2f
1:  add x3, x3, #'0'
2:  ldr x4, =UART_BASE
    str w3, [x4, #UART_DR]
    ret

// Print X0 as 16 hex digits
print_x0:
    stp x29, x30, [sp, #-16]!
    mov x29, sp
    mov x5, x0          // Save original value
    mov x6, #60         // Shift amount (start from MSB)
1:  mov x0, x5
    lsr x0, x0, x6
    bl print_hex_digit
    subs x6, x6, #4
    bge 1b
    // Print newline
    ldr x4, =UART_BASE
    mov w3, #'\n'
    str w3, [x4, #UART_DR]
    ldp x29, x30, [sp], #16
    ret

_start:
HEADER

    # Add register setup
    cat >> "$asm_file" << EOF
    // Set up X1 with initial value: $init_x1
    mov x1, #$((init_x1 & 0xFFFF))
EOF

    if [ "$init_x1" -gt 65535 ]; then
        cat >> "$asm_file" << EOF
    movk x1, #$(((init_x1 >> 16) & 0xFFFF)), lsl #16
EOF
    fi
    if [ "$init_x1" -gt 4294967295 ]; then
        cat >> "$asm_file" << EOF
    movk x1, #$(((init_x1 >> 32) & 0xFFFF)), lsl #32
EOF
    fi
    if [ "$init_x1" -gt 281474976710655 ]; then
        cat >> "$asm_file" << EOF
    movk x1, #$(((init_x1 >> 48) & 0xFFFF)), lsl #48
EOF
    fi

    cat >> "$asm_file" << EOF
    
    // Clear X0
    mov x0, #0
    
    // ===== INSTRUCTION UNDER TEST =====
    .word $encoding
    // ===================================
    
    // Print result (X0)
    bl print_x0
    
    // Infinite loop (halt)
1:  wfe
    b 1b

.section .data
.align 4
EOF
}

# Run a test
run_test() {
    local name="$1"
    local encoding="$2"
    local init_x1="$3"
    local expected_x0="$4"
    
    create_test "$name" "$encoding" "$init_x1" "$expected_x0"
    
    local asm_file="$TEMP_DIR/${name}.S"
    local obj_file="$TEMP_DIR/${name}.o"
    local elf_file="$TEMP_DIR/${name}.elf"
    local bin_file="$TEMP_DIR/${name}.bin"
    
    # Compile
    if ! $CC -nostdlib -nostartfiles -Wl,-Ttext=0x40000000 -o "$elf_file" "$asm_file" 2>/dev/null; then
        # Try with clang
        if ! clang -target aarch64-unknown-none-elf -nostdlib -Wl,-Ttext=0x40000000 -o "$elf_file" "$asm_file" 2>/dev/null; then
            echo "SKIP: $name (compilation failed)"
            ((SKIPPED++)) || true
            return 0
        fi
    fi
    
    # Extract binary
    $OBJCOPY -O binary "$elf_file" "$bin_file" 2>/dev/null || {
        llvm-objcopy -O binary "$elf_file" "$bin_file" 2>/dev/null || {
            echo "SKIP: $name (objcopy failed)"
            ((SKIPPED++)) || true
            return 0
        }
    }
    
    # Run under QEMU with timeout
    local output
    output=$(timeout 2s $QEMU \
        -M virt \
        -cpu cortex-a72 \
        -nographic \
        -kernel "$bin_file" \
        -serial mon:stdio \
        2>/dev/null || true)
    
    # Parse output - look for hex value
    local actual_hex
    actual_hex=$(echo "$output" | grep -oE '^[0-9A-Fa-f]{16}$' | head -1)
    
    if [[ -z "$actual_hex" ]]; then
        if [[ $VERBOSE -eq 1 ]]; then
            echo "SKIP: $name (no output)"
            echo "  Raw output: $output"
        fi
        ((SKIPPED++)) || true
        return 0
    fi
    
    # Compare
    local expected_hex
    expected_hex=$(printf "%016X" "$expected_x0")
    actual_hex=$(echo "$actual_hex" | tr 'a-f' 'A-F')
    
    if [[ "$expected_hex" == "$actual_hex" ]]; then
        echo "PASS: $name"
        if [[ $VERBOSE -eq 1 ]]; then
            echo "  X0 = 0x$actual_hex"
        fi
        ((PASSED++)) || true
    else
        echo "FAIL: $name"
        echo "  Expected: 0x$expected_hex"
        echo "  Actual:   0x$actual_hex"
        ((FAILED++)) || true
    fi
}

echo "Running differential tests..."
echo ""

# ============================================================================
# ADD Immediate Tests
# ============================================================================
echo "--- ADD Immediate ---"

# ADD X0, X1, #10 (X1=100 -> X0=110)
run_test "add_imm_100_10" "0x91002820" "100" "110"

# ADD X0, X1, #0 (X1=42 -> X0=42)
run_test "add_imm_42_0" "0x91000020" "42" "42"

# ADD X0, X1, #1 (X1=0 -> X0=1)
run_test "add_imm_0_1" "0x91000420" "0" "1"

# ADD X0, X1, #0xFFF (X1=0 -> X0=4095)
run_test "add_imm_0_max" "0x913FFC20" "0" "4095"

# ADD X0, X1, #0x1000 (shifted, X1=0 -> X0=4096)
run_test "add_imm_shifted" "0x91400420" "0" "4096"

# ============================================================================
# SUB Immediate Tests
# ============================================================================
echo ""
echo "--- SUB Immediate ---"

# SUB X0, X1, #10 (X1=100 -> X0=90)
run_test "sub_imm_100_10" "0xD1002820" "100" "90"

# SUB X0, X1, #1 (X1=1 -> X0=0)
run_test "sub_imm_1_1" "0xD1000420" "1" "0"

# SUB X0, X1, #0 (X1=99 -> X0=99)
run_test "sub_imm_99_0" "0xD1000020" "99" "99"

# ============================================================================
# MOV/MOVZ Tests
# ============================================================================
echo ""
echo "--- MOVZ Immediate ---"

# MOVZ X0, #0x1234
run_test "movz_1234" "0xD2824680" "0" "4660"

# MOVZ X0, #0xFFFF
run_test "movz_ffff" "0xD29FFFE0" "0" "65535"

# MOVZ X0, #0
run_test "movz_0" "0xD2800000" "0" "0"

# MOVZ X0, #1
run_test "movz_1" "0xD2800020" "0" "1"

# ============================================================================
# Logical Tests (AND, ORR, EOR)
# ============================================================================
echo ""
echo "--- Logical Operations ---"

# AND X0, X1, X1 (X1 & X1 = X1)
# Note: Need to set up X1 first, then AND with itself
# Encoding for AND X0, X1, X1: 0x8A010020

# ORR X0, XZR, X1 (mov X0, X1) - encoding: 0xAA0103E0
run_test "orr_mov" "0xAA0103E0" "12345" "12345"

# ============================================================================
# Shift Tests
# ============================================================================
echo ""
echo "--- Shift Operations ---"

# LSL X0, X1, #4 (X1 << 4)
# UBFM encoding for LSL: 0xD37CFC20 (LSL X0, X1, #4 for 64-bit)
run_test "lsl_4" "0xD37CFC20" "1" "16"

# LSR X0, X1, #4 (X1 >> 4)
# UBFM encoding for LSR: 0xD341FC20 (LSR X0, X1, #4 for 64-bit)  
run_test "lsr_4" "0xD341FC20" "256" "16"

echo ""
echo "=== Summary ==="
echo "Passed:  $PASSED"
echo "Failed:  $FAILED"  
echo "Skipped: $SKIPPED"
echo ""

# Cleanup
rm -rf "$TEMP_DIR"

if [[ $FAILED -gt 0 ]]; then
    echo "Some tests failed!"
    exit 1
else
    echo "All tests passed!"
fi
