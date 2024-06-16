
#[cfg(target_arch = "x86")]
pub mod x86;

pub fn read_byte(addr: usize) -> u32 {
    (unsafe { std::ptr::read_unaligned(addr as *const u8) }) as u32
}

pub fn read_pointer(addr: usize) -> usize {
    unsafe { std::ptr::read_unaligned(addr as *const usize) }
}

pub fn get_opcode_size(buffer: &[u8]) -> usize {
    #[cfg(target_arch = "x86")]
    {
        x86::get_opcode_size_32(buffer)
    }

    #[cfg(target_arch = "x86_64")]
    {
        0
    }
}
