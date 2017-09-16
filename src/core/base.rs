//! here are stored the base functions and macroes

use ::core::AST;
use ::core::store::Function;

pub fn echo(args: &Vec<AST>) -> AST {
    assert_eq!(args.len(), 1);
    args[0].clone()
}



pub fn add(args: &Vec<AST>) -> AST {
    let fn_add = Function::define("add".to_string(), vec![AST::Int(0), AST::Int(0)], AST::Int(0) );
    fn_add.call(args);
    match &args[..] {
        &[AST::Int(x), AST::Int(y)] => AST::Int(x + y),
        &[AST::Dec(x), AST::Dec(y)] => AST::Dec(x + y),
        _ => panic!("args error")
    }
}



pub fn if_fn(args: &Vec<AST>) -> AST {
    match &args[..] {
        &[AST::Bool(condition), ref on_true, ref on_false] => { if condition { on_true.clone() } else { on_false.clone() } },

        &[AST::Bool(condition), ref on_true] => { if condition { on_true.clone() } else { AST::Void } },

        _ => panic!("if error")
    }
}




pub fn eq_fn(args: &Vec<AST>) -> AST {
    match &args[..] {
        &[AST::Int(ref x), AST::Int(ref y)] => AST::Bool(x == y),
        &[AST::Dec(ref x), AST::Dec(ref y)] => AST::Bool(x == y),
        &[AST::Str(ref a), AST::Str(ref b)] => AST::Bool(a == b),
        _ => panic!("eq error")
    }
}



pub fn loop_fn(args: &Vec<AST>) -> AST {
    let x = match args.first() {
        Some(&AST::Int(x)) => x,
        _ => panic!("loop error")
    };
    match &args[1..] {
        body => { 
            for _x in 0..x {
                for elem in body { 
                    println!["{}", elem] 
                } 
            }
        },
        _ => panic!("loop error")
    }
    AST::Void
}
