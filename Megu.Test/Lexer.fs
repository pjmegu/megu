module Lexer

open System
open Xunit
open Megu.Compiler

[<Fact>]
let ``define test`` () =
    let input = "def x ="
    let expectedTokens = [ Token.Def; Token.Identifier "x"; Token.Equal ]
    let tokens = Lexer.lex input
    Assert.Equal<Token.Token>(expectedTokens, tokens)
