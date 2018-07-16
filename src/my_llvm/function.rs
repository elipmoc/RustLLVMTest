extern crate llvm_sys as llvm;
use std::ffi::CString;

pub struct Function {
    pub llvm_function: *mut llvm::LLVMValue,
}

impl Function {
    pub fn new(
        f_name: &str,
        module: &super::core::Module,
        f_type: *mut llvm::LLVMType,
    ) -> Function {
        unsafe {
            let f_name = CString::new(f_name).unwrap();
            Function {
                llvm_function: llvm::core::LLVMAddFunction(
                    module.llvm_module,
                    f_name.as_ptr(),
                    f_type,
                ),
            }
        }
    }
    pub fn append_basic_block(&self, name: &str) -> *mut llvm::LLVMBasicBlock {
        unsafe {
            let name = CString::new(name).unwrap();
            llvm::core::LLVMAppendBasicBlock(self.llvm_function, name.as_ptr())
        }
    }
}
