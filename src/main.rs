#[derive(Debug, Clone, Copy)]
enum Token {
    Number(u32),
    Add,
    Mul,
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

fn main() {
    let mut token = vec![
        Token::Number(10),
        Token::Add,
        Token::Number(12),
        Token::Mul,
        Token::Number(3),
    ];

  

    println!("{:?}", token.next());
    println!("{:?}", token.next());
    println!("{:?}", token.peek());
}
