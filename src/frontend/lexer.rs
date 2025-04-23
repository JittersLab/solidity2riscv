use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    // 关键字
    #[token("contract")]
    Contract,
    #[token("function")]
    Function,
    #[token("returns")]
    Returns,
    #[token("return")]
    Return,
    #[token("public")]
    Public,
    #[token("private")]
    Private,
    #[token("view")]
    View,
    #[token("pure")]
    Pure,
    #[token("payable")]
    Payable,
    
    // 类型
    #[token("uint")]
    Uint,
    #[token("int")]
    Int,
    #[token("bool")]
    Bool,
    #[token("address")]
    Address,
    #[token("string")]
    String,
    
    // 运算符
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("=")]
    Equals,
    #[token("==")]
    DoubleEquals,
    #[token("!=")]
    NotEquals,
    
    // 分隔符
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token(";")]
    Semicolon,
    #[token(",")]
    Comma,
    
    // 字面量
    #[regex(r"[0-9]+")]
    Number,
    #[regex(r#""[^"]*""#)]
    StringLiteral,
    #[regex(r"true|false")]
    Boolean,
    
    // 标识符
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,
    
    // 空白和注释
    #[regex(r"[ \t\n\f]+", logos::skip)]
    #[regex(r"//[^\n]*", logos::skip)]
    #[regex(r"/\*[^*]*\*+(?:[^/*][^*]*\*+)*/", logos::skip)]
    Whitespace,
}

pub struct Lexer<'a> {
    inner: logos::Lexer<'a, Token>,
    peeked: Option<(Token, &'a str)>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            inner: Token::lexer(input),
            peeked: None,
        }
    }

    pub fn peek(&mut self) -> Option<(Token, &'a str)> {
        if self.peeked.is_none() {
            self.peeked = match self.inner.next() {
                Some(Ok(token)) => {
                    let slice = self.inner.slice();
                    Some((token, slice))
                }
                Some(Err(_)) | None => None,
            };
        }
        self.peeked.clone()
    }

    pub fn next(&mut self) -> Option<(Token, &'a str)> {
        if let Some(token) = self.peeked.take() {
            return Some(token);
        }
        match self.inner.next() {
            Some(Ok(token)) => {
                let slice = self.inner.slice();
                Some((token, slice))
            }
            Some(Err(_)) | None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let input = "function add(uint a, uint b) returns (uint) { return a + b; }";
        let mut lexer = Lexer::new(input);
        let tokens: Vec<_> = lexer.collect();

        let expected = vec![
            (Token::Function, "function"),
            (Token::Identifier, "add"),
            (Token::LParen, "("),
            (Token::Uint, "uint"),
            (Token::Identifier, "a"),
            (Token::Comma, ","),
            (Token::Uint, "uint"),
            (Token::Identifier, "b"),
            (Token::RParen, ")"),
            (Token::Returns, "returns"),
            (Token::LParen, "("),
            (Token::Uint, "uint"),
            (Token::RParen, ")"),
            (Token::LBrace, "{"),
            (Token::Return, "return"),
            (Token::Identifier, "a"),
            (Token::Plus, "+"),
            (Token::Identifier, "b"),
            (Token::Semicolon, ";"),
            (Token::RBrace, "}"),
        ];

        assert_eq!(tokens.len(), expected.len());
        for (got, want) in tokens.iter().zip(expected.iter()) {
            assert_eq!(got.0, want.0);
        }
    }

    #[test]
    fn test_number_literals() {
        let input = "123 456 0 789";
        let mut lexer = Lexer::new(input);
        let tokens: Vec<_> = lexer.collect();

        assert_eq!(tokens.len(), 4);
        for (token, _) in tokens {
            assert_eq!(token, Token::Number);
        }
    }

    #[test]
    fn test_string_literals() {
        let input = r#""hello" "world" "test""#;
        let mut lexer = Lexer::new(input);
        let tokens: Vec<_> = lexer.collect();

        assert_eq!(tokens.len(), 3);
        for (token, _) in tokens {
            assert_eq!(token, Token::StringLiteral);
        }
    }

    #[test]
    fn test_comments() {
        let input = r#"
            // This is a line comment
            function test() {
                /* This is a 
                   block comment */
                return 42;
            }
        "#;
        let mut lexer = Lexer::new(input);
        let tokens: Vec<_> = lexer.collect();

        // 注释应该被跳过，不会出现在token流中
        assert!(tokens.iter().all(|(token, _)| 
            matches!(token, 
                Token::Function | 
                Token::Identifier | 
                Token::LParen | 
                Token::RParen | 
                Token::LBrace | 
                Token::RBrace | 
                Token::Return | 
                Token::Number | 
                Token::Semicolon
            )
        ));
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (Token, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
} 