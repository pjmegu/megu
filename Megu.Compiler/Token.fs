module Megu.Compiler.Token

type Token =
    // keywords
    | Def
    // literals
    | Identifier of string
    // signs
    | Backslash // \
    | LBracket // [
    | RBracket // ]
