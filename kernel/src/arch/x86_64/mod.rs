pub fn cr3() -> u64 {
    let result: u64;
    
    unsafe {
        asm!("mov {0}, cr3",
             out(reg) result);
    }

    result
}

pub fn set_cr3(value: u64) {
    unsafe {
        asm!("mov cr3, {0}",
             in(reg) value);
    }
}
