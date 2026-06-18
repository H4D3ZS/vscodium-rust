#!/usr/bin/env python3
"""Extract ALL tests from generated_tests.rs for QEMU validation."""

import re
import sys


def extract_tests(filepath):
    """Extract all tests from generated Rust test file."""
    with open(filepath) as f:
        content = f.read()

    # Split into individual test functions
    test_blocks = re.split(r"#\[test\]\s*\n\s*fn\s+", content)

    tests = []
    for block in test_blocks[1:]:  # Skip the first empty split
        # Extract test name
        name_match = re.match(r"(\w+)", block)
        if not name_match:
            continue
        test_name = name_match.group(1)

        # Determine instruction set from ISET comment
        iset = "A64"  # Default
        if "// ISET: A32" in block:
            iset = "A32"
        elif "// ISET: T32" in block:
            iset = "T32"
        elif "// ISET: T16" in block:
            iset = "T16"
        # Skip old-style Armv7 tests (not updated yet)
        elif "Armv7Cpu" in block:
            continue

        # Extract encoding (handle both u32 and u16)
        enc_match = re.search(r"let encoding: u32 = (0x[0-9A-Fa-f]+);", block)
        if not enc_match:
            enc_match = re.search(r"let encoding: u16 = (0x[0-9A-Fa-f]+);", block)
        if not enc_match:
            continue
        encoding = enc_match.group(1)

        # Extract all set_x calls for initial state (64-bit)
        set_x_pattern = re.compile(r"set_x\(&mut cpu, (\d+), (0x[0-9A-Fa-f]+)\)")
        registers = {}
        for match in set_x_pattern.finditer(block):
            reg_num = int(match.group(1))
            value = match.group(2)
            registers[reg_num] = value

        # Also extract set_w calls for A32/T32/T16 (32-bit)
        set_w_pattern = re.compile(r"set_w\(&mut cpu, (\d+), (0x[0-9A-Fa-f]+)\)")
        for match in set_w_pattern.finditer(block):
            reg_num = int(match.group(1))
            value = match.group(2)
            registers[reg_num] = value

        # Determine expected result type
        if "expected UNDEFINED" in block or "expected unallocated" in block:
            result_type = "FAULT"
        elif "expected exception" in block:
            result_type = "FAULT"
        elif "UNPREDICTABLE" in block:
            result_type = "SKIP"  # Can't reliably test these
        else:
            result_type = "PASS"

        # For oracle tests, extract expected value
        expected_value = None
        result_reg = None
        is_64bit = True

        result_match = re.search(
            r"assert_eq!\(get_x\(&cpu, (\d+)\), (0x[0-9A-Fa-f]+)", block
        )
        if result_match:
            result_reg = int(result_match.group(1))
            expected_value = result_match.group(2)
            is_64bit = True
        else:
            result_match = re.search(
                r"assert_eq!\(get_w\(&cpu, (\d+)\), (0x[0-9A-Fa-f]+)", block
            )
            if result_match:
                result_reg = int(result_match.group(1))
                expected_value = result_match.group(2)
                is_64bit = False

        tests.append(
            {
                "name": test_name,
                "encoding": encoding,
                "registers": registers,
                "result_type": result_type,
                "result_reg": result_reg,
                "expected_value": expected_value,
                "is_64bit": is_64bit,
                "iset": iset,
            }
        )

    return tests


def main():
    if len(sys.argv) < 2:
        filepath = "generated_tests.rs"
    else:
        filepath = sys.argv[1]

    # Optional iset filter (e.g., "A64", "A32", "T32", "T16")
    iset_filter = sys.argv[2].upper() if len(sys.argv) > 2 else None

    tests = extract_tests(filepath)

    # Apply iset filter if specified
    if iset_filter:
        tests = [t for t in tests if t.get("iset", "A64").upper() == iset_filter]
        print(f"# Filtered to {iset_filter}: {len(tests)} tests", file=sys.stderr)

    # Remove duplicates based on encoding + initial state
    seen = set()
    unique_tests = []
    for t in tests:
        if t["result_type"] == "SKIP":
            continue
        reg_key = tuple(sorted(t["registers"].items()))
        key = (t["encoding"], reg_key, t["result_type"], t.get("expected_value"))
        if key not in seen:
            seen.add(key)
            unique_tests.append(t)

    print(f"# Extracted {len(unique_tests)} unique tests", file=sys.stderr)

    # Output format: TYPE ENCODING X1 X2 [EXPECTED_REG EXPECTED_VAL WIDTH ISET]
    for t in unique_tests:
        x1 = t["registers"].get(1, "0x0")
        x2 = t["registers"].get(2, "0x0")
        iset = t.get("iset", "A64")

        if t["expected_value"] is not None:
            width = "64" if t["is_64bit"] else "32"
            print(
                f"ORACLE {t['encoding']} {x1} {x2} {t['result_reg']} {t['expected_value']} {width} {iset}"
            )
        else:
            print(f"{t['result_type']} {t['encoding']} {x1} {x2} {iset}")


if __name__ == "__main__":
    main()
