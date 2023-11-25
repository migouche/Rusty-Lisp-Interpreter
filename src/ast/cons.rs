pub mod cons {
    use crate::scheme_funcs::scheme_funcs;
    use std::fmt::{Debug, Formatter};
    use std::rc::Rc;

    pub enum Cons<'a> {
        Pair(Rc<Cons<'a>>, Rc<Cons<'a>>),
        Val(&'a str),
        Nil,
    }

    impl<'a> Cons<'a> {
        pub fn nil() -> Rc<Cons<'a>> {
            Rc::new(Cons::Nil)
        }

        pub fn car<'b>(&self) -> Result<Rc<Cons<'a>>, &'b str> {
            if let Cons::Pair(c1, _) = self {
                Ok(c1.clone())
            } else {
                Err("Can't car() a Value or Nil")
            }
        }

        pub fn cdr<'b>(&self) -> Result<Rc<Cons<'a>>, &'b str> {
            if let Cons::Pair(_, c2) = self {
                Ok(c2.clone())
            } else {
                Err("Can't car() a Value or Nil")
            }
        }

        pub fn val<'b>(&self) -> Result<&'a str, &'b str> {
            if let Cons::Val(s) = self {
                Ok(s)
            } else {
                Err("Can't get value out of non-Value Cons")
            }
        }

        pub fn is_nil(&self) -> bool {
            if let Cons::Nil = self
            // cant find better way to do this without deriving equality traits
            {
                true
            } else {
                false
            }
        }

        pub fn is_pair(&self) -> Option<(Rc<Cons<'a>>, Rc<Cons<'a>>)> {
            if let Cons::Pair(ca, cd) = self {
                Some((ca.clone(), cd.clone()))
            } else {
                None
            }
        }

        pub fn eval<'b>(&self) -> Result<Rc<Cons<'a>>, &'b str> {
            match self {
                Cons::Nil => Ok(Cons::nil()),
                Cons::Val(s) => Ok(Cons::new_val(s)),
                Cons::Pair(ca, cd) => {
                    // we suppose we eval on the go as we parse inner expressions, so everything found
                    // should already have been evaluated

                    let procedure = ca.val().unwrap_or_else(|e| {
                        panic!("Operator is not a PROCEDURE: {}", e);
                    });
                    if let Some(r) = scheme_funcs::scheme_funcs::FUNCTIONS.get(procedure) {
                        Ok(r(cd.clone())
                            .unwrap_or_else(|e| panic!("Error while evaling {}: {}", procedure, e)))
                    } else {
                        //println!("err");
                        Err("Unbound variable")
                    }
                    //Ok(Cons::nil())
                }
            }
        }

        //fn call_func(name: &'a str, args: Rc<Cons<'a>>)

        pub fn new_val(s: &'a str) -> Rc<Cons<'a>> {
            Rc::new(Cons::Val(s))
        }

        pub fn new_pair(c1: Rc<Cons<'a>>, c2: Rc<Cons<'a>>) -> Rc<Cons<'a>> {
            Rc::new(Cons::Pair(c1, c2))
        }
        pub fn new_list(vec: &Vec<&'a str>) -> Rc<Cons<'a>> {
            Cons::new_list_i(vec, 0)
        }

        fn new_list_i(vec: &Vec<&'a str>, i: usize) -> Rc<Cons<'a>> {
            if i < vec.len() {
                let c1 = Cons::new_val(
                    vec.get(i)
                        .unwrap_or_else(|| panic!("shouldn't get this error")),
                );
                let c2 = Cons::new_list_i(vec, i + 1);
                Cons::new_pair(c1, c2)
            } else {
                Cons::nil()
            }
        }
    }

    impl<'a> Debug for Cons<'a> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                Cons::Nil => write!(f, "NIL"),
                Cons::Val(s) => write!(f, "{}", s),
                Cons::Pair(ca, cd) => write!(f, "({:?} {:?})", ca, cd),
            }
        }
    }
}
