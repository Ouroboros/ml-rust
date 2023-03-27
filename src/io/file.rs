use std::fs;
use std::io;
use std::io::{Read as _, Seek, SeekFrom};
use std::path::Path;
use super::read::Read;

pub type Result<T> = io::Result<T>;

#[derive(Clone, Copy)]
pub enum ByteOrder {
    Little,
    Big,
}

pub struct File {
    inner: fs::File,
    byte_order: ByteOrder,
}

impl File {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<File> {
        let fs = fs::File::open(path)?;
        Ok(File{inner: fs, byte_order: ByteOrder::Little})
    }

    pub fn endian(&self) -> ByteOrder {
        self.byte_order
    }

    pub fn set_endian(&mut self, order: ByteOrder) {
        self.byte_order = order;
    }

    pub fn pos(&mut self) -> u64 {
        self.inner.seek(SeekFrom::Current(0)).unwrap()
    }

    pub fn size(&self) -> u64 {
        self.inner.metadata().unwrap().len()
    }
}

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.inner.read(buf)
    }

    fn read_array<const N: usize>(&mut self) -> Result<[u8; N]> {
        let mut buf = [0u8; N];
        self.inner.read(&mut buf)?;
        Ok(buf)
    }

    fn read_bytes(&mut self, size: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0u8; size];
        self.read(&mut buf)?;
        Ok(buf)
    }

    fn read_to_end(&mut self) -> Result<Vec<u8>> {
        let mut buf: Vec<u8> = Vec::new();
        self.inner.read_to_end(&mut buf)?;
        Ok(buf)
    }

    fn u8(&mut self) -> u8 {
        u8::from_le_bytes(self.read_array::<1>().unwrap())
    }

    fn u16(&mut self) -> u16 {
        match self.byte_order {
            ByteOrder::Little => u16::from_le_bytes(self.read_array::<2>().unwrap()),
            ByteOrder::Big => u16::from_be_bytes(self.read_array::<2>().unwrap()),
        }
    }

    fn u32(&mut self) -> u32 {
        match self.byte_order {
            ByteOrder::Little => u32::from_le_bytes(self.read_array::<4>().unwrap()),
            ByteOrder::Big => u32::from_be_bytes(self.read_array::<4>().unwrap()),
        }
    }

    fn u64(&mut self) -> u64 {
        match self.byte_order {
            ByteOrder::Little => u64::from_le_bytes(self.read_array::<8>().unwrap()),
            ByteOrder::Big => u64::from_be_bytes(self.read_array::<8>().unwrap()),
        }
    }

    fn f32(&mut self) -> f32 {
        match self.byte_order {
            ByteOrder::Little => f32::from_le_bytes(self.read_array::<4>().unwrap()),
            ByteOrder::Big => f32::from_be_bytes(self.read_array::<4>().unwrap()),
        }
    }

    fn f64(&mut self) -> f64 {
        match self.byte_order {
            ByteOrder::Little => f64::from_le_bytes(self.read_array::<8>().unwrap()),
            ByteOrder::Big => f64::from_be_bytes(self.read_array::<8>().unwrap()),
        }
    }
}
