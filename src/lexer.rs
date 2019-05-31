use std::io::BufRead;
use std::collections::VecDeque;
use std::str::FromStr;

extern crate logos;
use logos::{Logos, Extras};

#[derive(Default)]
pub struct IndentHelper {
    tokens: i32,
    whites: i32
}
impl Extras for IndentHelper {
    fn on_advance(&mut self) {
        self.tokens += 1;
    }

    fn on_whitespace(&mut self, _byte: u8) {
        if self.tokens == 1 {
            self.whites += 1;
        }
    }
}

#[derive(Logos, Debug, Copy, Clone, PartialEq)]
#[extras = "IndentHelper"]
enum LogosToken {
    // Keywords:
    #[token = "and"]
    And,
    #[token = "break"]
    Break,
    #[token = "def"]
    Def,
    #[token = "elif"]
    Elif,
    #[token = "else"]
    Else,
    #[token = "for"]
    For,
    #[token = "if"]
    If,
    #[token = "not"]
    Not,
    #[token = "or"]
    Or,
    #[token = "return"]
    Return,
    #[token = "while"]
    While,

    // Types:
    #[regex = "True|False"]
    Boolean,
    #[regex = "-?[0-9]+"]
    Integer,
    #[regex = "-?[0-9]*\\.[0-9]+"]
    Float,
    
    #[regex = "[a-zA-Z_][a-zA-Z0-9_]*"]
    Identifier,

    // Operators:
    #[token = "="]
    Assign,
    #[token = "+"]
    Add,
    #[token = "-"]
    Subtract,
    #[token = "*"]
    Multiply,
    #[token = "/"]
    Divide,
    #[token = "=="]
    Equal,
    #[token = "!="]
    NotEqual,
    #[token = ">"]
    GT,
    #[token = ">="]
    GTE,
    #[token = "<"]
    LT,
    #[token = "<="]
    LTE,

    #[token = "("]
    ParenOpen,
    #[token = ")"]
    ParenClose,

    #[token = ","]
    Comma,
    #[token = ":"]
    Colon,

    // Handled / Implemented by the overlay lexer
    // TODO: Handle Comments
    Newline,
    Indent,
    Dedent,

    #[regex = "#.*"]
    Comment,

    #[error]
    Error,

    #[end]
    End
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords:
    And,
    Break,
    Def,
    Elif,
    Else,
    For,
    If,
    Not,
    Or,
    Return,
    While,

    // Types:
    Boolean(bool),
    Integer(i32),
    Float(f32),
    Identifier(String),
    // Boolean,
    // Integer,
    // Float,
    // Identifier,

    // Operators:
    Assign,
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    GT,
    GTE,
    LT,
    LTE,

    ParenOpen,
    ParenClose,

    Comma,
    Colon,

    // Handled / Implemented by the overlay lexer
    // TODO: Handle Comments
    Newline,
    Indent,
    Dedent,
}

