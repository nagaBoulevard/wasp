
use ::lexer::atom::Atom;
use ::lexer::token::Token;

#[derive(Debug, Clone)]
pub enum Expr {

    Call{name: Atom, args: Vec<Expr>},

    Value(Atom)

}


impl Expr { // {{{

    pub fn new(mut a: &Atom) -> Self {
        // this is a single value, outside of a SExpr
        // TODO should this be permitted?
        if a.inside.is_empty() {
            Expr::Value(a.clone())
        } else {
            // this is a collection of Values. We need to check if it is a SExpr or a Vector
            match &a.token {
                &Token::VecLiteral => Expr::Value(a.clone()),
                &Token::SExpr => Self::parse_sexpr(a.clone()),
                _ => panic!("0x0Expr::new this should not happen")
            }
        }
    }


    fn parse_sexpr(a: Atom) -> Expr { // {{{
        let mut atoms: Vec<Atom> = vec![];
        let mut res: Vec<Expr> = vec![];
        let mut pipe = false;
        let mut i = 0;

        for atom in a.inside {
            match atom.token {

                Token::Dot => pipe = true,

                _ if pipe => {
                    pipe = false;
                    let this = Atom::rec(Token::SExpr, atoms.clone(), atom.start);
                    match res.is_empty() {
                        true => res.push(Expr::parse_call(this, vec![])),
                        _    => {
                            let prev = res[i-1].clone();
                            res.push(Expr::parse_call(this, vec![prev]))
                        }
                    }
                    i += 1;
                    atoms = vec![atom];
                },

                _ => atoms.push(atom)
            }
        }

        let this = Atom::rec(Token::SExpr, atoms.clone(), atoms[0].start);
        match res.is_empty() {
            true => res.push(Expr::parse_call(this, vec![])),
            _    => {
                let prev = res[i-1].clone();
                res.push(Expr::parse_call(this, vec![prev]))
            }
        }
        match res.last() {
            Some(x) => return x.clone(),
            None => panic!("Found None in Expr::parse_sexpr in res.last()")
        }

    } // }}}

    /// create an Expr based on the current SExpr.
    /// If the SExpr contains a single primitive, it is returned as a Value.
    /// If more than one consecutive primitive is found, an error is raised
    /// all other cases return a Expr::Call
    fn parse_call(current: Atom, prev: Vec<Expr>) -> Expr {
        let mut first_primitive = match current.inside[0].token {
            Token::StrLiteral | Token::IntLiteral | Token::DecLiteral | Token::RangeLiteral | Token::VecLiteral 
            | Token::ClassLiteral | Token::BoolLiteral => true,
            _ => false
        };
        // more than one consecutive primitive without a pipe is found
        if current.inside.len() > 1 && first_primitive {
            panic!("Found SExpr with a primitive followed by {} while a pipe was expected: \n{}", current.inside[1], current)
        // this is 
        } else if current.inside.len() == 1 && first_primitive {
            Expr::Value(current.inside[0].clone())
        } else {
            let name = current.inside[0].clone();
            let mut args = current.inside[1..].iter().map(|atom| Expr::new(&atom.clone()) ).collect::<Vec<Expr>>();
            args.extend(prev);
            Expr::Call{ name: name, args: args }
        }
    }


    pub fn is_call(&self) -> bool {
        match *self {
            Expr::Call{ref name, ref args} => true,
            _ => false
        }
    }


    pub fn is_value(&self) -> bool { !self.is_call() }

    pub fn print(&self, depth: i32) {
        let depth_repr = (0..depth).map(|_| "\t").collect::<String>();
        match *self {
            Expr::Value(ref a) => println!["{}Value: {}", depth_repr, a],
            Expr::Call{ref name, ref args} => { println!["{0}Call: \n{0}name: {1}", depth_repr, name];
                                                args.iter().map(|e| e.print(depth+1)).collect::<Vec<()>>(); }
        }
    }

} // }}}
