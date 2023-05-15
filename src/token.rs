pub trait TokenTrait {
    fn power(self) -> (u8, u8);
    fn is_end(self) -> bool;
    fn is_op(self) -> bool;
    fn is_atom(self) -> bool;
    fn end() -> Self;
}

