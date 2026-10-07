module Megu.Compiler.GenC

type CCode = { mutable Funcs: CFunc list }

and CFunc = { mutable Name: string; mutable ResultType: string; mutable Args: (string * string) list; mutable Body: string }

let genDef = function
    | AST.Define { Name = name; Value = _ } ->
        let func = { Name = name; ResultType = "int"; Args = []; Body = "" }
        func.Body <- sprintf "return 0; // TODO: Implement function %s" name
        func
    | _ -> failwith "Expected a Define node"

let genc ast =
    let mutable node = []
    match ast with
    | AST.Root defs ->
        defs |> List.iter (function
            | AST.Define _ as def ->
                let func = genDef def
                node <- func :: node
            | _ -> failwith "Top-level AST node must be Define"
        )
        { Funcs = List.rev node }
    | _ -> failwith "Top-level AST node must be Root"

let genCString ccode =
    let funcsStr = 
        ccode.Funcs
        |> List.map (fun func ->
            let argsStr = 
                func.Args
                |> List.map (fun (argType, argName) -> sprintf "%s %s" argType argName)
                |> String.concat ", "
            sprintf "%s %s(%s) {\n%s\n}" func.ResultType func.Name argsStr func.Body
        )
        |> String.concat "\n\n"
    funcsStr