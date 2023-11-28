pub mod tokenizer {
    use std::rc::Rc;

    use phf::phf_map;

    use crate::ast::cons::cons::Cons;

    #[derive(Debug, Clone)]
    enum AtomType
    {
        Number(i32),
        String(String),
        Bool(bool),
        Char(char)
    }

    #[derive(Debug, Clone)]
    pub enum Token
    {
        OpenParens(bool), // eval/not eval
        CloseParens,
        Atom(AtomType),
        Procedure(String)
    }

    pub struct StringStream(String);

    const SPLIT_CHARS: &[char] = &[' ', '\n', '\t', '\r'];
    const UNIQUE_CHARS: &[char] = &['\'', '(', ')'];

    const UNIQUE_PATTERNS: phf::Map<&'static str, (Token, usize)> = phf_map!(
        "(" => (Token::OpenParens(true), 1),
        "'(" => (Token::OpenParens(false), 2),
        ")" => (Token::CloseParens, 1),
    );

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

        pub fn consume_whitespaces(&mut self)
        {
            if SPLIT_CHARS.contains(&self.peek_char().unwrap())
            {
                self.get_char();
                self.consume_whitespaces();
            }
        }



        fn get_next_token(&mut self) -> Token
        {            
            self.consume_whitespaces();

            for (key, (token, size)) in UNIQUE_PATTERNS.entries()
            {
                if self.0.starts_with(key)
                {
                    for _ in 0..*size
                    {
                        self.get_char();

                    }
                    return token.clone();
                }
            }

            // we suppose we now have an expression

            let mut s = String::new();
            while !SPLIT_CHARS.contains(&self.peek_char().unwrap()) && 
                    !UNIQUE_CHARS.contains(&self.peek_char().unwrap())
            {
                s.push(self.get_char().unwrap());
            }

            tokenize_expression(&s)

        }




    }

    fn tokenize_expression(s: &str) -> Token
    {
        use Token::*;
        use AtomType::*;
        if(s.starts_with('"'))
        {
            if(!s.ends_with('"'))
            {
                panic!("expected  \" to close string: {}", s);
            }
            Atom(AtomType::String(s[1..s.len() - 1].to_string()))

        }
        else if let Ok(i) = s.parse::<i32>() {
            Atom(Number(i)) // TODO: add all possible numbers
        }
        else if s.starts_with('#') {
            // special chars
            if s == "#f"
            {
                Atom(Bool(false))
            }
            else if  s == "#t" {
                Atom(Bool(true))
            }
            else {
                panic!("Couldn't parse: {}", s)
            }

        }
        else {
            Procedure(s.to_string())
        }
    }


    pub fn tokenize(s: &str) -> Vec<Token>
    {
        let mut stream = StringStream::new(s);

        let mut v: Vec<Token> = Vec::new();
        while !stream.is_eof()
        {
            v.push(stream.get_next_token());
        }
        v
    }

    fn parse_tokens(v: &Vec<Token>) -> Rc<Cons>
    {
        Cons::nil()
    }

    pub fn parse(s: &str) -> Rc<Cons>
    {
        parse_tokens(&tokenize(s))
    }


}
