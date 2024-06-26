use super::Command;
use lief_ffi as ffi;
use crate::common::FromFFI;

use std::marker::PhantomData;

pub struct SourceVersion<'a> {
    ptr: cxx::UniquePtr<ffi::MachO_SourceVersion>,
    _owner: PhantomData<&'a ffi::MachO_Binary>
}

impl SourceVersion<'_> {
    pub fn version(&self) -> Vec<u64> {
        Vec::from(self.ptr.version().as_slice())
    }
}

impl std::fmt::Debug for SourceVersion<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let base = self as &dyn Command;
        f.debug_struct("SourceVersion")
            .field("base", &base)
            .field("version", &self.version())
            .finish()
    }
}

impl<'a> FromFFI<ffi::MachO_SourceVersion> for SourceVersion<'a> {
    fn from_ffi(cmd: cxx::UniquePtr<ffi::MachO_SourceVersion>) -> Self {
        Self {
            ptr: cmd,
            _owner: PhantomData
        }
    }
}

impl Command for SourceVersion<'_> {
    fn get_base(&self) -> &ffi::MachO_Command {
        self.ptr.as_ref().unwrap().as_ref()
    }
}
