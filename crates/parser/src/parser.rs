use std::cell::Cell;

use logos::Lexer;

use crate::{
    event::Event,
    node::{Child, Token, Tree},
    token_kind::TokenKind,
};

pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a, TokenKind>,
    pos: usize,
    current_token: Option<Token<'a>>,
    fuel: Cell<u32>,
    events: Vec<Event<'a>>,
}

pub enum Marker {
    Open(usize),
    Close(usize),
}

impl<'a> Parser<'a> {
    pub fn open(&mut self) -> Marker {
        let marker = Marker::Open(self.events.len());
        self.events.push(Event::Open {
            kind: TokenKind::Error,
        });
        marker
    }

    pub fn open_before(&mut self, marker_closed: Marker) -> Marker {
        match marker_closed {
            Marker::Close(index) => {
                let marker_opened = Marker::Open(index);
                self.events.insert(
                    index,
                    Event::Open {
                        kind: TokenKind::EOF,
                    },
                );
                marker_opened
            }
            _ => unreachable!(),
        }
    }

    pub fn close(&mut self, marker_close: Marker, kind: TokenKind) -> Marker {
        match marker_close {
            Marker::Open(index) => {
                self.events[index] = Event::Open { kind };
                self.events.push(Event::Close);
                Marker::Close(index)
            }
            _ => unreachable!(),
        }
    }

    pub fn advance(&mut self) {
        assert!(!self.eof());
        self.fuel.set(256);
        let token = Event::Token(self.current());
        self.events.push(token);
        self.skip();
    }

    fn advance_with_error(&mut self, error: &str) {
        let m = self.open();
        // TODO: Error reporting.
        eprintln!("{error}");
        self.advance();
        self.close(m, TokenKind::Error);
      }

    pub fn build_tree(self) -> Tree<'a> {
        let mut events = self.events;
        let mut stack = Vec::new();
        assert!(matches!(events.pop(), Some(Event::Close)));

        for event in events {
            match event {
                Event::Open { kind } => {
                    stack.push(Tree {
                        kind,
                        children: Vec::new(),
                    });
                }
                Event::Close => {
                    let tree = stack.pop().unwrap();
                    stack.last_mut().unwrap().children.push(Child::Tree(tree));
                }
                Event::Token(token) => {
                    stack.last_mut().unwrap().children.push(Child::Token(token));
                }
                _ => unreachable!(),
            }
        }

        let tree = stack.pop().unwrap();
        assert!(stack.is_empty());
        tree
    }
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a, TokenKind>) -> Self {
        Self {
            lexer,
            pos: 0,
            current_token: None,
            fuel: Cell::new(256),
            events: Vec::new(),
        }
    }

    pub fn current(&mut self) -> Token<'a> {
        if self.current_token.is_none() {
            self.next();
        }
        return self.current_token.unwrap();
    }

    pub fn next(&mut self) -> TokenKind {
        let kind = self.lexer.next().unwrap_or(TokenKind::EOF);
        self.current_token = Some(Token::new(kind, self.lexer.slice()));
        kind
    }

    pub fn at(&mut self, kind: TokenKind) -> bool {
        let token = self.current();
        token.kind == kind
    }

    pub fn skip(&mut self) {
        self.next();
    }

    pub fn eat(&mut self, kind: TokenKind) -> bool {
        if self.at(kind) {
            let text = self.lexer.slice();
            self.events.push(Event::Token(Token::new(kind, text)));
            self.skip();
            return true;
        }
        return false;
    }

    pub fn expect(&mut self, kind: TokenKind) {
        if self.eat(kind) {
            return;
        }
        // TODO: Error reporting.
        eprintln!("expected {kind:?}");
    }

    pub fn eof(&mut self) -> bool {
        self.current().kind == TokenKind::EOF
    }
}

pub fn file(p: &mut Parser) {
    let m = p.open();
    while !p.eof() {
        if p.at(TokenKind::Template) {
            template(p);
        } else {
            p.advance_with_error("expected a template")
        }
    }
    p.close(m, TokenKind::File);
}
/**
 * template name() {content}
 *
 */
pub fn template(p: &mut Parser) {
    assert!(p.at(TokenKind::Template));
    let m = p.open();
    p.expect(TokenKind::Template);
    p.expect(TokenKind::Name);
    p.expect(TokenKind::LParen);
    p.expect(TokenKind::RParen);
    block(p);
    p.close(m, TokenKind::Template);
}

pub fn block(p: &mut Parser) {
    eprintln!("{:?}", p.current());
    assert!(p.at(TokenKind::LCurly));
    let m = p.open();
    p.eat(TokenKind::LCurly);
    while !p.at(TokenKind::RCurly) && !p.eof() {
        p.advance();
    }

    p.expect(TokenKind::RCurly);

    p.close(m, TokenKind::Block);
}

#[cfg(test)]
mod tests {
    use logos::{Lexer};

    use crate::token_kind::TokenKind;

    use super::{template, Parser, file};

    #[test]
    fn test_parser() {
        let source = "
            template name() {
                block,
                abs 
                def
            }
            template another() {
                hello
            }
        ";
        let mut lexer = Lexer::<TokenKind>::new(source);
        let mut parser = Parser::new(&mut lexer);

        file(&mut parser);

        let tree = parser.build_tree();
        
        println!("{:?}", tree);
    }
}
