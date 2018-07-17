mod my_llvm;
use my_llvm::core::*;
use my_llvm::execution_engine::*;
use my_llvm::function::*;
use my_llvm::generic_value::*;
use my_llvm::target::*;
use my_llvm::types::*;
use my_llvm::value::*;

#[link(name = "foo", kind = "static")]
extern "C" {
    fn foo();
}

fn main() {
    init_llvm_all_target();
    let codegen = CodeGenerator::new();
    let module = Module::new("my_module");
    let function_type = function_type(int32_type(), vec![]);
    let function = Function::new("hoge", &module, function_type);
    let entry_block = function.append_basic_block("entry");
    codegen.position_builder_at_end(entry_block);
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
    /*  linkin_interpreter();
    let mut exe_engin = ExecutionEngine::new();
    if let Some(err_msg) = exe_engin.create_interpreter_for_module(&module) {
        panic!("llvm error:{}", err_msg);
    }
    let main_function = module.get_named_function("main");
    let func_result = exe_engin.run_function(main_function, vec![]);
    println!("{}", generic_value_to_int(func_result, false));*/
    module.dispose_module();
    unsafe {
        foo();
    }
}
