#![no_std]

#[macro_use]
extern crate axlog;

use core::panic::PanicInfo;
use axtype::{align_up_4k, align_down_4k, phys_to_virt, virt_to_phys};

/// Entry
#[no_mangle]
pub extern "Rust" fn runtime_main(_cpu_id: usize, _dtb_pa: usize) {
    axlog::init();
    axlog::set_max_level("debug");
    info!("[rt_ramdisk]: startup ...");

    let start = align_up_4k(virt_to_phys(_ekernel as usize));
    let end = align_down_4k(config::PHYS_MEMORY_END);
    axalloc::init(phys_to_virt(start), end - start);

    let _disk = ramdisk::RamDisk::new(0x100_0000); // 16M

    info!("exit ...");
    hal_base::terminate();
}

pub fn panic(info: &PanicInfo) -> ! {
    boot::panic(info)
}

extern "C" {
    fn _ekernel();
}
