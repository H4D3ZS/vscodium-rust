#!/bin/bash
TEMP_DIR="/tmp/qemu-validate"
QEMU="qemu-system-aarch64"
mkdir -p "$TEMP_DIR"

PASSED=0
FAILED=0
TOTAL=0

while read enc x1 exp; do
    ((TOTAL++))
    
    x1_dec=$(python3 -c "print(int('$x1', 16))")
    exp_dec=$(python3 -c "print(int('$exp', 16))")
    
    cat > "$TEMP_DIR/t.S" << EOF
.global _start
_start:
    mov x1, #$((x1_dec & 0xFFFF))
    movk x1, #$(((x1_dec >> 16) & 0xFFFF)), lsl #16
    movk x1, #$(((x1_dec >> 32) & 0xFFFF)), lsl #32
    movk x1, #$(((x1_dec >> 48) & 0xFFFF)), lsl #48
    mov x0, #0
    .word $enc
1:  wfe
    b 1b
EOF
    
    clang -target aarch64-unknown-none-elf -nostdlib -Wl,-Ttext=0x40000000 \
        -o "$TEMP_DIR/t.elf" "$TEMP_DIR/t.S" 2>/dev/null
    llvm-objcopy -O binary "$TEMP_DIR/t.elf" "$TEMP_DIR/t.bin" 2>/dev/null
    
    out=$(timeout 1s $QEMU -M virt -cpu cortex-a72 -nographic -d cpu \
        -kernel "$TEMP_DIR/t.bin" 2>&1)
    actual_hex=$(echo "$out" | grep -oE 'X00=[0-9a-fA-F]+' | tail -3 | head -1 | cut -d= -f2)
    actual_dec=$(python3 -c "print(int('${actual_hex:-0}', 16))")
    
    if [[ "$actual_dec" == "$exp_dec" ]]; then
        ((PASSED++))
        echo -ne "\rPassed: $PASSED/$TOTAL"
    else
        ((FAILED++))
        echo -e "\nFAIL #$TOTAL: $enc x1=$x1 exp=$exp got=0x$(printf '%X' $actual_dec)"
    fi
done < /tmp/qemu_tests.txt

echo -e "\n\n=== FINAL RESULTS ==="
echo "Total:  $TOTAL"
echo "Passed: $PASSED"
echo "Failed: $FAILED"

rm -rf "$TEMP_DIR"
