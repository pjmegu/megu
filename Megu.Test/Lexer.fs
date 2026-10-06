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

[<Fact>]
let ``string literal test`` () =
    let input = "\"Hello, World!\""

    let expectedTokens =
        [
            Token.String "Hello, World!"
        ]

    let tokens = Lexer.lex (List.ofSeq input)
    Assert.Equal<Token.Token>(expectedTokens, tokens)

[<Fact>]
let ``builtin identifier test`` () =
    let input = "@builtinFunc"

    let expectedTokens =
        [
            Token.BuiltinIdentifier "builtinFunc"
        ]

    let tokens = Lexer.lex (List.ofSeq input)
    Assert.Equal<Token.Token>(expectedTokens, tokens)

[<Fact>]
let ``identifier test`` () =
    let input = "@builtinIdent ident \"string\""

    let expectedTokens =
        [
            Token.BuiltinIdentifier "builtinIdent"
            Token.Identifier "ident"
            Token.String "string"
        ]
    
    let tokens = Lexer.lex (List.ofSeq input)
    Assert.Equal<Token.Token>(expectedTokens, tokens)
