use thiserror::Error;

#[derive(Error, Debug)]
pub enum HookError {
    #[error("invalid parameter")]
    InvalidParameter,

    #[error("change memory protection failed: va: 0x{0:08X}, size: 0x{1:04X}, err: 0x{2:08X}")]
    VirtualProtect(usize, usize, u32),

    #[error("invalid patch size: 0x{0:08X}")]
    InvalidPatchSize(usize),

    #[error("buffer too small: got 0x{0:08X}, expected 0x{1:08X}")]
    BufferTooSmall(usize, usize),

    #[error("io error")]
    Io(#[from] std::io::Error),

    #[error("try from error")]
    TryFrom(#[from] std::array::TryFromSliceError),
}

pub type Result<T> = std::result::Result<T, HookError>;
