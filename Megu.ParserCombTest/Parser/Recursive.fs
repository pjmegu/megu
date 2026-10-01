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

[<Fact>]
let ``recursive memo is shared within the same context`` () =
    let input = MemoryExtensions.AsSpan "a"
    let mutable calls = 0

    let parser2 =
        RecursiveDelegate<char, char, EmptyError, StdRecursiveCtx<EmptyError>>(fun input _ ->
            calls <- calls + 1
            Success('a', input.Slice 1))

    let parser1 =
        RecursiveDelegate<char, char, EmptyError, StdRecursiveCtx<EmptyError>>(fun input ctx ->
            match ctx.ParseRecursive(parser2, input) with
            | Success _ ->
                ctx.ParseRecursive(parser2, input)
            | Failure error ->
                Failure error)

    let ctx = StdRecursiveCtx<EmptyError>.Create input

    match ctx.ParseRecursive(parser1, input) with
    | Success(value, remaining) ->
        Assert.Equal('a', value)
        Assert.Equal("", remaining.ToString())
        Assert.Equal(1, calls)
    | Failure(error: EmptyError) ->
        Assert.True(false, $"Unexpected failure: {error}")
