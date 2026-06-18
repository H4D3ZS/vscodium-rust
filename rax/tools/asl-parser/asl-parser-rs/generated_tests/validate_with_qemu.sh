#!/bin/bash
# Extract ALL oracle tests from generated_tests.rs and validate with QEMU

TEMP_DIR="/tmp/qemu-validate"
QEMU="qemu-system-aarch64"
mkdir -p "$TEMP_DIR"
PASSED=0; FAILED=0; TOTAL=0

run_test() {
    local encoding="$1" x1="$2" expected="$3"
    ((TOTAL++))
    
    cat > "$TEMP_DIR/t.S" << EOF
.global _start
_start:
    mov x1, #$(($x1 & 0xFFFF))
    movk x1, #$((($x1 >> 16) & 0xFFFF)), lsl #16
    movk x1, #$((($x1 >> 32) & 0xFFFF)), lsl #32
    movk x1, #$((($x1 >> 48) & 0xFFFF)), lsl #48
    mov x0, #0
    .word $encoding
1:  wfe
    b 1b
EOF
    
    clang -target aarch64-unknown-none-elf -nostdlib -Wl,-Ttext=0x40000000 \
        -o "$TEMP_DIR/t.elf" "$TEMP_DIR/t.S" 2>/dev/null || return
    llvm-objcopy -O binary "$TEMP_DIR/t.elf" "$TEMP_DIR/t.bin" 2>/dev/null
    
    local out=$(timeout 1s $QEMU -M virt -cpu cortex-a72 -nographic -d cpu \
        -kernel "$TEMP_DIR/t.bin" 2>&1)
    local actual_hex=$(echo "$out" | grep -oE 'X00=[0-9a-fA-F]+' | tail -3 | head -1 | cut -d= -f2)
    local actual=$((16#${actual_hex:-0}))
    
    if [[ "$actual" -eq "$expected" ]]; then
        ((PASSED++))
    else
        echo "FAIL: enc=$encoding x1=$x1 exp=$expected got=$actual"
        ((FAILED++))
    fi
}

echo "=== Running ALL Oracle Tests Against QEMU ==="
echo "Extracting tests from generated_tests.rs..."

# Extract oracle tests: find set_x(&mut cpu, 1, VALUE) and assert_eq!(get_x(&cpu, 0), EXPECTED)
# along with the encoding
grep -B5 "assert_eq!(get_x(&cpu, 0)" generated_tests.rs | \
grep -E "(0x[0-9a-fA-F]+;|set_x.*1,|get_x.*0\),)" | \
paste - - - 2>/dev/null | \
while read line; do
    # Extract encoding (first hex value ending with ;)
    enc=$(echo "$line" | grep -oE '0x[0-9A-Fa-f]+;' | head -1 | tr -d ';')
    # Extract initial X1 value
    x1=$(echo "$line" | grep -oE 'set_x\(&mut cpu, 1, 0x[0-9A-Fa-f]+\)' | grep -oE '0x[0-9A-Fa-f]+' | tail -1)
    # Extract expected X0 value  
    exp=$(echo "$line" | grep -oE 'get_x\(&cpu, 0\), 0x[0-9A-Fa-f]+' | grep -oE '0x[0-9A-Fa-f]+')
    
    if [[ -n "$enc" && -n "$x1" && -n "$exp" ]]; then
        run_test "$enc" "$x1" "$exp"
    fi
done

echo ""
echo "=== RESULTS ==="
echo "Total:  $TOTAL"
echo "Passed: $PASSED"
echo "Failed: $FAILED"

rm -rf "$TEMP_DIR"
