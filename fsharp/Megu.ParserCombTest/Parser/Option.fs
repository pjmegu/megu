module Parser.Option

open System
open Xunit

open Megu.ParserComb

[<Fact>]
let ``option parse`` () =
    let input = MemoryExtensions.AsSpan "abc"

    let parser =
        opt (
            take (fun c ->
                match c with
                | 'a' -> Some 'a'
                | _ -> None)
        )

    match parse parser input (StdCtx.Create input) with
    | Success(value, remaining) ->
        Assert.Equal(Some 'a', value)
        Assert.Equal("bc", remaining.ToString())
    | Failure(error: EmptyError) -> Assert.True(false, $"Unexpected failure: {error}")
