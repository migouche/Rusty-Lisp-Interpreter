pub mod scheme_funcs {
    use crate::ast::cons::cons::Cons;
    use phf::phf_map;
    use std::rc::Rc;

    type Func<'a> = fn(Rc<Cons>) -> Result<Rc<Cons>, &'a str>;

    pub const FUNCTIONS: phf::Map<&'static str, Func> = phf_map!(
        "display" => display as Func,
    );

    fn display<'a>(c: Rc<Cons>) -> Result<Rc<Cons>, &'a str> {
        if let Cons::Pair(ca, cd) = c.as_ref() {
            assert!(cd.is_nil());
            if let Result::Ok(s) = ca.val() {
                print!("{}", s);
                Ok(Cons::nil())
            } else {
                Err("FIXME: will implement displaying not evaled lists later")
            }
        } else {
            Err("Wrong number of arguments passed to procedure")
        }
    }
}
