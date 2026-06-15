fn main() {
    let mut d = rax::arm::decoder::Decoder::new(rax::arm::ExecutionState::Thumb);
    d.set_state(rax::arm::ExecutionState::Thumb);
    for raw in [0xbc1cu16, 0xbdf0, 0xb013, 0xb580] {
        let b = raw.to_le_bytes();
        match d.decode(&b) {
            Ok(i) => println!("{raw:#06x} -> {:?} ops={:?}", i.mnemonic, i.operands),
            Err(e) => println!("{raw:#06x} ERR {e:?}"),
        }
    }
}
