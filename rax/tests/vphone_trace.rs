use rax::vphone_trace::{
    BOOT_ARGS_BASE, DTB_BASE, FRAMEBUFFER_BASE, KERNEL_ENTRY, build_boot_args,
};

#[test]
fn public_vphone_addresses_stay_fixed() {
    assert_eq!(KERNEL_ENTRY, 0x8227_0000);
    assert_eq!(DTB_BASE, 0x9000_0000);
    assert_eq!(BOOT_ARGS_BASE, 0x8080_0000);
    assert_eq!(FRAMEBUFFER_BASE, 0x17f00_0000);
}

#[test]
fn boot_args_advertise_framebuffer() {
    let args = build_boot_args(&Default::default(), 68_336, false);
    assert_eq!(
        u64::from_le_bytes(args[0x28..0x30].try_into().unwrap()),
        FRAMEBUFFER_BASE
    );
    assert_eq!(u32::from_le_bytes(args[0x3C..0x40].try_into().unwrap()), 1290);
    assert_eq!(u32::from_le_bytes(args[0x40..0x44].try_into().unwrap()), 2796);
}
