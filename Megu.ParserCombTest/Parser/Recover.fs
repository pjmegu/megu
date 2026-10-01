module Parser.Recover

open System
open Xunit

open Megu.ParserComb

[<Fact>]
let ``recover parse`` () =
    let input = MemoryExtensions.AsSpan "abc"
    let parser = just 'a' --> recover (fun _ -> 'x')

    match parse parser input (StdCtx.Create input) with
    | Success(value, remaining) ->
        Assert.Equal('a', value)
        Assert.Equal("bc", remaining.ToString())
    | Failure(error: EmptyError) -> Assert.True(false, $"Unexpected failure: {error}")

[<Fact>]
let ``recover parse with failure`` () =
    let input = MemoryExtensions.AsSpan "xbc"
    let parser = just 'a' --> recover (fun _ -> 'x')

    match parse parser input (StdCtx.Create input) with
    | Success(value, remaining) ->
        Assert.Equal('x', value)
        Assert.Equal("xbc", remaining.ToString())
    | Failure(error: EmptyError) -> Assert.True(false, $"Unexpected failure: {error}")
