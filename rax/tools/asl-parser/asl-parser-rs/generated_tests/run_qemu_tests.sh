#!/bin/bash
# Run all extracted oracle tests against QEMU
# Input: /tmp/qemu_tests.txt with format: encoding init_x1 init_x2 expected_x0 (all hex)

TEMP_DIR="/tmp/qemu-validate"
QEMU="qemu-system-aarch64"
mkdir -p "$TEMP_DIR"

PASSED=0
FAILED=0
TOTAL=0
FAILURES=""

echo "=== QEMU Oracle Test Runner ==="
echo "Reading tests from /tmp/qemu_tests.txt"
echo ""

TOTAL_LINES=$(wc -l < /tmp/qemu_tests.txt | tr -d ' ')

# Process each test - using a file descriptor to avoid stdin issues with python
exec 3< /tmp/qemu_tests.txt
while IFS=' ' read -r enc x1 x2 exp <&3; do
    # Skip empty lines or malformed lines
    [[ -z "$enc" || -z "$x1" || -z "$x2" || -z "$exp" ]] && continue
    
    ((TOTAL++))
    
    # Use Python to generate the assembly with proper 64-bit handling
    python3 -c "
x1_val = int('$x1', 16)
x2_val = int('$x2', 16)
print('.global _start')
print('_start:')
# Set X1
print(f'    mov x1, #{x1_val & 0xFFFF}')
print(f'    movk x1, #{(x1_val >> 16) & 0xFFFF}, lsl #16')
print(f'    movk x1, #{(x1_val >> 32) & 0xFFFF}, lsl #32')
print(f'    movk x1, #{(x1_val >> 48) & 0xFFFF}, lsl #48')
# Set X2
print(f'    mov x2, #{x2_val & 0xFFFF}')
print(f'    movk x2, #{(x2_val >> 16) & 0xFFFF}, lsl #16')
print(f'    movk x2, #{(x2_val >> 32) & 0xFFFF}, lsl #32')
print(f'    movk x2, #{(x2_val >> 48) & 0xFFFF}, lsl #48')
# Clear X0
print('    mov x0, #0')
# Execute the instruction
print('    .word $enc')
print('1:  wfe')
print('    b 1b')
" > "$TEMP_DIR/t.S"

    # Compile
    if ! clang -target aarch64-unknown-none-elf -nostdlib -Wl,-Ttext=0x40000000 \
        -o "$TEMP_DIR/t.elf" "$TEMP_DIR/t.S" 2>/dev/null; then
        echo ""
        echo "COMPILE ERROR: enc=$enc x1=$x1 x2=$x2"
        ((FAILED++))
        continue
    fi
    
    llvm-objcopy -O binary "$TEMP_DIR/t.elf" "$TEMP_DIR/t.bin" 2>/dev/null
    
    # Run QEMU and extract X0
    out=$(timeout 1s $QEMU -M virt -cpu cortex-a72 -nographic -d cpu \
        -kernel "$TEMP_DIR/t.bin" 2>&1)
    
    # Get the X00 value after the instruction executed (second-to-last occurrence)
    actual_hex=$(echo "$out" | grep -oE 'X00=[0-9a-fA-F]+' | tail -3 | head -1 | cut -d= -f2)
    
    if [[ -z "$actual_hex" ]]; then
        echo ""
        echo "QEMU ERROR: No X0 output for enc=$enc"
        ((FAILED++))
        continue
    fi
    
    # Compare using Python for proper 64-bit handling
    result=$(python3 -c "
actual = int('$actual_hex', 16)
expected = int('$exp', 16)
if actual == expected:
    print('PASS')
else:
    print(f'FAIL actual=0x{actual:X} ({actual}) expected=0x{expected:X} ({expected})')
")

    if [[ "$result" == "PASS" ]]; then
        ((PASSED++))
        printf "\r[%d/%d] Passed: %d, Failed: %d" "$TOTAL" "$TOTAL_LINES" "$PASSED" "$FAILED"
    else
        ((FAILED++))
        FAILURES="${FAILURES}\n  #${TOTAL}: enc=$enc x1=$x1 x2=$x2 $result"
        printf "\r[%d/%d] Passed: %d, Failed: %d" "$TOTAL" "$TOTAL_LINES" "$PASSED" "$FAILED"
    fi
done
exec 3<&-

echo ""
echo ""
echo "=== FINAL RESULTS ==="
echo "Total:  $TOTAL"
echo "Passed: $PASSED"
echo "Failed: $FAILED"

if [[ -n "$FAILURES" ]]; then
    echo ""
    echo "Failures:"
    echo -e "$FAILURES"
fi

# Cleanup
rm -rf "$TEMP_DIR"

# Exit with error if any failed
[[ "$FAILED" -eq 0 ]]
