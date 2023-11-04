use aoc::aoc_input;
use std::collections::{HashMap, VecDeque};
use std::iter::Peekable;
use std::mem::discriminant;
use std::str::Chars;

// Token
#[derive(Default, Debug, PartialEq, Clone)]
pub enum Token {
    #[default]
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

// Lexer
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(line: &'a str) -> Self {
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
    fn accept(&self, visitor: &mut Interpreter) -> Result<Token, String>;
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
    fn accept(&self, visitor: &mut Interpreter) -> Result<Token, String> {
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
    fn accept(&self, visitor: &mut Interpreter) -> Result<Token, String> {
        visitor.visit_sym(self)
    }
}

pub struct UnaryOpNode {
    pub token: Token,
    pub rhs: Box<dyn Node>,
}

impl Node for UnaryOpNode {
    fn accept(&self, visitor: &mut Interpreter) -> Result<Token, String> {
        visitor.visit_unop(self)
    }
}

pub struct BinaryOpNode {
    pub token: Token,
    pub lhs: Box<dyn Node>,
    pub rhs: Box<dyn Node>,
}

impl Node for BinaryOpNode {
    fn accept(&self, visitor: &mut Interpreter) -> Result<Token, String> {
        visitor.visit_binop(self)
    }
}

pub struct AssignmentNode {
    pub token: Token,
    pub lhs: Box<dyn Node>,
    pub rhs: Box<dyn Node>,
}

impl Node for AssignmentNode {
    fn accept(&self, visitor: &mut Interpreter) -> Result<Token, String> {
        visitor.visit_assign(self)
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
            println!("Invalid syntax");
            Err("Invalid syntax".to_string())
        }
    }

    // Grammar
    // assignment: expr ASSIGNMENT sym
    // expr: val ((AND | OR | LSHIFT | RSHIFT) val)?
    // val: (NOT)? sym | num

    pub fn sym(&mut self) -> Result<Box<dyn Node>, String> {
        match self.current_token.clone() {
            Token::Symbol(sym) => {
                self.eat(&Token::Symbol("".to_string()))?;
                Ok(Box::new(SymbolNode::from(sym)))
            }
            _ => Err("not a symbol".to_string()),
        }
    }

    pub fn val(&mut self) -> Result<Box<dyn Node>, String> {
        match self.current_token.clone() {
            Token::Symbol(sym) => {
                self.eat(&Token::Symbol("".to_string()))?;
                Ok(Box::new(SymbolNode::from(sym)))
            }
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
            Token::Or => {
                self.eat(&Token::Or)?;
                let node = Box::new(BinaryOpNode {
                    token: Token::Or,
                    lhs: node,
                    rhs: self.val()?,
                });
                Ok(node)
            }
            Token::RShift => {
                self.eat(&Token::RShift)?;
                let node = Box::new(BinaryOpNode {
                    token: Token::RShift,
                    lhs: node,
                    rhs: self.val()?,
                });
                Ok(node)
            }
            Token::LShift => {
                self.eat(&Token::LShift)?;
                let node = Box::new(BinaryOpNode {
                    token: Token::LShift,
                    lhs: node,
                    rhs: self.val()?,
                });
                Ok(node)
            }
            Token::Illegal => Err("Illegal input".to_string()),
            _ => Ok(node),
        }
    }

    pub fn assignment(&mut self) -> Result<Box<dyn Node>, String> {
        let node = self.expr()?;

        match self.current_token {
            Token::Assignment => {
                self.eat(&Token::Assignment)?;
                let node = Box::new(AssignmentNode {
                    token: Token::Assignment,
                    lhs: node,
                    rhs: self.sym()?,
                });
                Ok(node)
            }
            _ => Ok(node),
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

    pub fn with_globals(
        parser: &'a mut Parser<'a>,
        globals: HashMap<String, u16>,
    ) -> Interpreter<'a> {
        Interpreter { globals, parser }
    }

    pub fn interpret(&mut self) -> Result<Token, String> {
        let tree = self.parser.assignment()?;
        self.visit(&tree)
    }

    pub fn visit(&mut self, tree: &Box<dyn Node>) -> Result<Token, String> {
        tree.accept(self)
    }

    pub fn visit_num(&mut self, node: &NumberNode) -> Result<Token, String> {
        match &node.token {
            num @ Token::Number(_) => Ok(num.clone()),
            _ => Err("Not an integer".to_string()),
        }
    }

    pub fn visit_sym(&mut self, node: &SymbolNode) -> Result<Token, String> {
        match &node.token {
            Token::Symbol(ref sym) => match self.globals.get(sym) {
                Some(val) => Ok(Token::Number(*val)),
                _ => Ok(Token::Symbol(sym.clone())),
            },
            _ => Err("Not a symbol".to_string()),
        }
    }

    pub fn visit_unop(&mut self, node: &UnaryOpNode) -> Result<Token, String> {
        let rhs = match self.visit(&node.rhs) {
            Ok(Token::Number(val)) => val,
            _ => return Err("not a number".to_string()),
        };

        match node.token {
            Token::Not => Ok(Token::Number(!rhs)),
            _ => Err("Not an unary operator".to_string()),
        }
    }

    pub fn visit_binop(&mut self, node: &BinaryOpNode) -> Result<Token, String> {
        let lhs = match self.visit(&node.lhs) {
            Ok(Token::Number(val)) => val,
            _ => return Err("not a number".to_string()),
        };
        let rhs = match self.visit(&node.rhs) {
            Ok(Token::Number(val)) => val,
            _ => return Err("not a number".to_string()),
        };

        match node.token {
            Token::And => Ok(Token::Number(lhs & rhs)),
            Token::Or => Ok(Token::Number(lhs | rhs)),
            Token::RShift => Ok(Token::Number(lhs >> rhs)),
            Token::LShift => Ok(Token::Number(lhs << rhs)),
            _ => Err("Not a binary operator".to_string()),
        }
    }

    pub fn visit_assign(&mut self, node: &AssignmentNode) -> Result<Token, String> {
        let lhs = match self.visit(&node.lhs) {
            Ok(Token::Number(val)) => val,
            _ => return Err("not a number".to_string()),
        };
        let rhs = match self.visit(&node.rhs) {
            Ok(Token::Symbol(sym)) => sym,
            _ => return Err("not a symbol".to_string()),
        };
        self.globals.insert(rhs, lhs);
        Ok(Token::EndOfFile)
    }
}

struct Executor {
    globals: HashMap<String, u16>,
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            globals: HashMap::new(),
        }
    }

