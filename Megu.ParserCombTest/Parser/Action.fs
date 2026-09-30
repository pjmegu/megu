module Parser.Action

open System
open Xunit

open Megu.ParserComb

[<Fact>]
let ``action parse`` () =
    let input = MemoryExtensions.AsSpan "abc"
    let parser = 
        just 'a' --> action (fun _ -> 'b') 
        </> just 'b'

    match parse parser input (StdCtx.Create input) with
    | Success(value, remaining) ->
        Assert.Equal('b', value)
        Assert.Equal("bc", remaining.ToString())
    | Failure(error: EmptyError) -> Assert.True(false, $"Unexpected failure: {error}")
