module Parser.Or

open System
open Xunit

open Megu.ParserComb

[<Fact>]
let ``or parse`` () =
    let input = MemoryExtensions.AsSpan "abc"
    let parser = just 'x' </> just 'a'

    match parse parser input (StdCtx.Create input) with
    | Success(value, remaining) ->
        Assert.Equal('a', value)
        Assert.Equal("bc", remaining.ToString())
    | Failure(error: EmptyError) -> Assert.True(false, $"Unexpected failure: {error}")
