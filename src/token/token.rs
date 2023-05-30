
#[derive(Debug, PartialEq)]
pub enum Token {
    ILLEGAL,
    EOF,

    // identifier
    IDENT(String),
    INT(String),

    // Operators
    ASSIGN,
    PULS,

    // Delimiters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // keyword
    FUNCTION,
    LET,
}

#[cfg(test)]
mod tests {
    use super::Token;
    use crate::lexer::lexer::Lexer;

    #[test]
    fn check_symbols_token() {
        let program = "=+(){},;";
        let mut lexer = Lexer::new(program.to_string());
        let result: Vec<Token> = vec![
            Token::ASSIGN,
            Token::PULS,
            Token::LPAREN,
            Token::RPAREN,
            Token::LBRACE,
            Token::RBRACE,
            Token::COMMA,
            Token::SEMICOLON
        ];

        for token in result {
            let next_token = lexer.next_token();
            println!("expected: {:?}, received {:?}", token, next_token);
            assert_eq!(token, next_token);
        }
    }
    #[test]
    fn check_add_function() {
        let program = r#"
            let five = 5; let ten = 10;
            let add = fn(x, y) { x + y;};
            let result = add(five, ten);
        "#;
        let mut lexer = Lexer::new(program.to_string());
        let result: Vec<Token> = vec![
            Token::LET,
            Token::IDENT(String::from("five")),
            Token::ASSIGN,
            Token::INT(String::from("5")),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT(String::from("ten")),
            Token::ASSIGN,
            Token::INT(String::from("10")),
            Token::SEMICOLON,
            Token::IDENT(String::from("add")),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT(String::from("x")),
            Token::COMMA,
            Token::IDENT(String::from("y")),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT(String::from("x")),
            Token::PULS,
            Token::IDENT(String::from("y")),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT(String::from("result")),
            Token::ASSIGN,
            Token::IDENT(String::from("add")),
            Token::LPAREN,
            Token::IDENT(String::from("five")),
            Token::COMMA,
            Token::IDENT(String::from("ten")),
            Token::SEMICOLON
        ];
        for token in result {
            let next_token = lexer.next_token();
            println!("expected: {:?}, received {:?}", token, next_token);
            assert_eq!(token, next_token);
        }
    }
}
