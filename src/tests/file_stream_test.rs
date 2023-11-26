#[cfg(test)]
mod file_stream_test {
    use crate::parsing::tokenizer::tokenizer::StringStream;
    #[test]
    fn test_filestream() {
        let mut s = StringStream::new("123");
        assert_eq!(s.peek_char(), Some('1'));
        assert_eq!(s.get_char(), Some('1'));
        assert_eq!(s.peek_char(), Some('2'));
        assert_eq!(s.get_char(), Some('2'));
        assert_eq!(s.peek_char(), Some('3'));
        assert_eq!(s.get_char(), Some('3'));
        assert_eq!(s.peek_char(), None);
        assert_eq!(s.get_char(), None);
    }
}
