#![no_std]
#![feature(used_with_arg)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate alloc;

pub mod device;
pub mod memory;
pub mod rtc;

use alloc::{collections::BTreeMap, vec::Vec, sync::Arc};
use device::RtcDriver;
use core::sync::atomic::{AtomicUsize, Ordering};
use fdt::{self, node::FdtNode, Fdt};
use sync::Mutex;

pub static DEVICE_TREE_ADDR: AtomicUsize = AtomicUsize::new(0);
pub static DRIVER_REGS: Mutex<BTreeMap<&str, fn(&FdtNode)>> = Mutex::new(BTreeMap::new());
pub static RTC_DEVICES: Mutex<Vec<Arc<dyn RtcDriver>>> = Mutex::new(Vec::new());

pub fn init_drivers() {
    rtc::driver_init();
}

pub fn init_device(device_tree: usize) {
    // 初始化所有驱动
    init_drivers();

    DEVICE_TREE_ADDR.store(device_tree, Ordering::Relaxed);
    let fdt = unsafe { Fdt::from_ptr(device_tree as *const u8).unwrap() };
    info!("There has {} CPU(s)", fdt.cpus().count());

    fdt.memory().regions().for_each(|x| {
        info!(
            "memory region {:#X} - {:#X}",
            x.starting_address as usize,
            x.starting_address as usize + x.size.unwrap()
        );
    });

    let node = fdt.all_nodes();

    let driver_manager = DRIVER_REGS.lock();
    for child in node {
        if let Some(compatible) = child.compatible() {
            if let Some(f) = driver_manager.get(compatible.first()) {
                f(&child);
            }
            // info!("    {}  {}", child.name, compatible.first());
        }
    }
}

#[inline]
pub fn get_addr_from_name(name: &str) -> usize {
    for (i, c) in name.chars().enumerate() {
        if c == '@' {
            return name[(i + 1)..].parse::<usize>().unwrap();
        }
    }
    0
}
