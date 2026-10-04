module Megu.Compiler.AST

type Define = { Name: string; Value: Node }
and Lambda = { Body: Node }

and Node =
    | Root of Node list
    | Define of Define
    | Lambda of Lambda
    | Block of Node list
