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

// tokens
macro_rules! make_ptoken {
    ($name:ident, $variant:ident) => {
        fn $name<'a, 'source: 'a>() -> impl Parser<'a, &'a [Token<'source>], TokenInner<'source>> {
            select! { Token::$variant(inner) => inner }
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

// parser
fn proot<'a, 'source: 'a>() -> impl Parser<'a, &'a [Token<'source>], EventVec<'source>> {
    pdef().repeated().collect().then_ignore(end())
}

fn pdef<'a, 'source: 'a>() -> impl Parser<'a, &'a [Token<'source>], EventVec<'source>> {
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

fn pexpr<'a, 'source: 'a>() -> impl Parser<'a, &'a [Token<'source>], EventVec<'source>> {
    patom()
        .pratt((postfix(50, pcall(), |expr, op, _| {
            let mut events = EventVec::new();
            events.push_event(Event::Node(NodeKind::Expr));
            events.push_event(Event::Node(NodeKind::Call));
            events.push_vector(expr);
            events.push_vector(op);
            events.push_event(Event::FinNode);
            events.push_event(Event::FinNode);
            events
        }),))
        .boxed()
}

fn pcall<'a, 'source: 'a>() -> impl Parser<'a, &'a [Token<'source>], EventVec<'source>> {
    ptlparen()
        .then(
            pexpr()
                .map(|e| {
                    let mut events = EventVec::new();
                    events.push_event(Event::Node(NodeKind::CallArg));
                    events.push_vector(e);
                    events.push_event(Event::FinNode);
                    events
                })
                .separated_by(ptcomma())
                .allow_trailing()
                .collect(),
        )
        .then(ptrparen())
        .map(|((lparen, exprs), rparen)| {
            let mut events = EventVec::new();
            events.push_event(Event::Token(
                NodeKind::TLParen,
                str::from_utf8(lparen.token()).unwrap(),
            ));
            events.push_vector(exprs);
            events.push_event(Event::Token(
                NodeKind::TRParen,
                str::from_utf8(rparen.token()).unwrap(),
            ));
            events
        })
}

fn patom<'a, 'source: 'a>() -> impl Parser<'a, &'a [Token<'source>], EventVec<'source>> {
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

fn pblock<'a, 'source: 'a>() -> impl Parser<'a, &'a [Token<'source>], EventVec<'source>> {
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
