module Megu.App

open System.CommandLine
open System.IO

let exec script = printfn "Executing script: %s" script

[<EntryPoint>]
let main args =
    let rootCommand = RootCommand("Megu Compiler")

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
