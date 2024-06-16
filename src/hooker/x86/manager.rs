#![allow(dead_code, unused_imports)]

use lazy_static::lazy_static;
use std::{
    collections::btree_map::Range,
    io::Write,
    os::raw::c_void, ptr::{addr_of, addr_of_mut},
    sync::{Mutex, MutexGuard},
};

use crate::hooker::{
    MemoryProtector,
    err::{
        HookError,
        Result,
    },
    ldasm,
    x86::HookFlags,
};

use super::{
    function_jmp,
    patch_info::{
        FunctionInfo,
        MemoryInfo,
        PatchInfo,
        Value,
    },
    HookType,
    TrampolineData,
};

use windows_sys::Win32::{
    Foundation::HANDLE,
    System::{
        Diagnostics::Debug::FlushInstructionCache,
        Memory::{HeapAlloc, HeapCreate, HeapFree, HEAP_CREATE_ENABLE_EXECUTE, HEAP_ZERO_MEMORY},
    },
};

const TRAMPOLINE_SIZE: usize = 0x40;

lazy_static! {
    static ref MANAGER: Mutex<Manager> = Mutex::new(Manager::new());
}

#[derive(Debug)]
pub struct Manager {
    executable_heap: HANDLE,
}

impl Manager {
    pub fn get() -> MutexGuard<'static, Self> {
        MANAGER.lock().unwrap()
    }

    fn new() -> Self {
        Self {
            executable_heap: unsafe { HeapCreate(HEAP_CREATE_ENABLE_EXECUTE, 0, 0) },
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

        let _protector = MemoryProtector::new(va as usize, info.size)?;

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

        Ok(())
    }

    fn patch_function(&self, info: &FunctionInfo) -> Result<()> {
        println!("{info:?}");

        let va = info.base_address + info.rva;
        // let hook_opcode_size = info.hook_type.size_of_opcode();

        let (hook_address, hook_size) = self.get_hook_address_and_size(va, info.hook_type)?;

        let mut hook_buffer: [u8; TRAMPOLINE_SIZE] = [0; TRAMPOLINE_SIZE];

        if info.flags.contains(HookFlags::NakedTrampoline) == false {
            let size = self.generate_hook_code(&mut hook_buffer, hook_address, &info)?;
            self.patch_nop(&mut hook_buffer[size..], hook_size - size);
        }

        match info.trampoline {
            Some(ptr) => {
                let tramp = self.generate_trampoline(info, hook_address, hook_size)?;
                unsafe {
                    *ptr = Some(std::mem::transmute(tramp));
                }
            },

            None => {
                return Ok(());
            }
        }

        let _protector = MemoryProtector::new(hook_address, hook_size)?;

        unsafe {
            std::slice::from_raw_parts_mut(hook_address as *mut u8, hook_size).copy_from_slice(&hook_buffer[..hook_size]);
        }

        Ok(())
    }

    pub fn get_hook_address_and_size(&self, va: usize, hook_type: HookType) -> Result<(usize, usize)> {
        let mut total_op_size: usize = 0;
        let hook_opcode_size = hook_type.size_of_opcode();

        let mut ptr = va;
        let mut hook_addr = va;

        while total_op_size < hook_opcode_size {
            let buf = unsafe { std::slice::from_raw_parts(ptr as *const u8, 0x10) };
            let current_op_size = ldasm::get_opcode_size(buf);

            match buf[0] {
                0xEB => {
                    // jmp short const

                    if total_op_size == 0 {
                        // first inst is jmp short

                        if buf[1] != 0 {
                            /*
                                401058      /EB 01            jmp     short 40105B
                                40105A      |CC               int3
                                40105B      \33C0             xor     eax, eax
                            */

                            ptr += buf[1] as i8 as usize + current_op_size;
                            hook_addr = ptr;
                        }

                    } else if total_op_size + current_op_size < hook_opcode_size {
                        /*
                            401058       33C0             xor     eax, eax
                            40105A       EB 01            jmp     short 0x40105D
                        */

                        return Err(HookError::BufferTooSmall(total_op_size + current_op_size, hook_opcode_size));
                    }
                },

                0xFF if buf[1] == 0x25 => {
                    // 401058       FF25 00000000    jmp     dword ptr [0]

                    if total_op_size != 0 && total_op_size + current_op_size < hook_opcode_size {
                        return Err(HookError::BufferTooSmall(total_op_size + current_op_size, hook_opcode_size));
                    }

                    if total_op_size == 0 && hook_opcode_size > current_op_size {
                        // inst too short, follow [imm]

                        ptr = ldasm::read_pointer(ldasm::read_pointer(ptr + 2));
                        hook_addr = ptr;
                        continue;
                    }
                },

                _ => {},
            }

            ptr += current_op_size;
            total_op_size += current_op_size;
        }

        Ok((hook_addr, total_op_size))
    }

    fn generate_hook_code(&self, hook_buffer: &mut [u8], hook_address: usize, info: &FunctionInfo) -> Result<usize> {
        let source = hook_address;
        let target = info.target;

        let mut buf = std::io::Cursor::new(hook_buffer);

        match info.hook_type {
            HookType::Call => {
                // call imm

                let offset = target.wrapping_sub(source.wrapping_add(info.hook_type.size_of_opcode()));

                buf.write(&[0xE8])?;
                buf.write(&offset.to_le_bytes())?;
            },
            HookType::Jump => {
                // jmp imm

                let offset = target.wrapping_sub(source.wrapping_add(info.hook_type.size_of_opcode()));

                buf.write(&[0xE9])?;
                buf.write(&offset.to_le_bytes())?;
            },
            HookType::Push => {
                // push imm
                // ret

                buf.write(&[0x68])?;
                buf.write(&target.to_le_bytes())?;
                buf.write(&[0xC3])?;
            },
        }

        Ok(info.hook_type.size_of_opcode())
    }

    fn patch_nop(&self, nop_buffer: &mut [u8], nop_size: usize) {
        let nop_instructions: [&[u8]; 7] = [
            &[0x90],                                        // 1, nop
            &[0x8B, 0xC0],                                  // 2, mov eax, eax
            &[0x8D, 0x40, 0x00],                            // 3, lea eax, [eax+0]
            &[0x8D, 0x74, 0x26, 0x00],                      // 4, lea esi, [esi]
            &[0x8B, 0xC0, 0x8D, 0x40, 0x00],                // 5, 2 + 3
            &[0x8D, 0x80, 0x00, 0x00, 0x00, 0x00],          // 6, lea eax, [eax+0]
            &[0x8D, 0xB4, 0x26, 0x00, 0x00, 0x00, 0x00],    // 7, lea esi, [esi+0]
        ];

        match nop_size {
            0 => {
                return;
            },

            1..=7 => {
                let mut buf = std::io::Cursor::new(nop_buffer);
                buf.write(nop_instructions[nop_size - 1]).unwrap();
            }

            _ => {
                for start in (0..nop_size).step_by(7) {
                    let size = (nop_size - start).min(7);
                    self.patch_nop(&mut nop_buffer[start..start+size], size);
                }
            },
        }
    }

    fn generate_trampoline(&self, info: &FunctionInfo, hook_address: usize, hook_size: usize) -> Result<*mut TrampolineData> {
        let tramp_ptr = self.create_trampoline();
        let tramp = unsafe { &mut *tramp_ptr };

        let original_code_slice = unsafe { std::slice::from_raw_parts(hook_address as *const u8, hook_size) };

        tramp.original_size = hook_size;
        tramp.original_code[..hook_size].copy_from_slice(original_code_slice);
        tramp.jump_back_addr = hook_address + hook_size;

        let target_ip = tramp.trampoline.as_ptr() as usize;
        let mut buf = std::io::Cursor::new(tramp.trampoline.as_mut_slice());

        tramp.trampoline_size = self.generate_trampoline_code(&mut buf, target_ip, hook_address, hook_size)?;

        match info.hook_type {
            HookType::Call if tramp.trampoline_size == hook_size && info.flags.contains(HookFlags::KeepRawTrampoline) => {
                // call -> jmp
                tramp.trampoline[0] = 0xE9;
            },

            _ => {
                self.generate_jump_back(&mut buf, addr_of!(tramp.jump_back_addr) as usize)?;
            },
        }

        unsafe { FlushInstructionCache(0xFFFFFFFF_usize as HANDLE, tramp.trampoline.as_ptr() as *const c_void, std::mem::size_of_val(&tramp.trampoline)) };

        Ok(tramp_ptr)
    }

    fn generate_trampoline_code<T: Write>(&self, buf: &mut T, target_ip: usize, source_address: usize, source_size: usize) -> Result<usize> {
        let mut source_address    = source_address;
        let mut source_size       = source_size;
        let mut target_ip         = target_ip;
        let mut trampoline_size   = 0_usize;
        let trampoline_range      = source_address..source_address + source_size;

        while source_size > 0 {
            let (opcode_size, copied_size) = self.copy_one_opcode(buf, target_ip, source_address, source_size, &trampoline_range)?;

            source_size       -= opcode_size;
            source_address    += opcode_size;
            trampoline_size   += copied_size;
            target_ip         += copied_size;
        }

        Ok(trampoline_size)
    }

    fn generate_jump_back<T: Write>(&self, buf: &mut T, jump_back_address: usize) -> Result<()> {
        buf.write(&[0xFF, 0x25])?;
        buf.write(&jump_back_address.to_le_bytes())?;
        Ok(())
    }

    fn copy_one_opcode<T: Write>(&self, buf: &mut T, target_ip: usize, source_ip: usize, source_size: usize, func_range: &std::ops::Range<usize>) -> Result<(usize, usize)> {
        #[derive(Copy, Clone)]
        enum OpCode {
            Byte(u8),
            UShort(u16),
        }

        impl OpCode {
            pub fn size(&self) -> usize {
                match self {
                    OpCode::Byte(_) => 1,
                    OpCode::UShort(_) => 2,
                }
            }
        }

        const LONG_OFFSET_SIZE: usize = 4;

        let code = unsafe { std::slice::from_raw_parts(source_ip as *const u8, source_size) };
        let opcode_size = ldasm::get_opcode_size(code);
        let next_source_ip = source_ip + opcode_size;

        let op = code[0];
        let mut copied_size: usize = 0;
        let mut new_offset: Option<usize> = None;
        let mut new_opcode = OpCode::Byte(op);

        let calc_new_offset = |opcode: OpCode, orig_offset: usize| -> Option<usize> {
            let orig_target = next_source_ip.wrapping_add(orig_offset);
            if func_range.contains(&orig_target) {
                return None;
            }

            Some(orig_target.wrapping_sub(target_ip + opcode.size() + LONG_OFFSET_SIZE))
        };

        println!("op: {op:02X}");

        match op {
            0x70..=0x7F => {
                /*
                    70: jo    short
                    71: jno   short
                    72: jb    short
                    73: jnb   short
                    74: je    short
                    75: jnz   short
                    76: jbe   short
                    77: ja    short
                    78: js    short
                    79: jns   short
                    7A: jpe   short
                    7B: jpo   short
                    7C: jl    short
                    7D: jge   short
                    7E: jle   short
                    7F: jg    short
                */

                let opcode: u16 = 0x800F | (((op - 0x70) as u16) << 8);

                new_opcode = OpCode::UShort(opcode);  // jc long
                new_offset = calc_new_offset(new_opcode, code[1] as i8 as usize);
            },

            0xEB => {
                // EB: jmp short

                new_opcode = OpCode::Byte(0xE9); // jmp long
                new_offset = calc_new_offset(new_opcode, code[1] as i8 as usize);
            },

            0xE8 | 0xE9 => {
                // E8: call long
                // E9: jmp  long

                new_opcode = OpCode::Byte(op); // jmp long
                new_offset = calc_new_offset(new_opcode, i32::from_le_bytes(code[1..=4].try_into()?) as usize);
            }

            _ => {},
        }

        match new_offset {
            Some(offset) => {
                match new_opcode {
                    OpCode::Byte(b) => {
                        copied_size += buf.write(&[b])?;
                    },

                    OpCode::UShort(w) => {
                        copied_size += buf.write(&w.to_le_bytes())?;
                    },
                }

                copied_size += buf.write(&offset.to_le_bytes())?;
            },

            None => {
                copied_size += buf.write(&code[..opcode_size])?;
            },
        }

        Ok((opcode_size, copied_size))
    }

    fn create_trampoline(&self) -> *mut TrampolineData {
        unsafe {
            let p = HeapAlloc(self.executable_heap, HEAP_ZERO_MEMORY, std::mem::size_of::<TrampolineData>());
            p as *mut TrampolineData
        }
    }

    fn release_trampoline(&self, ptr: *mut TrampolineData) {
        unsafe {
            HeapFree(self.executable_heap, 0, ptr as *const c_void);
        }
    }
}

fn patch(infos: &[PatchInfo]) -> Result<()> {
    Manager::get().patch(infos)
}

pub fn inline_hook_jmp<T>(base_address: usize, rva: usize, target: usize, trampoline: &mut Option<T>, flags: Option<HookFlags>) -> Result<()> {
    Manager::get().patch(&[function_jmp(base_address, rva, target, trampoline, flags)])
}
