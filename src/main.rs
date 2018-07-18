mod my_llvm;
use my_llvm::core::*;
use my_llvm::execution_engine::*;
use my_llvm::function::*;
use my_llvm::generic_value::*;
use my_llvm::target::*;
use my_llvm::types::*;
use my_llvm::value::*;
use std::process::Command;

fn extern_foo(module: &Module) -> Function {
    let function_type = function_type(void_type(), vec![]);
    let function = Function::new("foo", &module, function_type);
    setLinkage(function.llvm_function, LLVMLinkage::LLVMExternalLinkage);
    function
}

fn main() {
    init_llvm_all_target();
    let codegen = CodeGenerator::new();
    let module = Module::new("my_module");
    let foo_func = extern_foo(&module);
    let function_type = function_type(int32_type(), vec![]);
    let function = Function::new("main", &module, function_type);
    let entry_block = function.append_basic_block("entry");
    codegen.position_builder_at_end(entry_block);
    codegen.build_call(foo_func.llvm_function, vec![], "");
    let a_value = codegen.build_alloca(int32_type(), "a");
    codegen.build_store(const_int(int32_type(), 114, false), a_value);
    let b_value = codegen.build_load(a_value, "b");
    codegen.build_ret(b_value);
    if let Some(err_msg) = module.verify_module() {
        panic!("llvm error:{}", err_msg);
    }
    codegen.dispose_builder();
    module.dump_module();
    let target_machine = TargetMachine::create(
        "generic",
        "",
        LLVMCodeGenOptLevel::LLVMCodeGenLevelDefault,
        LLVMRelocMode::LLVMRelocDefault,
        LLVMCodeModel::LLVMCodeModelDefault,
    ).unwrap();
    module.set_data_layout(target_machine.create_data_layout());
    module.set_target_triple(target_machine.target_triple);
    module.write_bitcode_to_file("hoge.bc");
    module.dispose_module();

    let current_dir = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "llc", "-march=x86-64", "-filetype=obj", "hoge.bc"])
            .output()
            .expect("failed to execute process");
        Command::new("cmd")
            .args(&[
                "/C",
                &(current_dir.clone() + "\\compile.bat"),
                &(current_dir + "\\hoge.obj"),
            ])
            .output()
            .expect("failed to execute process")
    } else {
        panic!("support windows only!!")
    };
}
