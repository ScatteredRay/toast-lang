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
enum TokenType {
    OpenParen,
    CloseParen,
    Symbol,
    Number,
    Unknown,
    End
}


pub struct Token {
    token_type: TokenType,
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
                Token { token_type: TokenType::OpenParen }
        },
        Some(')') => {
            state.inc();
            Token { token_type: TokenType::CloseParen }
        },
        Some('0'..='9') => {
            while match state.peek() {
                Some('0'..='9') => true,
                _ => false
            }  {
                state.inc();
            }
            Token { token_type: TokenType::Number }
        },
        None => Token { token_type: TokenType::End },
        _ => Token { token_type: TokenType::Unknown }
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
                &mut sparser::State { s : "(", offset : 0}).token_type,
                   sparser::TokenType::OpenParen);

        assert_eq!(sparser::parse_token(
                &mut sparser::State { s : ")", offset : 0}).token_type,
                   sparser::TokenType::CloseParen);

        assert_eq!(sparser::parse_token(
                &mut sparser::State { s : "", offset : 0}).token_type,
                   sparser::TokenType::End);

        assert_eq!(sparser::parse_token(
                &mut sparser::State { s : " (", offset : 0}).token_type,
                   sparser::TokenType::OpenParen);

        assert_eq!(sparser::parse_token(
                &mut sparser::State { s : "0", offset : 0}).token_type,
                   sparser::TokenType::Number);

        assert_eq!(sparser::parse_token(
                &mut sparser::State { s : "1", offset : 0}).token_type,
                   sparser::TokenType::Number);

        assert_eq!(sparser::parse_token(
                &mut sparser::State { s : "9", offset : 0}).token_type,
                   sparser::TokenType::Number);

        let mut state = sparser::State {
            s : " ( 987 )( ",
            offset : 0
        };
        assert_eq!(sparser::parse_token(&mut state).token_type, sparser::TokenType::OpenParen);
        assert_eq!(sparser::parse_token(&mut state).token_type, sparser::TokenType::Number);
        assert_eq!(sparser::parse_token(&mut state).token_type, sparser::TokenType::CloseParen);
        assert_eq!(sparser::parse_token(&mut state).token_type, sparser::TokenType::OpenParen);
        assert_eq!(sparser::parse_token(&mut state).token_type, sparser::TokenType::End);
    }

    #[test]
    fn basic_parse() {
        let result = sparser::parse_string("(+ 2 2)");
        assert_eq!(result, 4);
    }
}