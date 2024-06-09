use thiserror::Error;

#[derive(Error, Debug)]
pub enum HookError {
    #[error("invalid parameter")]
    InvalidParameter,

    #[error("change memory protect failed: 0x{0:08X}")]
    VirtualProtect(u32),

    #[error("invalid patch size: 0x{0:08X}")]
    InvalidPatchSize(usize),
}

pub type Result<T> = std::result::Result<T, HookError>;
