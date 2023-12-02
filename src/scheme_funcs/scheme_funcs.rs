pub mod scheme_funcs {
    use crate::ast::cons::cons::ValType;
    use phf::phf_map;
    use std::rc::Rc;

    type Func<'a> = fn(Rc<ValType>) -> Result<Rc<ValType>, &'a str>;

    pub const  FUNCTIONS: phf::Map<&'static str, Func> = phf_map!(
        "display" => display as Func,
    );

    fn display<'a>(c: Rc<ValType>) -> Result<Rc<ValType>, &'a str> {
        println!("TODO :)");
        Ok(ValType::nil())
    }
}
