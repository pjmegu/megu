module Parser.Recursive

open System
open Xunit

open Megu.ParserComb

[<Fact>]
let ``recursive parse`` () =
    let input = MemoryExtensions.AsSpan "a+a+a"
    #nowarn "40"
    let rec expr =
        recursive (fun () ->
            let a = expr .>> just '+' .>> expr
            let b = just 'a'
            a </> b 
        )

    match parse expr input (StdRecursiveCtx.Create input) with
    | Success(value, remaining) ->
        Assert.Equal('a', value)
        Assert.Equal("", remaining.ToString())
    | Failure(error: EmptyError) -> Assert.True(false, $"Unexpected failure: {error}")