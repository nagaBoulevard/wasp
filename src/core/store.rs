
use ::core::ast::AST;

pub struct Function {
    name: String,
    args_required: Vec<Vec<AST>>,
    args_given: Vec<AST>, 
    return_class: AST
}


impl Function {

    pub fn define(name: String, args_required: Vec<AST>, return_class: AST) -> Self {
        Function { name: name, args_required: vec![args_required], return_class: return_class,
                    args_given: vec![] }
    }

    pub fn call(&self, args_given: &Vec<AST>) {
        let mut ok = true;
        for args in &self.args_required {
            if args_given != args {
                ok = false;
            }
        }

        if !ok {
            panic!("No fn named `{}` found with args {:?}\nPossible solution: `({0} {:?})`",
                    self.name, args_given, self.args_required) 
        }
 
    }

}


