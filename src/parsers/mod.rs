pub mod parser_trs;
pub mod parser_interpret;

pub use parser_trs::ParserTRS;
pub use parser_interpret::ParserInterpret;

use crate::models::ParsedData;

pub trait Parse {
    fn parse(&mut self) -> Result<(ParsedData), Vec<String>>;
}