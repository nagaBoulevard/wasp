
pub mod atom;
pub mod token;
use ::core::Core;
use self::atom::Atom;
use self::token::Token;


const SEXPR_START: char = '(';
const SEXPR_END  : char = ')';
const VEC_START  : char = '[';
const VEC_END    : char = ']';


// SPECIALS {{{

const SPECIALS: [char; 19] = 
            ['!', '@', '#', '$',
             '%', '^', '&', ':',
             '+', '-', '*', '/',
             '=', '>', '<', ',',
             ';', '.', '?'];
fn is_special(ch: char) -> bool {
    if SPECIALS.contains(&ch) {true}
    else {false}
}


// }}}



pub struct Lexer<'s> {
    core: &'s Core,
    pos: usize,
    pub atoms: Vec<Atom>
}


impl<'s> Lexer<'s> {

    pub fn new(core: &'s Core) -> Self {
        Lexer { core: core, pos: 0, atoms: vec![] }
    }

    // movement functions {{{

   /// return the current char being read, and DON'T move
    pub fn this(&mut self) -> Option<char> {
        if self.pos < self.core.input.len() {
            Some(self.core.input[self.pos])
        } else {
            None
        }
    }


   /// return the current char being read, and move
    pub fn next(&mut self) -> Option<char> {
        if self.pos < self.core.input.len() {
            self.pos += 1;
            Some(self.core.input[self.pos-1])
        } else {
            None
        }
    }


    /// return the current char being read, and move back
    pub fn back(&mut self) -> Option<char> {
        if self.pos > 0 {
            self.pos -= 1;
            Some(self.core.input[self.pos])
        } else {
            None
        }
    }

    // movement functions }}}



    // main_parser {{{
    pub fn main_parser(&mut self) {
        // the container for all the atoms parsed
        // let mut expr: Vec<Atom> = Vec::new();

        while let Some(ch) = self.this() { // switch loop {{{
            let atom = match ch {
                // spaces and similar are ignored
                ' ' | '\t' | '\n' => {self.next(); continue},

                '(' => self.sexpr_parser(),

                ')' => panic!("Found ')' unclosed delimiter at {}", self.pos),

                '0'...'9' => self.num_parser(),

                'a'...'z' | '_' => self.ident_parser(),

                'A'...'Z' => self.class_parser(),

                ch @ _ if is_special(ch) => self.op_parser(),

                '\'' => self.str_parser(),

                '[' => self.vec_parser(),

                // in theory the closing ']' should be handled by the 
                ']' => panic!("Found ']' unclosed delimter at {}", self.pos),

                unk @ _ => panic!["Found unkown atom: {:?} at {}", unk, self.pos]
            };

            self.atoms.push(atom);

        } // }}}

    } // }}}



    // expr_parser {{{
    fn sexpr_parser(&mut self) -> Atom {
        // the container for all the atoms inside the Expr
        let mut expr: Vec<Atom> = Vec::new();
        // save the starting pos
        let start = self.pos;
        // discard the current '('
        self.next();

        while let Some(ch) = self.this() { // switch loop {{{
            let atom = match ch {
                // spaces and similar are ignored
                ' ' | '\t' | '\n' => {self.next(); continue}

                '0'...'9' => self.num_parser(),

                'a'...'z' | '_' => self.ident_parser(),

                'A'...'Z' => self.class_parser(),

                ch @ _ if is_special(ch) => self.op_parser(),

                '\'' => self.str_parser(),

                '(' => self.sexpr_parser(),

                // in theory this should be the closing ')',
                // since all the sub vectors ']' should have been handled
                ')' => { self.next(); break },

                '[' => self.vec_parser(),

                // in theory the closing ']' should be handled by the 
                ']' => panic!("Found ']' unclosed delimter at {}", self.pos),

                unk @ _ => panic!["Found unkown atom: {:?} at {}", unk, self.pos]
            };

            expr.push(atom);
        } // }}}

        Atom::rec(Token::SExpr, expr, start)

    } // }}}


    // vec_parser {{{
    fn vec_parser(&mut self) -> Atom {
        // the container for all the atoms inside the Expr
        let mut expr: Vec<Atom> = Vec::new();
        // save the start pos
        let start = self.pos;
        // discard the current '['
        self.next();

        while let Some(ch) = self.this() { // switch loop {{{
            let atom = match ch {
                // spaces and similar are ignored
                ' ' | '\t' | '\n' => {self.next(); continue}

                '0'...'9' => self.num_parser(),

                'a'...'z' | '_' => self.ident_parser(),

                'A'...'Z' => self.class_parser(),

                ch @ _ if is_special(ch) => self.op_parser(),

                '\'' => self.str_parser(),

                '[' => self.vec_parser(),

                // in theory this should be the closing ')',
                // since all the sub vectors ']' should have been handled
                ']' => { self.next(); break },

                '(' => self.sexpr_parser(),

                ')' => panic!("Found unclosed `)` delimter at {}", self.pos),

                unk @ _ => panic!["Found unkown atom: {:?} at {}", unk, self.pos]
            };

            expr.push(atom);
        } // }}}

        Atom::rec(Token::VecLiteral, expr, start)

    } // }}}



