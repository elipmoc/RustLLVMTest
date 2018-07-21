extern crate llvm_sys as llvm;
use self::llvm::prelude::{
    LLVMBasicBlockRef, LLVMBuilderRef, LLVMContextRef, LLVMModuleRef, LLVMTypeRef, LLVMValueRef,
};
use super::function::Function;
use std::ffi::CString;
use std::os::raw::c_char;

pub struct CodeGenerator {
    builder: LLVMBuilderRef,
    context: LLVMContextRef,
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

    pub fn position_builder_at_end(&self, block: LLVMBasicBlockRef) {
        unsafe {
            llvm::core::LLVMPositionBuilderAtEnd(self.builder, block);
        }
    }

    pub fn build_alloca(&self, llvm_type: LLVMTypeRef, name: &str) -> LLVMValueRef {
        let name = CString::new(name).unwrap();
        unsafe { llvm::core::LLVMBuildAlloca(self.builder, llvm_type, name.as_ptr()) }
    }

    pub fn build_load(&self, source: LLVMValueRef, dest_name: &str) -> LLVMValueRef {
        let dest_name = CString::new(dest_name).unwrap();
        unsafe { llvm::core::LLVMBuildLoad(self.builder, source, dest_name.as_ptr()) }
    }

    pub fn build_store(&self, source: LLVMValueRef, dest: LLVMValueRef) -> LLVMValueRef {
        unsafe { llvm::core::LLVMBuildStore(self.builder, source, dest) }
    }

    pub fn build_ret(&self, val: LLVMValueRef) -> LLVMValueRef {
        unsafe { llvm::core::LLVMBuildRet(self.builder, val) }
    }

    pub fn dispose(&self) {
        unsafe {
            llvm::core::LLVMDisposeBuilder(self.builder);
            llvm::core::LLVMContextDispose(self.context);
        }
    }

    pub fn build_call(
        &self,
        func: LLVMValueRef,
        mut args: Vec<LLVMValueRef>,
        name: &str,
    ) -> LLVMValueRef {
        unsafe {
            let name = CString::new(name).unwrap();
            llvm::core::LLVMBuildCall(
                self.builder,
                func,
                args.as_mut_ptr(),
                args.len() as u32,
                name.as_ptr(),
            )
        }
    }
}

use super::types::{TargetDataRef, TargetTriple};
#[derive(Clone)]
pub struct Module {
    pub llvm_module: LLVMModuleRef,
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

    pub fn get_named_function(&self, f_name: &str) -> LLVMValueRef {
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

    pub fn get_memory_buffer_ref(&self) -> llvm::prelude::LLVMMemoryBufferRef {
        unsafe { llvm::bit_writer::LLVMWriteBitcodeToMemoryBuffer(self.llvm_module) }
    }

    pub fn write_bitcode_to_file(&self, path: &str) -> i32 {
        let path = CString::new(path).unwrap();
        unsafe { llvm::bit_writer::LLVMWriteBitcodeToFile(self.llvm_module, path.as_ptr()) }
    }
}
