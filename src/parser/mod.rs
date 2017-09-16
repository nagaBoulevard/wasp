
pub mod expr;
use self::expr::Expr;
use ::lexer::Lexer;
use ::core::Core;


pub struct Parser<'p> {
    core: &'p Core,
    lexer: &'p Lexer<'p>,
    pub exprs: Vec<Expr>
}

impl<'p> Parser<'p> { // {{{

    pub fn new(c: &'p Core, lexer: &'p Lexer) -> Self {
        Parser{ core: c, lexer: lexer, exprs: vec![] }
    }

    pub fn parse(&mut self) {
        for atom in &self.lexer.atoms {
            self.exprs.push(Expr::new(atom));
        }
    }

} // }}}