pub struct Lexer<'input> {
    input: &'input mut BufRead,
    indents: Vec<i32>,
    tokens: VecDeque<Token> // TODO: Include Span information.
}
impl<'input> Lexer<'input> {
    pub fn new(source: &'input mut BufRead) -> Self {
        Lexer { 
            input: source,
            indents: vec![0],
            tokens: VecDeque::new()
        }
    }
}
#[derive(Debug)]
pub enum LexError {
    ReadError,
    InvalidError(&'static str)
}
impl<'input> Iterator for Lexer<'input> {
    type Item = Result<(usize, Token, usize), LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        // If we have tokens to give then use them up first
        match self.tokens.pop_front() {
            Some(t) => {
                // TODO: Stop losing span information.
                return Some(Ok((0, t, 0)));
            },
            None => {
                // Otherwise, lex a new line from our input being sure to include any Indents / Dedents / Newlines
                let mut buffer = String::new();
                match self.input.read_line(&mut buffer) {
                    Ok(bytes) => {
                        if bytes > 0 {
                            let mut lex = LogosToken::lexer(buffer.as_str());
                            if lex.token == LogosToken::End {
                                // Blank line:
                                // println!("Blank Line");
                                return self.next();
                            }
                            // Handle Indents / Dedents
                            while let Some(top) = self.indents.last() {
                                if *top < lex.extras.whites {
                                    self.tokens.push_back(Token::Indent);
                                    self.indents.push(lex.extras.whites);
                                } else if *top > lex.extras.whites {
                                    self.tokens.push_back(Token::Dedent);
                                    self.indents.pop();
                                } else {
                                    break;
                                }
                            }
                            // Handle Indent errors
                            if self.indents.len() == 0 {
                                return Some(Err(LexError::InvalidError("Indentation Error")));
                            }
                            // Parse the rest of the line:
                            loop {
                                if lex.token == LogosToken::End {
                                    self.tokens.push_back(Token::Newline);
                                    break;
                                }
                                if lex.token == LogosToken::Comment {
                                    if self.tokens.len() == 0 {
                                        break;
                                    } else {
                                        self.tokens.push_back(Token::Newline)
                                    }
                                }
                                if lex.token == LogosToken::Error {
                                    return Some(Err(LexError::InvalidError("Logos Error")));
                                }
                                if lex.token != LogosToken::Comment {
                                    self.tokens.push_back(match lex.token {
                                        // Unfortunately, Logos doesn't allow properties on Tokens so I have to convert them all because of those 4 tokens that need props :(
                                        LogosToken::And => Token::And,
                                        LogosToken::Break => Token::Break,
                                        LogosToken::Def => Token::Def,
                                        LogosToken::Elif => Token::Elif,
                                        LogosToken::Else => Token::Else,
                                        LogosToken::For => Token::For,
                                        LogosToken::If => Token::If,
                                        LogosToken::Not => Token::Not,
                                        LogosToken::Or => Token::Or,
                                        LogosToken::Return => Token::Return,
                                        LogosToken::While => Token::While,
                                        
                                        LogosToken::Boolean => Token::Boolean(lex.slice() == "True"),
                                        LogosToken::Integer => Token::Integer(i32::from_str(lex.slice()).unwrap()),
                                        LogosToken::Float => Token::Float(f32::from_str(lex.slice()).unwrap()),
                                        LogosToken::Identifier => Token::Identifier(lex.slice().to_string()),

                                        LogosToken::Assign => Token::Assign,
                                        LogosToken::Add => Token::Add,
                                        LogosToken::Subtract => Token::Subtract,
                                        LogosToken::Multiply => Token::Multiply,
                                        LogosToken::Divide => Token::Divide,
                                        LogosToken::Equal => Token::Equal,
                                        LogosToken::NotEqual => Token::NotEqual,
                                        LogosToken::GT => Token::GT,
                                        LogosToken::GTE => Token::GTE,
                                        LogosToken::LT => Token::LT,
                                        LogosToken::LTE => Token::LTE,
                                        LogosToken::ParenOpen => Token::ParenOpen,
                                        LogosToken::ParenClose => Token::ParenClose,
                                        LogosToken::Comma => Token::Comma,
                                        LogosToken::Colon => Token::Colon,
                                        LogosToken::Newline => Token::Newline,
                                        LogosToken::Indent => Token::Indent,
                                        LogosToken::Dedent => Token::Dedent,
                                        _ => unreachable!(), // Should never happen
                                    });
                                }
                                lex.advance();
                            }
                            // Now that we've processed a line, let's start over:
                            return self.next();
                        } else if bytes == 1 {
                            // Blank line (we only read a newline)
                            // println!("Blank line");
                            return self.next();
                        } else {
                            return match self.indents.pop() {
                                Some(s) => {
                                    if s == 0 {
                                        None
                                    } else {
                                        Some(Ok((0, Token::Dedent, 0)))
                                    }
                                },
                                None => None,
                            };
                        }
                    },
                    // Handle read errors
                    Err(_) => return Some(Err(LexError::ReadError))
                }
            }
        }
    }
}