use std::{fs::File, os::fd::AsRawFd, ptr::{null_mut}, slice, hint::{black_box}};

use libc::{mmap, PROT_READ, MAP_PRIVATE};

fn spin_second() { // (~ a second on a 4Ghz rig compiled in release mode)
    for i in 0..4_000_000_000_u64 {
        black_box(i);
    }
}

pub fn test(map_slice: &[u8], map_ptr: *const u8) {
    println!("slice: {}, ptr: {}, volatile: {}",
        char::from(map_slice[0]),
        char::from(unsafe { *map_ptr }),
        char::from(unsafe { map_ptr.read_volatile() }),
    );
    spin_second();
    spin_second();
    println!("Offset 0: slice: {}, ptr: {}, volatile: {}",
        char::from(map_slice[0]),
        char::from(unsafe { *map_ptr }),
        char::from(unsafe { map_ptr.read_volatile() }),
    );
    println!("Offset 1: slice: {}, ptr: {}, volatile: {}",
        char::from(map_slice[1]),
        char::from(unsafe { *map_ptr.offset(1) }),
        char::from(unsafe { map_ptr.offset(1).read_volatile() }),
    );
    println!("Offset 32: slice: {}, ptr: {}, volatile: {}",
        char::from(map_slice[32]),
        char::from(unsafe { *map_ptr.offset(32) }),
        char::from(unsafe { map_ptr.offset(32).read_volatile() }),
    );
    println!("Offset 64: slice: {}, ptr: {}, volatile: {}",
        char::from(map_slice[64]),
        char::from(unsafe { *map_ptr.offset(64) }),
        char::from(unsafe { map_ptr.offset(64).read_volatile() }),
    );
}


fn main() {
    let file = File::open("test.txt").unwrap();
    let len = file.metadata().unwrap().len() as usize;
    let map_ptr = unsafe {
        mmap(null_mut(), len, PROT_READ, MAP_PRIVATE, file.as_raw_fd(), 0)
    } as *const u8;
    let map_slice = unsafe { slice::from_raw_parts(map_ptr, len) };
    test(map_slice, map_ptr);
}
