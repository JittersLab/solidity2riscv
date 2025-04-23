#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty_function() {
        let input = "function test() { }";
        let mut parser = Parser::new(input);
        let result = parser.parse().unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "test");
        assert!(result[0].params.is_empty());
        assert!(result[0].returns.is_empty());
        assert!(result[0].body.is_empty());
    }

    #[test]
    fn test_parse_function_with_params() {
        let input = "function add(uint a, uint b) { }";
        let mut parser = Parser::new(input);
        let result = parser.parse().unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "add");
        assert_eq!(result[0].params.len(), 2);
        assert_eq!(result[0].params[0], ("uint".to_string(), "a".to_string()));
        assert_eq!(result[0].params[1], ("uint".to_string(), "b".to_string()));
    }

    #[test]
    fn test_parse_function_with_return() {
        let input = "function get() returns (uint) { }";
        let mut parser = Parser::new(input);
        let result = parser.parse().unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "get");
        assert_eq!(result[0].returns.len(), 1);
        assert_eq!(result[0].returns[0], "uint");
    }

    #[test]
    fn test_parse_function_with_body() {
        let input = r#"
            function calculate(uint x) returns (uint) {
                uint result = 0;
                return result;
            }
        "#;
        let mut parser = Parser::new(input);
        let result = parser.parse().unwrap();

        assert_eq!(result.len(), 1);
        let func = &result[0];
        assert_eq!(func.name, "calculate");
        assert_eq!(func.params.len(), 1);
        assert_eq!(func.returns.len(), 1);
        
        // 检查函数体
        assert_eq!(func.body.len(), 2);
        match &func.body[0] {
            Statement::VariableDecl(type_name, var_name, init) => {
                assert_eq!(type_name, "uint");
                assert_eq!(var_name, "result");
                assert!(init.is_some());
            }
            _ => panic!("Expected variable declaration"),
        }
        match &func.body[1] {
            Statement::Return(expr) => {
                assert!(expr.is_some());
            }
            _ => panic!("Expected return statement"),
        }
    }

    #[test]
    fn test_parse_error_handling() {
        let inputs = vec![
            "function", // 不完整的函数声明
            "function test", // 缺少参数列表
            "function test(", // 未闭合的参数列表
            "function test() {", // 未闭合的函数体
            "function test() returns", // 不完整的返回值声明
        ];

        for input in inputs {
            let mut parser = Parser::new(input);
            assert!(parser.parse().is_err(), "Should fail to parse: {}", input);
        }
    }
} 