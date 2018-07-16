extern crate llvm_sys as llvm;
use self::llvm::execution_engine as llex;
use std::ffi::CString;
use std::os::raw::c_char;

pub fn linkin_interpreter() {
    unsafe {
        llex::LLVMLinkInInterpreter();
    }
}

pub struct ExecutionEngine {
    engine: llex::LLVMExecutionEngineRef,
}

impl ExecutionEngine {
    pub fn new() -> ExecutionEngine {
        let engine = 0 as llex::LLVMExecutionEngineRef;
        ExecutionEngine { engine: engine }
    }

    pub fn create_interpreter_for_module(
        &mut self,
        module: &super::core::Module,
    ) -> Option<String> {
        let mut error = 0 as *mut c_char;
        let ok = unsafe {
            let buf: *mut *mut c_char = &mut error;
            llex::LLVMCreateInterpreterForModule(&mut self.engine, module.llvm_module, buf)
        };
        if ok == 0 {
            Option::None
        } else {
            Option::Some(unsafe { CString::from_raw(error).into_string().unwrap() })
        }
    }

    pub fn run_function(
        &self,
        function: *mut llvm::LLVMValue,
        mut params: Vec<llex::LLVMGenericValueRef>,
    ) -> llex::LLVMGenericValueRef {
        unsafe {
            llex::LLVMRunFunction(
                self.engine,
                function,
                params.len() as u32,
                params.as_mut_ptr(),
            )
        }
    }
}

pub fn excalibur() {
    panic!("Excalibur error!");
}
