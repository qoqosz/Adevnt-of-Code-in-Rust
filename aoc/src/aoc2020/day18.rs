use aoc::{aoc, aoc_input};
use std::iter::Peekable;
use std::mem::discriminant;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
    // default
    Illegal,
    EndOfFile,

    // number
    Integer(i64),

    // operations
    Add,
    Mul,
    LParen,
    RParen,
}

impl Default for Token {
    fn default() -> Self {
        Token::Illegal
    }
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    fn new(line: &'a String) -> Self {
        Lexer {
            input: line.chars().peekable(),
        }
    }

    fn next_token(&mut self) -> Token {
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
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        match self.read_char() {
            Some('+') => Some(Token::Add),
            Some('*') => Some(Token::Mul),
            Some('(') => Some(Token::LParen),
            Some(')') => Some(Token::RParen),

            // `ch @ _` saves `_` in `ch` variable
            Some(ch @ _) => {
                if ch.is_numeric() {
                    Some(Token::Integer(self.read_number(ch).parse::<i64>().unwrap()))
                } else {
                    println!("ch <{}>", ch);
                    Some(Token::Illegal)
                }
            }

            None => None,
        }
    }
}

/// A node in the Abstract Syntax Tree (AST)
pub trait Node {
    fn accept(&self, visitor: &Interpreter) -> Result<i64, String>;
}

/// Integer node
pub struct IntegerNode {
    pub token: Token,
}

impl From<i64> for IntegerNode {
    fn from(num: i64) -> Self {
        IntegerNode {
            token: Token::Integer(num),
        }
    }
}

impl Node for IntegerNode {
    fn accept(&self, visitor: &Interpreter) -> Result<i64, String> {
        visitor.visit_int(self)
    }
}

pub struct BinaryOpNode {
    pub token: Token,
    pub lhs: Box<dyn Node>,
    pub rhs: Box<dyn Node>,
}

impl Node for BinaryOpNode {
    fn accept(&self, visitor: &Interpreter) -> Result<i64, String> {
        visitor.visit_binop(self)
    }
}

macro_rules! box_node {
    ($lhs:tt * $rhs:expr) => {
        Box::new(BinaryOpNode {
            token: Token::Mul,
            lhs: $lhs,
            rhs: $rhs,
        })
    };
    ($lhs:tt + $rhs:expr) => {
        Box::new(BinaryOpNode {
            token: Token::Add,
            lhs: $lhs,
            rhs: $rhs,
        })
    };
    ($num:tt) => {
        Box::new(IntegerNode::from($num))
    };
}

trait AbstractParser {
    fn expr(&mut self) -> Result<Box<dyn Node>, String>;
}

pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    fn new(lexer: &'a mut Lexer<'a>) -> Self {
        let current_token = lexer.next_token();
        Self {
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

    /// factor : (PLUS | MINUS) factor | INTEGER | LPAREN expr RPAREN
    fn factor(&mut self) -> Result<Box<dyn Node>, String> {
        match self.current_token {
            Token::Integer(num) => {
                self.eat(&Token::Integer(0))?;
                Ok(box_node!(num))
            }
            Token::LParen => {
                self.eat(&Token::LParen)?;
                let result = self.term();
                self.eat(&Token::RParen)?;
                result
            }
            Token::Illegal => Err("Illegal input".to_string()),
            _ => Err("Invalid syntax - expecting an integer \
                     or a left parenthesis"
                .to_string()),
        }
    }

    /// term : factor ((MUL | PLUS) factor)*
    fn term(&mut self) -> Result<Box<dyn Node>, String> {
        let mut node = self.factor()?;

        loop {
            match self.current_token {
                Token::Mul => {
                    self.eat(&Token::Mul)?;
                    node = box_node!(node * self.factor()?);
                }
                Token::Add => {
                    self.eat(&Token::Add)?;
                    node = box_node!(node + self.factor()?);
                }
                _ => break,
            }
        }

        Ok(node)
    }
}

impl<'a> AbstractParser for Parser<'a> {
    fn expr(&mut self) -> Result<Box<dyn Node>, String> {
        self.term()
    }
}

pub struct Interpreter<'a> {
    parser: &'a mut dyn AbstractParser,
}

impl<'a> Interpreter<'a> {
    fn new(parser: &'a mut impl AbstractParser) -> Self {
        Interpreter { parser }
    }

    fn interpret(&mut self) -> Result<i64, String> {
        let tree = self.parser.expr()?;
        self.visit(&tree)
    }

    fn visit(&self, tree: &Box<dyn Node>) -> Result<i64, String> {
        tree.accept(self)
    }

    fn visit_int(&self, node: &IntegerNode) -> Result<i64, String> {
        match node.token {
            Token::Integer(val) => Ok(val),
            _ => Err("Not an integer".to_string()),
        }
    }

    fn visit_binop(&self, node: &BinaryOpNode) -> Result<i64, String> {
        let lhs = self.visit(&node.lhs)?;
        let rhs = self.visit(&node.rhs)?;

        match node.token {
            Token::Add => Ok(lhs + rhs),
            Token::Mul => Ok(lhs * rhs),
            _ => Err("Not a binary operator".to_string()),
        }
    }
}

pub struct ParserAdvanced<'a> {
    lexer: &'a mut Lexer<'a>,
    current_token: Token,
}

impl<'a> ParserAdvanced<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Self {
        let current_token = lexer.next_token();
        Self {
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

    fn factor(&mut self) -> Result<Box<dyn Node>, String> {
        match self.current_token {
            Token::Integer(num) => {
                self.eat(&Token::Integer(0))?;
                Ok(box_node!(num))
            }
            Token::LParen => {
                self.eat(&Token::LParen)?;
                let result = self.expr();
                self.eat(&Token::RParen)?;
                result
            }
            Token::Illegal => Err("Illegal input".to_string()),
            _ => Err("Invalid syntax - expecting an integer \
                     or a left parenthesis"
                .to_string()),
        }
    }

    /// term : factor ((MUL | DIV) factor)*
    fn term(&mut self) -> Result<Box<dyn Node>, String> {
        let mut node = self.factor()?;

        loop {
            match self.current_token {
                Token::Add => {
                    self.eat(&Token::Add)?;
                    node = box_node!(node + self.factor()?);
                }
                _ => break,
            }
        }

        Ok(node)
    }
}

impl<'a> AbstractParser for ParserAdvanced<'a> {
    fn expr(&mut self) -> Result<Box<dyn Node>, String> {
        let mut node = self.term()?;

        loop {
            match self.current_token {
                Token::Mul => {
                    self.eat(&Token::Mul)?;
                    node = box_node!(node * self.term()?);
                }
                _ => break,
            }
        }

        Ok(node)
    }
}

fn calculate(line: &str) -> i64 {
    let mut line = line.to_owned();
    let mut lexer = Lexer::new(&mut line);
    let mut parser = Parser::new(&mut lexer);
    let mut interpreter = Interpreter::new(&mut parser);

    interpreter.interpret().ok().unwrap()
}

fn calculate_advanced(line: &str) -> i64 {
    let mut line = line.to_owned();
    let mut lexer = Lexer::new(&mut line);
    let mut parser = ParserAdvanced::new(&mut lexer);
    let mut interpreter = Interpreter::new(&mut parser);

    interpreter.interpret().ok().unwrap()
}

#[aoc(2020, 18)]
pub fn main() {
    let data = aoc_input!(2020, 18).unwrap();

    // Part I
    let res: i64 = data.trim().lines().map(calculate).sum();
    println!("{res}");

    // Part II
    let res: i64 = data.trim().lines().map(calculate_advanced).sum();
    println!("{res}");
}
