use crate::scanner::{Scanner, TokenType};

pub fn compile(source: String) {
    let mut scanner = Scanner::init(source);

    let mut line = -1;
    loop {
        let token = scanner.scan_token();
        if token.line != line {
            print!("   {} ", token.line);
            line = token.line;
        } else {
            print!("    | ");
        }

        println!("{:?} '{}'", token.t_type, token.content);

        if token.t_type == TokenType::Eof {
            break;
        } else if token.t_type == TokenType::Error {
            break;
        }
    }
}
