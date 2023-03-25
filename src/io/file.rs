use std::fs;
use std::io;
use std::io::Read as IoRead;
use std::path::Path;
use super::read::Read;

pub type Result<T> = io::Result<T>;

pub struct File {
    inner: fs::File,
}

impl File {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<File> {
        let fs = fs::File::open(path)?;
        Ok(File{inner: fs})
    }
}

impl Read for File {
    fn read_array<const N: usize>(&mut self) -> Result<[u8; N]> {
        let mut buf = [0u8; N];
        self.inner.read(&mut buf)?;
        Ok(buf)
    }

    fn read_bytes(&mut self, size: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0u8; size];
        self.inner.read(&mut buf)?;
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
        u16::from_le_bytes(self.read_array::<2>().unwrap())
    }

    fn u32(&mut self) -> u32 {
        u32::from_le_bytes(self.read_array::<4>().unwrap())
    }

    fn u64(&mut self) -> u64 {
        u64::from_le_bytes(self.read_array::<8>().unwrap())
    }
}
