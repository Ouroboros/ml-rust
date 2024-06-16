pub mod err;

#[cfg(target_arch = "x86")]
pub mod x86;

pub mod ldasm;

#[cfg(target_os = "windows")]
use windows_sys::Win32::{
    Foundation::{GetLastError, FALSE},
    System::Memory::{VirtualProtect, PAGE_EXECUTE_READWRITE},
};

pub struct MemoryProtector {
    va            : usize,
    size          : usize,
    old_protect   : u32,
}

#[cfg(target_os = "windows")]
impl MemoryProtector {
    pub fn new(va: usize, size: usize) -> err::Result<Self> {
        let mut old_protect: u32 = 0;

        let success = unsafe {
            VirtualProtect(
                va as *const std::os::raw::c_void,
                size,
                PAGE_EXECUTE_READWRITE,
                &mut old_protect,
            )
        };

        if success == FALSE {
            return Err(err::HookError::VirtualProtect(va, size, unsafe { GetLastError() }));
        }

        Ok(Self{
            va,
            size,
            old_protect,
        })
    }
}

#[cfg(target_os = "windows")]
impl Drop for MemoryProtector {
    fn drop(&mut self) {
        if self.old_protect == PAGE_EXECUTE_READWRITE {
            return;
        }

        unsafe {
            VirtualProtect(
                self.va as *const std::os::raw::c_void,
                self.size,
                self.old_protect,
                &mut self.old_protect,
            );
        }
    }
}
