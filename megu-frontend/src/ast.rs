pub(crate) mod event;
pub(crate) mod rwn;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u16)]
pub enum NodeKind {
    // Tokens
    TDef,
    TIdent,
    TBuiltinIdent,
    TString,
    TLParen,
    TRParen,
    TLBracket,
    TRBracket,
    TComma,
    TBackslash,

    // Top Nodes
    Root,
    Def,

    // stmts
    StmtExpr,

    // expr
    Expr,
    Block,
    Call,
    CallArg,
    Lambda,

    // literal
    Ident,
    BuiltinIdent,
    String,

    Rest,
    Error,
}

impl From<NodeKind> for u16 {
    fn from(kind: NodeKind) -> Self {
        kind as u16
    }
}
