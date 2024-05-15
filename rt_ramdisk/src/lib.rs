#![no_std]

use core::panic::PanicInfo;

/// Entry
#[no_mangle]
pub extern "Rust" fn runtime_main(_cpu_id: usize, _dtb_pa: usize) {
    let msg = "\n[ramdisk]: Hello, RamDisk!\n";
    earlycon::write_bytes(msg.as_bytes());
}

pub fn panic(info: &PanicInfo) -> ! {
    boot::panic(info)
}
