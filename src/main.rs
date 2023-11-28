mod ast;
mod parsing;
mod scheme_funcs;
mod tests;

use ast::cons::cons::Cons;
use parsing::tokenizer::tokenizer;

use crate::parsing::tokenizer::tokenizer::tokenize;

fn main() {
    let mut x = "hey";
    let c = Cons::new_val(x);
    x = "oi";
    println!("{}", x);
    println!(
        "{}",
        c.val()
            .unwrap_or_else(|error| { panic!("Couldn't get value out of c: {}!", error) })
    );

    let c2 = Cons::new_pair(Cons::new_val("ho"), Cons::new_val("hi"));
    println!("{}", c2.car().unwrap().val().unwrap());

    let v = vec!["a", "b", "c", "d"];

    let c3 = Cons::new_list(&v);

    println!("{:?}", c3);

    let v = vec!["display", "hola"];
    let dc = Cons::new_list(&v);
    dc.eval();


    for (char_i, (i, c)) in "abcdefghi".char_indices().enumerate() {
        println!("char_i: {}, i: {}, c: {}", char_i, i, c);
    }

    let sc = tokenize(r#"(car( + "3" 2 #t))"#);
    println!("{:?}", sc);
}
