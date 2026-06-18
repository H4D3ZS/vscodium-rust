# Makefile for Rax Boot Test Kernel
#
# This builds a minimal 64-bit kernel that tests the same initialization
# patterns as the Linux kernel but runs much faster.

# Tools
AS = as
LD = ld
OBJCOPY = objcopy

# Flags
ASFLAGS = --64
LDFLAGS = -m elf_x86_64 -T linker.ld --no-relax

# Output
TARGET = boot_test.bin
ELF = boot_test.elf

# Sources
ASM_SOURCES = asm/boot.S

# Objects
OBJECTS = $(ASM_SOURCES:.S=.o)

.PHONY: all clean run

all: $(TARGET)

# Link the ELF
$(ELF): $(OBJECTS) linker.ld
	$(LD) $(LDFLAGS) -o $@ $(OBJECTS)

# Create flat binary
$(TARGET): $(ELF)
	$(OBJCOPY) -O binary $< $@
	@echo "Built $(TARGET) ($(shell stat -c%s $@ 2>/dev/null || stat -f%z $@) bytes)"

# Assemble
asm/%.o: asm/%.S
	$(AS) $(ASFLAGS) -o $@ $<

# Run with rax (adjust path as needed)
run: $(TARGET)
	@echo "Running boot test with rax..."
	cd ../.. && cargo run --release -- \
		--backend emulator \
		--arch x86_64 \
		--memory 128M \
		--kernel tests/boot_test/$(TARGET) \
		--cmdline "console=ttyS0"

# Run with verbose output
run-verbose: $(TARGET)
	@echo "Running boot test with rax (verbose)..."
	cd ../.. && RUST_LOG=debug cargo run --release -- \
		--backend emulator \
		--arch x86_64 \
		--memory 128M \
		--kernel tests/boot_test/$(TARGET) \
		--cmdline "console=ttyS0"

# Run with QEMU for comparison
run-qemu: $(TARGET)
	@echo "Running boot test with QEMU..."
	qemu-system-x86_64 \
		-machine q35 \
		-cpu qemu64 \
		-m 128M \
		-kernel $(TARGET) \
		-nographic \
		-serial mon:stdio \
		-no-reboot

clean:
	rm -f $(OBJECTS) $(ELF) $(TARGET)

# Debug: show disassembly
disasm: $(ELF)
	objdump -d $<

# Debug: show sections
sections: $(ELF)
	readelf -S $<

# Debug: show symbols
symbols: $(ELF)
	nm -n $<
