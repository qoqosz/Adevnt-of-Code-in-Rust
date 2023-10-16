use aoc::load_input;
use std::collections::HashMap;
use std::iter::Peekable;
use std::mem::discriminant;
use std::str::Chars;

// Token
#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    EndOfFile,

    Number(u16),
    Symbol(String),
    Assignment,

    And,
    Or,
    LShift,
    RShift,
    Not,
}

impl Default for Token {
    fn default() -> Self {
        Token::Illegal
    }
}

// Lexer
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(line: &'a String) -> Self {
        Lexer {
            input: line.chars().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        match self.next() {
            Some(token) => token,
            None => Token::EndOfFile,
        }
    }

    fn read_char(&mut self) -> Option<char> {
        self.input.next()
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.peek_char() {
            if !ch.is_whitespace() {
                break;
            }
            self.read_char();
        }
    }

    fn read_number(&mut self, first: char) -> String {
        let mut number = String::new();
        number.push(first);

        while let Some(&ch) = self.peek_char() {
            if !ch.is_numeric() {
                break;
            }
            number.push(self.read_char().unwrap());
        }

        number
    }

    fn read_symbol(&mut self, first: char) -> String {
        let mut sym = String::new();
        sym.push(first);

        while let Some(&ch) = self.peek_char() {
            if !ch.is_alphabetic() {
                break;
            }
            sym.push(self.read_char().unwrap());
        }

        sym
    }

    fn read_assignment(&mut self) {
        if let Some(ch) = self.peek_char() {
            if *ch == '>' {
                self.read_char();
            }
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        match self.read_char() {
            Some('-') => {
                self.read_assignment();
                Some(Token::Assignment)
            }
            Some(ch) => {
                if ch.is_numeric() {
                    Some(Token::Number(self.read_number(ch).parse::<u16>().unwrap()))
                } else {
                    let sym = self.read_symbol(ch);

                    match sym.as_str() {
                        "AND" => Some(Token::And),
                        "OR" => Some(Token::Or),
                        "NOT" => Some(Token::Not),
                        "LSHIFT" => Some(Token::LShift),
                        "RSHIFT" => Some(Token::RShift),
                        _ => Some(Token::Symbol(sym)),
                    }
                }
            }
            None => None,
        }
    }
}

// Parser
/// A node in the Abstract Syntax Tree (AST)
pub trait Node {
    fn accept(&self, visitor: &Interpreter) -> Result<u16, String>;
}

/// Integer node
pub struct NumberNode {
    pub token: Token,
}

impl From<u16> for NumberNode {
    fn from(num: u16) -> Self {
        NumberNode {
            token: Token::Number(num),
        }
    }
}

impl Node for NumberNode {
    fn accept(&self, visitor: &Interpreter) -> Result<u16, String> {
        visitor.visit_num(self)
    }
}

pub struct SymbolNode {
    pub token: Token,
}

impl From<String> for SymbolNode {
    fn from(sym: String) -> Self {
        SymbolNode {
            token: Token::Symbol(sym),
        }
    }
}

impl Node for SymbolNode {
    fn accept(&self, visitor: &Interpreter) -> Result<u16, String> {
        visitor.visit_sym(self)
    }
}

pub struct UnaryOpNode {
    pub token: Token,
    pub rhs: Box<dyn Node>,
}

impl Node for UnaryOpNode {
    fn accept(&self, visitor: &Interpreter) -> Result<u16, String> {
        visitor.visit_unop(self)
    }
}

pub struct BinaryOpNode {
    pub token: Token,
    pub lhs: Box<dyn Node>,
    pub rhs: Box<dyn Node>,
}

impl Node for BinaryOpNode {
    fn accept(&self, visitor: &Interpreter) -> Result<u16, String> {
        visitor.visit_binop(self)
    }
}

pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Parser<'a> {
        let current_token = lexer.next_token();
        Parser {
            lexer,
            current_token,
        }
    }

    fn eat(&mut self, token: &Token) -> Result<(), String> {
        if discriminant(&self.current_token) == discriminant(token) {
            self.current_token = self.lexer.next_token();
            Ok(())
        } else {
            Err("Invalid syntax".to_string())
        }
    }

    // Grammar
    // assignment: expr ASSIGNMENT sym
    // expr: val ((AND | OR | LSHIFT | RSHIFT) val)?
    // val: (NOT)? sym | num

    pub fn val(&mut self) -> Result<Box<dyn Node>, String> {
        match self.current_token {
            Token::Number(num) => {
                self.eat(&Token::Number(0))?;
                Ok(Box::new(NumberNode::from(num)))
            }
            Token::Not => {
                self.eat(&Token::Not)?;
                let node = Box::new(UnaryOpNode {
                    token: Token::Not,
                    rhs: self.expr()?,
                });
                Ok(node)
            }
            _ => Err("Not a `val`".to_string()),
        }
    }

    pub fn expr(&mut self) -> Result<Box<dyn Node>, String> {
        let node = self.val()?;

        match self.current_token {
            Token::And => {
                self.eat(&Token::And)?;
                let node = Box::new(BinaryOpNode {
                    token: Token::And,
                    lhs: node,
                    rhs: self.val()?,
                });
                Ok(node)
            }
            Token::Illegal => Err("Illegal input".to_string()),
            Token::EndOfFile => Ok(node),
            _ => Err("Unknown token".to_string()),
        }
    }
}

