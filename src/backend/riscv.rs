use std::collections::HashMap;
use crate::ir::{IRFunction, Value, Instruction, Terminator};

#[allow(dead_code)]
pub struct RiscVGenerator {
    registers: HashMap<String, String>,
    stack_offset: i32,
    current_function: Option<String>,
}

impl RiscVGenerator {
    pub fn new() -> Self {
        Self {
            registers: HashMap::new(),
            stack_offset: 0,
            current_function: None,
        }
    }

    pub fn generate(&mut self, func: &IRFunction) -> String {
        self.current_function = Some(func.name.clone());
        let mut asm = String::new();

        // 生成函数头
        asm.push_str(&format!(".globl {}\n", func.name));
        asm.push_str(&format!("{}:\n", func.name));

        // 生成序言
        asm.push_str("    addi sp, sp, -16\n");  // 分配栈空间
        asm.push_str("    sd ra, 8(sp)\n");      // 保存返回地址
        asm.push_str("    sd fp, 0(sp)\n");      // 保存帧指针
        asm.push_str("    addi fp, sp, 16\n");   // 设置新的帧指针

        // 生成函数体
        for block in &func.blocks {
            asm.push_str(&format!("{}:\n", block.label));
            
            for inst in &block.instructions {
                asm.push_str(&self.generate_instruction(inst));
            }
            
            asm.push_str(&self.generate_terminator(&block.terminator));
        }

        // 生成结语
        asm.push_str(".Lreturn:\n");
        asm.push_str("    ld ra, 8(sp)\n");      // 恢复返回地址
        asm.push_str("    ld fp, 0(sp)\n");      // 恢复帧指针
        asm.push_str("    addi sp, sp, 16\n");   // 释放栈空间
        asm.push_str("    ret\n");               // 返回

        asm
    }

    fn generate_instruction(&mut self, inst: &Instruction) -> String {
        match inst {
            Instruction::Add(lhs, rhs) => {
                let lhs_reg = self.get_register(lhs);
                let rhs_reg = self.get_register(rhs);
                format!("    add {}, {}, {}\n", lhs_reg, lhs_reg, rhs_reg)
            }
            Instruction::Sub(lhs, rhs) => {
                let lhs_reg = self.get_register(lhs);
                let rhs_reg = self.get_register(rhs);
                format!("    sub {}, {}, {}\n", lhs_reg, lhs_reg, rhs_reg)
            }
            Instruction::Mul(lhs, rhs) => {
                let lhs_reg = self.get_register(lhs);
                let rhs_reg = self.get_register(rhs);
                format!("    mul {}, {}, {}\n", lhs_reg, lhs_reg, rhs_reg)
            }
            Instruction::Div(lhs, rhs) => {
                let lhs_reg = self.get_register(lhs);
                let rhs_reg = self.get_register(rhs);
                format!("    div {}, {}, {}\n", lhs_reg, lhs_reg, rhs_reg)
            }
            _ => String::new(), // TODO: 实现其他指令
        }
    }

    fn generate_terminator(&mut self, term: &Terminator) -> String {
        match term {
            Terminator::Return(val) => {
                if let Some(v) = val {
                    let reg = self.get_register(v);
                    format!("    mv a0, {}\n    j .Lreturn\n", reg)
                } else {
                    "    j .Lreturn\n".to_string()
                }
            }
            Terminator::Branch(cond, true_label, false_label) => {
                let cond_reg = self.get_register(cond);
                format!("    bnez {}, {}\n    j {}\n", cond_reg, true_label, false_label)
            }
            Terminator::Jump(label) => {
                format!("    j {}\n", label)
            }
        }
    }

    fn get_register(&mut self, _val: &Value) -> String {
        // TODO: 实现寄存器分配
        "a0".to_string()
    }
} 