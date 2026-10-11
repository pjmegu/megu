use rowan::ast::AstNode;

use crate::ast::NodeKind;

impl From<NodeKind> for rowan::SyntaxKind {
    fn from(kind: NodeKind) -> Self {
        rowan::SyntaxKind(kind.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Lang {}
impl rowan::Language for Lang {
    type Kind = NodeKind;
    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= NodeKind::Error as u16);
        unsafe { std::mem::transmute(raw.0) }
    }
    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.into())
    }
}

pub type SyntaxNode = rowan::SyntaxNode<Lang>;
pub type SyntaxNodePtr = rowan::ast::SyntaxNodePtr<Lang>;
pub type SyntaxToken = rowan::SyntaxToken<Lang>;

macro_rules! make_ast {
    ($name:ident, $kind:ident) => {
        #[derive(Debug, Clone)]
        pub struct $name(SyntaxNode);
        impl AstNode for $name {
            type Language = Lang;

            fn can_cast(kind: <<$name as AstNode>::Language as rowan::Language>::Kind) -> bool {
                kind == NodeKind::$kind
            }

            fn cast(node: rowan::SyntaxNode<Self::Language>) -> Option<Self> {
                if Self::can_cast(node.kind()) {
                    Some(Self(node))
                } else {
                    None
                }
            }

            fn syntax(&self) -> &rowan::SyntaxNode<Self::Language> {
                &self.0
            }
        }
    };
}

make_ast!(Root, Root);
impl Root {
    pub fn defs(&self) -> impl Iterator<Item = Def> {
        self.syntax().children().filter_map(Def::cast)
    }
}

make_ast!(Def, Def);
impl Def {
    pub fn name(&self) -> Option<Ident> {
        self.syntax().children().find_map(Ident::cast)
    }

    pub fn body(&self) -> Option<Expr> {
        self.syntax().children().find_map(Expr::cast)
    }
}

#[derive(Debug, Clone)]
pub struct Stmt(SyntaxNode);
impl AstNode for Stmt {
    type Language = Lang;

    fn can_cast(kind: <<Stmt as AstNode>::Language as rowan::Language>::Kind) -> bool {
        if Expr::can_cast(kind) {
            true
        } else {
            false
        }
    }

    fn cast(node: rowan::SyntaxNode<Self::Language>) -> Option<Self> {
        if Self::can_cast(node.kind()) {
            Some(Self(node))
        } else {
            None
        }
    }

    fn syntax(&self) -> &rowan::SyntaxNode<Self::Language> {
        &self.0
    }
}

impl Stmt {
    pub fn inside_kind(&self) -> NodeKind {
        self.syntax().kind()
    }

    pub fn inside_cast<T: AstNode<Language = <Self as AstNode>::Language>>(&self) -> Option<T> {
        T::cast(self.syntax().clone())
    }
}


#[derive(Debug, Clone)]
pub struct Expr(SyntaxNode);
impl AstNode for Expr {
    type Language = Lang;

    fn can_cast(kind: <<Expr as AstNode>::Language as rowan::Language>::Kind) -> bool {
        match kind {
            NodeKind::Block
            | NodeKind::Call
            | NodeKind::Lambda
            | NodeKind::Ident
            | NodeKind::BuiltinIdent
            | NodeKind::String => true,
            _ => false,
        }
    }

    fn cast(node: rowan::SyntaxNode<Self::Language>) -> Option<Self> {
        if Self::can_cast(node.kind()) {
            Some(Self(node))
        } else {
            None
        }
    }

    fn syntax(&self) -> &rowan::SyntaxNode<Self::Language> {
        &self.0
    }
}

impl Expr {
    pub fn inside_kind(&self) -> NodeKind {
        self.syntax().kind()
    }

    pub fn inside_cast<T: AstNode<Language = <Self as AstNode>::Language>>(&self) -> Option<T> {
        T::cast(self.syntax().clone())
    }
}

make_ast!(Block, Block);
impl Block {
    pub fn stmts(&self) -> impl Iterator<Item = Stmt> {
        self.syntax().children().filter_map(Stmt::cast)
    }
}

make_ast!(Call, Call);
impl Call {
    pub fn callee(&self) -> Option<Expr> {
        self.syntax().children().find_map(Expr::cast)
    }

    pub fn args(&self) -> impl Iterator<Item = CallArg> {
        self.syntax().children().filter_map(CallArg::cast)
    }
}

make_ast!(CallArg, CallArg);
impl CallArg {
    pub fn expr(&self) -> Option<Expr> {
        self.syntax().children().find_map(Expr::cast)
    }
}

make_ast!(Lambda, Lambda);
impl Lambda {
    pub fn body(&self) -> Option<Expr> {
        self.syntax().children().find_map(Expr::cast)
    }
}

make_ast!(Ident, Ident);
impl Ident {
    pub fn token(&self) -> Option<SyntaxToken> {
        self.syntax().children_with_tokens().find_map(|m| match m {
            rowan::NodeOrToken::Node(_) => None,
            rowan::NodeOrToken::Token(t) => Some(t),
        })
    }
}

make_ast!(BuiltinIdent, BuiltinIdent);
impl BuiltinIdent {
    pub fn token(&self) -> Option<SyntaxToken> {
        self.syntax().children_with_tokens().find_map(|m| match m {
            rowan::NodeOrToken::Node(_) => None,
            rowan::NodeOrToken::Token(t) => Some(t),
        })
    }
}

make_ast!(String, String);
impl String {
    pub fn token(&self) -> Option<SyntaxToken> {
        self.syntax().children_with_tokens().find_map(|m| match m {
            rowan::NodeOrToken::Node(_) => None,
            rowan::NodeOrToken::Token(t) => Some(t),
        })
    }
}
