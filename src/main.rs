mod ast;
mod parsing;
mod scheme_funcs;
mod tests;

use ast::cons::cons::ValType;
use parsing::tokenizer::tokenizer;

use crate::parsing::{parser::parse::parse, tokenizer::tokenizer::tokenize};

fn main() {
    let mut x = "hey";
    let c = ValType::string(x);
    x = "oi";
    println!("{}", x);
    println!(
        "{}",
        c.get_string().unwrap_or_else(|| {
            panic!("Couldn't get value out of c:");
        })
    );

    let c2 = ValType::cons(ValType::string("ho"), ValType::string("hi"));
    println!("{}", c2.get_cons().unwrap().car().get_string().unwrap());

    let v = vec!["a", "b", "c", "d"];

    let eval_c = ValType::cons(
        ValType::new_procedure("display"),
        ValType::cons(ValType::string("display"), ValType::nil()),
    );
    eval_c.eval();

    for (char_i, (i, c)) in "abcdefghi".char_indices().enumerate() {
        println!("char_i: {}, i: {}, c: {}", char_i, i, c);
    }

    let sc = tokenize(r#"(car( + "3" 2 #t))"#);
    println!("{:?}", sc);
    let scheme = parse(r#"(car( + "3" 2 #t))"#);
    println!("{:?}", scheme);
}
