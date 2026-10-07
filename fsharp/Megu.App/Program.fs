module Megu.App

open System.CommandLine
open System.IO
open System

let exec script =
    eprintfn "Executing script: %s" script
    let token = Compiler.Lexer.lex (List.ofSeq script)
    eprintfn "Tokens: %A" token
    let ast = Compiler.Parser.parse (ReadOnlySpan (Array.ofList token))
    eprintfn "AST: %A" ast
    let genc = Compiler.GenC.genc ast
    let gencstr = Compiler.GenC.genCString genc
    eprintfn "Generated C code: %s" gencstr
    printfn "%s" gencstr

[<EntryPoint>]
let main args =
    let rootCommand = RootCommand "Megu Compiler"

    let inputPath =
        Option<string>("--input", [| "-i" |])
        |> fun o ->
            o.Description <- "Input file path"
            o

    rootCommand.Options.Add(inputPath)

    let script =
        Option<string>("--script", [| "-s" |])
        |> fun o ->
            o.Description <- "Script"
            o

    rootCommand.Options.Add(script)

    rootCommand.SetAction(fun result ->
        let inputPath = result.GetValue(inputPath)
        let script = result.GetValue(script)

        match inputPath, script with
        | i, null ->
            let script = File.ReadAllText(i)
            exec script
        | null, s -> exec s
        | null, null
        | _, _ -> printfn "Error: Please provide only one of input file path or script.")

    rootCommand.Parse(args).Invoke()
