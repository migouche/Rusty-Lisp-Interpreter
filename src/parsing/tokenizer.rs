pub mod tokenizer {
    use std::rc::Rc;

    use crate::ast::cons::cons::Cons;
    pub struct Token<'a>(pub &'a str);

    pub struct StringStream(String);

    impl StringStream {
        pub fn new(s: &str) -> StringStream {
            StringStream(s.to_string()) // we reverse for easier popping
        }

        pub fn get_char(&mut self) -> Option<char> {
            if self.is_eof() {
                None
            } else {
                Some(self.0.remove(0))
            }
        }

        pub fn peek_char(&self) -> Option<char> {
            self.0.chars().nth(0)
        }

        pub fn is_eof(&self) -> bool {
            self.0.is_empty()
        }
    }

    const SPLIT_CHARS: &[char] = &[' ', '\n', '\t', '\r', '(', ')', '\''];

    pub fn tokenize(s: &str) -> Vec<Rc<Cons>> {
        let mut v: Vec<Rc<Cons>> = Vec::new();
        let mut stream = StringStream::new(s);
        v
    }

    pub fn parse_token(s: &mut StringStream) -> Rc<Cons> //
    {
        while let Some(c) = s.peek_char() {
            if SPLIT_CHARS.contains(&c) {
                s.get_char();
            } else {
                break;
            }
        }

        let mut token = String::new();

        while let Some(c) = s.peek_char() {
            if SPLIT_CHARS.contains(&c) {
                break;
            } else {
                token.push(s.get_char().unwrap());
            }
        }

        if (s.is_eof()) {
            panic!("Unexpected EOF");
        }

        Cons::new_val(&token)
    }

    pub fn tokenize_list(s: &mut StringStream) -> Rc<Cons> {}

    pub fn tokenize_expression(s: &mut StringStream) -> Rc<Cons> {
        while s.peek_char().unwrap_or_default() != ')'
        // default definitely != ')'
        {}
        Cons::nil()
    }
}
