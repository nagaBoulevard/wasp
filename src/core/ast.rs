
use ::lexer::atom::Atom;
use ::lexer::token::Token;

#[derive(Debug, Clone)]
pub enum AST {
    Str(String),
    Int(i32),
    Dec(f64),
    Bool(bool),
    Void,


    Comment(String)

}


impl AST {

    pub fn from_atom(atom: &Atom) -> Self { // {{{
        match atom.token {
            Token::StrLiteral => AST::Str(atom.repr.clone()),
            Token::IntLiteral => AST::Int(atom.repr.parse::<i32>().expect(&format!("Failed to parse {} to AST::Int", atom))),
            Token::DecLiteral => AST::Dec(atom.repr.parse::<f64>().expect(&format!("Failed to parse {} to AST::Dec", atom))),
            Token::BoolLiteral => AST::Bool(atom.repr.parse::<bool>().expect(&format!("Failed to parse {} to AST::Bool", atom))),
            Token::Comment    => AST::Comment(atom.repr.clone()),
            _ => unimplemented!()
        }
    } // }}}

}


impl PartialEq for AST {

	fn eq(&self, other: &AST) -> bool {
		match (self, other) {
			(&AST::Int(_), &AST::Int(_)) => true,
			(&AST::Dec(_), &AST::Dec(_)) => true,
			(&AST::Str(_), &AST::Str(_)) => true,
			_ => false
		}
    }
}


use std::fmt; // {{{
impl fmt::Display for AST {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AST::Str(ref s) => write!(f, "{}", s),
            AST::Int(ref i) => write!(f, "{}", i),
            AST::Dec(ref d) => write!(f, "{}", d),
            AST::Bool(ref b) => write!(f, "{}", b),
            AST::Void       => write!(f, "void"),
            AST::Comment(ref c) => write!(f, ""), // TODO what to print with a comment?
        }
    }
} // }}}
