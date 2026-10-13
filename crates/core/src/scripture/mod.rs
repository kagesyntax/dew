//! Scripture reference engine: book catalogue, reference types and the parser that turns
//! "Rom. 12:1-2; 13:8" in manual text into tappable links.

pub mod books;
pub mod parser;
pub mod reference;

pub use books::{Book, Testament, BOOKS};
pub use parser::{find_references, parse, segment, Segment};
pub use reference::{verse_key, Passage, Reference};
