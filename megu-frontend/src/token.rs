use logos::{Lexer, Logos};

pub fn lex(source: &[u8]) -> Result<Vec<Token<'_>>, ()> {
    Token::lexer(source).collect()
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
    trailing_trivia: Option<&'source [u8]>,
}

fn token_inner<'source>(lexer: &mut Lexer<'source, Token<'source>>) -> TokenInner<'source> {
    let remain = lexer.remainder();
    let mut leading_trivia_end = 0;

    // check leading trivia
    for b in remain.iter() {
        match b {
            b' ' | b'\t' | b'\r' => {
                leading_trivia_end += 1;
                lexer.bump(1);
            }
            _ => break,
        }
    }

    let mut trailing_trivia_end = leading_trivia_end;

    // check \n
    if let Some(b'\n') = remain.get(trailing_trivia_end) {
        trailing_trivia_end += 1;
        lexer.bump(1);
    }

    // check trailing trivia
    for b in remain[trailing_trivia_end..].iter() {
        match b {
            b' ' | b'\t' | b'\n' | b'\r' => {
                trailing_trivia_end += 1;
                lexer.bump(1);
            }
            _ => break,
        }
    }

    let this_trailing = lexer.extras.trailing_trivia.unwrap_or(&[]);
    lexer.extras.trailing_trivia = Some(&remain[leading_trivia_end..trailing_trivia_end]);

    TokenInner {
        leading_trivia: &remain[..leading_trivia_end],
        token: lexer.slice(),
        trailing_trivia: this_trailing,
    }
}
