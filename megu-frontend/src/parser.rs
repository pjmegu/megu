use crate::{
    ast::{
        NodeKind::*,
        event::{Event, EventVec},
    },
    event_vec,
    token::{Token, TokenInner},
};
use chumsky::pratt::*;
use chumsky::prelude::*;

pub fn parse<'a, 'source: 'a>(token: &'a [Token<'source>]) -> Result<EventVec<'source>, ()> {
    let parser = proot();
    let presult = parser.parse(token);
    match presult.into_result() {
        Ok(events) => Ok(events),
        Err(_) => Err(()),
    }
}

enum Separated<T, S> {
    Parser(T),
    Separator(S),
}

// support function
fn separated_by_with_separator<'a, 'source: 'a, P, S, O, O2>(
    parser: P,
    separator: S,
) -> impl Parser<'a, &'a [Token<'source>], Vec<Separated<O, O2>>> + Clone
where
    P: Parser<'a, &'a [Token<'source>], O> + Clone + 'a,
    S: Parser<'a, &'a [Token<'source>], O2> + Clone + 'a,
{
    (parser
        .clone()
        .map(Separated::Parser)
        .then(
            separator
                .clone()
                .map(Separated::Separator::<O, O2>)
                .then(parser.map(Separated::Parser::<O, O2>))
                .repeated()
                .collect::<Vec<_>>(),
        )
        .then(separator.or_not())
        .map(|((first, rest), last)| {
            let mut result = vec![first];
            for (sep, item) in rest {
                result.push(sep);
                result.push(item);
            }
            if let Some(sep) = last {
                result.push(Separated::Separator(sep));
            }
            result
        }))
    .or_not()
    .map(|v| v.unwrap_or_else(|| vec![]))
}

// tokens
macro_rules! make_ptoken {
    ($name:ident, $variant:ident) => {
        fn $name<'a, 'source: 'a>()
        -> impl Parser<'a, &'a [Token<'source>], &'a TokenInner<'source>> + Clone {
            select_ref! { Token::$variant(inner) => inner }
        }
    };
}

make_ptoken!(ptdef, Def);
make_ptoken!(ptident, Ident);
make_ptoken!(ptbident, BuiltinIdent);
make_ptoken!(ptstring, String);
make_ptoken!(ptlparen, LParen);
make_ptoken!(ptrparen, RParen);
make_ptoken!(ptlbracket, LBracket);
make_ptoken!(ptrbracket, RBracket);
make_ptoken!(ptcomma, Comma);
make_ptoken!(ptbackslash, Backslash);

// parser
fn proot<'a, 'source: 'a>() -> impl Parser<'a, &'a [Token<'source>], EventVec<'source>> + Clone {
    pdef()
        .repeated()
        .collect::<EventVec<'_>>()
        .then(select_ref! { Token::Rest(s) => s })
        .map(|(defs, rest)| {
            let rest = if rest.is_empty() {
                event_vec! {}
            } else {
                EventVec::new_with(move |v| {
                    v.push_event(Event::Token(Trivia, str::from_utf8(rest).unwrap()));
                })
            };
            event_vec! {
                Root {
                    vec(defs);
                    vec(rest);
                }
            }
        })
}

fn pdef<'a, 'source: 'a>() -> impl Parser<'a, &'a [Token<'source>], EventVec<'source>> + Clone {
    ptdef()
        .then(ptident())
        .then(pexpr())
        .map(|((def, ident), expr)| {
            event_vec! {
                Def {
                    tokeni(TDef, def);
                    tokeni(TIdent, ident);
                    vec(expr);
                }
            }
        })
}

fn pexpr<'a, 'source: 'a>() -> impl Parser<'a, &'a [Token<'source>], EventVec<'source>> + Clone {
    recursive(|expr| {
        patom(expr.clone()).pratt((
            postfix(50, pcall(expr.clone()), |expr, op, _| {
                event_vec! {
                    Expr {
                        Call {
                            vec(expr);
                            vec(op);
                        }
                    }
                }
            }),
            prefix(25, plambda(expr.clone()), |op, expr, _| {
                event_vec! {
                    Expr {
                        Lambda {
                            vec(op);
                            vec(expr);
                        }
                    }
                }
            }),
        ))
    })
}

