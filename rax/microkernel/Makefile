# Microkernel Build System
#
# Targets:
#   make           - Build bare-metal version for rax emulator
#   make usermode  - Build usermode version for Intel SDE
#   make all       - Build both versions
#   make test-sde  - Run usermode version under Intel SDE (requires SDE installed)
#   make test-rax  - Run bare-metal version under rax emulator
#   make clean     - Clean build artifacts

.PHONY: all baremetal usermode test-sde test-rax clean

OBJCOPY ?= $(shell command -v objcopy 2>/dev/null || command -v llvm-objcopy 2>/dev/null)
ifeq ($(OBJCOPY),)
$(error objcopy not found; install binutils or llvm-objcopy)
endif

# Default: build bare-metal
all: baremetal usermode

baremetal: microkernel.bin

usermode: target/x86_64-unknown-linux-gnu/release/microkernel

microkernel.bin: target/x86_64-unknown-none/release/microkernel
	$(OBJCOPY) -O binary $< $@

target/x86_64-unknown-none/release/microkernel: src/main.rs Cargo.toml linker.ld
	cargo build --release -Z build-std=core -Z build-std-features=compiler-builtins-mem

target/x86_64-unknown-linux-gnu/release/microkernel: src/main.rs Cargo.toml
	cargo build --release --features usermode --target x86_64-unknown-linux-gnu

# Run usermode version directly (for quick testing)
test-native: usermode
	./target/x86_64-unknown-linux-gnu/release/microkernel

# Run under Intel SDE (requires SDE_PATH environment variable or sde64 in PATH)
test-sde: usermode
	@if command -v sde64 >/dev/null 2>&1; then \
		sde64 -- ./target/x86_64-unknown-linux-gnu/release/microkernel; \
	elif [ -n "$$SDE_PATH" ]; then \
		$$SDE_PATH/sde64 -- ./target/x86_64-unknown-linux-gnu/release/microkernel; \
	else \
		echo "Error: Intel SDE not found. Set SDE_PATH or add sde64 to PATH"; \
		exit 1; \
	fi

# Run under rax emulator
test-rax: baremetal
	cd .. && cargo run --release --bin rax -- --backend emulator --vcpus 1 --memory 256M --kernel microkernel/microkernel.bin

clean:
	cargo clean
	rm -f microkernel.bin
