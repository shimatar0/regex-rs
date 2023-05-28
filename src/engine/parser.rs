use core::fmt;
use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    InvalidEscape(usize, char),
    NoPrev(usize),
    NoRightParen,
    Empty,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidEscape(pos, ch) => {
                write!(
                    f,
                    "ParseError: Invalid escape sequence at position {}: {}",
                    pos, ch
                )
            }
            ParseError::NoPrev(pos) => {
                write!(f, "ParseError: No previous character at position {}", pos)
            }
            ParseError::NoRightParen => write!(f, "ParseError: No right parenthesis"),
            ParseError::Empty => write!(f, "ParseError: Empty expression"),
        }
    }
}

impl Error for ParseError {}

#[derive(Debug, PartialEq, Eq)]
pub enum AST {
    Char(char),
    Plus(Box<AST>),
    Star(Box<AST>),
    Questiong(Box<AST>),
    Or(Box<AST>, Box<AST>),
    Seq(Vec<AST>),
}

fn parse_escape(pos: usize, c: char) -> Result<AST, ParseError> {
    match c {
        '\\' | '(' | ')' | '|' | '+' | '*' | '_' => Ok(AST::Char(c)),
        _ => {
            let err = ParseError::InvalidEscape(pos, c);
            Err(err)
        }
    }
}

mod test {
    #[test]
    fn parse_escape() {
        use super::parse_escape;
        use super::ParseError;
        use super::AST;

        assert_eq!(parse_escape(0, '\\'), Ok(AST::Char('\\')));
        assert_eq!(parse_escape(0, '('), Ok(AST::Char('(')));
        assert_eq!(parse_escape(0, ')'), Ok(AST::Char(')')));
        assert_eq!(parse_escape(0, '|'), Ok(AST::Char('|')));
        assert_eq!(parse_escape(0, '+'), Ok(AST::Char('+')));
        assert_eq!(parse_escape(0, '*'), Ok(AST::Char('*')));
        assert_eq!(parse_escape(0, '_'), Ok(AST::Char('_')));
        assert_eq!(parse_escape(0, 'a'), Err(ParseError::InvalidEscape(0, 'a')));
    }
}
