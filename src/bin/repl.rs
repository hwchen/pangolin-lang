use pangolin_lang::{
    lexer::Lexer,
    token::Token,
};
use std::io::{self, Write};

const PROMPT: &str = ">> ";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut line_buf = String::new();

    loop {
        print!("{}", PROMPT);
        io::stdout().flush()?;

        match io::stdin().read_line(&mut line_buf) {
            Ok(_) => {
                // TODO important, remove this clone.
                let mut lexer = Lexer::from_str(line_buf.clone());

                loop {
                    let token = lexer.next_token();

                    println!("  {:?}", token);
                    if token == Token::Eof {
                        break;
                    }
                }
            },
            Err(err) => eprintln!("Could not read line: {}", err),
        }
    }
    // Ok(()) unreachable
}
