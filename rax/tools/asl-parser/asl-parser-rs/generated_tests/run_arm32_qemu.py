#!/usr/bin/env python3
"""Run ARM32 (A32/T32/T16) tests against QEMU.

This script validates A32, T32, and T16 instruction oracle tests.
It requires either:
- qemu-arm (user-mode emulation) - simpler, faster
- qemu-system-arm (full system emulation) - more complex setup

Usage:
    python3 run_arm32_qemu.py <test_file> [iset]

    iset: A32, T32, or T16 (default: all ARM32 tests)

Example:
    # Extract A32 oracle tests
    python3 extract_all_tests.py generated_tests.rs A32 2>&1 | grep "^ORACLE" > /tmp/a32_tests.txt

    # Run A32 tests
    python3 run_arm32_qemu.py /tmp/a32_tests.txt
"""

import subprocess
import sys
import re
import tempfile
import os


def check_qemu():
    """Check which QEMU is available."""
    for qemu in ["qemu-arm", "qemu-arm-static"]:
        try:
            subprocess.run([qemu, "--version"], capture_output=True, check=True)
            return qemu, "user"
        except (subprocess.CalledProcessError, FileNotFoundError):
            pass

    try:
        subprocess.run(
            ["qemu-system-arm", "--version"], capture_output=True, check=True
        )
        return "qemu-system-arm", "system"
    except (subprocess.CalledProcessError, FileNotFoundError):
        pass

    return None, None


def load_tests(filepath, iset_filter=None):
    """Load ARM32 tests from file."""
    tests = []
    with open(filepath) as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith("#"):
                continue
            parts = line.split()
            if len(parts) >= 4:
                test_type = parts[0]
                enc = int(parts[1], 16)
                r1 = int(parts[2], 16)
                r2 = int(parts[3], 16)

                # Get iset (last field)
                iset = parts[-1] if parts[-1] in ("A32", "T32", "T16") else "A32"

                if iset_filter and iset != iset_filter:
                    continue

                if test_type == "ORACLE" and len(parts) >= 7:
                    tests.append(
                        {
                            "type": "ORACLE",
                            "enc": enc,
                            "r1": r1,
                            "r2": r2,
                            "result_reg": int(parts[4]),
                            "expected": int(parts[5], 16),
                            "width": int(parts[6]),
                            "iset": iset,
                        }
                    )
                elif test_type in ("PASS", "FAULT"):
                    tests.append(
                        {
                            "type": test_type,
                            "enc": enc,
                            "r1": r1,
                            "r2": r2,
                            "iset": iset,
                        }
                    )
    return tests


def mov32(reg, val):
    """Generate ARM32 instructions to load a 32-bit value into a register."""
    val = val & 0xFFFFFFFF
    lines = []
    # Use MOV + MOVT for 32-bit value
    lines.append(f"    movw r{reg}, #{val & 0xFFFF}")
    if val > 0xFFFF:
        lines.append(f"    movt r{reg}, #{(val >> 16) & 0xFFFF}")
    return "\n".join(lines)


def run_single_test_usermode(test, idx, qemu_cmd):
    """Run a single test using user-mode QEMU."""
    iset = test.get("iset", "A32")

    # Generate assembly
    lines = [
        ".syntax unified",
        ".arch armv7-a",
        ".global _start",
    ]

    if iset in ("T32", "T16"):
        lines.append(".thumb")
    else:
        lines.append(".arm")

    lines.extend(
        [
            "_start:",
            mov32(1, test["r1"]),
            mov32(2, test["r2"]),
            "    mov r0, #0",
            "    mov r3, #0",  # Sentinel
        ]
    )

    # The instruction under test
    if iset == "T16":
        lines.append(f"    .hword 0x{test['enc'] & 0xFFFF:04X}")
    else:
        lines.append(f"    .word 0x{test['enc']:08X}")

    # If we get here, instruction didn't fault
    lines.append("    mov r3, #1")  # Mark success

    # Store result for oracle tests
    if test["type"] == "ORACLE":
        reg = test["result_reg"]
        lines.append(f"    mov r4, r{reg}")

    # Exit via syscall
    lines.append("    mov r7, #1")  # SYS_EXIT
    lines.append("    mov r0, r3")  # Exit code = sentinel
    lines.append("    svc #0")

    asm_code = "\n".join(lines)

    with tempfile.TemporaryDirectory() as tmpdir:
        asm_path = os.path.join(tmpdir, "test.S")
        elf_path = os.path.join(tmpdir, "test.elf")

        with open(asm_path, "w") as f:
            f.write(asm_code)

        try:
            # Compile
            subprocess.run(
                [
                    "arm-linux-gnueabi-gcc"
                    if iset == "A32"
                    else "arm-linux-gnueabihf-gcc",
                    "-nostdlib",
                    "-static",
                    "-o",
                    elf_path,
                    asm_path,
                ],
                check=True,
                capture_output=True,
            )
        except subprocess.CalledProcessError:
            return {"idx": idx, "pass": False, "error": "compile", "test": test}
        except FileNotFoundError:
            return {"idx": idx, "pass": False, "error": "no_compiler", "test": test}

        # Run
        result = subprocess.run(
            ["timeout", "0.5s", qemu_cmd, elf_path], capture_output=True
        )

        exit_code = result.returncode

        if test["type"] == "PASS":
            return {"idx": idx, "pass": exit_code == 1, "test": test}
        elif test["type"] == "FAULT":
            return {"idx": idx, "pass": exit_code != 1, "test": test}
        elif test["type"] == "ORACLE":
            if exit_code != 1:
                return {"idx": idx, "pass": False, "error": "faulted", "test": test}
            # For oracle tests with user-mode, we'd need to extract r4
            # This requires more complex output parsing
            return {
                "idx": idx,
                "pass": False,
                "error": "oracle_not_implemented",
                "test": test,
            }

        return {"idx": idx, "pass": False, "error": "unknown", "test": test}


def main():
    qemu_cmd, mode = check_qemu()

    if not qemu_cmd:
        print("ERROR: No ARM32 QEMU found.")
        print("Install one of:")
        print("  - qemu-arm (user-mode, recommended)")
        print("  - qemu-arm-static")
        print("  - qemu-system-arm (system emulation)")
        print()
        print("On macOS with Homebrew:")
        print("  brew install qemu")
        print()
        print("On Ubuntu/Debian:")
        print("  apt install qemu-user qemu-user-static")
        sys.exit(1)

    print(f"Using {qemu_cmd} ({mode} mode)")

    test_file = sys.argv[1] if len(sys.argv) > 1 else "/tmp/arm32_tests.txt"
    iset_filter = sys.argv[2].upper() if len(sys.argv) > 2 else None

    tests = load_tests(test_file, iset_filter)

    print(f"Loaded {len(tests)} tests")

    if mode == "user":
        passed = 0
        failed = 0
        for i, test in enumerate(tests):
            result = run_single_test_usermode(test, i, qemu_cmd)
            if result["pass"]:
                passed += 1
            else:
                failed += 1
            if (i + 1) % 100 == 0:
                print(
                    f"\r[{i + 1}/{len(tests)}] Passed: {passed}, Failed: {failed}",
                    end="",
                    flush=True,
                )

        print(
            f"\n\nResults: {passed}/{len(tests)} passed ({100 * passed / len(tests):.1f}%)"
        )
    else:
        print("System emulation mode not yet implemented.")
        print("For now, use A64 tests which are validated with qemu-system-aarch64.")
        sys.exit(1)


if __name__ == "__main__":
    main()
