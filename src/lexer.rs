use crate::token::Token;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8, // not unicode?
}


impl Lexer {
    pub fn from_str(s: String) -> Self {
        let mut lexer = Self {
            input: s,
            position: 0,
            read_position: 0,
            ch: 0,
        };

        lexer.read_char();

        lexer
    }

    // TODO only works with ascii
    fn read_char(&mut self) {
        if let Some(ch) = self.input.as_bytes().get(self.read_position) {
            self.ch = *ch;
        } else {
            self.ch = 0;
        }

        // increment. Done this way to accomodate the first read.
        self.position = self.read_position;

        self.read_position += 1;
    }

    // TODO handle eof better?
    pub fn next_token(&mut self) -> Token {
        // match token from current state, then read the next char.

        // skip whitespace at the beginning; because there's an
        // early return for ident and keywords and literals,
        // it's easier to handle whitespace here rather than
        // at the end
        self.read_whitespace();

        let res = match &[self.ch] {
            b"=" => Token::Assign,
            b"+" => Token::Plus,
            b"-" => Token::Minus,
            b"!" => Token::Bang,
            b"*" => Token::Asterisk,
            b"/" => Token::Slash,

            b"<" => Token::LessThan,
            b">" => Token::GreaterThan,

            b"," => Token::Comma,
            b";" => Token::Semicolon,

            b"(" => Token::LParen,
            b")" => Token::RParen,
            b"{" => Token::LBrace,
            b"}" => Token::RBrace,

            //b"fn" => Token::Function,
            //b"let" => Token::Let,

            //b"" => Token::Ident(String),
            //b"" => Token::Int(String),
            [0] => Token::Eof,
            c => {
                let res = if is_letter_or_underscore(c[0]) {
                    self.read_ident_or_keyword()
                } else if is_digit(c[0]) {
                    self.read_int()
                } else {
                    Token::Illegal
                };

                return res;
                // early return because we've already read_char
                // far enough
            }
        };

        self.read_char();

        res
    }

    // returns an ident (which could be a keyword)
    // TODO does not allow digits
    fn read_ident_or_keyword(&mut self) -> Token {
        let init_position = self.position;
        while is_letter_or_underscore(self.ch) {
            self.read_char();
        }

        let bytes = &self.input.as_bytes()[init_position..self.position];

        // match to keyword or ident
        match bytes {
            b"let" => Token::Let,
            b"fn" => Token::Function,
            b"true" => Token::True,
            b"false" => Token::False,
            b"if" => Token::If,
            b"else" => Token::Else,
            b"return" => Token::Return,
            b => Token::Ident(String::from_utf8(b.to_vec()).unwrap()),
        }
    }

    fn read_int(&mut self) -> Token {
        let init_position = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }

        let bytes = &self.input.as_bytes()[init_position..self.position];

        Token::Int(String::from_utf8(bytes.to_vec()).unwrap())
    }

    fn read_whitespace(&mut self) {
        while is_whitespace(self.ch) {
            self.read_char();
        }
    }
}

// used for checking the first ch of an ident
fn is_letter_or_underscore(b: u8) -> bool {
    b >= 65 && b <= 90
    ||
    b >= 97 && b <= 122
    || b == 95
}

fn is_digit(b: u8) -> bool {
    b >= 48 && b <= 57
    || b == 95
}

fn is_whitespace(b: u8) -> bool {
    b == 32 // " "
    || b == 10 // "\n"
    || b == 13 // "\r"
    || b == 9 // "\t"
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::token::Token;

    fn run_lexer_test(
        input: String,
        expected: Vec<Token>,
    )
    {
        let mut lexer = Lexer::from_str(input);
        let mut lexed_tokens = vec![];

        loop {
            let token = lexer.next_token();

            if token == Token::Eof {
                lexed_tokens.push(token);
                break;
            } else if token == Token::Illegal {
                panic!()
            } else {
                lexed_tokens.push(token);
            }
        }
        assert_eq!(lexed_tokens, expected);
    }

    #[test]
    fn test_next_token_basics() {
        let input = "=+(){},;-!*/<>".to_owned();

        let expected = vec![
            Token::Assign,
            Token::Plus,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Comma,
            Token::Semicolon,
            Token::Minus,
            Token::Bang,
            Token::Asterisk,
            Token::Slash,
            Token::LessThan,
            Token::GreaterThan,
            Token::Eof,
        ];

        run_lexer_test(input, expected);
    }

    #[test]
    fn test_next_token_idents() {
        let input = "
            let five = 5;
            let ten = 10;

            let add = fn(x, y) {
                x + y;
            };

            let result = add(five, ten);

            !-/5;
            5 < 10 > 5;
        ".to_owned();

        let expected = vec![
            Token::Let,
            Token::Ident("five".into()),
            Token::Assign,
            Token::Int("5".into()),
            Token::Semicolon,

            Token::Let,
            Token::Ident("ten".into()),
            Token::Assign,
            Token::Int("10".into()),
            Token::Semicolon,

            Token::Let,
            Token::Ident("add".into()),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Ident("x".into()),
            Token::Comma,
            Token::Ident("y".into()),
            Token::RParen,
            Token::LBrace,
            Token::Ident("x".into()),
            Token::Plus,
            Token::Ident("y".into()),
            Token::Semicolon,
            Token::RBrace,
            Token::Semicolon,

            Token::Let,
            Token::Ident("result".into()),
            Token::Assign,
            Token::Ident("add".into()),
            Token::LParen,
            Token::Ident("five".into()),
            Token::Comma,
            Token::Ident("ten".into()),
            Token::RParen,
            Token::Semicolon,

            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Int("5".into()),
            Token::Semicolon,

            Token::Int("5".into()),
            Token::LessThan,
            Token::Int("10".into()),
            Token::GreaterThan,
            Token::Int("5".into()),
            Token::Semicolon,

            Token::Eof,
        ];

        run_lexer_test(input, expected);
    }

    #[test]
    fn test_next_token_more_keywords() {
        let input = "
            if (5 < 10) {
                return true;
            } else {
                return false;
            }
        ".to_owned();

        let expected = vec![
            Token::If,
            Token::LParen,
            Token::Int("5".into()),
            Token::LessThan,
            Token::Int("10".into()),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RBrace,
            Token::Else,
            Token::LBrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RBrace,

            Token::Eof,
        ];

        run_lexer_test(input, expected);
    }
}
