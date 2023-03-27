use super::file::Result;

pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
    fn read_array<const N: usize>(&mut self) -> Result<[u8; N]>;
    fn read_bytes(&mut self, size: usize) -> Result<Vec<u8>>;
    fn read_to_end(&mut self) -> Result<Vec<u8>>;

    fn u8(&mut self) -> u8;
    fn u16(&mut self) -> u16;
    fn u32(&mut self) -> u32;
    fn u64(&mut self) -> u64;
    fn f32(&mut self) -> f32;
    fn f64(&mut self) -> f64;

    fn i8(&mut self) -> i8 {
        self.u8() as i8
    }

    fn i32(&mut self) -> i32 {
        self.u32() as i32
    }

    fn i64(&mut self) -> i64 {
        self.u64() as i64
    }
}
