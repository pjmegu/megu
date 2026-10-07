module Parser.Just

open System
open Xunit

open Megu.ParserComb

[<Fact>]
let ``just parse`` () =
    let input = MemoryExtensions.AsSpan "abc"
    let parser = just 'a'

    match parse parser input (StdCtx.Create input) with
    | Success(value, remaining) ->
        Assert.Equal('a', value)
        Assert.Equal("bc", remaining.ToString())
    | Failure(error: EmptyError) -> Assert.True(false, $"Unexpected failure: {error}")

[<Fact>]
let ``take parse`` () =
    let input = MemoryExtensions.AsSpan "abc"

    let parser =
        take (fun c ->
            match c with
            | 'a' -> Some 'a'
            | 'b' -> Some 'b'
            | _ -> None)

    match parse parser input (StdCtx.Create input) with
    | Success(value, remaining) ->
        Assert.Equal('a', value)
        Assert.Equal("bc", remaining.ToString())
    | Failure(error: EmptyError) -> Assert.True(false, $"Unexpected failure: {error}")
