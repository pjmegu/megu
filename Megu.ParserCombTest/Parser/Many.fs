module Parser.Many

open System
open Xunit

open Megu.ParserComb

[<Fact>]
let ``many0 should parse zero or more occurrences`` () =
    let parser = just 'a' --> many0
    let input = MemoryExtensions.AsSpan "aaabc"

    match parse parser input (StdCtx.Create input) with
    | Success(value, remaining) ->
        Assert.Equal<char>(['a'; 'a'; 'a'], value)
        Assert.Equal("bc", remaining.ToString())
    | Failure(error: EmptyError) -> Assert.True(false, $"Unexpected failure: {error}")

[<Fact>]
let ``many0 should parse zero occurrences`` () =
    let parser = just 'a' --> many0
    let input = MemoryExtensions.AsSpan "bc"

    match parse parser input (StdCtx.Create input) with
    | Success(value, remaining) ->
        Assert.Equal<char>([], value)
        Assert.Equal("bc", remaining.ToString())
    | Failure(error: EmptyError) -> Assert.True(false, $"Unexpected failure: {error}")

[<Fact>]
let ``many1 should parse one or more occurrences`` () =
    let parser = just 'a' --> many1
    let input = MemoryExtensions.AsSpan "aaabc"

    match parse parser input (StdCtx.Create input) with
    | Success(value, remaining) ->
        Assert.Equal<char>(['a'; 'a'; 'a'], value)
        Assert.Equal("bc", remaining.ToString())
    | Failure(error: EmptyError) -> Assert.True(false, $"Unexpected failure: {error}")

[<Fact>]
let ``many1 should fail on zero occurrences`` () =
    let parser = just 'a' --> many1
    let input = MemoryExtensions.AsSpan "bc"

    match parse parser input (StdCtx.Create input) with
    | Success(value, remaining) ->
        Assert.True(false, $"Unexpected success: {value}, remaining: {remaining.ToString()}")
    | Failure(error: EmptyError) -> Assert.True(true, $"Expected failure: {error}")