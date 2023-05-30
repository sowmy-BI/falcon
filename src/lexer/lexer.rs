use crate::token::token::Token;

pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8,
}
impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lex = Lexer {
            input: input.into_bytes(),
            position: 0,
            read_position: 0,
            ch: 0,
        };
        lex.read_char();
        lex
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_white_space();

        let token = match self.ch {
            b'=' => Token::ASSIGN,
            b'+' => Token::PULS,
            b'{' => Token::LBRACE,
            b'}' => Token::RBRACE,
            b'(' => Token::LPAREN,
            b')' => Token::RPAREN,
            b';' => Token::SEMICOLON,
            b',' => Token::COMMA,
            b'a'..=b'z'| b'A'..=b'Z' | b'_' => {
                let ident = self.read_identifier();
                match ident.as_str() {
                    "function" => Token::FUNCTION,
                    "let" => Token::LET,
                    _ => Token::IDENT(ident),
                }
            }
            b'0'..=b'9' => {
                let int = self.read_integer();
                Token::INT(int)
            }
            0 => Token::EOF,
            _ => todo!("")
        };
        self.read_char();
        token
    }

    fn read_integer(&mut self) -> String {
        let pos = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }

        return String::from_utf8_lossy(&self.input[pos..self.position]).to_string();
    }

    fn read_identifier(&mut self) -> String {
        let pos = self.position;
        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.read_char();
        }

        return String::from_utf8_lossy(&self.input[pos..self.position]).to_string();

    }

    fn skip_white_space(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }
}
