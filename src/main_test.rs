use std::env;
use std::fs;

mod frontend;
mod ir;
mod backend;

use frontend::parser::Parser;
use frontend::lexer::Lexer;
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

fn main() {
    // 读取测试文件
    let input = std::fs::read_to_string("test.sol").expect("Failed to read test.sol");
    
    // 词法分析
    println!("=== 词法分析结果 ===");
    let mut lexer = Lexer::new(&input);
    let tokens: Vec<_> = lexer.collect();
    for (token, slice) in &tokens {
        println!("Token: {:?}, Slice: {}", token, slice);
    }
    
    // 语法分析
    println!("\n=== 语法分析结果 ===");
    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(functions) => {
            for func in functions {
                println!("函数名: {}", func.name);
                println!("参数: {:?}", func.params);
                println!("返回值: {:?}", func.returns);
                println!("函数体: {:?}", func.body);
                println!();
            }
        }
        Err(e) => println!("解析错误: {}", e),
    }
}
