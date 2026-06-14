#!/usr/bin/env python3
"""Run all oracle tests in a single QEMU invocation."""

import subprocess
import sys
import re
import tempfile
import os


def load_tests(filepath):
    """Load tests from file: encoding x1 x2 expected"""
    tests = []
    with open(filepath) as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith("#"):
                continue
            parts = line.split()
            if len(parts) == 4:
                tests.append(
                    {
                        "enc": int(parts[0], 16),
                        "x1": int(parts[1], 16),
                        "x2": int(parts[2], 16),
                        "expected": int(parts[3], 16),
                    }
                )
    return tests


def mov64(reg, val):
    """Generate instructions to load a 64-bit value into a register."""
    lines = []
    lines.append(f"    mov x{reg}, #{val & 0xFFFF}")
    if val > 0xFFFF:
        lines.append(f"    movk x{reg}, #{(val >> 16) & 0xFFFF}, lsl #16")
    if val > 0xFFFFFFFF:
        lines.append(f"    movk x{reg}, #{(val >> 32) & 0xFFFF}, lsl #32")
    if val > 0xFFFFFFFFFFFF:
        lines.append(f"    movk x{reg}, #{(val >> 48) & 0xFFFF}, lsl #48")
    return "\n".join(lines)


def generate_batch_asm(tests):
    """Generate assembly that runs all tests and stores results."""
    lines = [
        ".global _start",
        "_start:",
        "    // Results stored at 0x50000000",
        f"    mov x10, #0x5000",
        "    lsl x10, x10, #16  // x10 = 0x50000000 (result base)",
        "",
    ]

    for i, t in enumerate(tests):
        lines.append(f"    // Test {i}: enc=0x{t['enc']:08X}")
        lines.append(mov64(1, t["x1"]))
        lines.append(mov64(2, t["x2"]))
        lines.append(f"    mov x0, #0")
        lines.append(f"    .word 0x{t['enc']:08X}")
        # Store result
        lines.append(f"    str x0, [x10, #{i * 8}]")
        lines.append("")

    # Halt
    lines.append("1:  wfe")
    lines.append("    b 1b")

    return "\n".join(lines)


def run_qemu(asm_code, num_tests):
    """Compile and run in QEMU, return results array."""
    with tempfile.TemporaryDirectory() as tmpdir:
        asm_path = os.path.join(tmpdir, "test.S")
        elf_path = os.path.join(tmpdir, "test.elf")
        bin_path = os.path.join(tmpdir, "test.bin")

        with open(asm_path, "w") as f:
            f.write(asm_code)

        # Compile
        subprocess.run(
            [
                "clang",
                "-target",
                "aarch64-unknown-none-elf",
                "-nostdlib",
                "-Wl,-Ttext=0x40000000",
                "-o",
                elf_path,
                asm_path,
            ],
            check=True,
            capture_output=True,
        )

        subprocess.run(
            ["llvm-objcopy", "-O", "binary", elf_path, bin_path],
            check=True,
            capture_output=True,
        )

        # Run QEMU with memory dump
        # We'll use -d cpu to get final state and parse memory
        result = subprocess.run(
            [
                "timeout",
                "2s",
                "qemu-system-aarch64",
                "-M",
                "virt",
                "-cpu",
                "cortex-a72",
                "-nographic",
                "-d",
                "cpu",
                "-kernel",
                bin_path,
            ],
            capture_output=True,
            text=True,
        )

        output = result.stderr + result.stdout

        # Parse X10 to find result base, then look for memory
        # Actually easier: just look at the last CPU state dump
        # The results are stored sequentially from 0x50000000

        # For now, let's use a different approach - run QEMU with GDB
        # Or simpler: dump individual register states

        # Let's parse the last few register dumps
        # Find all X00= values and take the ones after test execution

        # Actually, the results are in memory. Let's use QEMU monitor to dump.
        # Simpler approach: run fewer instructions and check X0 after each

        return output


def run_tests_individually_fast(tests):
    """Run tests in small batches for speed."""
    BATCH_SIZE = 20
    results = []

    for batch_start in range(0, len(tests), BATCH_SIZE):
        batch = tests[batch_start : batch_start + BATCH_SIZE]
        batch_results = run_single_batch(batch, batch_start)
        results.extend(batch_results)

        passed = sum(1 for r in results if r["pass"])
        failed = len(results) - passed
        print(
            f"\r[{len(results)}/{len(tests)}] Passed: {passed}, Failed: {failed}",
            end="",
            flush=True,
        )

    return results


