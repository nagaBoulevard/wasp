
mod ast;
mod base;
mod store;

use self::ast::AST;
use ::parser::Parser;
use ::parser::expr::Expr;

pub struct Core {
    pub input: Vec<char>
}

impl Core {

    pub fn new(input: String) -> Self {
        Core { input: input.chars().collect() }
    }

    pub fn eval_all(p: &Parser) {
        for expr in &p.exprs {
            println!["{}", Core::eval(&expr) ]
        }
    }

    pub fn eval(e: &Expr) -> AST {
        match *e {
            Expr::Value(ref atom) => AST::from_atom(atom),
            Expr::Call{ref name, ref args} => {
                let args: Vec<AST> = args.iter().map(|expr| Core::eval(expr) ).collect();
                match &*name.repr {
                    "echo" => base::echo(&args),
                    "add"  => base::add(&args),
                    "if"   => base::if_fn(&args),
                    "eq"   => base::eq_fn(&args),
                    "loop" => base::loop_fn(&args),
                    _ => panic!("Unknown fn {}", &name.repr)
                }
            }
        }
    }
}

