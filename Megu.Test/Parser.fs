module Parser

open System
open Xunit

open Megu.Compiler

[<Fact>]
let ``define test`` () =
    // def define = <missing>
    let tokens = [ Token.Def; Token.Identifier "define"; Token.Equal ]

    let expected =
        AST.Root
            [
                AST.Define
                    {
                        Name = "define"
                        Value = AST.Missing "expr"
                    }
            ]

    let ast: AST.Node = Parser.parse tokens
    Assert.Equal(expected, ast)
