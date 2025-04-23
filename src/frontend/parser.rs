use crate::frontend::lexer::{Lexer, Token};

#[derive(Debug)]
pub enum Expr {
    Number(i64),
    String(String),
    Boolean(bool),
    Identifier(String),
    BinaryOp(Box<Expr>, BinaryOp, Box<Expr>),
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equals,
    NotEquals,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<(String, String)>, // (type, name)
    pub returns: Vec<String>,
    pub body: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    VariableDecl(String, String, Option<Expr>), // type, name, initializer
    Assignment(String, Expr),
    Return(Option<Expr>),
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Lexer::new(input),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Function>, String> {
        let mut functions = Vec::new();
        
        while let Some((token, _)) = self.lexer.next() {
            match token {
                Token::Function => {
                    if let Some(func) = self.parse_function()? {
                        functions.push(func);
                    }
                }
                _ => continue,
            }
        }
        
        Ok(functions)
    }

    fn parse_function(&mut self) -> Result<Option<Function>, String> {
        // 解析函数名
        let name = match self.lexer.next() {
            Some((Token::Identifier, name)) => name.to_string(),
            _ => return Ok(None),
        };

        // 解析参数列表
        let params = self.parse_params()?;

        // 解析返回值
        let returns = self.parse_returns()?;

        // 解析函数体
        let body = self.parse_function_body()?;

        Ok(Some(Function {
            name,
            params,
            returns,
            body,
        }))
    }

    fn parse_params(&mut self) -> Result<Vec<(String, String)>, String> {
        let mut params = Vec::new();
        
        // 检查左括号
        match self.lexer.next() {
            Some((Token::LParen, _)) => (),
            _ => return Err("Expected '(' after function name".to_string()),
        }

        // 如果下一个 token 是右括号，说明没有参数
        match self.lexer.next() {
            Some((Token::RParen, _)) => return Ok(params),
            Some((token, _)) => {
                let param_type = match token {
                    Token::Uint => "uint",
                    Token::Int => "int",
                    Token::Bool => "bool",
                    Token::Address => "address",
                    Token::String => "string",
                    _ => return Err("Expected parameter type".to_string()),
                }.to_string();

                match self.lexer.next() {
                    Some((Token::Identifier, name)) => {
                        params.push((param_type, name.to_string()));
                    }
                    _ => return Err("Expected parameter name".to_string()),
                }
            }
            None => return Err("Expected parameter type".to_string()),
        }

        // 解析更多参数
        loop {
            match self.lexer.next() {
                Some((Token::Comma, _)) => {
                    match self.lexer.next() {
                        Some((token, _)) => {
                            let param_type = match token {
                                Token::Uint => "uint",
                                Token::Int => "int",
                                Token::Bool => "bool",
                                Token::Address => "address",
                                Token::String => "string",
                                _ => return Err("Expected parameter type".to_string()),
                            }.to_string();

                            match self.lexer.next() {
                                Some((Token::Identifier, name)) => {
                                    params.push((param_type, name.to_string()));
                                }
                                _ => return Err("Expected parameter name".to_string()),
                            }
                        }
                        None => return Err("Expected parameter type".to_string()),
                    }
                }
                Some((Token::RParen, _)) => break,
                _ => return Err("Expected ',' or ')'".to_string()),
            }
        }

        Ok(params)
    }

    fn parse_returns(&mut self) -> Result<Vec<String>, String> {
        let mut returns = Vec::new();
        
        // 检查 returns 关键字
        match self.lexer.next() {
            Some((Token::Returns, _)) => (),
            Some(_) => return Ok(returns), // 如果不是 returns，说明没有返回值
            None => return Ok(returns),    // 如果到达输入末尾，也说明没有返回值
        }

        // 检查左括号
        match self.lexer.next() {
            Some((Token::LParen, _)) => (),
            _ => return Err("Expected '(' after returns".to_string()),
        }

        loop {
            // 解析返回类型
            let return_type = match self.lexer.next() {
                Some((Token::Uint, _)) => "uint".to_string(),
                Some((Token::Int, _)) => "int".to_string(),
                Some((Token::Bool, _)) => "bool".to_string(),
                Some((Token::Address, _)) => "address".to_string(),
                Some((Token::String, _)) => "string".to_string(),
                Some((Token::RParen, _)) => break,
                _ => return Err("Expected return type".to_string()),
            };

            if !return_type.is_empty() {
                returns.push(return_type);

                // 检查是否有更多返回类型
                match self.lexer.next() {
                    Some((Token::Comma, _)) => continue,
                    Some((Token::RParen, _)) => break,
                    _ => return Err("Expected ',' or ')'".to_string()),
                }
            }
        }

        Ok(returns)
    }

