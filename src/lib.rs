type Tokenized<'i> = Vec<Token<'i>>;

#[derive(Debug, PartialEq)]
pub struct Token<'input> {
    pub raw: &'input str,
}
impl<'i> Token<'i> {
    fn from_raw(raw: &'i str) -> Self {
        Token { raw }
    }
}

impl<'i> FromIterator<Token<'i>> for Vec<&'i str> {
    fn from_iter<I: IntoIterator<Item = Token<'i>>>(iter: I) -> Self {
        let mut c = Vec::new();
        for i in iter {
            c.push(i.raw)
        }
        c
    }
}

pub fn tokenize<'i>(input: &'i str) -> Tokenized<'i> {
    input.split_whitespace().map(Token::from_raw).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tokenize() {
        assert_eq!(
            vec![
                Token::from_raw("Hello"),
                Token::from_raw("this"),
                Token::from_raw("is"),
                Token::from_raw("a"),
                Token::from_raw("sentence"),
                Token::from_raw("for"),
                Token::from_raw("me"),
                Token::from_raw("to"),
                Token::from_raw("say."),
            ],
            tokenize("Hello this is a sentence for me to say.")
        )
    }
}
