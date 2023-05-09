#[derive(Debug, Clone, Copy)]
enum Token {
    Number(u32),
    Add,
    Mul,
}

impl Token {
    fn power(self) -> (u8, u8) {
        match self {
            Token::Number(_) => (0, 0),
            Token::Add => (1, 2),
            Token::Mul => (3, 4),
        }
    }
}

trait Lexer<T: Copy> {
    fn next(&mut self) -> T;
    fn peek(self) -> T;
}

impl<T: Copy> Lexer<T> for Vec<T> {
    fn next(&mut self) -> T {
        self.remove(0)
    }
    fn peek(self) -> T {
        *self.first().unwrap()
    }
}


struct Parser<T: Copy> {
    lexer: Box<dyn Lexer<T>>
}

impl<T: Copy> Parser<T> {
    fn new(lexer: Box<impl Lexer<T> + 'static>) -> Self{
        return Parser{
            lexer
        }
    }   

    fn parse(self) -> Vec<Token> {
        
        vec![]
    } 

}

fn main() {
    let mut token = vec![
        Token::Number(10),
        Token::Add,
        Token::Number(12),
        Token::Mul,
        Token::Number(3),
    ];

    token.next();
    let parser = Parser::new(Box::new(token));

    println!("{:?}", parser.parse());    
}
