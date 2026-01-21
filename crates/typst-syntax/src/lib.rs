//! Parser and syntax tree for Typst.

pub mod ast;
pub mod package;

mod file;
mod highlight;
mod kind;
mod lexer;
<<<<<<< HEAD
=======
mod lines;
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
mod node;
mod parser;
mod path;
mod reparser;
mod set;
mod source;
mod span;

pub use self::file::FileId;
<<<<<<< HEAD
pub use self::highlight::{highlight, highlight_html, Tag};
=======
pub use self::highlight::{Tag, highlight, highlight_html};
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
pub use self::kind::SyntaxKind;
pub use self::lexer::{
    is_id_continue, is_id_start, is_ident, is_newline, is_valid_label_literal_id,
    link_prefix, split_newlines,
};
<<<<<<< HEAD
=======
pub use self::lines::Lines;
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
pub use self::node::{LinkedChildren, LinkedNode, Side, SyntaxError, SyntaxNode};
pub use self::parser::{parse, parse_code, parse_math};
pub use self::path::VirtualPath;
pub use self::source::Source;
pub use self::span::{Span, Spanned};

<<<<<<< HEAD
use self::lexer::{LexMode, Lexer};
use self::parser::{reparse_block, reparse_markup};
=======
use self::lexer::Lexer;
use self::parser::{reparse_block, reparse_markup};

/// The syntax mode of a portion of Typst code.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum SyntaxMode {
    /// Text and markup, as in the top level.
    Markup,
    /// Math atoms, operators, etc., as in equations.
    Math,
    /// Keywords, literals and operators, as after hashes.
    Code,
}
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
