module Megu.Compiler.Token

type Token =
    // keywords
    | Def
    // literals
    | Identifier of string
    | BuiltinIdentifier of string
    | String of string
    // signs
    | Backslash // \
    | LBracket // [
    | RBracket // ]
    | LParen // (
    | RParen // )
