use std::{ops::Deref, rc::Rc};

use crate::{
    chunk::{Chunk, OpCode},
    debug::disassemble_chunk,
    scanner::{Scanner, Token, TokenType},
    values::Value,
    DEBUG_PRINT,
};

struct Parser {
    current: Rc<Option<Token>>,
    previous: Rc<Option<Token>>,
    had_error: bool,
    panic_mode: bool,
}

impl Parser {
    fn error_at_current(&mut self, message: String) {
        self.error_at(true, message);
    }

    fn error(&mut self, message: String) {
        self.error_at(false, message);
    }

    fn error_at(&mut self, current: bool, message: String) {
        if self.panic_mode {
            return;
        }

        let token = if current {
            self.current.deref().as_ref().unwrap()
        } else {
            self.previous.deref().as_ref().unwrap()
        };

        self.panic_mode = true;
        eprint!("[line {}] Error", token.line);

        if token.t_type == TokenType::Eof {
            eprint!(" at end");
        } else if token.t_type == TokenType::Error {
            // nothing yet
        } else {
            eprint!(" at '{}'", token.content);
        }

        eprintln!(": {}", message);
        self.had_error = true;
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
    None = 0,
    Assignment = 1, // =
    Or = 2,         // or
    And = 3,        // and
    Equality = 4,   // == !=
    Comparison = 5, // < > <= >=
    Term = 6,       // + -
    Factor = 7,     // * /
    Unary = 8,      // ! -
    Call = 9,       // . ()
    Primary = 10,
}

impl Precedence {
    fn next(&self) -> Precedence {
        return match self {
            Precedence::None => Self::Assignment,
            Precedence::Assignment => Self::Or,
            Precedence::Or => Self::And,
            Precedence::And => Self::Equality,
            Precedence::Equality => Self::Comparison,
            Precedence::Comparison => Self::Term,
            Precedence::Term => Self::Factor,
            Precedence::Factor => Self::Unary,
            Precedence::Unary => Self::Call,
            Precedence::Call => Self::Primary,
            Precedence::Primary => Self::None,
        };
    }
}

fn advance(scanner: &mut Scanner, parser: &mut Parser) {
    parser.previous = Rc::new(parser.current.deref().clone());

    loop {
        let current = scanner.scan_token();

        if current.t_type != TokenType::Error {
            parser.current = Rc::new(Some(current));
            break;
        }

        let current_content = current.content.clone();
        parser.current = Rc::new(Some(current));
        parser.error_at_current(current_content);
    }
}

fn consume(t_type: TokenType, message: String, scanner: &mut Scanner, parser: &mut Parser) {
    if parser.current.is_some() && parser.current.as_ref().as_ref().unwrap().t_type == t_type {
        advance(scanner, parser);
        return;
    }

    parser.error_at_current(message);
}

fn parse_precedence(
    scanner: &mut Scanner,
    parser: &mut Parser,
    precedence: Precedence,
    chunk: &mut Chunk,
) {
    advance(scanner, parser);
    let prefix_rule = get_rule(&parser.previous.deref().as_ref().unwrap().t_type).prefix;
    match prefix_rule {
        Some(func) => func(parser, scanner, chunk),
        None => parser.error("Expect expression".to_string()),
    }

    while precedence <= get_rule(&parser.current.deref().as_ref().unwrap().t_type).precedence {
        advance(scanner, parser);
        let infix_rule = get_rule(&parser.previous.deref().as_ref().unwrap().t_type).infix;
        match infix_rule {
            Some(func) => func(parser, scanner, chunk),
            None => panic!("this shouldn't error"),
        }
    }
}

fn expression(scanner: &mut Scanner, parser: &mut Parser, chunk: &mut Chunk) {
    parse_precedence(scanner, parser, Precedence::Assignment, chunk);
}

// emitting byte code
fn emit_byte(parser: &Parser, chunk: &mut Chunk, byte: OpCode) {
    let line = match parser.previous.deref().as_ref() {
        Some(tok) => tok.line,
        None => 0,
    };

    chunk.write(byte, line);
}

fn emit_bytes(parser: &Parser, chunk: &mut Chunk, byte1: OpCode, byte2: OpCode) {
    emit_byte(parser, chunk, byte1);
    emit_byte(parser, chunk, byte2);
}

fn emit_return(parser: &Parser, chunk: &mut Chunk) {
    emit_byte(parser, chunk, OpCode::OpReturn);
}

fn make_constant(value: Value, chunk: &mut Chunk) -> usize {
    return chunk.add_constant(value);
}

fn emit_constant(parser: &Parser, value: Value, chunk: &mut Chunk) {
    let constant = make_constant(value, chunk);
    emit_byte(parser, chunk, OpCode::OpConstant(constant));
}

fn end_compiler(parser: &Parser, chunk: &mut Chunk) {
    emit_return(parser, chunk);

    if DEBUG_PRINT {
        if !parser.had_error {
            disassemble_chunk(chunk, "code");
        }
    }
}

type ParseFn = fn(&mut Parser, &mut Scanner, &mut Chunk);

struct ParseRule {
    prefix: Option<ParseFn>,
    infix: Option<ParseFn>,
    precedence: Precedence,
}

fn get_rule(operator_type: &TokenType) -> ParseRule {
    match operator_type {
        TokenType::LeftParen => ParseRule {
            prefix: Some(grouping),
            infix: None,
            precedence: Precedence::None,
        },
        TokenType::Minus => ParseRule {
            prefix: Some(unary),
            infix: Some(binary),
            precedence: Precedence::Term,
        },
        TokenType::Plus => ParseRule {
            prefix: None,
            infix: Some(binary),
            precedence: Precedence::Term,
        },
        TokenType::Slash => ParseRule {
            prefix: None,
            infix: Some(binary),
            precedence: Precedence::Factor,
        },
        TokenType::Star => ParseRule {
            prefix: None,
            infix: Some(binary),
            precedence: Precedence::Factor,
        },
        TokenType::Number => ParseRule {
            prefix: Some(number),
            infix: None,
            precedence: Precedence::None,
        },
        TokenType::Bang => ParseRule {
            prefix: Some(unary),
            infix: None,
            precedence: Precedence::None,
        },
        TokenType::BangEqual => ParseRule {
            prefix: None,
            infix: Some(binary),
            precedence: Precedence::Equality,
        },
        TokenType::EqualEqual => ParseRule {
            prefix: None,
            infix: Some(binary),
            precedence: Precedence::Equality,
        },
        TokenType::Greater => ParseRule {
            prefix: None,
            infix: Some(binary),
            precedence: Precedence::Comparison,
        },
        TokenType::GreaterEqual => ParseRule {
            prefix: None,
            infix: Some(binary),
            precedence: Precedence::Comparison,
        },
        TokenType::Less => ParseRule {
            prefix: None,
            infix: Some(binary),
            precedence: Precedence::Comparison,
        },
        TokenType::LessEqual => ParseRule {
            prefix: None,
            infix: Some(binary),
            precedence: Precedence::Comparison,
        },
        TokenType::True | TokenType::Nil | TokenType::False => ParseRule {
            prefix: Some(literal),
            infix: None,
            precedence: Precedence::None,
        },
        TokenType::String => ParseRule {
            prefix: Some(string),
            infix: None,
            precedence: Precedence::None,
        },
        _ => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
    }
}

fn number(parser: &mut Parser, _scanner: &mut Scanner, chunk: &mut Chunk) {
    let token = parser.previous.deref().as_ref().unwrap();
    let value: f64 = token.content.parse().unwrap();
    emit_constant(parser, Value::from_number(value), chunk);
}

fn string(parser: &mut Parser, _scanner: &mut Scanner, chunk: &mut Chunk) {
    let token = parser.previous.deref().as_ref().unwrap();
    emit_constant(parser, Value::from_string(token.content.to_string()), chunk)
}

fn grouping(parser: &mut Parser, scanner: &mut Scanner, chunk: &mut Chunk) {
    expression(scanner, parser, chunk);
    consume(
        TokenType::RightParen,
        "Expect ')' after expression".to_string(),
        scanner,
        parser,
    )
}

fn unary(parser: &mut Parser, scanner: &mut Scanner, chunk: &mut Chunk) {
    let token = parser.previous.deref().as_ref().unwrap().clone();
    let operator_type = &token.t_type;

    parse_precedence(scanner, parser, Precedence::Unary, chunk);

    match operator_type {
        TokenType::Bang => emit_byte(parser, chunk, OpCode::OpNot),
        TokenType::Minus => emit_byte(parser, chunk, OpCode::OpNegate),
        _ => return,
    }
}

fn binary(parser: &mut Parser, scanner: &mut Scanner, chunk: &mut Chunk) {
    let token = parser.previous.deref().as_ref().unwrap().clone();
    let operator_type = &token.t_type;

    let rule = get_rule(operator_type);
    parse_precedence(scanner, parser, rule.precedence.next(), chunk);

    match operator_type {
        TokenType::BangEqual => emit_bytes(parser, chunk, OpCode::OpEqual, OpCode::OpNot),
        TokenType::EqualEqual => emit_byte(parser, chunk, OpCode::OpEqual),
        TokenType::Greater => emit_byte(parser, chunk, OpCode::OpGreater),
        TokenType::GreaterEqual => emit_bytes(parser, chunk, OpCode::OpLess, OpCode::OpNot),
        TokenType::Less => emit_byte(parser, chunk, OpCode::OpLess),
        TokenType::LessEqual => emit_bytes(parser, chunk, OpCode::OpGreater, OpCode::OpNot),
        TokenType::Plus => emit_byte(parser, chunk, OpCode::OpAdd),
        TokenType::Minus => emit_byte(parser, chunk, OpCode::OpSubtract),
        TokenType::Star => emit_byte(parser, chunk, OpCode::OpMultiply),
        TokenType::Slash => emit_byte(parser, chunk, OpCode::OpDivide),

        _ => return,
    }
}

fn literal(parser: &mut Parser, _scanner: &mut Scanner, chunk: &mut Chunk) {
    match parser.previous.deref().as_ref().unwrap().t_type {
        TokenType::False => emit_byte(parser, chunk, OpCode::OpFalse),
        TokenType::Nil => emit_byte(parser, chunk, OpCode::OpNil),
        TokenType::True => emit_byte(parser, chunk, OpCode::OpTrue),
        _ => return,
    }
}

pub fn compile(source: String, chunk: &mut Chunk) -> bool {
    let mut parser = Parser {
        current: Rc::new(None),
        previous: Rc::new(None),
        had_error: false,
        panic_mode: false,
    };
    let mut scanner = Scanner::init(source);

    advance(&mut scanner, &mut parser);
    expression(&mut scanner, &mut parser, chunk);
    consume(
        TokenType::Eof,
        "Expect end of expression.".to_string(),
        &mut scanner,
        &mut parser,
    );
    end_compiler(&parser, chunk);
    return !parser.had_error;
}
