#![allow(dead_code, unused_imports)]

use std::fmt;
use bitflags::{bitflags, Flag};

bitflags! {
    #[derive(Debug)]
    pub struct Flags: u32 {
        const VirtualAddress    = 0b00000001;
        const NakedTrampoline   = 0b00000010;
    }
}

#[derive(Debug)]
pub enum HookType {
    Jump,
    Call,
    Push,
}

#[derive(Debug)]
pub enum Value {
    Value(u64),
    Bytes(Vec<u8>),
}

pub struct MemoryInfo {
    pub base_address    : usize,
    pub rva : usize,
    pub value           : Value,
    pub size            : usize,
    pub flags           : Flags,
}

pub struct FunctionInfo {
    pub base_address    : usize,
    pub virtual_address : usize,
    pub target          : usize,
    pub trampoline      : Option<usize>,
    pub hook_type       : HookType,
    pub flags           : Flags,
}

impl FunctionInfo {
    pub fn get_size_of_hook_opcode(&self) -> usize {
        match self.hook_type {
            HookType::Jump => 5,
            HookType::Call => 5,
            HookType::Push => 6,
        }
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
            .field("base_address",      &format_args!("0x{:08X}", &self.base_address))
            .field("virtual_address",   &format_args!("0x{:08X}", &self.rva))
            .field("value",             &self.value)
            .field("size",              &format_args!("0x{:02X}", &self.size))
            .field("flags",             &self.flags)
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
            .field("base_address",      &format_args!("0x{:08X}", &self.base_address))
            .field("virtual_address",   &format_args!("0x{:08X}", &self.virtual_address))
            .field("target",            &format_args!("0x{:08X}", self.target))
            .field("trampoline",        &format_args!("{}", self.trampoline.map_or_else(|| "None".to_string(), |v| format!("0x{:08X}", v))))
            .field("hook_type",         &self.hook_type)
            .finish()
    }
}

impl fmt::Display for FunctionInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn memory(base_address: usize, virtual_address: usize, value: Value, size: usize) -> PatchInfo {
    PatchInfo::Memory(MemoryInfo{
        base_address,
        rva: virtual_address,
        value,
        size,
        flags: Flags::empty(),
    })
}

pub fn memory_value(base_address: usize, virtual_address: usize, value: u64, size: usize) -> PatchInfo {
    memory(base_address, virtual_address, Value::Value(value), size)
}

pub fn memory_bytes(base_address: usize, virtual_address: usize, bytes: &[u8]) -> PatchInfo {
    memory(base_address, virtual_address, Value::Bytes(bytes.into()), bytes.len())
}

pub fn function(base_address: usize, virtual_address: usize, target: usize, hook_type: HookType, trampoline: Option<usize>, flags: Option<Flags>) -> PatchInfo {
    PatchInfo::Function(FunctionInfo{
        base_address,
        virtual_address,
        target,
        trampoline,
        hook_type,
        flags: flags.map_or(Flags::empty(), |v| v),
    })
}

pub fn function_jmp(base_address: usize, virtual_address: usize, target: usize, trampoline: Option<usize>, flags: Option<Flags>) -> PatchInfo {
    function(
        base_address,
        virtual_address,
        target,
        HookType::Jump,
        trampoline,
        flags,
    )
}

pub fn function_call(base_address: usize, virtual_address: usize, target: usize, trampoline: Option<usize>, flags: Option<Flags>) -> PatchInfo {
    function(
        base_address,
        virtual_address,
        target,
        HookType::Call,
        trampoline,
        flags,
    )
}

pub fn function_push(base_address: usize, virtual_address: usize, target: usize, trampoline: Option<usize>, flags: Option<Flags>) -> PatchInfo {
    function(
        base_address,
        virtual_address,
        target,
        HookType::Push,
        trampoline,
        flags,
    )
}

pub fn trampoline_addr<T>(func: *const T) -> Option<usize> {
    Some(func as usize)
}

pub fn trampoline_addr_mut<T>(func: *mut T) -> Option<usize> {
    Some(func as usize)
}