def run_single_batch(tests, start_idx):
    """Run a batch of tests, each storing result then moving to next."""
    lines = [
        ".global _start",
        "_start:",
    ]

    for i, t in enumerate(tests):
        lines.append(f"test_{i}:")
        lines.append(mov64(1, t["x1"]))
        lines.append(mov64(2, t["x2"]))
        lines.append(f"    mov x0, #0")
        lines.append(f"    .word 0x{t['enc']:08X}")
        # Copy result to x{20+i} for later inspection (x20-x28 available)
        if i < 9:
            lines.append(f"    mov x{20 + i}, x0")

    lines.append("1:  wfe")
    lines.append("    b 1b")

    asm_code = "\n".join(lines)

    with tempfile.TemporaryDirectory() as tmpdir:
        asm_path = os.path.join(tmpdir, "test.S")
        elf_path = os.path.join(tmpdir, "test.elf")
        bin_path = os.path.join(tmpdir, "test.bin")

        with open(asm_path, "w") as f:
            f.write(asm_code)

        try:
            subprocess.run(
                [
                    "clang",
                    "-target",
                    "aarch64-unknown-none-elf",
                    "-nostdlib",
                    "-Wl,-Ttext=0x40000000",
                    "-o",
                    elf_path,
                    asm_path,
                ],
                check=True,
                capture_output=True,
            )

            subprocess.run(
                ["llvm-objcopy", "-O", "binary", elf_path, bin_path],
                check=True,
                capture_output=True,
            )
        except subprocess.CalledProcessError as e:
            return [{"pass": False, "error": "compile"} for _ in tests]

        result = subprocess.run(
            [
                "timeout",
                "1s",
                "qemu-system-aarch64",
                "-M",
                "virt",
                "-cpu",
                "cortex-a72",
                "-nographic",
                "-d",
                "cpu",
                "-kernel",
                bin_path,
            ],
            capture_output=True,
            text=True,
        )

        output = result.stderr + result.stdout

        # Parse register values from last CPU dump
        # Find all register dumps and use the last complete one
        dumps = re.findall(
            r"X00=([0-9a-fA-F]+).*?X20=([0-9a-fA-F]+).*?X21=([0-9a-fA-F]+).*?X22=([0-9a-fA-F]+).*?X23=([0-9a-fA-F]+).*?X24=([0-9a-fA-F]+).*?X25=([0-9a-fA-F]+).*?X26=([0-9a-fA-F]+).*?X27=([0-9a-fA-F]+).*?X28=([0-9a-fA-F]+)",
            output,
            re.DOTALL,
        )

        if not dumps:
            return [{"pass": False, "error": "no output"} for _ in tests]

        last_dump = dumps[-1]
        # x20-x28 hold results for first 9 tests
        reg_values = [int(last_dump[i + 1], 16) for i in range(min(9, len(tests)))]

        results = []
        for i, t in enumerate(tests):
            if i < len(reg_values):
                actual = reg_values[i]
                passed = actual == t["expected"]
                results.append(
                    {
                        "pass": passed,
                        "actual": actual,
                        "expected": t["expected"],
                        "enc": t["enc"],
                    }
                )
            else:
                results.append({"pass": False, "error": "no reg"})

        return results


def main():
    test_file = sys.argv[1] if len(sys.argv) > 1 else "/tmp/qemu_tests.txt"
    tests = load_tests(test_file)

    print(f"=== QEMU Batch Test Runner ===")
    print(f"Loaded {len(tests)} tests")
    print()

    # Run in batches of 9 (using x20-x28 for results)
    BATCH_SIZE = 9
    all_results = []

    for batch_start in range(0, len(tests), BATCH_SIZE):
        batch = tests[batch_start : batch_start + BATCH_SIZE]
        batch_results = run_single_batch(batch, batch_start)
        all_results.extend(batch_results)

        passed = sum(1 for r in all_results if r.get("pass"))
        failed = len(all_results) - passed
        print(
            f"\r[{len(all_results)}/{len(tests)}] Passed: {passed}, Failed: {failed}",
            end="",
            flush=True,
        )

    print("\n")

    passed = sum(1 for r in all_results if r.get("pass"))
    failed = len(all_results) - passed

    print("=== FINAL RESULTS ===")
    print(f"Total:  {len(all_results)}")
    print(f"Passed: {passed}")
    print(f"Failed: {failed}")

    if failed > 0:
        print("\nFailures:")
        for i, r in enumerate(all_results):
            if not r.get("pass"):
                if "error" in r:
                    print(f"  #{i + 1}: {r['error']}")
                else:
                    print(
                        f"  #{i + 1}: enc=0x{r['enc']:08X} expected=0x{r['expected']:X} actual=0x{r['actual']:X}"
                    )


if __name__ == "__main__":
    main()
