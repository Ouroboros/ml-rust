#![allow(dead_code, unused_imports)]

use lazy_static::lazy_static;
use std::{os::raw::c_void, sync::{Mutex, MutexGuard}};
use crate::hooker::err::{Result, HookError};
use super::patch_info::{PatchInfo, MemoryInfo, FunctionInfo, Value};
use windows_sys::Win32::{
    Foundation::{FALSE, GetLastError},
    System::Memory::{VirtualProtect, PAGE_EXECUTE_READWRITE},
};

lazy_static! {
    static ref MANAGER: Mutex<Manager> = Mutex::new(Manager::new());
}

#[derive(Debug)]
pub struct Manager {
    executable_heap: usize,
}

impl Manager {
    pub fn get() -> MutexGuard<'static, Self> {
        MANAGER.lock().unwrap()
    }

    fn new() -> Self {
        Self {
            executable_heap: 0,
        }
    }

    pub fn patch(&self, infos: &[PatchInfo]) -> Result<()> {
        for info in infos.iter() {
            match info {
                PatchInfo::Memory(mem) => {
                    self.patch_memory(mem)?;
                },

                PatchInfo::Function(func) => {
                    self.patch_function(func)?;
                },
            }
        }

        Ok(())
    }

    fn patch_memory(&self, info: &MemoryInfo) -> Result<()> {
        println!("{info:?}");

        let va = (info.base_address + info.rva) as *const c_void;

        let mut old_prot: u32 = 0;
        let new_prot = PAGE_EXECUTE_READWRITE;

        let success = unsafe { VirtualProtect(va, info.size, new_prot, &mut old_prot) };

        if success == FALSE {
            return Err(HookError::VirtualProtect(unsafe { GetLastError() }));
        }

        let buf = unsafe { std::slice::from_raw_parts_mut(va as *mut u8, info.size) };

        match &info.value {
            Value::Value(u) => {
                if info.size > std::mem::size_of_val(&u) {
                    return Err(HookError::InvalidPatchSize(info.size));
                }

                let data = u.to_le_bytes();
                buf[..info.size].copy_from_slice(&data[..info.size]);
            },

            Value::Bytes(b) => {
                buf[0..b.len()].copy_from_slice(&b);
            },
        }

        if old_prot != new_prot {
            unsafe { VirtualProtect(va, info.size, old_prot, &mut old_prot) };
        }

        Ok(())
    }

    fn patch_function(&self, info: &FunctionInfo) -> Result<()> {
        println!("{info:?}");
        Ok(())
    }

}

pub fn patch(infos: &[PatchInfo]) -> Result<()> {
    Manager::get().patch(infos)
}
