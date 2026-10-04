module Megu.Compiler.AST

[<Struct>]
type Define = { Name: string; Value: Node }
and [<Struct>] Lambda = { Body: Node }

and Node =
    | Root of Node list
    | Define of Define
    | Lambda of Lambda
    | Block of Node list
