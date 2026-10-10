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
pub type SyntaxToken = rowan::SyntaxToken<Lang>;

macro_rules! make_ast {
    ($name:ident, $kind:ident) => {
        #[derive(Debug, Clone)]
        pub struct $name(SyntaxNode);
        impl $name {
            pub fn cast(node: SyntaxNode) -> Option<Self> {
                if node.kind() == NodeKind::$kind {
                    Some(Self(node))
                } else {
                    None
                }
            }

            pub fn syntax(&self) -> &SyntaxNode {
                &self.0
            }
        }
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
make_ast!(Def, Def);
make_ast!(StmtExpr, StmtExpr);
make_ast!(Expr, Expr);
make_ast!(Block, Block);
make_ast!(Call, Call);
make_ast!(CallArg, CallArg);
make_ast!(Ident, Ident);
make_ast!(BuiltinIdent, BuiltinIdent);
make_ast!(String, String);
