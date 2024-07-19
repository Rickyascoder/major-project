#[derive(Debug,PartialEq,Clone)]

pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Equals,
    Identifier(String),
    Comma,
    EOF
}

pub struct Lexer<'a>{
    input: &'a str,
    pos: usize,
    read_pos: usize,
    ch: Option<char>,
}

impl<'a> Lexer<'a>{
    pub fn new(input: &'a str) -> Self{
        let mut lexer = Lexer{
            input,
            pos: 0,
            read_pos: 0,
            ch: None,
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        self.ch = if self.read_pos >= self.input.len() {
            None
        } else {
            Some(self.input.as_bytes()[self.read_pos] as char)
        };
        self.pos = self.read_pos;
        self.read_pos += 1;
    }

    fn skip_whitespace(&mut self){
        while let Some(ch) = self.ch {
            if !ch.is_whitespace(){
                break;
            }
            self.read_char();
        }
    }

    fn read_number(&mut self) -> Token{
        let start = self.pos;
        while let Some(ch) = self.ch {
            if !ch.is_digit(10) && ch != '.'{
                break;
            }
            self.read_char();
        }
        let number = self.input[start..self.pos].parse().unwrap();
        Token::Number(number)
    }

    fn read_ident(&mut self) -> Token{
        let start = self.pos;
        while let Some(ch) = self.ch {
            if !ch.is_alphabetic() {
                break;
            }
            self.read_char();
        }
        let identifier = &self.input[start..self.pos];
        Token::Identifier(identifier.to_string())
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch{
            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            Some('/') => Token::Slash,
            Some('*') => Token::Star,
            Some('(') => Token::LParen,
            Some(')') => Token::RParen,
            Some(',') => Token::Comma,
            Some('=') => Token::Equals,
            Some(ch) if ch.is_digit(10) => self.read_number(),
            Some(ch) if ch.is_alphabetic() => self.read_ident(),
            None => Token::EOF,
            _ => panic!("UNEXPECTED ERROR ! : {}",self.ch.unwrap()),
        };
        self.read_char();
        token
    }
}