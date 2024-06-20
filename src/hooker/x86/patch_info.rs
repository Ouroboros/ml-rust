#![allow(dead_code, unused_imports)]

use std::{fmt, ptr::addr_of};
use bitflags::{bitflags, Flag};

bitflags! {
    #[derive(Debug)]
    pub struct HookFlags: u32 {
        const VirtualAddress    = 0b00000001;
        const NakedTrampoline   = 0b00000010;
        const KeepRawTrampoline = 0b00000100;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum HookType {
    Jump,
    Call,
    Push,
}

impl HookType {
    pub fn size_of_opcode(&self) -> usize {
        match self {
            Self::Jump | Self::Call => {
                /*
                    E8 00000000  call    const
                    E9 00000000  jmp     const
                */

                5
            },

            Self::Push => {
                /*
                    68 00000000  push const
                    C3           ret
                */

                6
            },
        }
    }
}

#[derive(Debug)]
pub enum Value {
    Value(u64),
    Bytes(Vec<u8>),
}

pub struct MemoryInfo {
    pub base_address    : usize,
    pub rva             : usize,
    pub value           : Value,
    pub size            : usize,
    pub flags           : HookFlags,
}

const TRAMPOLINE_SIZE: usize = 0x40;

#[repr(C)]
pub struct TrampolineData {
    pub trampoline      : [u8; TRAMPOLINE_SIZE],
    pub original_code   : [u8; TRAMPOLINE_SIZE],
    pub trampoline_size : usize,
    pub original_size   : usize,
    pub jump_back_addr  : usize,
}

type TrampolineDataPtr = *mut Option<fn()>;

impl TrampolineData {
    pub fn new() -> Self {
        Self {
            trampoline        : [0; TRAMPOLINE_SIZE],
            original_code     : [0; TRAMPOLINE_SIZE],
            trampoline_size   : 0,
            original_size     : 0,
            jump_back_addr    : 0,
        }
    }
}

pub struct FunctionInfo {
    pub base_address    : usize,
    pub rva             : usize,
    pub target          : usize,
    pub trampoline      : Option<TrampolineDataPtr>,
    pub hook_type       : HookType,
    pub flags           : HookFlags,
}

impl FunctionInfo {
    pub fn get_size_of_hook_opcode(&self) -> usize {
        match self.hook_type {
            HookType::Jump => 5,
            HookType::Call => 5,
            HookType::Push => 6,
        }
    }

    pub fn virtual_address(&self) -> usize {
        self.base_address + self.rva
    }
}

#[derive(Debug)]
pub enum PatchInfo {
    Memory(MemoryInfo),
    Function(FunctionInfo),
}

impl fmt::Debug for MemoryInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MemoryInfo")
            .field("base_address",  &format_args!("0x{:08X}", &self.base_address))
            .field("rva",           &format_args!("0x{:08X}", &self.rva))
            .field("value",         &self.value)
            .field("size",          &format_args!("0x{:02X}", &self.size))
            .field("flags",         &self.flags)
            .finish()
    }
}

impl fmt::Display for MemoryInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Debug for FunctionInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("FunctionInfo")
            .field("base_address",  &format_args!("0x{:08X}", &self.base_address))
            .field("rva",           &format_args!("0x{:08X}", &self.rva))
            .field("target",        &format_args!("0x{:08X}", self.target))
            .field("trampoline",    &format_args!("{}", self.trampoline.map_or_else(|| "None".to_string(), |v| format!("0x{:08X}", v as usize))))
            .field("hook_type",     &self.hook_type)
            .finish()
    }
}

impl fmt::Display for FunctionInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub(crate) fn memory(base_address: usize, rva: usize, value: Value, size: usize) -> PatchInfo {
    PatchInfo::Memory(MemoryInfo{
        base_address,
        rva,
        value,
        size,
        flags: HookFlags::empty(),
    })
}

pub(crate) fn memory_value(base_address: usize, rva: usize, value: u64, size: usize) -> PatchInfo {
    memory(base_address, rva, Value::Value(value), size)
}

pub(crate) fn memory_bytes(base_address: usize, rva: usize, bytes: &[u8]) -> PatchInfo {
    memory(base_address, rva, Value::Bytes(bytes.into()), bytes.len())
}

pub(crate) fn function<T>(base_address: usize, rva: usize, target: usize, hook_type: HookType, trampoline: Option<&mut Option<T>>, flags: Option<HookFlags>) -> PatchInfo {
    PatchInfo::Function(FunctionInfo{
        base_address,
        rva,
        target,
        trampoline: if let Some(trampoline) = trampoline { trampoline_addr(trampoline) } else { None },
        hook_type,
        flags: flags.map_or(HookFlags::empty(), |v| v),
    })
}

pub(crate) fn function_jmp<T>(base_address: usize, rva: usize, target: usize, trampoline: Option<&mut Option<T>>, flags: Option<HookFlags>) -> PatchInfo {
    function(
        base_address,
        rva,
        target,
        HookType::Jump,
        trampoline,
        flags,
    )
}

pub(crate) fn function_call<T>(base_address: usize, rva: usize, target: usize, trampoline: Option<&mut Option<T>>, flags: Option<HookFlags>) -> PatchInfo {
    function(
        base_address,
        rva,
        target,
        HookType::Call,
        trampoline,
        flags,
    )
}

pub(crate) fn function_push<T>(base_address: usize, rva: usize, target: usize, trampoline: Option<&mut Option<T>>, flags: Option<HookFlags>) -> PatchInfo {
    function(
        base_address,
        rva,
        target,
        HookType::Push,
        trampoline,
        flags,
    )
}

fn trampoline_addr<T>(p: &mut Option<T>) -> Option<TrampolineDataPtr> {
    unsafe {
        Some(std::mem::transmute(p))
    }
}
