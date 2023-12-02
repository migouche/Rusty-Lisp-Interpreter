pub mod cons {
    use crate::scheme_funcs::scheme_funcs;
    use std::backtrace;
    use std::fmt::{Debug, Formatter, Display};
    use std::rc::Rc;

    struct Cons(Rc<ValType>, Rc<ValType>);

    pub enum Number
    {
        Int(i64),
        Real(f64),
        Rational(i64, i64),
        Complex(f64, f64),
    }

    pub enum ValType
    {
        Nil,
        Bool(bool),
        Number(Number),
        Char(char),
        Cons(Cons),
        Procedure(String),
        Symbol(Rc<ValType>)
    }

    impl Cons {
        pub fn car(&self) ->Rc<ValType>
        {
            self.0
        }

        pub fn cdr(&self) -> Rc<ValType>
        {
            self.1
        }
    }

    impl ValType
    {
        pub fn nil() -> Rc<Self>
        {
            Rc::new(ValType::Nil)
        }


        pub fn is_nil(&self) -> bool
        {
            if let ValType::Nil = self
            {
                true
            }
            else {
                false
            }
        }

        pub fn get_bool(&self) -> Option<bool>
        {
            if let ValType::Bool(b) = self
            {
                Some(*b)
            }
            else {
                None
            }
        }

        pub fn get_number(&self) -> Option<&Number>
        {
            if let ValType::Number(n) = self
            {
                Some(n)
            }
            else {
                None
            }
        }

        pub fn get_char(&self) -> Option<char>
        {
            if let ValType::Char(c) = self
            {
                Some(*c)
            }
            else {
                None
            }
        }

        pub fn get_cons(&self) -> Option<&Cons>
        {
            if let ValType::Cons(c) = self
            {
                Some(c)
            }
            else {
                None
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
                Self::Symbol(s) => write!(f, "{s:?}")
            }
        }
    }

    impl Display for Number
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Int(z) => write!(f, "{z}"),
                Self::Real(r) => write!(f, "{r}"),
                Self::Rational(q1, q2) => write!(f, "{q1}/{q2}"),
                Self::Complex(r, i) =>  write!(f, "{r}+{i}i"),
            }
        }
    }
}
