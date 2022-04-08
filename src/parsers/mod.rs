pub mod types;
pub mod javascript;
pub mod typescript;

pub trait Parser {
    fn parse(&self);
}
