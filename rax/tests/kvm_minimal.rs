//! Minimal KVM test to reproduce EEXIST issue

#[test]
#[cfg(all(feature = "kvm", target_os = "linux"))]
fn test_kvm_vcpu_creation_with_gap() {
    use kvm_bindings::{kvm_pit_config, kvm_userspace_memory_region};
    use kvm_ioctls::Kvm;
    use std::os::unix::io::AsRawFd;

    println!("Opening KVM...");
    let kvm = Kvm::new().expect("Failed to open KVM");

    println!("Creating VM...");
    let vm = kvm.create_vm().expect("Failed to create VM");

    // Register memory in TWO slots with a gap for TSS/identity map
    // Gap at [0xC0000000, 0xC0100000) = [3GB, 3GB+1MB)
    // TSS at 0xC0001000, identity map at 0xC0000000
    let gap_start: u64 = 0xC0000000;
    let gap_end: u64 = 0xC0100000;

    println!("Allocating 6GB of backing memory...");
    let total_size: usize = 6 * 1024 * 1024 * 1024; // 6GB
    let mem_ptr = unsafe {
        let ptr = libc::mmap(
            std::ptr::null_mut(),
            total_size,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS | libc::MAP_NORESERVE,
            -1,
            0,
        );
        if ptr == libc::MAP_FAILED {
            panic!("mmap failed");
        }
        ptr
    };

    // Register in two slots, BOTH below the gap
    // Slot 0: [0, 1GB)
    let slot0_size: u64 = 1024 * 1024 * 1024; // 1GB
    println!("Registering slot 0: [0x0, 0x{:x})", slot0_size);
    let mem_region0 = kvm_userspace_memory_region {
        slot: 0,
        guest_phys_addr: 0,
        memory_size: slot0_size,
        userspace_addr: mem_ptr as u64,
        flags: 0,
    };
    unsafe {
        vm.set_user_memory_region(mem_region0)
            .expect("Failed to set slot 0")
    };

    // Slot 1: [1GB, 2GB) - still below the gap at 3GB
    let slot1_guest_start: u64 = 1024 * 1024 * 1024; // 1GB
    let slot1_size: u64 = 1024 * 1024 * 1024; // 1GB
    let slot1_host_addr = (mem_ptr as u64) + slot1_guest_start;
    println!(
        "Registering slot 1: [0x{:x}, 0x{:x})",
        slot1_guest_start,
        slot1_guest_start + slot1_size
    );
    let mem_region1 = kvm_userspace_memory_region {
        slot: 1,
        guest_phys_addr: slot1_guest_start,
        memory_size: slot1_size,
        userspace_addr: slot1_host_addr,
        flags: 0,
    };
    unsafe {
        vm.set_user_memory_region(mem_region1)
            .expect("Failed to set slot 1")
    };

    // Create IRQ chip
    println!("Creating IRQ chip...");
    vm.create_irq_chip().expect("Failed to create IRQ chip");

    // Create PIT2
    println!("Creating PIT2...");
    vm.create_pit2(kvm_pit_config {
        flags: 0,
        pad: [0; 15],
    })
    .expect("Failed to create PIT2");

    // Set TSS address (in the gap)
    let tss_addr = gap_start + 0x1000;
    println!("Setting TSS address at 0x{:x}...", tss_addr);
    vm.set_tss_address(tss_addr as usize)
        .expect("Failed to set TSS address");

    // Set identity map address (in the gap)
    let identity_map_addr = gap_start;
    println!(
        "Setting identity map address at 0x{:x}...",
        identity_map_addr
    );
    vm.set_identity_map_address(identity_map_addr)
        .expect("Failed to set identity map address");

    // Try to create vCPU
    println!("Creating vCPU 0...");
    match vm.create_vcpu(0) {
        Ok(vcpu) => println!("vCPU 0 created successfully: fd={}", vcpu.as_raw_fd()),
        Err(e) => panic!("Failed to create vCPU 0: error code {}", e),
    }

    println!("Test passed!");

    // Cleanup
    unsafe { libc::munmap(mem_ptr, total_size) };
}

#[test]
#[cfg(all(feature = "kvm", target_os = "linux"))]
fn test_kvm_vcpu_creation_simple() {
    use kvm_bindings::{kvm_pit_config, kvm_userspace_memory_region};
    use kvm_ioctls::Kvm;
    use std::os::unix::io::AsRawFd;

    println!("Opening KVM...");
    let kvm = Kvm::new().expect("Failed to open KVM");

    println!("Creating VM...");
    let vm = kvm.create_vm().expect("Failed to create VM");

    // Register a SMALL memory region that doesn't overlap with TSS/identity map
    // Only register 64MB starting at 0
    println!("Registering memory region (64MB at 0x0)...");
    let mem_size: usize = 64 * 1024 * 1024;

    // Allocate page-aligned memory using mmap
    let mem_ptr = unsafe {
        let ptr = libc::mmap(
            std::ptr::null_mut(),
            mem_size,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS | libc::MAP_NORESERVE,
            -1,
            0,
        );
        if ptr == libc::MAP_FAILED {
            panic!("mmap failed");
        }
        ptr
    };

    let mem_region = kvm_userspace_memory_region {
        slot: 0,
        guest_phys_addr: 0,
        memory_size: mem_size as u64,
        userspace_addr: mem_ptr as u64,
        flags: 0,
    };
    unsafe {
        vm.set_user_memory_region(mem_region)
            .expect("Failed to set memory region")
    };
    println!("  Memory registered: slot=0, guest=[0x0, 0x{:x})", mem_size);

    // Create IRQ chip FIRST
    println!("Creating IRQ chip...");
    vm.create_irq_chip().expect("Failed to create IRQ chip");

    // Create PIT2
    println!("Creating PIT2...");
    vm.create_pit2(kvm_pit_config {
        flags: 0,
        pad: [0; 15],
    })
    .expect("Failed to create PIT2");

    // Set TSS address (at high address, well above our 64MB)
    println!("Setting TSS address at 0xfffbd000...");
    vm.set_tss_address(0xfffbd000)
        .expect("Failed to set TSS address");

    // Set identity map address (at high address)
    println!("Setting identity map address at 0xffffc000...");
    vm.set_identity_map_address(0xffffc000)
        .expect("Failed to set identity map address");

    // Try to create vCPU
    println!("Creating vCPU 0...");
    match vm.create_vcpu(0) {
        Ok(vcpu) => println!("vCPU 0 created successfully: fd={}", vcpu.as_raw_fd()),
        Err(e) => panic!("Failed to create vCPU 0: error code {}", e),
    }

    println!("Test passed!");

    // Cleanup
    unsafe { libc::munmap(mem_ptr, mem_size) };
}
