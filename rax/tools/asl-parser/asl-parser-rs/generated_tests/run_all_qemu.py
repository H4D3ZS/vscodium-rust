#!/usr/bin/env python3
"""Run ALL tests against QEMU - PASS, FAULT, and ORACLE types."""

import subprocess
import sys
import re
import tempfile
import os
from concurrent.futures import ThreadPoolExecutor, as_completed
import threading

# Thread-local temp directories
local = threading.local()


def load_tests(filepath):
    """Load tests from file."""
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
                x1 = int(parts[2], 16)
                x2 = int(parts[3], 16)

                if test_type == "ORACLE" and len(parts) >= 7:
                    tests.append(
                        {
                            "type": "ORACLE",
                            "enc": enc,
                            "x1": x1,
                            "x2": x2,
                            "result_reg": int(parts[4]),
                            "expected": int(parts[5], 16),
                            "width": int(parts[6]),
                        }
                    )
                elif test_type in ("PASS", "FAULT"):
                    tests.append(
                        {
                            "type": test_type,
                            "enc": enc,
                            "x1": x1,
                            "x2": x2,
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


def run_single_test(test, idx):
    """Run a single test and return result."""
    # Generate assembly for this test
    lines = [
        ".global _start",
        "_start:",
        mov64(1, test["x1"]),
        mov64(2, test["x2"]),
        "    mov x0, #0",
        "    mov x3, #0",  # Sentinel
    ]

    # The instruction under test
    lines.append(f"    .word 0x{test['enc']:08X}")

    # If we get here, instruction didn't fault
    lines.append("    mov x3, #1")  # Mark success

    # Store result for oracle tests
    if test["type"] == "ORACLE":
        reg = test["result_reg"]
        if test["width"] == 32:
            # For 32-bit, read W register
            lines.append(f"    mov w4, w{reg}")
        else:
            lines.append(f"    mov x4, x{reg}")

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
        except subprocess.CalledProcessError:
            return {"idx": idx, "pass": False, "error": "compile", "test": test}

        result = subprocess.run(
            [
                "timeout",
                "0.5s",
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

        # Find X03 (sentinel) and X04 (result) from CPU dumps
        x3_matches = re.findall(r"X03=([0-9a-fA-F]+)", output)
        x4_matches = re.findall(r"X04=([0-9a-fA-F]+)", output)

        if not x3_matches:
            # No CPU dump - likely crashed hard
            if test["type"] == "FAULT":
                return {"idx": idx, "pass": True, "test": test}
            return {"idx": idx, "pass": False, "error": "no_output", "test": test}

        sentinel = int(x3_matches[-1], 16)
        did_execute = sentinel == 1

        if test["type"] == "PASS":
            return {
                "idx": idx,
                "pass": did_execute,
                "test": test,
                "error": "faulted" if not did_execute else None,
            }
        elif test["type"] == "FAULT":
            return {
                "idx": idx,
                "pass": not did_execute,
                "test": test,
                "error": "didnt_fault" if did_execute else None,
            }
        elif test["type"] == "ORACLE":
            if not did_execute:
                return {"idx": idx, "pass": False, "error": "faulted", "test": test}
            if not x4_matches:
                return {"idx": idx, "pass": False, "error": "no_result", "test": test}
            actual = int(x4_matches[-1], 16)
            if test["width"] == 32:
                actual = actual & 0xFFFFFFFF
            passed = actual == test["expected"]
            return {"idx": idx, "pass": passed, "actual": actual, "test": test}

        return {"idx": idx, "pass": False, "error": "unknown_type", "test": test}


def run_batch(tests, start_idx, batch_size=10):
    """Run a batch of tests, each writing sentinel and result to fixed regs."""
    # Limit batch size to avoid register pressure
    batch_size = min(batch_size, len(tests))

    lines = [
        ".global _start",
        "_start:",
        "    // Use memory at 0x50000000 for results",
        "    mov x10, #0x5000",
        "    lsl x10, x10, #16",
        "",
    ]

    for i, t in enumerate(tests[:batch_size]):
        lines.append(f"test_{i}:")
        lines.append(mov64(1, t["x1"]))
        lines.append(mov64(2, t["x2"]))
        lines.append(f"    mov x0, #0")
        lines.append(f"    mov x9, #0")  # Sentinel = 0 before
        lines.append(f"    .word 0x{t['enc']:08X}")
        lines.append(f"    mov x9, #1")  # Sentinel = 1 after (success)
        # Store sentinel
        lines.append(f"    str x9, [x10, #{i * 16}]")
        # Store result (x0 for most instructions)
        lines.append(f"    str x0, [x10, #{i * 16 + 8}]")
        lines.append("")

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
        except subprocess.CalledProcessError:
            return [
                {"idx": start_idx + i, "pass": False, "error": "compile"}
                for i in range(len(tests))
            ]

        # Run with longer timeout for batch
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

        # Parse X10 memory dumps - look for the final register state
        x10_match = re.search(r"X10=([0-9a-fA-F]+)", output)
        if not x10_match:
            return [
                {"idx": start_idx + i, "pass": False, "error": "no_output"}
                for i in range(len(tests))
            ]

        # For now, use simpler approach - look at last CPU state
        # and check if we got past all tests
        pc_matches = re.findall(r"PC=([0-9a-fA-F]+)", output)
        if pc_matches:
            last_pc = int(pc_matches[-1], 16)
            # If PC is at or after the WFE loop, all tests passed
            # Each test is roughly 20-30 instructions

        # Actually, let's just check for faults by looking for exception messages
        has_fault = "exception" in output.lower() or "illegal" in output.lower()

        results = []
        for i, t in enumerate(tests[:batch_size]):
            if t["type"] == "PASS":
                results.append({"idx": start_idx + i, "pass": not has_fault, "test": t})
            elif t["type"] == "FAULT":
                results.append({"idx": start_idx + i, "pass": has_fault, "test": t})
            else:
                results.append(
                    {
                        "idx": start_idx + i,
                        "pass": False,
                        "error": "batch_oracle",
                        "test": t,
                    }
                )

        return results


def main():
    test_file = sys.argv[1] if len(sys.argv) > 1 else "/tmp/all_tests.txt"
    tests = load_tests(test_file)

    print(f"=== QEMU Full Test Runner ===")
    print(f"Loaded {len(tests)} tests")

    # Count by type
    by_type = {}
    for t in tests:
        by_type[t["type"]] = by_type.get(t["type"], 0) + 1
    for typ, count in sorted(by_type.items()):
        print(f"  {typ}: {count}")
    print()

    # Run tests in parallel
    results = [None] * len(tests)
    passed = 0
    failed = 0

    with ThreadPoolExecutor(max_workers=8) as executor:
        futures = {
            executor.submit(run_single_test, t, i): i for i, t in enumerate(tests)
        }

        for future in as_completed(futures):
            idx = futures[future]
            try:
                result = future.result()
                results[idx] = result
                if result["pass"]:
                    passed += 1
                else:
                    failed += 1
            except Exception as e:
                results[idx] = {"idx": idx, "pass": False, "error": str(e)}
                failed += 1

            done = passed + failed
            if done % 100 == 0 or done == len(tests):
                print(
                    f"\r[{done}/{len(tests)}] Passed: {passed}, Failed: {failed}",
                    end="",
                    flush=True,
                )

    print("\n")

    print("=== FINAL RESULTS ===")
    print(f"Total:  {len(tests)}")
    print(f"Passed: {passed}")
    print(f"Failed: {failed}")
    print(f"Rate:   {100 * passed / len(tests):.1f}%")

    # Show failures by type
    if failed > 0:
        print("\nFailures by type:")
        fail_by_type = {}
        for r in results:
            if r and not r["pass"]:
                t = r.get("test", {})
                typ = t.get("type", "unknown")
                fail_by_type[typ] = fail_by_type.get(typ, 0) + 1
        for typ, count in sorted(fail_by_type.items()):
            print(f"  {typ}: {count}")

        print("\nFirst 20 failures:")
        shown = 0
        for r in results:
            if r and not r["pass"] and shown < 20:
                t = r.get("test", {})
                err = r.get("error", "mismatch")
                if "actual" in r:
                    print(
                        f"  enc=0x{t.get('enc', 0):08X} expected=0x{t.get('expected', 0):X} actual=0x{r['actual']:X}"
                    )
                else:
                    print(
                        f"  enc=0x{t.get('enc', 0):08X} type={t.get('type')} error={err}"
                    )
                shown += 1


if __name__ == "__main__":
    main()
