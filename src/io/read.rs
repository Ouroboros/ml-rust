
use std::io::Read;
use super::file::Result;
use super::byteorder::{ByteOrder, LittleEndian};

#[allow(unused_imports)]
use super::byteorder::BigEndian;

/// Extends [`Read`] with methods for reading numbers. (For `std::io`.)
///
/// Most of the methods defined here have an unconstrained type parameter that
/// must be explicitly instantiated. Typically, it is instantiated with either
/// the [`BigEndian`] or [`LittleEndian`] types defined in this crate.
///
/// # Errors
///
/// read_\<number\> methods return the same errors as [`Read::read`].
pub trait ReadExt: Read {
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

    fn read_i8(&mut self) -> Result<i8> {
        LittleEndian::read_i8(self)
    }

    fn read_u8(&mut self) -> Result<u8> {
        LittleEndian::read_u8(self)
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

    fn i8(&mut self) -> i8 {
        self.read_i8().unwrap()
    }

    fn u8(&mut self) -> u8 {
        self.read_u8().unwrap()
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

impl<R: Read + ?Sized> ReadExt for R {}
