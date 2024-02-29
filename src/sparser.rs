#[derive(Debug)]
pub struct Expr {
}

#[derive(Debug)]
pub struct State<'a> {
    s: &'a str,
    offset: usize,
}

impl State<'_> {
    fn peek(&self) -> Option<char> {
        return self.s.chars().nth(self.offset);
    }

    fn inc(&mut self) {
        self.offset += 1;
    }

    fn next(&mut self) -> Option<char> {
        let c = self.peek();
        self.inc();
        return c;
    }
}

#[derive(Debug, PartialEq)]
enum Token {
    OpenParen,
    CloseParen,
    Symbol,
    Unknown,
    End
}

pub fn consume_whitespace(state: &mut State) {
    while match state.peek() {
            Some(' ' | '\n' | '\r' | '\t') => true,
            _ => false
        }  {
        state.inc();
    }
}

pub fn parse_token(state: &mut State) -> Token {
    consume_whitespace(state);
    let c = state.peek();
    let tok = match c {
        Some('(') => {
                state.inc();
                Token::OpenParen
        },
        Some(')') => {
            state.inc();
            Token::CloseParen
        },
        None => Token::End,
        _ => Token::Unknown
    };
    return tok;
}

pub fn parse_string(s: &str) -> i32 {
    let s = State { s: s, offset: 0 };
    return 4;
}

#[cfg(test)]
mod tests {
    use crate::sparser;

    #[test]
    fn token() {
        assert_eq!(sparser::parse_token(
                &mut sparser::State { s : "(", offset : 0}),
                   sparser::Token::OpenParen);

        assert_eq!(sparser::parse_token(
                &mut sparser::State { s : ")", offset : 0}),
                   sparser::Token::CloseParen);

        assert_eq!(sparser::parse_token(
                &mut sparser::State { s : "", offset : 0}),
                   sparser::Token::End);

        assert_eq!(sparser::parse_token(
                &mut sparser::State { s : " (", offset : 0}),
                   sparser::Token::OpenParen);

        let mut state = sparser::State {
            s : " ( )( ",
            offset : 0
        };
        assert_eq!(sparser::parse_token(&mut state), sparser::Token::OpenParen);
        assert_eq!(sparser::parse_token(&mut state), sparser::Token::CloseParen);
        assert_eq!(sparser::parse_token(&mut state), sparser::Token::OpenParen);
        assert_eq!(sparser::parse_token(&mut state), sparser::Token::End);
    }

    #[test]
    fn basic_parse() {
        let result = sparser::parse_string("(+ 2 2)");
        assert_eq!(result, 4);
    }
}