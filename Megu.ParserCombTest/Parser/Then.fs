module Parser.Then

open System
open Xunit

open Megu.ParserComb

[<Fact>]
let ``Then parser should parse two parsers in sequence`` () =
    let parser1 = just 'a'
    let parser2 = just 'b'
    let combinedParser = parser1 .>>. parser2

    let input = MemoryExtensions.AsSpan "abc"

    match parse combinedParser input (StdCtx.Create input) with
    | Success((value1, value2), remaining) ->
        Assert.Equal('a', value1)
        Assert.Equal('b', value2)
        Assert.Equal("c", remaining.ToString())
    | Failure(error: EmptyError) -> Assert.True(false, $"Unexpected failure: {error}")

[<Fact>]
let ``ThenIgnore parser should parse two parsers in sequence and ignore the second result`` () =
    let parser1 = just 'a'
    let parser2 = just 'b'
    let combinedParser = parser1 .>> parser2

    let input = MemoryExtensions.AsSpan "abc"

    match parse combinedParser input (StdCtx.Create input) with
    | Success(value1, remaining) ->
        Assert.Equal('a', value1)
        Assert.Equal("c", remaining.ToString())
    | Failure(error: EmptyError) -> Assert.True(false, $"Unexpected failure: {error}")

[<Fact>]
let ``IgnoreThen parser should parse two parsers in sequence and ignore the first result`` () =
    let parser1 = just 'a'
    let parser2 = just 'b'
    let combinedParser = parser1 >>. parser2

    let input = MemoryExtensions.AsSpan "abc"

    match parse combinedParser input (StdCtx.Create input) with
    | Success(value2, remaining) ->
        Assert.Equal('b', value2)
        Assert.Equal("c", remaining.ToString())
    | Failure(error: EmptyError) -> Assert.True(false, $"Unexpected failure: {error}")
