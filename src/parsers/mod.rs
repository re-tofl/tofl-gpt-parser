pub mod parser_trs;
pub mod parser_interpret;

pub use parser_trs::ParserTRS;
pub use parser_interpret::ParserInterpret;

use crate::models::Parser;

pub trait Parse {
    type Output;
    fn parse(&mut self, data: &str) -> Result<(Self::Output), String>;
}