extern crate llvm_sys as llvm;
use std::os::raw::c_char;

pub type TargetDataRef = *mut llvm::target::LLVMOpaqueTargetData;
pub type TargetTriple = *mut c_char;

pub fn int32_type() -> *mut llvm::LLVMType {
    unsafe { llvm::core::LLVMInt32Type() }
}

pub fn function_type(
    ret_type: *mut llvm::LLVMType,
    mut param_types: Vec<*mut llvm::LLVMType>,
) -> *mut llvm::LLVMType {
    unsafe {
        let function_type = llvm::core::LLVMFunctionType(
            ret_type,
            param_types.as_mut_ptr(),
            param_types.len() as u32,
            0,
        );
        function_type
    }
}
