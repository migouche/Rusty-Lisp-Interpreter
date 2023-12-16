pub mod parse {
    use std::rc::Rc;

    use crate::{
        ast::cons::cons::ValType,
        parsing::tokenizer::tokenizer::{tokenize, AtomType, Token},
    };

    fn parse_tokens(v: &mut Vec<Token>) -> Vec<Rc<ValType>> {
        let mut tokens: Vec<Rc<ValType>> = Vec::new();

        while !v.is_empty() {
            tokens.push(parse_next_token(v))
        }

        tokens
    }

    fn parse_next_token(v: &mut Vec<Token>) -> Rc<ValType> {
        if v.is_empty() {
            panic!("Unexpected EOF");
        }

        match v.remove(0) {
            Token::OpenParens(_) => parse_list(v),
            Token::CloseParens => ValType::nil(),
            Token::Procedure(s) => ValType::new_procedure(&s),
            Token::Atom(a) => ValType::from_atom(&a),
        }
    }

    fn parse_list(v: &mut Vec<Token>) -> Rc<ValType> {
        if let Token::CloseParens = v
            .get(0)
            .unwrap_or_else(|| panic!("Unexpected EOF parsing list"))
        {
            v.remove(0);
            ValType::nil()
        } else {
            ValType::cons(parse_next_token(v), parse_list(v))
        }
    }

    pub fn parse(s: &str) -> Vec<Rc<ValType>> {
        parse_tokens(&mut tokenize(s))
    }
}
