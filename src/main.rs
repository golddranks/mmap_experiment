use std::{fs::File, os::fd::AsRawFd, ptr::{null_mut}, slice};

use libc::{mmap, PROT_READ, MAP_PRIVATE};

pub fn test(map_slice: &[u8], map_ptr: *const u8) {
    let test = map_slice[0];
    let test2 = unsafe { *map_ptr };
    println!("slice: {}, ptr: {}, volatile: {}",
        char::from(test),
        char::from(test2),
        char::from(unsafe { map_ptr.read_volatile() }),
    );
    let mut a: u64 = 4;
    for _ in 0..2_450_000_000_u64 { // spins for a few seconds on 2015 MBP
        a = a.wrapping_mul(a);
    }
    if a != 0 {
        return; // This is to prevent getting the loop optimized away
    }
    let test = map_slice[0];
    let test2 = unsafe { *map_ptr };
    println!("Offset 0: slice: {}, ptr: {}, volatile: {}",
        char::from(test),
        char::from(test2),
        char::from(unsafe { map_ptr.read_volatile() }),
    );
    let test = map_slice[64];
    let test2 = unsafe { *map_ptr.offset(64) };
    println!("Offset 64: slice: {}, ptr: {}, volatile: {}",
        char::from(test),
        char::from(test2),
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