    /// parse a number
    /// a number must follow the syntax "\d[\d_]*(.\d[\d_]*)?"
    /// thus floats are supported.
    /// This parser also handles the 'num.num' (decimal)
    /// and 'num:num' (range) syntax.
    /// In this way, both the '.' and ':' chars can be used as standalone
    /// operators.
    // num_parser {{{
    fn num_parser(&mut self) -> Atom { 
        let mut res: Vec<char> = Vec::new();

        let mut token = Token::IntLiteral;

        while let Some(ch) = self.this() {
            match ch {
                '0'...'9' => res.push(self.next().unwrap()),
                '_'      => {self.next();},
                '.' if token == Token::IntLiteral => {
                        self.next(); // discard the dot
                        match self.this() {
                            Some('0'...'9') => {token = Token::DecLiteral; res.push('.')},
                            _ => {self.back(); break} }
                    }
                ':' if token == Token::IntLiteral => {
                        self.next(); // discard the :
                        match self.this() {
                            Some('0'...'9') => {token = Token::RangeLiteral; res.push(':')},
                            _ => {self.back(); break} }
                    }
                _ => break

            }
        }
        Atom::new(res, token, self.pos)
    } // }}}


    /// parse an identifier
    /// This is any repr that is either lower or uppercase,
    /// starts with a '_' or a letter, and can contains numbers
    // ident_parser {{{
    fn ident_parser(&mut self) -> Atom {
        let mut res: Vec<char> = Vec::new();

        while let Some(ch) = self.this() {
            match ch {
                'a'...'z' | 'A'...'Z' | '_' | '0'...'9' => res.push(self.next().unwrap()),
                _ => break
            }
        }

        // parse the string to find out if it is a keyrepr
        let token = match &*res.clone().into_iter().collect::<String>() {
            // is this a primitive type?
            "int" | "dec" | "bool" | "str" | "vec" | "void" => Token::ClassLiteral,
            "true" | "false" => Token::BoolLiteral,
            _ => Token::Ident
        };

        Atom::new(res, token, self.pos)
    } // }}}


    /// parse a Class identifier
    /// It's the same as an ident, but starts with an uppercase letter
    // class_parser {{{
    fn class_parser(&mut self) -> Atom {
        let mut res: Vec<char> = Vec::new();
        loop {
            match self.this() {
                Some('a'...'z') | Some('A'...'Z') | Some('_')
                | Some('0'...'9') => res.push(self.next().unwrap()),
                _ => break
            }
        }
        Atom::new(res, Token::ClassLiteral, self.pos)
    } // }}}


    /// parse a string
    /// similar to other collection, but the only item that could lies
    /// inside are '{' '}' delimited string interpolations
    // str_parser {{{
    fn str_parser(&mut self) -> Atom {
        // TODO let mut expr: Vec<Atom> = Vec::new();
        // discard the starting `'`:m-2
        let mut res: Vec<char> = vec![self.next().unwrap()];

        while let Some(ch) = self.this() {
            let sym = match ch {
                '\'' => { res.push(self.next().unwrap()); break },
                // '{' => string interpolation,
                _ => { res.push(self.next().unwrap()); continue }
            };
        }
        Atom::new(res, Token::StrLiteral, self.pos)
    } // }}}


    /// parse an operator
    /// An operator is any other ASCII char, that is not represented in any other parser,
    /// like `+` or `:=`.
    // atoms with no space between them are parsed as one, and represented as a string
    // op_parser {{{
    fn op_parser(&mut self) -> Atom {
        let mut res: Vec<char> = Vec::new();
        while let Some(ch) = self.this() {
            match ch {
                ch @ _ if is_special(ch) => res.push(self.next().unwrap()),
                _ => break
            }
        }

        /// the `-` atom can also refer to a comment
        /// this will start the comment parsing.
        /// By doing it in the op_parser, it allows to easily parse `-`

        // transform the atoms stored in a string
        let res_str: String = res.clone().into_iter().collect();
        if res_str.starts_with("----") {
            return self.comment_doc_parser()
        } else if res_str.starts_with("--") {
            return self.comment_parser()
        }

        // for a meaning of the operators check the reader/token.rs
        let token = match &*res_str {
            ","  => Token::Comma,
            ";"  => Token::Smc,
            "."  => Token::Dot,
            ":"  => Token::Dbc,
            "="  => Token::Equal,
            "-"  => Token::Dash,
            "+"  => Token::Plus,
            "*"  => Token::Star,
            "/"  => Token::Slash,
            "!"  => Token::Esc,
            "->" => Token::Arrow,
             _   => panic!["Found unknown operator: {}", res_str]
        };

        Atom::new(res, token, self.pos)
    } // }}}


    /// parse a comment
    /// return a Atom with the content of the commment
    // comment_parser {{{
    fn comment_parser(&mut self) -> Atom {
        // the op_parser should have already handled the '--'
        let mut res: Vec<char> = vec!['-','-'];
        while let Some(ch) = self.this() {
            match ch {
                '\n' => break,
                _ => res.push(self.next().unwrap())
            }
        }
        Atom::new(res, Token::Comment, self.pos)
    }
    // }}}


    /// parse a multiline comment, that is, a doc
    // comment_doc_parser {{{
    fn comment_doc_parser(&mut self) -> Atom {
        let mut res: Vec<char> = vec!['-', '-', '-', '-'];
        // the number of consecutive '-'. If reach 4, then exit the parser
        let mut dash = 0;
        while let Some(ch) = self.this() {
            match ch {
                _   if dash == 4 => break,
                '-' => {dash += 1; res.push(self.next().unwrap())},
                _   => {dash = 0; res.push(self.next().unwrap())}
            }
        }
        // TODO remove leading ----
        Atom::new(res, Token::CommentDoc, self.pos)
    } // }}}



}