    fn parse_function_body(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();
        
        // 检查左大括号
        match self.lexer.next() {
            Some((Token::LBrace, _)) => (),
            _ => return Err("Expected '{' at start of function body".to_string()),
        }

        loop {
            match self.lexer.next() {
                Some((Token::RBrace, _)) => break,
                Some((Token::Return, _)) => {
                    // 处理 return 语句
                    match self.lexer.peek() {
                        Some((Token::Semicolon, _)) => {
                            self.lexer.next(); // 消耗分号
                            statements.push(Statement::Return(None));
                        }
                        Some(_) => {
                            let expr = self.parse_expr()?;
                            statements.push(Statement::Return(Some(expr)));
                            
                            // 确保有分号
                            match self.lexer.next() {
                                Some((Token::Semicolon, _)) => (),
                                _ => return Err("Expected ';' after return expression".to_string()),
                            }
                        }
                        None => return Err("Unexpected end of input".to_string()),
                    }
                }
                Some((Token::Uint, _)) | Some((Token::Int, _)) | Some((Token::Bool, _)) | 
                Some((Token::Address, _)) | Some((Token::String, _)) => {
                    // 变量声明
                    let var_type = match self.lexer.next() {
                        Some((Token::Uint, _)) => "uint",
                        Some((Token::Int, _)) => "int",
                        Some((Token::Bool, _)) => "bool",
                        Some((Token::Address, _)) => "address",
                        Some((Token::String, _)) => "string",
                        _ => return Err("Expected variable type".to_string()),
                    }.to_string();

                    let var_name = match self.lexer.next() {
                        Some((Token::Identifier, name)) => name.to_string(),
                        _ => return Err("Expected variable name".to_string()),
                    };

                    let initializer = match self.lexer.next() {
                        Some((Token::Equals, _)) => Some(self.parse_expr()?),
                        Some((Token::Semicolon, _)) => None,
                        _ => return Err("Expected '=' or ';'".to_string()),
                    };

                    if initializer.is_some() {
                        // 如果有初始化器，确保有分号
                        match self.lexer.next() {
                            Some((Token::Semicolon, _)) => (),
                            _ => return Err("Expected ';' after variable initialization".to_string()),
                        }
                    }

                    statements.push(Statement::VariableDecl(var_type, var_name, initializer));
                }
                Some(_) => continue,
                None => return Err("Unexpected end of input".to_string()),
            }
        }

        Ok(statements)
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        let mut expr = match self.lexer.next() {
            Some((Token::Identifier, name)) => Ok(Expr::Identifier(name.to_string())),
            Some((Token::Number, num)) => {
                match num.parse::<i64>() {
                    Ok(n) => Ok(Expr::Number(n)),
                    Err(_) => Err("Invalid number".to_string()),
                }
            }
            Some((Token::LParen, _)) => {
                let inner_expr = self.parse_expr()?;
                match self.lexer.next() {
                    Some((Token::RParen, _)) => Ok(inner_expr),
                    _ => Err("Expected ')' after expression".to_string()),
                }
            }
            Some(token) => Err(format!("Expected expression, got {:?}", token)),
            None => Err("Unexpected end of input".to_string()),
        }?;

        // 查看下一个token，看是否是运算符
        match self.lexer.peek() {
            Some((Token::Plus, _)) => {
                self.lexer.next(); // 消耗运算符
                let rhs = self.parse_expr()?;
                expr = Expr::BinaryOp(Box::new(expr), BinaryOp::Add, Box::new(rhs));
            }
            Some((Token::Minus, _)) => {
                self.lexer.next(); // 消耗运算符
                let rhs = self.parse_expr()?;
                expr = Expr::BinaryOp(Box::new(expr), BinaryOp::Subtract, Box::new(rhs));
            }
            Some((Token::Star, _)) => {
                self.lexer.next(); // 消耗运算符
                let rhs = self.parse_expr()?;
                expr = Expr::BinaryOp(Box::new(expr), BinaryOp::Multiply, Box::new(rhs));
            }
            Some((Token::Slash, _)) => {
                self.lexer.next(); // 消耗运算符
                let rhs = self.parse_expr()?;
                expr = Expr::BinaryOp(Box::new(expr), BinaryOp::Divide, Box::new(rhs));
            }
            Some((Token::Semicolon, _)) | Some((Token::RParen, _)) => {
                // 如果是分号或右括号，不消耗它，让parse_function_body处理它
            }
            Some(token) => {
                return Err(format!("Expected operator, semicolin, or right parenthesis, got {:?}", token));
            }
            None => {
                return Err("Unexpected end of input".to_string());
            }
        }

        Ok(expr)
    }
} 