module Lexer

open System
open Xunit
open Megu.Compiler

[<Fact>]
let ``define test`` () =
    let input = @"def x \\ []"

    let expectedTokens =
        [
            Token.Def
            Token.Identifier "x"
            Token.Backslash
            Token.Backslash
            Token.LBracket
            Token.RBracket
        ]

    let tokens = Lexer.lex (List.ofSeq input)
    Assert.Equal<Token.Token>(expectedTokens, tokens)
