use crate::{
    ast::{
        NodeKind,
        event::{Event, EventVec},
    },
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
    parser
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
        })
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
    pdef().repeated().collect().then_ignore(end())
}

fn pdef<'a, 'source: 'a>() -> impl Parser<'a, &'a [Token<'source>], EventVec<'source>> + Clone {
    ptdef()
        .then(ptident())
        .then(pexpr())
        .map(|((def, ident), expr)| {
            let mut events = EventVec::new();
            events.push_event(Event::Node(NodeKind::Def));
            events.push_event(Event::Token(
                NodeKind::TDef,
                std::str::from_utf8(def.token()).unwrap(),
            ));
            events.push_event(Event::Token(
                NodeKind::TIdent,
                std::str::from_utf8(ident.token()).unwrap(),
            ));
            events.push_vector(expr);
            events.push_event(Event::FinNode);
            events
        })
}

fn pexpr<'a, 'source: 'a>() -> impl Parser<'a, &'a [Token<'source>], EventVec<'source>> + Clone {
    patom()
        .pratt((
            postfix(50, pcall(), |expr, op, _| {
                let mut events = EventVec::new();
                events.push_event(Event::Node(NodeKind::Expr));
                events.push_event(Event::Node(NodeKind::Call));
                events.push_vector(expr);
                events.push_vector(op);
                events.push_event(Event::FinNode);
                events.push_event(Event::FinNode);
                events
            }),
            prefix(25, plambda(), |op, expr, _| {
                let mut events = EventVec::new();
                events.push_event(Event::Node(NodeKind::Expr));
                events.push_event(Event::Node(NodeKind::Lambda));
                events.push_vector(op);
                events.push_vector(expr);
                events.push_event(Event::FinNode);
                events.push_event(Event::FinNode);
                events
            }),
        ))
        .boxed()
}

fn plambda<'a, 'source: 'a>() -> impl Parser<'a, &'a [Token<'source>], EventVec<'source>> + Clone {
    ptbackslash().then(ptbackslash()).map(|(first, second)| {
        let mut events = EventVec::new();
        events.push_event(Event::Token(
            NodeKind::TBackslash,
            std::str::from_utf8(first.token()).unwrap(),
        ));
        events.push_event(Event::Token(
            NodeKind::TBackslash,
            std::str::from_utf8(second.token()).unwrap(),
        ));
        events
    })
}

fn pcall<'a, 'source: 'a>() -> impl Parser<'a, &'a [Token<'source>], EventVec<'source>> + Clone {
    ptlparen()
        .then(separated_by_with_separator(
            pexpr().map(|e| {
                let mut events = EventVec::new();
                events.push_event(Event::Node(NodeKind::CallArg));
                events.push_vector(e);
                events.push_event(Event::FinNode);
                events
            }),
            ptcomma(),
        ))
        .then(ptrparen())
        .map(|((lparen, exprs), rparen)| {
            let mut events = EventVec::new();
            events.push_event(Event::Token(
                NodeKind::TLParen,
                str::from_utf8(lparen.token()).unwrap(),
            ));
            for expr in exprs {
                match expr {
                    Separated::Parser(arg) => events.push_vector(arg),
                    Separated::Separator(comma) => events.push_event(Event::Token(
                        NodeKind::TComma,
                        str::from_utf8(comma.token()).unwrap(),
                    )),
                }
            }
            events.push_event(Event::Token(
                NodeKind::TRParen,
                str::from_utf8(rparen.token()).unwrap(),
            ));
            events
        })
}

fn patom<'a, 'source: 'a>() -> impl Parser<'a, &'a [Token<'source>], EventVec<'source>> + Clone {
    choice((
        select_ref! {
            Token::Ident(inner) => {
                let mut events = EventVec::new();
                events.push_event(Event::Node(NodeKind::Expr));
                events.push_event(Event::Node(NodeKind::Ident));
                events.push_event(Event::Token(NodeKind::Ident, std::str::from_utf8(inner.token()).unwrap()));
                events.push_event(Event::FinNode);
                events.push_event(Event::FinNode);
                events
            },
            Token::BuiltinIdent(inner) => {
                let mut events = EventVec::new();
                events.push_event(Event::Node(NodeKind::Expr));
                events.push_event(Event::Node(NodeKind::BuiltinIdent));
                events.push_event(Event::Token(NodeKind::BuiltinIdent, std::str::from_utf8(inner.token()).unwrap()));
                events.push_event(Event::FinNode);
                events.push_event(Event::FinNode);
                events
            },
            Token::String(inner) => {
                let mut events = EventVec::new();
                events.push_event(Event::Node(NodeKind::Expr));
                events.push_event(Event::Node(NodeKind::String));
                events.push_event(Event::Token(NodeKind::String, std::str::from_utf8(inner.token()).unwrap()));
                events.push_event(Event::FinNode);
                events.push_event(Event::FinNode);
                events
            }
        },
        pblock(),
    ))
}

fn pblock<'a, 'source: 'a>() -> impl Parser<'a, &'a [Token<'source>], EventVec<'source>> + Clone {
    ptlbracket()
        .then(
            pexpr()
                .map(|e| {
                    let mut events = EventVec::new();
                    events.push_event(Event::Node(NodeKind::StmtExpr));
                    events.push_vector(e);
                    events.push_event(Event::FinNode);
                    events
                })
                .repeated()
                .collect(),
        )
        .then(ptrbracket())
        .map(|((lbracket, exprs), rbracket)| {
            let mut events = EventVec::new();
            events.push_event(Event::Node(NodeKind::Block));
            events.push_event(Event::Token(
                NodeKind::TLBracket,
                str::from_utf8(lbracket.token()).unwrap(),
            ));
            events.push_vector(exprs);
            events.push_event(Event::Token(
                NodeKind::TRBracket,
                str::from_utf8(rbracket.token()).unwrap(),
            ));
            events.push_event(Event::FinNode);
            events
        })
}
