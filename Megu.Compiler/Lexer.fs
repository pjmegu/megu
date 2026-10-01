module Megu.Compiler.Lexer

open Megu.Compiler.Token

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
    | '=' :: rest -> Equal :: tokenize rest
    | c :: rest when System.Char.IsLetter(c) ->
        let identifier, remaining = parseIdentifier (c :: rest)

        match identifier with
        | "def" -> Def :: tokenize remaining
        | _ -> Identifier identifier :: tokenize remaining
    | _ :: rest -> tokenize rest

let lex input = tokenize (List.ofSeq input)
