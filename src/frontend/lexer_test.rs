#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let input = "function add(uint a, uint b) returns (uint) { return a + b; }";
        let lexer = Lexer::new(input);
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
        let lexer = Lexer::new(input);
        let tokens: Vec<_> = lexer.collect();

        assert_eq!(tokens.len(), 4);
        for (token, _) in tokens {
            assert_eq!(token, Token::Number);
        }
    }

    #[test]
    fn test_string_literals() {
        let input = r#""hello" "world" "test""#;
        let lexer = Lexer::new(input);
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
        let lexer = Lexer::new(input);
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