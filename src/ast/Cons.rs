pub mod cons {
    use std::panic::panic_any;

    pub enum Cons<'a> {
        Pair(Box<Cons<'a>>, Box<Cons<'a>>),
        Val(&'a str),
        Nil,
    }

    impl<'a> Cons<'a> {
        pub fn nil() -> Box<Cons<'a>> {
            Box::new(Cons::Nil)
        }

        pub fn car<'b>(self) -> Result<Box<Cons<'a>>, &'b str> {
            if let Cons::Pair(c1, _) = self {
                Ok(c1)
            } else {
                Err("Can't car() a Value or Nil")
            }
        }

        pub fn cdr<'b>(self) -> Result<Box<Cons<'a>>, &'b str> {
            if let Cons::Pair(_, c2) = self {
                Ok(c2)
            } else {
                Err("Can't car() a Value or Nil")
            }
        }

        pub fn val<'b>(self) -> Result<&'a str, &'b str> {
            if let Cons::Val(s) = self {
                Ok(s)
            } else {
                Err("Can't get value out of non-Value Cons")
            }
        }
        fn eval<'b>(self) -> Result<Box<Cons<'a>>, &'b str> {
            Ok(Cons::nil())
        }

        pub fn new_val(s: &'a str) -> Box<Cons<'a>> {
            Box::new(Cons::Val(s))
        }

        pub fn new_pair(c1: Box<Cons<'a>>, c2: Box<Cons<'a>>) -> Box<Cons<'a>> {
            Box::new(Cons::Pair(c1, c2))
        }
        pub fn new_list(vec: &Vec<&'a str>) -> Box<Cons<'a>> {
            Cons::new_list_i(vec, 0)
        }

        fn new_list_i(vec: &Vec<&'a str>, i: usize) -> Box<Cons<'a>> {
            if i < vec.len() {
                let c1 = Cons::new_val(
                    vec.get(i)
                        .unwrap_or_else(|| panic!("shouldn't get this error")));
                let c2 = Cons::new_list_i(vec, i + 1);
                Cons::new_pair(c1, c2)
            } else {
                Cons::nil()
            }
        }
    }
}
