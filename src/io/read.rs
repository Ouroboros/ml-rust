use std::io;
use super::file::Result;
use super::byteorder::ByteOrder;

pub trait ReadExt: io::Read {
    fn read_array<const N: usize>(&mut self) -> Result<[u8; N]> {
        let mut buf = [0u8; N];
        self.read(&mut buf)?;
        Ok(buf)
    }

    fn read_bytes(&mut self, size: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0u8; size];
        self.read(&mut buf)?;
        Ok(buf)
    }

    fn read_i8<T: ByteOrder>(&mut self) -> Result<i8> {
        T::read_i8(self)
    }

    fn read_u8<T: ByteOrder>(&mut self) -> Result<u8> {
        T::read_u8(self)
    }

    fn read_i16<T: ByteOrder>(&mut self) -> Result<i16> {
        T::read_i16(self)
    }

    fn read_u16<T: ByteOrder>(&mut self) -> Result<u16> {
        T::read_u16(self)
    }

    fn read_i32<T: ByteOrder>(&mut self) -> Result<i32> {
        T::read_i32(self)
    }

    fn read_u32<T: ByteOrder>(&mut self) -> Result<u32> {
        T::read_u32(self)
    }

    fn read_i64<T: ByteOrder>(&mut self) -> Result<i64> {
        T::read_i64(self)
    }

    fn read_u64<T: ByteOrder>(&mut self) -> Result<u64> {
        T::read_u64(self)
    }

    fn read_f32<T: ByteOrder>(&mut self) -> Result<f32> {
        T::read_f32(self)
    }

    fn read_f64<T: ByteOrder>(&mut self) -> Result<f64> {
        T::read_f64(self)
    }

    fn i8<T: ByteOrder>(&mut self) -> i8 {
        self.read_i8::<T>().unwrap()
    }

    fn u8<T: ByteOrder>(&mut self) -> u8 {
        self.read_u8::<T>().unwrap()
    }

    fn i16<T: ByteOrder>(&mut self) -> i16 {
        self.read_i16::<T>().unwrap()
    }

    fn u16<T: ByteOrder>(&mut self) -> u16 {
        self.read_u16::<T>().unwrap()
    }

    fn i32<T: ByteOrder>(&mut self) -> i32 {
        self.read_i32::<T>().unwrap()
    }

    fn u32<T: ByteOrder>(&mut self) -> u32 {
        self.read_u32::<T>().unwrap()
    }

    fn i64<T: ByteOrder>(&mut self) -> i64 {
        self.read_i64::<T>().unwrap()
    }

    fn u64<T: ByteOrder>(&mut self) -> u64 {
        self.read_u64::<T>().unwrap()
    }

    fn f32<T: ByteOrder>(&mut self) -> f32 {
        self.read_f32::<T>().unwrap()
    }

    fn f64<T: ByteOrder>(&mut self) -> f64 {
        self.read_f64::<T>().unwrap()
    }
}

impl<R: io::Read + ?Sized> ReadExt for R {}
