module Megu.Compiler.Lexer

open Megu.Compiler.Token

let parseString chars =
    let rec loop acc chars =
        match chars with
        | [] -> failwith "Unterminated string literal"
        | '\\' :: '"' :: rest -> loop ('"' :: acc) rest
        | '"' :: rest -> System.String(List.toArray (List.rev acc)), rest
        | c :: rest -> loop (c :: acc) rest

    loop [] chars

let parseIdentifier chars =
    let rec loop acc chars =
        match chars with
        | [] -> System.String(List.toArray (List.rev acc)), []
        | c :: rest when System.Char.IsLetterOrDigit(c) -> loop (c :: acc) rest
        | _ -> System.String(List.toArray (List.rev acc)), chars

    loop [] chars

let rec tokenize chars =
    match chars with
    | [] -> []
    // ignores
    | ' ' :: rest -> tokenize rest
    | '\t' :: rest -> tokenize rest
    | '\n' :: rest -> tokenize rest
    // signs
    | '\\' :: rest -> Backslash :: tokenize rest
    | '[' :: rest -> LBracket :: tokenize rest
    | ']' :: rest -> RBracket :: tokenize rest
    | '(' :: rest -> LParen :: tokenize rest
    | ')' :: rest -> RParen :: tokenize rest
    // keywords and identifiers
    | '"' :: rest ->
        let str, remaining = parseString rest
        String str :: tokenize remaining
    | '@' :: c :: rest when System.Char.IsLetter c ->
        let identifier, remaining = parseIdentifier (c :: rest)
        BuiltinIdentifier identifier :: tokenize remaining
    | c :: rest when System.Char.IsLetter c ->
        let identifier, remaining = parseIdentifier (c :: rest)

        match identifier with
        | "def" -> Def :: tokenize remaining
        | _ -> Identifier identifier :: tokenize remaining
    | _ :: rest -> tokenize rest

let lex input = tokenize input
