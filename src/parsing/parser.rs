pub mod parse {
    use std::rc::Rc;

    use crate::{
        ast::cons::cons::Cons,
        parsing::tokenizer::tokenizer::{tokenize, AtomType, Token},
    };

    fn parse_tokens(v: &mut Vec<Token>) -> Vec<Rc<Cons>> {
        let mut tokens: Vec<Rc<Cons>> = Vec::new();

        while !v.is_empty() {
            tokens.push(parse_next_token(v))
        }

        tokens
    }

    fn parse_next_token(v: &mut Vec<Token>) -> Rc<Cons> {
        if v.is_empty() {
            panic!("Unexpected EOF");
        }

        match (v.remove(0)) {
            Token::OpenParens(_) => parse_list(v),
            Token::CloseParens => Cons::nil(),
            Token::Procedure(s) => Cons::new_val(&s),
            Token::Atom(a) => parse_atom(a),
        }
    }

    fn parse_list(v: &mut Vec<Token>) -> Rc<Cons> {
        if let Token::CloseParens = v
            .get(0)
            .unwrap_or_else(|| panic!("Unexpected EOF parsing list"))
        {
            v.remove(0);
            Cons::nil()
        } else {
            Cons::new_pair(parse_next_token(v), parse_list(v))
        }
    }

    fn parse_atom(a: AtomType) -> Rc<Cons> {
        match a {
            // TODO change this to something more meaningful
            AtomType::Bool(b) => Cons::new_val(&b.to_string()),
            AtomType::Char(c) => Cons::new_val(&c.to_string()),
            AtomType::Number(n) => Cons::new_val(&n.to_string()),
            AtomType::String(s) => Cons::new_val(&s),
        }
    }

    pub fn parse(s: &str) -> Vec<Rc<Cons>> {
        parse_tokens(&mut tokenize(s))
    }
}