pub struct Interpreter<'a> {
    globals: HashMap<String, u16>,
    parser: &'a mut Parser<'a>,
}

impl<'a> Interpreter<'a> {
    pub fn new(parser: &'a mut Parser<'a>) -> Interpreter<'a> {
        Interpreter {
            globals: HashMap::new(),
            parser,
        }
    }

    pub fn interpret(&mut self) -> Result<u16, String> {
        let tree = self.parser.expr()?;
        self.visit(&tree)
    }

    pub fn visit(&self, tree: &Box<dyn Node>) -> Result<u16, String> {
        tree.accept(self)
    }

    pub fn visit_num(&self, node: &NumberNode) -> Result<u16, String> {
        match node.token {
            Token::Number(val) => Ok(val),
            _ => Err("Not an integer".to_string()),
        }
    }

    pub fn visit_sym(&self, node: &SymbolNode) -> Result<u16, String> {
        match &node.token {
            Token::Symbol(sym) => {
                let val = *self.globals.get(&(sym.clone())).unwrap_or(&(0 as u16));
                Ok(val)
            }
            _ => Err("Not a symbol".to_string()),
        }
    }

    pub fn visit_unop(&self, node: &UnaryOpNode) -> Result<u16, String> {
        let rhs = self.visit(&node.rhs)?;

        match node.token {
            Token::Not => Ok(!rhs),
            _ => Err("Not an unary operator".to_string()),
        }
    }

    pub fn visit_binop(&self, node: &BinaryOpNode) -> Result<u16, String> {
        let lhs = self.visit(&node.lhs)?;
        let rhs = self.visit(&node.rhs)?;

        match node.token {
            Token::And => Ok(lhs & rhs),
            Token::Or => Ok(lhs | rhs),
            Token::RShift => Ok(lhs >> rhs),
            Token::LShift => Ok(lhs << rhs),
            _ => Err("Not a binary operator".to_string()),
        }
    }
}

static TEST: &str = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";

fn main() {
    let data = load_input!("/Users/qoqosz/Documents/Coding/Rust/Advent of Code/data/2015/day7.txt");
    let lines = data
        .split('\n')
        .filter(|&x| !x.is_empty())
        .collect::<Vec<_>>();

    man_test();
}

fn interpret(input: &str) -> Result<u16, String> {
    let mut line = input.to_string();
    let mut lexer = Lexer::new(&mut line);
    let mut parser = Parser::new(&mut lexer);
    let mut interpreter = Interpreter::new(&mut parser);

    interpreter.interpret()
}

fn man_test() {
    let mut registers: HashMap<&str, u32> = HashMap::new();
    registers.insert("x", 123);
    registers.insert("y", 456);

    for line in TEST.split('\n') {
        let mut input = line.to_owned();
        let lexer = Lexer::new(&mut input);

        for token in lexer {
            println!("{:?}", token);
        }

        println!("-------------");
    }

    // test all
    println!("{}={}", "NOT 123", interpret("NOT 123").unwrap());

    // test 2
    //interpret("123 AND 456");
}
