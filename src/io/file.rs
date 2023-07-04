use std::fs;
use std::io::{self, Seek as _};
use std::path::Path;

pub type Result<T> = io::Result<T>;

pub struct File {
    inner: fs::File,
}

impl File {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<File> {
        let fs = fs::File::open(path)?;
        Ok(File{inner: fs})
    }

    pub fn into_inner(self) -> fs::File {
        self.inner
    }

    pub const fn get_ref(&self) -> &fs::File {
        &self.inner
    }

    pub fn get_mut(&mut self) -> &mut fs::File {
        &mut self.inner
    }

    pub fn pos(&mut self) -> Result<u64> {
        self.stream_position()
    }

    pub fn size(&self) -> Result<u64> {
        Ok(self.inner.metadata()?.len())
    }
}

impl io::Read for File {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }

    fn read_vectored(&mut self, bufs: &mut [io::IoSliceMut<'_>]) -> io::Result<usize> {
        self.inner.read_vectored(bufs)
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        self.inner.read_exact(buf)
    }
}

impl io::Seek for File {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        self.inner.seek(pos)
    }

    fn rewind(&mut self) -> io::Result<()> {
        self.inner.rewind()
    }

    fn stream_position(&mut self) -> io::Result<u64> {
        self.inner.stream_position()
    }
}
