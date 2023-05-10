use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
enum Token {
    Number(u32),
    Add,
    Mul,
    Begin,
    End,
}

impl TokenTrait for Token {
    fn power(self) -> (u8, u8) {
        match self {
            Token::Number(_) => (0, 0),
            Token::Add => (1, 2),
            Token::Mul => (3, 4),
            Token::Begin => (0, 0),
            Token::End => (0, 0),
        }
    }
    fn is_end(self) -> bool {
        matches!(self, Self::End)
    }
}

trait TokenTrait {
    fn power(self) -> (u8, u8);
    fn is_end(self) -> bool;
}

trait Lexer<T: Copy> {
    fn next(&mut self) -> T;
    fn peek(&self) -> T;
}

impl<T: Copy> Lexer<T> for Vec<T> {
    fn next(&mut self) -> T {
        self.remove(0)
    }
    fn peek(&self) -> T {
        *self.first().unwrap()
    }
}

struct Parser<T: TokenTrait + Copy + Debug> {
    lexer: Box<dyn Lexer<T>>,
}

impl<T: TokenTrait + Copy + Debug> Parser<T> {
    fn new(lexer: Box<impl Lexer<T> + 'static>) -> Self {
        return Parser { lexer };
    }

    fn parse(&mut self) -> Vec<T> {
        self.parse_bp(0)
    }

    // bp = binding power
    fn parse_bp(&mut self, min_bp: u8) -> Vec<T> {
        println!("min bp = {}", min_bp);
        let current_token = self.lexer.peek();
        if current_token.is_end() {
            return vec![];
        }

        let mut tokens = vec![current_token];
        let (mut left_bp, _) = current_token.power();
        while left_bp >= min_bp  {
            let curr_token = self.lexer.next();

            println!("{:?} {}", curr_token, min_bp);
            if !curr_token.is_end() {
                left_bp = curr_token.power().0;
                let right_bp = curr_token.power().1;
                // println!("{}, {}", left_bp, right_bp);
                let next_parsing = self.parse_bp(right_bp);

                tokens = [[vec![curr_token], tokens].concat(), next_parsing].concat();
            } else {
                break;
            }
        }
        println!("{:?}", tokens);
        tokens
    }
}

fn main() {
    let mut token = vec![
        Token::Begin,
        Token::Number(10),
        Token::Add,
        Token::Number(12),
        Token::Mul,
        Token::Number(3),
        Token::End,
    ];

    let mut parser = Parser::new(Box::new(token));

    println!("{:?}", parser.parse());
}
