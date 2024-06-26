use lief_ffi as ffi;
use std::marker::PhantomData;

use crate::common::FromFFI;
use crate::declare_iterator;
use crate::to_slice;


#[derive(Debug)]
pub enum Notes<'a> {
    Generic(Generic<'a>)
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Type {
    GNU_ABI_TAG,
    GNU_HWCAP,
    GNU_BUILD_ID,
    GNU_GOLD_VERSION,
    GNU_PROPERTY_TYPE_0,
    GNU_BUILD_ATTRIBUTE_OPEN,
    GNU_BUILD_ATTRIBUTE_FUNC,
    CRASHPAD,
    CORE_PRSTATUS,
    CORE_FPREGSET,
    CORE_PRPSINFO,
    CORE_TASKSTRUCT,
    CORE_AUXV,
    CORE_PSTATUS,
    CORE_FPREGS,
    CORE_PSINFO,
    CORE_LWPSTATUS,
    CORE_LWPSINFO,
    CORE_WIN32PSTATUS,
    CORE_FILE,
    CORE_PRXFPREG,
    CORE_SIGINFO,
    CORE_ARM_VFP,
    CORE_ARM_TLS,
    CORE_ARM_HW_BREAK,
    CORE_ARM_HW_WATCH,
    CORE_ARM_SYSTEM_CALL,
    CORE_ARM_SVE,
    CORE_ARM_PAC_MASK,
    CORE_ARM_PACA_KEYS,
    CORE_ARM_PACG_KEYS,
    CORE_TAGGED_ADDR_CTRL,
    CORE_PAC_ENABLED_KEYS,
    CORE_X86_TLS,
    CORE_X86_IOPERM,
    CORE_X86_XSTATE,
    CORE_X86_CET,
    ANDROID_IDENT,
    ANDROID_MEMTAG,
    ANDROID_KUSER,
    GO_BUILDID,
    STAPSDT,
    UNKNOWN(u32),
}

impl Type {
    pub fn from_value(value: u32) -> Self {
        match value {
            0x00000001 => Type::GNU_ABI_TAG,
            0x00000002 => Type::GNU_HWCAP,
            0x00000003 => Type::GNU_BUILD_ID,
            0x00000004 => Type::GNU_GOLD_VERSION,
            0x00000005 => Type::GNU_PROPERTY_TYPE_0,
            0x00000006 => Type::GNU_BUILD_ATTRIBUTE_OPEN,
            0x00000007 => Type::GNU_BUILD_ATTRIBUTE_FUNC,
            0x00000008 => Type::CRASHPAD,
            0x00000009 => Type::CORE_PRSTATUS,
            0x0000000a => Type::CORE_FPREGSET,
            0x0000000b => Type::CORE_PRPSINFO,
            0x0000000c => Type::CORE_TASKSTRUCT,
            0x0000000d => Type::CORE_AUXV,
            0x0000000e => Type::CORE_PSTATUS,
            0x0000000f => Type::CORE_FPREGS,
            0x00000010 => Type::CORE_PSINFO,
            0x00000011 => Type::CORE_LWPSTATUS,
            0x00000012 => Type::CORE_LWPSINFO,
            0x00000013 => Type::CORE_WIN32PSTATUS,
            0x00000014 => Type::CORE_FILE,
            0x00000015 => Type::CORE_PRXFPREG,
            0x00000016 => Type::CORE_SIGINFO,
            0x00000017 => Type::CORE_ARM_VFP,
            0x00000018 => Type::CORE_ARM_TLS,
            0x00000019 => Type::CORE_ARM_HW_BREAK,
            0x0000001a => Type::CORE_ARM_HW_WATCH,
            0x0000001b => Type::CORE_ARM_SYSTEM_CALL,
            0x0000001c => Type::CORE_ARM_SVE,
            0x0000001d => Type::CORE_ARM_PAC_MASK,
            0x0000001e => Type::CORE_ARM_PACA_KEYS,
            0x0000001f => Type::CORE_ARM_PACG_KEYS,
            0x00000020 => Type::CORE_TAGGED_ADDR_CTRL,
            0x00000021 => Type::CORE_PAC_ENABLED_KEYS,
            0x00000022 => Type::CORE_X86_TLS,
            0x00000023 => Type::CORE_X86_IOPERM,
            0x00000024 => Type::CORE_X86_XSTATE,
            0x00000025 => Type::CORE_X86_CET,
            0x00000026 => Type::ANDROID_IDENT,
            0x00000027 => Type::ANDROID_MEMTAG,
            0x00000028 => Type::ANDROID_KUSER,
            0x00000029 => Type::GO_BUILDID,
            0x0000002a => Type::STAPSDT,
            _ => Type::UNKNOWN(value),

        }
    }
}

pub trait NoteBase {
    #[doc(hidden)]
    fn get_base(&self) -> &ffi::ELF_Note;

    fn name(&self) -> String {
        self.get_base().name().to_string()
    }
    fn get_type(&self) -> Type {
        Type::from_value(self.get_base().get_type())
    }
    fn original_type(&self) -> u32 {
        self.get_base().original_type()
    }
    fn size(&self) -> u64 {
        self.get_base().size()
    }
    fn description(&self) -> &[u8] {
        to_slice!(self.get_base().description());
    }
}

impl NoteBase for Generic<'_> {
    fn get_base(&self) -> &ffi::ELF_Note {
        self.ptr.as_ref().unwrap()
    }
}

impl FromFFI<ffi::ELF_Note> for Notes<'_> {
    fn from_ffi(entry: cxx::UniquePtr<ffi::ELF_Note>) -> Self {
        Notes::Generic(Generic::from_ffi(entry))
    }
}

pub struct Generic<'a> {
    ptr: cxx::UniquePtr<ffi::ELF_Note>,
    _owner: PhantomData<&'a ffi::ELF_Binary>
}


impl std::fmt::Debug for &dyn NoteBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NoteBase")
            .field("name", &self.name())
            .field("type", &self.get_type())
            .field("size", &self.size())
            .finish()
    }
}

impl std::fmt::Debug for Generic<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let base = self as &dyn NoteBase;
        f.debug_struct("GenericNote")
            .field("base", &base)
            .finish()
    }
}

impl FromFFI<ffi::ELF_Note> for Generic<'_> {
    fn from_ffi(cmd: cxx::UniquePtr<ffi::ELF_Note>) -> Self {
        Self {
            ptr: cmd,
            _owner: PhantomData
        }
    }
}

declare_iterator!(ItNotes, Notes<'a>, ffi::ELF_Note, ffi::ELF_Binary, ffi::ELF_Binary_it_notes);
