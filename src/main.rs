mod ast;

use ast::cons::cons::Cons;

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

    let mut c3 = Cons::new_list(&v);
    let mut t = *c3;
    while let Cons::Pair(car, cdr) = t
    {
        println!("{}", car.val().unwrap());
        t = *cdr;
    }
}
