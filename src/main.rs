use std::env;
use std::fs;

mod frontend;
mod ir;
mod backend;

use frontend::parser::Parser;
use backend::riscv::RiscVGenerator;
use ir::IRType;

fn convert_solidity_type_to_ir_type(solidity_type: &str) -> IRType {
    match solidity_type {
        "uint" => IRType::I64,
        "int" => IRType::I64,
        "bool" => IRType::Bool,
        "address" => IRType::Address,
        "string" => IRType::String,
        _ => panic!("Unsupported type: {}", solidity_type),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input.sol> <output.s>", args[0]);
        std::process::exit(1);
    }

    let input_file = &args[1];
    let output_file = &args[2];

    // 读取输入文件
    let source = fs::read_to_string(input_file)?;

    // 解析 Solidity 代码
    let mut parser = Parser::new(&source);
    let functions = parser.parse()?;

    // 生成 RISC-V 汇编代码
    let mut generator = RiscVGenerator::new();
    let mut asm = String::new();

    // 添加汇编文件头
    asm.push_str(".text\n");
    asm.push_str(".align 2\n\n");

    // 为每个函数生成汇编代码
    for func in functions {
        let mut ir_func = ir::IRFunction::new(func.name.clone());
        
        // 添加参数
        for (param_type, param_name) in &func.params {
            let ir_type = convert_solidity_type_to_ir_type(param_type);
            ir_func.params.push((param_name.clone(), ir_type));
        }

        // 添加返回值
        for return_type in &func.returns {
            let ir_type = convert_solidity_type_to_ir_type(return_type);
            ir_func.returns.push(ir_type);
        }

        let func_asm = generator.generate(&ir_func);
        asm.push_str(&func_asm);
        asm.push_str("\n");
    }

    // 写入输出文件
    fs::write(output_file, asm)?;

    println!("Compilation successful!");
    Ok(())
}
