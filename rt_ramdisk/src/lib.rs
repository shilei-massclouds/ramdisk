#![no_std]

#[macro_use]
extern crate axlog;
extern crate alloc;
use alloc::vec;

use core::panic::PanicInfo;
use axtype::{align_up_4k, align_down_4k, phys_to_virt, virt_to_phys};
use driver_common::{BaseDriverOps, BlockDriverOps, DeviceType};

const DISK_SIZE: usize = 0x1000;    // 4K
const BLOCK_SIZE: usize = 0x200;    // 4K

/// Entry
#[no_mangle]
pub extern "Rust" fn runtime_main(_cpu_id: usize, _dtb_pa: usize) {
    axlog::init();
    axlog::set_max_level("info");
    info!("[rt_ramdisk]: ...");

    let start = align_up_4k(virt_to_phys(_ekernel as usize));
    let end = align_down_4k(config::PHYS_MEMORY_END);
    axalloc::init(phys_to_virt(start), end - start);

    let mut disk = ramdisk::RamDisk::new(0x1000);
    assert_eq!(disk.device_type(), DeviceType::Block);
    assert_eq!(disk.device_name(), "ramdisk");
    assert_eq!(disk.block_size(), BLOCK_SIZE);
    assert_eq!(disk.num_blocks() as usize, DISK_SIZE/BLOCK_SIZE);

    let block_id = 1;

    let mut buf = vec![0u8; BLOCK_SIZE];
    assert!(disk.read_block(block_id, &mut buf).is_ok());
    assert!(buf[0..4] != *b"0123");

    buf[0] = b'0';
    buf[1] = b'1';
    buf[2] = b'2';
    buf[3] = b'3';

    assert!(disk.write_block(block_id, &buf).is_ok());
    assert!(disk.flush().is_ok());

    assert!(disk.read_block(block_id, &mut buf).is_ok());
    assert!(buf[0..4] == *b"0123");

    info!("[rt_ramdisk]: ok!");
    hal_base::terminate();
}

pub fn panic(info: &PanicInfo) -> ! {
    error!("{}", info);
    boot::panic(info)
}

extern "C" {
    fn _ekernel();
}
