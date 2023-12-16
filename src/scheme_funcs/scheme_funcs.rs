pub mod scheme_funcs {
    use crate::ast::cons::cons::ValType;
    use phf::phf_map;
    use std::rc::Rc;
    
    type Func<'a> = fn(&Vec<Rc<ValType>>) -> Result<Rc<ValType>, &'a str>;

    pub const FUNCTIONS: phf::Map<&'static str, Func> = phf_map!(
        "display" => display as Func,
    );

    fn display<'a>(d: &Vec<Rc<ValType>>) -> Result<Rc<ValType>, &'a str> { // for now only strings :)
        //println!("display called");
        if d.len() == 1
        {
            if let ValType::String(s) = d.get(0).unwrap().as_ref()
            {
                print!("{}", s);
                Ok(ValType::nil())
            }
            else {
                Err("Can only display strings for now!")
            }
        }
        else {
            Err("Wrong number of arguments, expected 1")
        }
    }


}
