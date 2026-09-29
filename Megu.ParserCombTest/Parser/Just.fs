module Parser.Just

open System
open Xunit

open Megu.ParserComb

[<Fact>]
let ``just parse`` () =
    let input = MemoryExtensions.AsSpan "abc"
    let parser = just 'a'

    match parse parser input EmptyCtx with
    | Success(value, remaining) ->
        Assert.Equal('a', value)
        Assert.Equal("bc", remaining.ToString())
    | Failure(error: StdError) -> Assert.True(false, $"Unexpected failure: {error}")

