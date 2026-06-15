# Rax Emulator Makefile
#
# Targets:
#   build          - Build rax library and binaries (release)
#   build-debug    - Build rax in debug mode
#   test/tests     - Run ALL tests (unit, hexagon, asm, x86_64-suite)
#   test-quick     - Run tests without x86_64-suite (faster)
#   microkernel    - Build the bare-metal microkernel
#   run-microkernel- Build and run the microkernel in the emulator
#   linux          - Fetch and build Linux kernel (uncompressed vmlinux)
#   run-linux      - Run a Linux kernel in the emulator
#   clean          - Clean all build artifacts
#   clean-linux    - Remove fetched Linux source
#   help           - Show this help message

# Linux kernel configuration
LINUX_VERSION ?= v6.12
LINUX_DIR     := linux/kernel/linux
LINUX_VMLINUX := linux/vmlinux
NPROC         := $(shell nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo 4)

.PHONY: all build build-debug pgo bench test tests test-quick microkernel run-microkernel linux run-linux clean clean-linux help

# Default target
all: build

# Build rax in release mode
build:
	cargo build --release

# Build rax in debug mode
build-debug:
	cargo build

# Profile-guided optimization build (~+20% interpreter throughput).
# Produces target/release/rax optimized for the build host (target-cpu=native).
# For a portable PGO build: PGO_TARGET_CPU=x86-64-v3 make pgo
pgo:
	@bash scripts/pgo-build.sh

# Build + run the interpreter throughput benchmarks (MIPS).
bench:
	RUSTFLAGS="-C target-cpu=native" cargo build --release --example bench_loop --example bench_mem
	./target/release/examples/bench_loop
	./target/release/examples/bench_mem

# Run all tests (unit tests, integration tests, hexagon tests, and x86_64-suite)
test:
	cargo test --release --features x86_64-suite -- --include-ignored

# Alias for test
tests: test

# Run tests without the full x86_64 instruction suite (faster)
test-quick:
	cargo test --release

# Build the bare-metal microkernel
microkernel: microkernel/microkernel.bin

microkernel/microkernel.bin: microkernel/src/main.rs microkernel/Cargo.toml microkernel/linker.ld
	cd microkernel && cargo +nightly build --release
	llvm-objcopy -O binary microkernel/target/x86_64-unknown-none/release/microkernel microkernel/microkernel.bin
	@echo "Built microkernel/microkernel.bin ($$(stat -c%s microkernel/microkernel.bin 2>/dev/null || stat -f%z microkernel/microkernel.bin) bytes)"

# Build and run the microkernel in the emulator
run-microkernel: microkernel
	cargo run --release --no-default-features --example run_microkernel

# Fetch Linux kernel source
$(LINUX_DIR):
	@mkdir -p linux/kernel
	git clone --depth 1 --branch $(LINUX_VERSION) https://github.com/torvalds/linux.git $(LINUX_DIR)

# Build uncompressed Linux kernel (vmlinux)
$(LINUX_VMLINUX): $(LINUX_DIR)
	@echo "Configuring Linux kernel..."
	cd $(LINUX_DIR) && make defconfig
	@echo "Disabling kernel compression..."
	cd $(LINUX_DIR) && ./scripts/config --disable CONFIG_KERNEL_GZIP
	cd $(LINUX_DIR) && ./scripts/config --disable CONFIG_KERNEL_BZIP2
	cd $(LINUX_DIR) && ./scripts/config --disable CONFIG_KERNEL_LZMA
	cd $(LINUX_DIR) && ./scripts/config --disable CONFIG_KERNEL_XZ
	cd $(LINUX_DIR) && ./scripts/config --disable CONFIG_KERNEL_LZO
	cd $(LINUX_DIR) && ./scripts/config --disable CONFIG_KERNEL_LZ4
	cd $(LINUX_DIR) && ./scripts/config --disable CONFIG_KERNEL_ZSTD
	cd $(LINUX_DIR) && ./scripts/config --enable CONFIG_KERNEL_UNCOMPRESSED
	cd $(LINUX_DIR) && make olddefconfig
	@echo "Building Linux kernel (this may take a while)..."
	cd $(LINUX_DIR) && make -j$(NPROC) vmlinux
	cp $(LINUX_DIR)/vmlinux $(LINUX_VMLINUX)
	@echo "Built $(LINUX_VMLINUX) ($$(stat -c%s $(LINUX_VMLINUX) 2>/dev/null || stat -f%z $(LINUX_VMLINUX)) bytes)"

# Convenience target to fetch and build Linux
linux: $(LINUX_VMLINUX)

# Run the bundled/local Linux kernel through the software emulator.
run-linux:
	./run.sh

# Clean all build artifacts (preserves Linux source)
clean:
	cargo clean
	cd microkernel && cargo clean
	rm -f microkernel/microkernel.bin

# Remove fetched Linux source and built kernel
clean-linux:
	rm -rf linux/kernel/linux
	rm -f $(LINUX_VMLINUX)

# Show help
help:
	@echo "Rax Emulator - Available targets:"
	@echo ""
	@echo "  make build           - Build rax library (release mode)"
	@echo "  make build-debug     - Build rax library (debug mode)"
	@echo "  make test            - Run ALL tests (unit, hexagon, asm, x86_64-suite)"
	@echo "  make tests           - Alias for 'make test'"
	@echo "  make test-quick      - Run tests without x86_64-suite (faster)"
	@echo "  make microkernel     - Build the bare-metal microkernel binary"
	@echo "  make run-microkernel - Build and run microkernel in emulator"
	@echo "  make linux           - Fetch and build Linux kernel (uncompressed vmlinux)"
	@echo "  make run-linux       - Run the bundled/local Linux kernel in emulator"
	@echo "  make clean           - Clean build artifacts (preserves Linux source)"
	@echo "  make clean-linux     - Remove fetched Linux source"
	@echo "  make help            - Show this help message"
	@echo ""
	@echo "Linux kernel options:"
	@echo "  LINUX_VERSION=v6.12  - Kernel version to fetch (default: v6.12)"
	@echo ""
	@echo "Examples:"
	@echo "  make build test      - Build and run all tests"
	@echo "  make run-microkernel - Quick demo of the emulator"
	@echo "  make linux           - Fetch and build uncompressed Linux kernel"
	@echo "  make run-linux       - Build and run Linux in emulator"
	@echo "  make linux LINUX_VERSION=v6.6 - Build a specific kernel version"
