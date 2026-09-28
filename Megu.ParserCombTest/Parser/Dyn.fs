module Parser.Dyn

open System
open Xunit

open Megu.ParserComb

[<Fact>]
let ``asDyn parser`` () =
    let input = MemoryExtensions.AsSpan "abc"
    let parser = asDyn (fun () -> just 'a')

    match parse parser input EmptyCtx with
    | Success(value, remaining) ->
        Assert.Equal('a', value)
        Assert.Equal("bc", remaining.ToString())
    | Failure(error: StdError) -> Assert.True(false, $"Unexpected failure: {error}")
