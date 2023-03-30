use std::io;
use super::file::{Result};

const MAX_NUMBER_BYTES: usize = 16;
trait Number {
    type Output;
    const SIZE: usize = 0;
    fn from_le_bytes(bytes: &[u8; MAX_NUMBER_BYTES]) -> Self::Output;
    fn from_be_bytes(bytes: &[u8; MAX_NUMBER_BYTES]) -> Self::Output;
}

impl Number for u8 {
    type Output = u8;
    const SIZE: usize = 1;
    fn from_le_bytes(bytes: &[u8; MAX_NUMBER_BYTES]) -> u8 {
        u8::from_le_bytes([bytes[0]])
    }

    fn from_be_bytes(bytes: &[u8; MAX_NUMBER_BYTES]) -> u8 {
        u8::from_be_bytes([bytes[0]])
    }
}

impl Number for u16 {
    type Output = u16;
    const SIZE: usize = 2;
    fn from_le_bytes(bytes: &[u8; MAX_NUMBER_BYTES]) -> u16 {
        u16::from_le_bytes([bytes[0], bytes[1]])
    }

    fn from_be_bytes(bytes: &[u8; MAX_NUMBER_BYTES]) -> u16 {
        u16::from_be_bytes([bytes[0], bytes[1]])
    }
}

impl Number for u32 {
    type Output = u32;
    const SIZE: usize = 4;
    fn from_le_bytes(bytes: &[u8; MAX_NUMBER_BYTES]) -> u32 {
        u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }

    fn from_be_bytes(bytes: &[u8; MAX_NUMBER_BYTES]) -> u32 {
        u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }
}

impl Number for u64 {
    type Output = u64;
    const SIZE: usize = 8;
    fn from_le_bytes(bytes: &[u8; MAX_NUMBER_BYTES]) -> u64 {
        u64::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]])
    }

    fn from_be_bytes(bytes: &[u8; MAX_NUMBER_BYTES]) -> u64 {
        u64::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]])
    }
}

impl Number for f32 {
    type Output = f32;
    const SIZE: usize = 4;
    fn from_le_bytes(bytes: &[u8; MAX_NUMBER_BYTES]) -> f32 {
        f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }

    fn from_be_bytes(bytes: &[u8; MAX_NUMBER_BYTES]) -> f32 {
        f32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }
}

impl Number for f64 {
    type Output = f64;
    const SIZE: usize = 8;
    fn from_le_bytes(bytes: &[u8; MAX_NUMBER_BYTES]) -> f64 {
        f64::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]])
    }

    fn from_be_bytes(bytes: &[u8; MAX_NUMBER_BYTES]) -> f64 {
        f64::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]])
    }
}

const LITTLE_ENDIAN: u32 = 1;
const BIG_ENDIAN: u32 = 2;

fn read_number<const E: u32, R: io::Read, T: Number<Output = T>>(mut r: R) -> Result<T> {
    let mut buf = [0u8; MAX_NUMBER_BYTES];

    r.read(&mut buf[..T::SIZE])?;

    if E == LITTLE_ENDIAN {
        Ok(T::from_le_bytes(&buf))
    } else {
        Ok(T::from_be_bytes(&buf))
    }
}

/// `ByteOrder` describes types that can serialize integers as bytes.
///
/// This crate provides two types that implement `ByteOrder`: [`BigEndian`] and [`LittleEndian`].
pub trait ByteOrder {
    fn read_u8<T: io::Read>(r: T) -> Result<u8>;
    fn read_u16<T: io::Read>(r: T) -> Result<u16>;
    fn read_u32<T: io::Read>(r: T) -> Result<u32>;
    fn read_u64<T: io::Read>(r: T) -> Result<u64>;
    fn read_f32<T: io::Read>(r: T) -> Result<f32>;
    fn read_f64<T: io::Read>(r: T) -> Result<f64>;

    fn read_i8<T: io::Read>(r: T) -> Result<i8> {
        Ok(Self::read_u8(r)? as i8)
    }

    fn read_i16<T: io::Read>(r: T) -> Result<i16> {
        Ok(Self::read_u16(r)? as i16)
    }

    fn read_i32<T: io::Read>(r: T) -> Result<i32> {
        Ok(Self::read_u32(r)? as i32)
    }

    fn read_i64<T: io::Read>(r: T) -> Result<i64> {
        Ok(Self::read_u64(r)? as i64)
    }
}

pub enum LittleEndian {}
pub enum BigEndian {}

impl ByteOrder for LittleEndian {
    fn read_u8<T: io::Read>(r: T) -> Result<u8> {
        read_number::<LITTLE_ENDIAN, _, _>(r)
    }

    fn read_u16<T: io::Read>(r: T) -> Result<u16> {
        read_number::<LITTLE_ENDIAN, _, _>(r)
    }

    fn read_u32<T: io::Read>(r: T) -> Result<u32> {
        read_number::<LITTLE_ENDIAN, _, _>(r)
    }

    fn read_u64<T: io::Read>(r: T) -> Result<u64> {
        read_number::<LITTLE_ENDIAN, _, _>(r)
    }

    fn read_f32<T: io::Read>(r: T) -> Result<f32> {
        read_number::<LITTLE_ENDIAN, _, _>(r)
    }

    fn read_f64<T: io::Read>(r: T) -> Result<f64> {
        read_number::<LITTLE_ENDIAN, _, _>(r)
    }

}

impl ByteOrder for BigEndian {
    fn read_u8<T: io::Read>(r: T) -> Result<u8> {
        read_number::<BIG_ENDIAN, _, _>(r)
    }

    fn read_u16<T: io::Read>(r: T) -> Result<u16> {
        read_number::<BIG_ENDIAN, _, _>(r)
    }

    fn read_u32<T: io::Read>(r: T) -> Result<u32> {
        read_number::<BIG_ENDIAN, _, _>(r)
    }

    fn read_u64<T: io::Read>(r: T) -> Result<u64> {
        read_number::<BIG_ENDIAN, _, _>(r)
    }

    fn read_f32<T: io::Read>(r: T) -> Result<f32> {
        read_number::<BIG_ENDIAN, _, _>(r)
    }

    fn read_f64<T: io::Read>(r: T) -> Result<f64> {
        read_number::<BIG_ENDIAN, _, _>(r)
    }

}
