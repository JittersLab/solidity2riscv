use crate::frontend::lexer::Token;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Expr {
    Number(i64),
    String(String),
    Boolean(bool),
    Identifier(String),
    BinaryOp(Box<Expr>, BinaryOp, Box<Expr>),
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<(String, String)>, // (type, name)
    pub returns: Vec<String>,
    pub body: Vec<Statement>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Statement {
    VariableDecl(String, String, Option<Expr>), // type, name, initializer
    Assignment(String, Expr),
    Return(Option<Expr>),
}

pub struct Parser<'a> {
    tokens: Vec<(Token, &'a str)>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<(Token, &'a str)>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    fn next(&mut self) -> Option<(Token, &'a str)> {
        if self.current < self.tokens.len() {
            let token = self.tokens[self.current].clone();
            self.current += 1;
            Some(token)
        } else {
            None
        }
    }

    fn peek(&self) -> Option<(Token, &'a str)> {
        if self.current < self.tokens.len() {
            Some(self.tokens[self.current].clone())
        } else {
            None
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Function>, String> {
        let mut functions = Vec::new();
        
        while let Some((token, _)) = self.next() {
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
        let name = match self.next() {
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
        match self.next() {
            Some((Token::LParen, _)) => (),
            _ => return Err("Expected '(' after function name".to_string()),
        }

        // 如果下一个 token 是右括号，说明没有参数
        match self.next() {
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

                match self.next() {
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
            match self.next() {
                Some((Token::Comma, _)) => {
                    match self.next() {
                        Some((token, _)) => {
                            let param_type = match token {
                                Token::Uint => "uint",
                                Token::Int => "int",
                                Token::Bool => "bool",
                                Token::Address => "address",
                                Token::String => "string",
                                _ => return Err("Expected parameter type".to_string()),
                            }.to_string();

                            match self.next() {
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
        match self.next() {
            Some((Token::Returns, _)) => (),
            Some(_) => return Ok(returns), // 如果不是 returns，说明没有返回值
            None => return Ok(returns),    // 如果到达输入末尾，也说明没有返回值
        }

        // 检查左括号
        match self.next() {
            Some((Token::LParen, _)) => (),
            _ => return Err("Expected '(' after returns".to_string()),
        }

        loop {
            // 解析返回类型
            let return_type = match self.next() {
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
                match self.next() {
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
        match self.next() {
            Some((Token::LBrace, _)) => (),
            _ => return Err("Expected '{' at start of function body".to_string()),
        }

        loop {
            match self.peek() {
                Some((Token::RBrace, _)) => {
                    self.next(); // 消耗右大括号
                    break;
                }
                Some((Token::Return, _)) => {
                    self.next(); // 消耗 return 关键字
                    let expr = match self.peek() {
                        Some((Token::Semicolon, _)) => {
                            self.next(); // 消耗分号
                            None
                        }
                        _ => {
                            let expr = self.parse_expr()?;
                            match self.next() {
                                Some((Token::Semicolon, _)) => (),
                                _ => return Err("Expected ';' after return expression".to_string()),
                            }
                            Some(expr)
                        }
                    };
                    statements.push(Statement::Return(expr));
                }
                Some((Token::Uint, _)) | Some((Token::Int, _)) | Some((Token::Bool, _)) |
                Some((Token::Address, _)) | Some((Token::String, _)) => {
                    let type_token = self.next().unwrap(); // 消耗类型
                    let var_type = match type_token {
                        (Token::Uint, _) => "uint",
                        (Token::Int, _) => "int",
                        (Token::Bool, _) => "bool",
                        (Token::Address, _) => "address",
                        (Token::String, _) => "string",
                        _ => return Err("Expected variable type".to_string()),
                    }.to_string();

                    let var_name = match self.next() {
                        Some((Token::Identifier, name)) => name.to_string(),
                        _ => return Err("Expected variable name".to_string()),
                    };

                    let initializer = match self.next() {
                        Some((Token::Equals, _)) => {
                            let expr = self.parse_expr()?;
                            match self.next() {
                                Some((Token::Semicolon, _)) => (),
                                _ => return Err("Expected ';' after variable declaration".to_string()),
                            }
                            Some(expr)
                        }
                        Some((Token::Semicolon, _)) => None,
                        _ => return Err("Expected '=' or ';' after variable name".to_string()),
                    };

                    statements.push(Statement::VariableDecl(var_type, var_name, initializer));
                }
                Some((Token::Identifier, _)) => {
                    let name = match self.next() {
                        Some((Token::Identifier, name)) => name.to_string(),
                        _ => return Err("Expected identifier".to_string()),
                    };

                    match self.next() {
                        Some((Token::Equals, _)) => {
                            let expr = self.parse_expr()?;
                            match self.next() {
                                Some((Token::Semicolon, _)) => (),
                                _ => return Err("Expected ';' after assignment".to_string()),
                            }
                            statements.push(Statement::Assignment(name, expr));
                        }
                        _ => return Err("Expected '=' after identifier".to_string()),
                    }
                }
                _ => return Err("Expected statement".to_string()),
            }
        }

        Ok(statements)
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_term()?;

        loop {
            match self.peek() {
                Some((Token::Plus, _)) => {
                    self.next(); // 消耗运算符
                    let right = self.parse_term()?;
                    left = Expr::BinaryOp(Box::new(left), BinaryOp::Add, Box::new(right));
                }
                Some((Token::Minus, _)) => {
                    self.next(); // 消耗运算符
                    let right = self.parse_term()?;
                    left = Expr::BinaryOp(Box::new(left), BinaryOp::Subtract, Box::new(right));
                }
                Some((Token::GreaterThan, _)) => {
                    self.next(); // 消耗运算符
                    let right = self.parse_term()?;
                    left = Expr::BinaryOp(Box::new(left), BinaryOp::GreaterThan, Box::new(right));
                }
                Some((Token::LessThan, _)) => {
                    self.next(); // 消耗运算符
                    let right = self.parse_term()?;
                    left = Expr::BinaryOp(Box::new(left), BinaryOp::LessThan, Box::new(right));
                }
                Some((Token::GreaterThanOrEqual, _)) => {
                    self.next(); // 消耗运算符
                    let right = self.parse_term()?;
                    left = Expr::BinaryOp(Box::new(left), BinaryOp::GreaterThanOrEqual, Box::new(right));
                }
                Some((Token::LessThanOrEqual, _)) => {
                    self.next(); // 消耗运算符
                    let right = self.parse_term()?;
                    left = Expr::BinaryOp(Box::new(left), BinaryOp::LessThanOrEqual, Box::new(right));
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_factor()?;

        loop {
            match self.peek() {
                Some((Token::Star, _)) => {
                    self.next(); // 消耗运算符
                    let right = self.parse_factor()?;
                    left = Expr::BinaryOp(Box::new(left), BinaryOp::Multiply, Box::new(right));
                }
                Some((Token::Slash, _)) => {
                    self.next(); // 消耗运算符
                    let right = self.parse_factor()?;
                    left = Expr::BinaryOp(Box::new(left), BinaryOp::Divide, Box::new(right));
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Expr, String> {
        match self.next() {
            Some((Token::Number, n)) => Ok(Expr::Number(n.parse().unwrap())),
            Some((Token::StringLiteral, s)) => Ok(Expr::String(s.to_string())),
            Some((Token::Boolean, b)) => Ok(Expr::Boolean(b.parse().unwrap())),
            Some((Token::Identifier, name)) => Ok(Expr::Identifier(name.to_string())),
            Some((Token::LParen, _)) => {
                let expr = self.parse_expr()?;
                match self.next() {
                    Some((Token::RParen, _)) => Ok(expr),
                    _ => Err("Expected ')'".to_string()),
                }
            }
            _ => Err("Expected expression".to_string()),
        }
    }
} 