pub use insert::*;
pub use normal::*;
pub use replace::*;
pub use select::*;

mod insert;
mod normal;
mod replace;
mod select;

#[derive(Debug, Clone)]
pub enum Movement {
    Up(u8),
    Down(u8),
    Left(u8),
    Right(u8),
}
