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
        //

        let res = match &[self.ch] {
            b"=" => Token::Assign,
            b"+" => Token::Plus,

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
            _ => Token::Illegal,
        };

        self.read_char();

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::token::Token;

    #[test]
    fn test_next_token_non_ident() {
        let input = "=+(){},;".to_owned();

        let expected = vec![
            Token::Assign,
            Token::Plus,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Comma,
            Token::Semicolon,
            Token::Eof,
        ];

        let mut lexer = Lexer::from_str(input);
        let mut lexed_tokens = vec![];

        loop {
            let token = lexer.next_token();

            if token == Token::Eof {
                lexed_tokens.push(token);
                break;
            } else {
                lexed_tokens.push(token);
            }
        }
        assert_eq!(lexed_tokens, expected);
    }
}
