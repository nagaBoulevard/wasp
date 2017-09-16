
use std::fmt;

use ::lexer::token::Token;

/// A Atom represent the basic token present in the code.
#[derive(Debug, Clone)]
pub struct Atom {
    /// the representation of the Token
    pub repr: String,

    pub token: Token,

    /// use by Atoms that are collections, like Vector.
    /// Store the various Atom present in the collection,
    /// otherwise is empty for non-collection Atom
    pub inside: Vec<Atom>,

    /// the index of the Atom in the source text
    pub start: usize,
    pub end: usize

}


impl Atom { // {{{


    pub fn new(s: Vec<char>, t: Token, end: usize) -> Self {
        Atom{
            repr: s.iter().collect(),
            token: t,
            inside: Vec::new(),
            start: end - s.len(),
            end: end - 1
        }
    }

    pub fn rec(t: Token, inside: Vec<Atom>, start: usize) -> Self {
        let len: usize = inside.iter().map(|atom| atom.repr.len() ).sum();
        Atom{
            repr: "".to_string(),
            token: t,
            inside: inside,
            // TODO
            start: start,
            end: start + len
        }
    }


} // }}}

impl fmt::Display for Atom { // {{{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.inside.is_empty() {
            write!(f, "[{}:{}]({:?} {})", self.start, self.end, self.token, self.repr)
        } else {
            let mut inside_repr = "".to_string();
            for sym in &self.inside {
                inside_repr.push_str(&format!(" {} ", sym))
            }
            write!(f, "[{}:{}]({:?} {} )", self.start, self.end, self.token, inside_repr)
        }
    }
} // }}}
