extern crate llvm_sys as llvm;
use super::function::Function;
use std::ffi::CString;
use std::os::raw::c_char;

pub struct CodeGenerator {
    builder: *mut llvm::LLVMBuilder,
    context: *mut llvm::LLVMContext,
}
impl CodeGenerator {
    pub fn new() -> CodeGenerator {
        unsafe {
            CodeGenerator {
                builder: llvm::core::LLVMCreateBuilder(),
                context: llvm::core::LLVMContextCreate(),
            }
        }
    }

    pub fn position_builder_at_end(&self, block: *mut llvm::LLVMBasicBlock) {
        unsafe {
            llvm::core::LLVMPositionBuilderAtEnd(self.builder, block);
        }
    }

    pub fn build_alloca(&self, llvm_type: *mut llvm::LLVMType, name: &str) -> *mut llvm::LLVMValue {
        let name = CString::new(name).unwrap();
        unsafe { llvm::core::LLVMBuildAlloca(self.builder, llvm_type, name.as_ptr()) }
    }

    pub fn build_load(
        &self,
        source: *mut llvm::LLVMValue,
        dest_name: &str,
    ) -> *mut llvm::LLVMValue {
        let dest_name = CString::new(dest_name).unwrap();
        unsafe { llvm::core::LLVMBuildLoad(self.builder, source, dest_name.as_ptr()) }
    }

    pub fn build_store(
        &self,
        source: *mut llvm::LLVMValue,
        dest: *mut llvm::LLVMValue,
    ) -> *mut llvm::LLVMValue {
        unsafe { llvm::core::LLVMBuildStore(self.builder, source, dest) }
    }

    pub fn build_ret(&self, val: *mut llvm::LLVMValue) -> *mut llvm::LLVMValue {
        unsafe { llvm::core::LLVMBuildRet(self.builder, val) }
    }

    pub fn dispose_builder(&self) {
        unsafe { llvm::core::LLVMDisposeBuilder(self.builder) }
    }
}

use super::types::{TargetDataRef, TargetTriple};
#[derive(Clone)]
pub struct Module {
    pub llvm_module: *mut llvm::LLVMModule,
}

impl Module {
    pub fn new(m_name: &str) -> Module {
        unsafe {
            let m_name = CString::new(m_name).unwrap();
            let module = llvm::core::LLVMModuleCreateWithName(m_name.as_ptr());
            Module {
                llvm_module: module,
            }
        }
    }

    pub fn verify_module(&self) -> Option<String> {
        let mut error: *mut c_char = 0 as *mut c_char;
        let ok = unsafe {
            let buf: *mut *mut c_char = &mut error;
            llvm::analysis::LLVMVerifyModule(
                self.llvm_module,
                llvm::analysis::LLVMVerifierFailureAction::LLVMReturnStatusAction,
                buf,
            )
        };
        if ok == 0 {
            Option::None
        } else {
            Option::Some(unsafe { CString::from_raw(error).into_string().unwrap() })
        }
    }

    pub fn get_named_function(&self, f_name: &str) -> *mut llvm::LLVMValue {
        let f_name = CString::new(f_name).unwrap();
        unsafe { llvm::core::LLVMGetNamedFunction(self.llvm_module, f_name.as_ptr()) }
    }

    pub fn dump_module(&self) {
        unsafe { llvm::core::LLVMDumpModule(self.llvm_module) }
    }

    pub fn dispose_module(self) {
        unsafe {
            llvm::core::LLVMDisposeModule(self.llvm_module);
        }
    }
    pub fn set_data_layout(&self, data_layout: TargetDataRef) {
        unsafe { llvm::core::LLVMSetDataLayout(self.llvm_module, data_layout as *const c_char) }
    }

    pub fn set_target_triple(&self, target_triple: TargetTriple) {
        unsafe { llvm::core::LLVMSetTarget(self.llvm_module, target_triple) }
    }

    pub fn get_memoryBufferRef(&self) -> llvm::prelude::LLVMMemoryBufferRef {
        unsafe { llvm::bit_writer::LLVMWriteBitcodeToMemoryBuffer(self.llvm_module) }
    }

    pub fn write_bitcode_to_file(&self, path: &str) -> i32 {
        let path = CString::new(path).unwrap();
        unsafe { llvm::bit_writer::LLVMWriteBitcodeToFile(self.llvm_module, path.as_ptr()) }
    }
}