    pub fn reset(&mut self) {
        self.globals = HashMap::new();
    }

    pub fn eval(&mut self, input: &str) -> Result<(), String> {
        let line = input.to_string();
        let mut lexer = Lexer::new(&line);
        let mut parser = Parser::new(&mut lexer);
        let mut interpreter = Interpreter::with_globals(&mut parser, self.globals.clone());

        match interpreter.interpret() {
            Ok(_) => {
                self.globals.extend(interpreter.globals);
                Ok(())
            }
            Err(_) => Err("".to_string()),
        }
    }

    pub fn get<S: Into<String>>(&self, var: S) -> Option<u16> {
        let key = var.into();
        let val = self.globals.get(&key);
        val.copied()
    }

    pub fn solve(&mut self, lines: &Vec<&str>) {
        self.globals.reserve(lines.len());
        let mut lines: VecDeque<_> = VecDeque::from(lines.clone());

        while !lines.is_empty() {
            let Some(line) = lines.pop_front() else { break };
            match self.eval(line) {
                Ok(_) => continue,
                Err(_) => lines.push_back(line),
            }
        }
    }
}

fn main() {
    let data = aoc_input!(2015, 7).unwrap();
    let lines: Vec<_> = data.split('\n').filter(|&x| !x.is_empty()).collect();
    let mut executor = Executor::new();

    // Part I
    executor.solve(&lines);
    let a_signal = executor.get("a").unwrap();
    println!("{}", a_signal);

    // Part II
    executor.reset();
    let lines: Vec<_> = lines.into_iter().filter(|&x| !x.ends_with(" b")).collect();
    executor.globals.insert("b".to_string(), a_signal);
    executor.solve(&lines);
    let a_signal = executor.get("a").unwrap();
    println!("{}", a_signal);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let example: Vec<_> = vec![
            "123 -> x",
            "NOT x -> h",
            "456 -> y",
            "x AND y -> d",
            "x OR y -> e",
            "x LSHIFT 2 -> f",
            "y RSHIFT 2 -> g",
            "NOT y -> i",
        ];
        let mut executor = Executor::new();
        executor.solve(&example);
        assert_eq!(executor.get("x").unwrap(), 123);
    }
}
