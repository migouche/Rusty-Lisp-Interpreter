pub mod cons {
    use std::fmt::{Debug, Display, Formatter};
    use std::rc::Rc;

    use crate::parsing::tokenizer::tokenizer::AtomType;
    use crate::scheme_funcs;

    #[derive(PartialEq, Debug)]
    pub struct Cons(Rc<ValType>, Rc<ValType>);


    #[derive(PartialEq, Clone, Copy)]
    pub enum Number {
        Int(i64),
        Real(f64),
        Rational(i64, i64),
        Complex(f64, f64),
    }

    #[derive(PartialEq)]
    pub enum ValType {
        Nil,
        Bool(bool),
        Number(Number),
        Char(char),
        String(String),
        Cons(Cons),
        Procedure(String),
        Symbol(Rc<ValType>),
    }

    impl Cons {
        pub fn car(&self) -> Rc<ValType> {
            self.0.clone()
        }

        pub fn cdr(&self) -> Rc<ValType> {
            self.1.clone()
        }
    }

    impl ValType {
        pub fn nil() -> Rc<Self> {
            Rc::new(ValType::Nil)
        }

        pub fn bool(b: bool) -> Rc<Self> {
            Rc::new(ValType::Bool(b))
        }

        pub fn number(n: Number) -> Rc<Self> {
            Rc::new(ValType::Number(n))
        }

        pub fn char(c: char) -> Rc<Self> {
            Rc::new(ValType::Char(c))
        }

        pub fn string(s: &str) -> Rc<Self> {
            Rc::new(ValType::String(s.to_string()))
        }

        pub fn cons(car: Rc<Self>, cdr: Rc<Self>) -> Rc<Self> {
            Rc::new(ValType::Cons(Cons(car, cdr)))
        }

        pub fn new_procedure(s: &str) -> Rc<Self> {
            Rc::new(ValType::Procedure(s.to_string()))
        }

        pub fn new_symbol(s: Rc<Self>) -> Rc<Self> {
            Rc::new(ValType::Symbol(s))
        }

        pub fn from_atom(a: &AtomType) -> Rc<Self> {
            match a {
                AtomType::Bool(b) => Self::bool(*b),
                AtomType::Char(c) => Self::char(*c),
                AtomType::Number(n) => Self::number(Number::Int(*n as i64)),
                AtomType::String(s) => Self::string(s),
            }
        }

        pub fn is_nil(&self) -> bool {
            if let ValType::Nil = self {
                true
            } else {
                false
            }
        }

        pub fn get_bool(&self) -> Option<bool> {
            if let ValType::Bool(b) = self {
                Some(*b)
            } else {
                None
            }
        }

        pub fn get_number(&self) -> Option<&Number> {
            if let ValType::Number(n) = self {
                Some(n)
            } else {
                None
            }
        }

        pub fn get_char(&self) -> Option<char> {
            if let ValType::Char(c) = self {
                Some(*c)
            } else {
                None
            }
        }

        pub fn get_cons(&self) -> Option<&Cons> {
            if let ValType::Cons(c) = self {
                Some(c)
            } else {
                None
            }
        }

        pub fn get_string(&self) -> Option<&str> {
            if let ValType::String(s) = self {
                Some(&s)
            } else {
                None
            }
        }

        pub fn get_podedure_name(&self) -> Option<&str>
        {
            if let ValType::Procedure(p) = self
            {
                Some(&p)
            }
            else {
                None
            }
        }

        pub fn eval(&self) -> Rc<Self> {
            match self {
                ValType::Bool(b) => ValType::bool(*b),
                ValType::Char(c) => ValType::char(*c),
                ValType::Nil => ValType::nil(),
                ValType::String(s) => ValType::string(&s),
                ValType::Number(n) => ValType::number(*n),
                ValType::Symbol(v) => v.clone(),
                ValType::Cons(c) => {
                    let p = c.car().get_podedure_name().unwrap_or_else(|| panic!("Cant eval procedure{:?}", c)).to_string();
                    let mut v = Vec::<Rc<ValType>>::new();
                    let mut temp = c.cdr();
                    while let ValType::Cons(cd) = temp.as_ref()
                    {
                        println!("iteration");
                        v.push(cd.car().eval());
                        temp = cd.cdr();
                    }
                    scheme_funcs::scheme_funcs::scheme_funcs::FUNCTIONS.get(&p).unwrap_or_else(|| panic!("Procedure not bound"))(&v).unwrap_or_else(|e| panic!("error evaluating procedure: {}", e))
                }
                ValType::Procedure(_) => panic!("Can't eval a procedure name!")
                
            }
        }
    }

    impl Debug for ValType {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Nil => write!(f, "nil"),
                Self::Bool(b) => write!(f, "{b}"),
                Self::Number(n) => write!(f, "{n}"),
                Self::Char(c) => write!(f, "{c}"),
                Self::Cons(c) => write!(f, "({:?} {:?})", c.car(), c.cdr()),
                Self::Procedure(s) => write!(f, "{s}"),
                Self::String(s) => write!(f, "{s:?}"),
                Self::Symbol(s) => write!(f, "{s:?}"),
            }
        }
    }

    impl Display for Number {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Int(z) => write!(f, "{z}"),
                Self::Real(r) => write!(f, "{r}"),
                Self::Rational(q1, q2) => write!(f, "{q1}/{q2}"),
                Self::Complex(r, i) => write!(f, "{r}+{i}i"),
            }
        }
    }
}
