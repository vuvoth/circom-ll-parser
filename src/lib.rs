use std::fmt::Debug;

use token::TokenTrait;
use trace::TreeShape;

mod lexer;
pub mod syntax;
mod token;
pub mod trace;

#[derive(Debug, Clone, Copy)]
pub enum Token {
    Number(u32),
    Add,
    Mul,
    End,
}

impl TokenTrait for Token {
    fn power(self) -> (u8, u8) {
        match self {
            Token::Number(_) => (9, 0),
            Token::Add => (1, 2),
            Token::Mul => (3, 4),
            Token::End => (10, 10),
        }
    }
    fn is_end(self) -> bool {
        matches!(self, Self::End)
    }
    fn is_atom(self) -> bool {
        match self {
            Token::Number(_) | Token::End => true,
            _ => false,
        }
    }
    fn is_op(self) -> bool {
        !self.is_atom()
    }
    fn end() -> Self {
        Self::End
    }
}



impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}
pub trait Lexer<T: TokenTrait + Copy> {
    fn next(&mut self) -> T;
    fn peek(&self) -> T;
}

impl<T: TokenTrait + Copy> Lexer<T> for Vec<T> {
    fn next(&mut self) -> T {
        if self.peek().is_end() {
            return T::end();
        }
        self.remove(0)
    }
    fn peek(&self) -> T {
        *self.first().unwrap_or(&T::end())
    }
}

pub struct Parser<T: TokenTrait + Copy + Debug> {
    lexer: Box<dyn Lexer<T>>,
}

impl<T: TokenTrait + Copy + Debug> Parser<T> {
    pub fn new(lexer: Box<impl Lexer<T> + 'static>) -> Self {
        return Parser { lexer };
    }

    pub fn parse(&mut self) -> Vec<T> {
        self.parse_bp(0)
    }

    // bp = binding power
    pub fn parse_bp(&mut self, min_bp: u8) -> Vec<T> {
        let token = self.lexer.next();
        let mut lhs = if token.is_atom() {
            vec![token]
        } else {
            let op = vec![token];
            let (_left_bp, right_pb) = token.power();
            let rhs = self.parse_bp(right_pb);
            [rhs, op].concat()
        };

        loop {
            let op = self.lexer.peek();

            if op.is_end() {
                break;
            }

            if op.is_atom() {
                panic!("atom can't follow after atom!!!");
            }

            // now op is + or *

            let (left_bp, right_bp) = op.power();
            if left_bp < min_bp {
                break;
            }
            self.lexer.next();
            let rhs = self.parse_bp(right_bp);

            lhs = [lhs, rhs, vec![op]].concat()
        }
        lhs
    }
}

#[cfg(test)]
mod tests {
    use super::{Token, Parser};
    #[test]
    fn test_parser() {
        // 10 + 12 * 3
        let token = vec![
            Token::Number(10),
            Token::Add,
            Token::Number(12),
            Token::Mul,
            Token::Number(3),
        ];

        let mut parser = Parser::new(Box::new(token));

        assert!(parser.parse().iter().eq([
            Token::Number(10),
            Token::Number(12),
            Token::Number(3),
            Token::Mul,
            Token::Add
        ]
        .iter()));
    }
}

