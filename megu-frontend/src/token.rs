use logos::{Lexer, Logos};

pub fn lex(source: &[u8]) -> Result<Vec<Token<'_>>, ()> {
    // check leading trivia
    let mut leading_trivia_end = 0;
    for b in source.iter() {
        match b {
            b' ' | b'\t' | b'\n' | b'\r' => {
                leading_trivia_end += 1;
            }
            _ => break,
        }
    }

    let leading_trivia = &source[..leading_trivia_end];

    let extra = LexerExtra {
        leading_trivia,
    };
    
    let mut lexer = Token::lexer_with_extras(&source[leading_trivia_end..], extra);
    let tokens: Result<Vec<_>, _> = lexer.by_ref().collect();
    tokens.map(|mut token| {
        let rest = lexer.extras.leading_trivia;
        token.push(Token::Rest(rest));
        token
    })
}

#[derive(Logos, Debug, PartialEq, Eq, Clone)]
#[logos(extras = LexerExtra<'s>)] // depends on logos undocumented behavior
#[logos(utf8 = false)]
pub enum Token<'source> {
    // Keywords
    #[token("def", token_inner)]
    Def(TokenInner<'source>),

    // Literals
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", token_inner)]
    Ident(TokenInner<'source>),
    #[regex(r"@[a-zA-Z_][a-zA-Z0-9_]*", token_inner)]
    BuiltinIdent(TokenInner<'source>),
    #[regex(r#""([^"\\]|\\.)*""#, token_inner)]
    String(TokenInner<'source>),

    // signs
    #[token("(", token_inner)]
    LParen(TokenInner<'source>),
    #[token(")", token_inner)]
    RParen(TokenInner<'source>),
    #[token("[", token_inner)]
    LBracket(TokenInner<'source>),
    #[token("]", token_inner)]
    RBracket(TokenInner<'source>),
    #[token(",", token_inner)]
    Comma(TokenInner<'source>),
    #[token("\\", token_inner)]
    Backslash(TokenInner<'source>),

    // for rest leading trivia
    Rest(&'source [u8]),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenInner<'source> {
    leading_trivia: &'source [u8],
    token: &'source [u8],
    trailing_trivia: &'source [u8],
}

impl<'source> TokenInner<'source> {
    pub fn leading_trivia(&self) -> &'source [u8] {
        self.leading_trivia
    }

    pub fn token(&self) -> &'source [u8] {
        self.token
    }

    pub fn trailing_trivia(&self) -> &'source [u8] {
        self.trailing_trivia
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Default)]
pub struct LexerExtra<'source> {
    leading_trivia: &'source [u8],
}

fn token_inner<'source>(lexer: &mut Lexer<'source, Token<'source>>) -> TokenInner<'source> {
    let result = lexer.slice();
    let remain = lexer.remainder();
    let mut trailing_trivia_end = 0;

    // check leading trivia
    for b in remain.iter() {
        match b {
            b' ' | b'\t' | b'\r' => {
                trailing_trivia_end += 1;
                lexer.bump(1);
            }
            _ => break,
        }
    }

    let mut leading_trivia_end = trailing_trivia_end;

    // check \n
    if let Some(b'\n') = remain.get(leading_trivia_end) {
        leading_trivia_end += 1;
        lexer.bump(1);
    }

    // check leading trivia
    for b in remain[leading_trivia_end..].iter() {
        match b {
            b' ' | b'\t' | b'\n' | b'\r' => {
                leading_trivia_end += 1;
                lexer.bump(1);
            }
            _ => break,
        }
    }

    let this_leading = lexer.extras.leading_trivia;
    lexer.extras.leading_trivia = &remain[trailing_trivia_end..leading_trivia_end];

    TokenInner {
        leading_trivia: this_leading,
        token: result,
        trailing_trivia: &remain[..trailing_trivia_end],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_inner() {
        let source = b"  def   \n   def   \n   ";
        let tokens = lex(source).unwrap();
        assert_eq!(tokens.len(), 3);
        match &tokens[0] {
            Token::Def(inner) => {
                assert_eq!(inner.leading_trivia(), b"  ");
                assert_eq!(inner.token(), b"def");
                assert_eq!(inner.trailing_trivia(), b"   ");
            }
            _ => panic!("Expected Def token"),
        }
        match &tokens[1] {
            Token::Def(inner) => {
                assert_eq!(inner.leading_trivia(), b"\n   ");
                assert_eq!(inner.token(), b"def");
                assert_eq!(inner.trailing_trivia(), b"   ");
            }
            _ => panic!("Expected Def token"),
        }
        match &tokens[2] {
            Token::Rest(inner) => {
                assert_eq!(inner, b"\n   ");
            }
            _ => panic!("Expected Rest token"),
        }
    }
}