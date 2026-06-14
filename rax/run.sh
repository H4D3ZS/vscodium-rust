#!/usr/bin/env bash
# Build and boot the bundled Linux image on the software x86-64 emulator.

set -euo pipefail

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
cd "$SCRIPT_DIR"

KERNEL=${RAX_KERNEL:-"$SCRIPT_DIR/linux/vmlinux"}
INITRD=${RAX_INITRD:-"$SCRIPT_DIR/initrd.cpio.gz"}
ARCH=${RAX_ARCH:-x86-64}
BACKEND=${RAX_BACKEND:-emulator}
MEMORY=${RAX_MEMORY:-512M}
CMDLINE=${RAX_CMDLINE:-"console=ttyS0 earlycon=uart,io,0x3f8 earlyprintk=serial,ttyS0,115200 rdinit=/init root=/dev/ram0 nokaslr noapic nolapic nosmp mitigations=off tsc=reliable nohz=off clocksource=tsc"}

if [[ ! -f "$KERNEL" ]]; then
    echo "kernel not found: $KERNEL" >&2
    echo "Set RAX_KERNEL=/path/to/vmlinux or build/provide linux/vmlinux." >&2
    exit 1
fi

if [[ ! -f "$INITRD" ]]; then
    echo "initrd not found: $INITRD" >&2
    echo "Set RAX_INITRD=/path/to/initrd.cpio.gz or provide initrd.cpio.gz." >&2
    exit 1
fi

cargo build --release --bin rax

if [[ -t 0 && -t 1 ]]; then
    SAVED_STTY=$(stty -g)
    cleanup() {
        stty "$SAVED_STTY"
        echo
        echo "VM exited."
    }
    trap cleanup EXIT
    stty raw -echo
fi

"$SCRIPT_DIR/target/release/rax" \
    --arch "$ARCH" \
    --backend "$BACKEND" \
    --memory "$MEMORY" \
    --kernel "$KERNEL" \
    --initrd "$INITRD" \
    --cmdline "$CMDLINE"
