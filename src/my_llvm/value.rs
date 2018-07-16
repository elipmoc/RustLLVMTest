extern crate llvm_sys as llvm;

pub fn const_int(
    llvm_type: *mut llvm::LLVMType,
    val: u64,
    sign_flag: bool,
) -> *mut llvm::LLVMValue {
    unsafe { llvm::core::LLVMConstInt(llvm_type, val, if sign_flag { 1 } else { 0 }) }
}
