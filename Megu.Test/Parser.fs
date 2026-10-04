module Parser

open System
open Xunit

open Megu.Compiler

[<Fact>]
let ``define test`` () =
    // def define \\ []
    let tokens =
        [
            Token.Def
            Token.Identifier "define"
            Token.Backslash
            Token.Backslash
            Token.LBracket
            Token.RBracket
        ] |> Array.ofList
    
    let span = ReadOnlySpan tokens

    let expected =
        AST.Root
            [
                AST.Define
                    {
                        Name = "define"
                        Value = AST.Lambda { Body = AST.Block [] }
                    }
            ]

    let ast = Parser.parse span
    Assert.Equal(expected, ast)
