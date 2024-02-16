
use crate::parsing::ast::ValType;
use phf::phf_map;
use std::rc::Rc;

type Func<'a> = fn(&Vec<Rc<ValType>>) -> Result<Rc<ValType>, &'a str>;

pub const FUNCTIONS: phf::Map<&'static str, Func> = phf_map!(
    "display" => display as Func,
    "cons" => cons as Func,
    "car" => car as Func,
    "cdr" => cdr as Func,
);

fn display<'a>(d: &Vec<Rc<ValType>>) -> Result<Rc<ValType>, &'a str> {
    // for now only strings :)
    //println!("display called");
    if d.len() == 1 {
        if let ValType::String(s) = d.get(0).unwrap().as_ref() {
            print!("{}", s);
            Ok(ValType::nil())
        } else {
            Err("Can only display strings for now!")
        }
    } else {
        Err("Wrong number of arguments, expected 1")
    }
}

fn cons<'a>(d: &Vec<Rc<ValType>>) -> Result<Rc<ValType>, &'a str> {
    if d.len() == 2 {
        Ok(ValType::cons(
            d.get(0).unwrap().eval(),
            d.get(1).unwrap().eval(),
        ))
    } else {
        Err("Expected 2 arguments")
    }
}

fn car<'a>(d: &Vec<Rc<ValType>>) -> Result<Rc<ValType>, &'a str> {
    if d.len() == 1 {
        if let ValType::Cons(c) = d.get(0).unwrap().as_ref() {
            Ok(c.car())
        } else {
            Err("expected a Cons")
        }
    } else {
        Err("Expected 1 argument")
    }
}

fn cdr<'a>(d: &Vec<Rc<ValType>>) -> Result<Rc<ValType>, &'a str> {
    if d.len() == 1 {
        if let ValType::Cons(c) = d.get(0).unwrap().as_ref() {
            Ok(c.cdr())
        } else {
            Err("expected a Cons")
        }
    } else {
        Err("Expected 1 argument")
    }
}
