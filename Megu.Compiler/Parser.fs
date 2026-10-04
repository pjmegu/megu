#nowarn "40"

module Megu.Compiler.Parser

open Megu.ParserComb

open Megu.Compiler.Token
open Megu.Compiler.AST

let parse tokens =
    let ident =
        take (function
            | Identifier name -> Some name
            | _ -> None)

    let rec expr =
        recursive (fun _ ->
            lambda
            </> block
        )
    and lambda =
        just Backslash .*> just Backslash *>. expr
        --> action (fun body -> Lambda { Body = body })
    and block =
        just LBracket *>. many0 expr .*> just RBracket
        --> action (fun body -> Block body)

    let def =
        just Def *>. ident .*>. expr
        --> action (fun (name, _) -> Define { Name = name; Value = Lambda { Body = Block [] } })
    
    let root =
        many0 def
        --> action (fun defs -> Root defs)

    let parser = root
    let ctx = StdRecursiveCtx<StdError>.Create tokens

    match parse parser tokens ctx with
    | Success(result, _) -> result
    | Failure(error: StdError) ->
        failwith (
            match error with
            | StdError msg -> msg
        )