fn plambda<'a, 'source: 'a>(
    _expr: impl Parser<'a, &'a [Token<'source>], EventVec<'source>> + Clone,
) -> impl Parser<'a, &'a [Token<'source>], EventVec<'source>> + Clone {
    ptbackslash().then(ptbackslash()).map(|(first, second)| {
        event_vec! {
            tokeni(TBackslash, first);
            tokeni(TBackslash, second);
        }
    })
}

fn pcall<'a, 'source: 'a>(
    expr: impl Parser<'a, &'a [Token<'source>], EventVec<'source>> + Clone + 'a,
) -> impl Parser<'a, &'a [Token<'source>], EventVec<'source>> + Clone {
    ptlparen()
        .then(separated_by_with_separator(
            expr.map(|e| {
                event_vec! {
                    CallArg {
                        vec(e);
                    }
                }
            }),
            ptcomma(),
        ))
        .then(ptrparen())
        .map(|((lparen, exprs), rparen)| {
            let exprs = exprs
                .into_iter()
                .map(|expr| match expr {
                    Separated::Parser(arg) => arg,
                    Separated::Separator(comma) => event_vec! {
                        tokeni(TComma, comma);
                    },
                })
                .collect::<EventVec<'_>>();
            event_vec! {
                tokeni(TLParen, lparen);
                vec(exprs);
                tokeni(TRParen, rparen);
            }
        })
}

fn patom<'a, 'source: 'a>(
    expr: impl Parser<'a, &'a [Token<'source>], EventVec<'source>> + Clone + 'a,
) -> impl Parser<'a, &'a [Token<'source>], EventVec<'source>> + Clone {
    choice((
        select_ref! {
            Token::Ident(inner) => {
                event_vec! {
                    Expr {
                        Ident {
                            tokeni(TIdent, inner);
                        }
                    }
                }
            },
            Token::BuiltinIdent(inner) => {
                event_vec! {
                    Expr {
                        BuiltinIdent {
                            tokeni(TBuiltinIdent, inner);
                        }
                    }
                }
            },
            Token::String(inner) => {
                event_vec! {
                    Expr {
                        String {
                            tokeni(TString, inner);
                        }
                    }
                }
            }
        },
        pblock(expr),
    ))
}

fn pblock<'a, 'source: 'a>(
    expr: impl Parser<'a, &'a [Token<'source>], EventVec<'source>> + Clone + 'a,
) -> impl Parser<'a, &'a [Token<'source>], EventVec<'source>> + Clone {
    ptlbracket()
        .then(
            expr.map(|e| {
                event_vec! {
                    StmtExpr {
                        vec(e);
                    }
                }
            })
            .repeated()
            .collect(),
        )
        .then(ptrbracket())
        .map(|((lbracket, exprs), rbracket)| {
            event_vec! {
                Block {
                    tokeni(TLBracket, lbracket);
                    vec(exprs);
                    tokeni(TRBracket, rbracket);
                }
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{event_vec, token::lex};

    macro_rules! make_parse {
        ($func:expr, $token:expr) => {{
            let parser = $func.lazy();
            let presult = parser.parse($token);
            match presult.into_result() {
                Ok(events) => Ok(events),
                Err(_) => Err(()),
            }
        }};
    }

    #[test]
    fn test_parse() {
        let source = b"def foo \\\\ foo()";
        let tokens = lex(source).unwrap();
        let events: Vec<_> = parse(&tokens).unwrap().into_iter().collect();
        let expected: Vec<_> = event_vec! {
            Root {
                Def {
                    token(TDef, "", "def", " ");
                    token(TIdent, "", "foo", " ");
                    Expr {
                        Lambda {
                            token(TBackslash, "", "\\", "");
                            token(TBackslash, "", "\\", " ");
                            Expr {
                                Call {
                                    Expr {
                                        Ident {
                                            token(TIdent, "", "foo", "");
                                        }
                                    }
                                    token(TLParen, "", "(", "");
                                    token(TRParen, "", ")", "");
                                }
                            }
                        }
                    }
                }
            }
        }
        .into_iter()
        .collect();

        assert_eq!(events, expected);
    }

    #[test]
    fn test_block_parse() {
        let source = b"[ foo() bar() ]";
        let tokens = lex(source).unwrap();
        let events: Vec<_> = make_parse!(pblock(pexpr()), &tokens)
            .unwrap()
            .into_iter()
            .collect();
        let expected: Vec<_> = event_vec! {
            Block {
                token(TLBracket, "", "[", " ");
                StmtExpr {
                    Expr {
                        Call {
                            Expr {
                                Ident {
                                    token(TIdent, "", "foo", "");
                                }
                            }
                            token(TLParen, "", "(", "");
                            token(TRParen, "", ")", " ");
                        }
                    }
                }
                StmtExpr {
                    Expr {
                        Call {
                            Expr {
                                Ident {
                                    token(TIdent, "", "bar", "");
                                }
                            }
                            token(TLParen, "", "(", "");
                            token(TRParen, "", ")", " ");
                        }
                    }
                }
                token(TRBracket, "", "]", "");
            }
        }
        .into_iter()
        .collect();
        assert_eq!(events, expected);
    }
}
