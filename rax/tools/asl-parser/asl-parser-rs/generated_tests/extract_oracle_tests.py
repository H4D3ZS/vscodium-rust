#!/usr/bin/env python3
"""Extract oracle tests from generated_tests.rs for QEMU validation."""

import re
import sys


def extract_tests(filepath):
    """Extract oracle tests from generated Rust test file."""
    with open(filepath) as f:
        content = f.read()

    # Split into individual test functions
    test_blocks = re.split(r"#\[test\]\s*\n\s*fn\s+", content)

    tests = []
    for block in test_blocks[1:]:  # Skip the first empty split
        # Only process oracle tests
        if "oracle" not in block.lower():
            continue

        # Extract test name
        name_match = re.match(r"(\w+)", block)
        if not name_match:
            continue
        test_name = name_match.group(1)

        # Extract encoding
        enc_match = re.search(r"let encoding: u32 = (0x[0-9A-Fa-f]+);", block)
        if not enc_match:
            continue
        encoding = enc_match.group(1)

        # Extract all set_x calls
        set_x_pattern = re.compile(r"set_x\(&mut cpu, (\d+), (0x[0-9A-Fa-f]+)\)")
        registers = {}
        for match in set_x_pattern.finditer(block):
            reg_num = int(match.group(1))
            value = match.group(2)
            registers[reg_num] = value

        # Extract expected result - prefer get_x over get_w for 64-bit tests
        result_match = re.search(
            r"assert_eq!\(get_x\(&cpu, (\d+)\), (0x[0-9A-Fa-f]+)", block
        )
        if result_match:
            result_reg = int(result_match.group(1))
            expected = result_match.group(2)
            is_64bit = True
        else:
            # Try 32-bit
            result_match = re.search(
                r"assert_eq!\(get_w\(&cpu, (\d+)\), (0x[0-9A-Fa-f]+)", block
            )
            if result_match:
                result_reg = int(result_match.group(1))
                expected = result_match.group(2)
                is_64bit = False
            else:
                continue

        # Only include 64-bit tests for now (easier QEMU validation)
        if not is_64bit:
            continue

        tests.append(
            {
                "name": test_name,
                "encoding": encoding,
                "registers": registers,
                "result_reg": result_reg,
                "expected": expected,
            }
        )

    return tests


def main():
    if len(sys.argv) < 2:
        filepath = "generated_tests.rs"
    else:
        filepath = sys.argv[1]

    tests = extract_tests(filepath)

    # Remove duplicates while preserving order
    seen = set()
    unique_tests = []
    for t in tests:
        # Create key from encoding + all register values + expected
        reg_key = tuple(sorted(t["registers"].items()))
        key = (t["encoding"], reg_key, t["expected"])
        if key not in seen:
            seen.add(key)
            unique_tests.append(t)

    print(f"# Extracted {len(unique_tests)} unique oracle tests", file=sys.stderr)

    # Output format: encoding x1_val x2_val expected
    # (where x1_val and x2_val default to 0 if not set)
    for t in unique_tests:
        x1 = t["registers"].get(1, "0x0")
        x2 = t["registers"].get(2, "0x0")
        print(f"{t['encoding']} {x1} {x2} {t['expected']}")


if __name__ == "__main__":
    main()
