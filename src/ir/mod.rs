use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum IRType {
    I32,
    I64,
    Bool,
    Address,
    String,
}

#[derive(Debug, Clone)]
pub struct IRFunction {
    pub name: String,
    pub params: Vec<(String, IRType)>,
    pub returns: Vec<IRType>,
    pub locals: HashMap<String, IRType>,
    pub blocks: Vec<BasicBlock>,
}

#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub label: String,
    pub instructions: Vec<Instruction>,
    pub terminator: Terminator,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    // 算术运算
    Add(Value, Value),
    Sub(Value, Value),
    Mul(Value, Value),
    Div(Value, Value),
    
    // 比较运算
    Eq(Value, Value),
    Ne(Value, Value),
    Lt(Value, Value),
    Gt(Value, Value),
    
    // 内存操作
    Load(Value),
    Store(Value, Value),
    
    // 函数调用
    Call(String, Vec<Value>),
    
    // 类型转换
    Trunc(Value),
    Extend(Value),
}

#[derive(Debug, Clone)]
pub enum Terminator {
    Return(Option<Value>),
    Branch(Value, String, String), // condition, true_label, false_label
    Jump(String),
}

#[derive(Debug, Clone)]
pub enum Value {
    Constant(Constant),
    Local(String),
    Temporary(u32),
}

#[derive(Debug, Clone)]
pub enum Constant {
    I32(i32),
    I64(i64),
    Bool(bool),
    Address(String),
    String(String),
}

impl IRFunction {
    pub fn new(name: String) -> Self {
        Self {
            name,
            params: Vec::new(),
            returns: Vec::new(),
            locals: HashMap::new(),
            blocks: Vec::new(),
        }
    }

    pub fn add_block(&mut self, block: BasicBlock) {
        self.blocks.push(block);
    }

    pub fn add_local(&mut self, name: String, ty: IRType) {
        self.locals.insert(name, ty);
    }
} 