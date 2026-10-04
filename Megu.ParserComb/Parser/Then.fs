namespace Megu.ParserComb
#nowarn "1189" // fsharp compiler bug: https://github.com/dotnet/fsharp/issues/7797#issuecomment-3133850052

open System

[<AutoOpen>]
module Then =
    [<Struct>]
    type Then< ^i, ^o1, ^o2, ^e, ^c, ^p1, ^p2 when Parser< ^i, ^o1, ^e, ^c, ^p1 > and Parser< ^i, ^o2, ^e, ^c, ^p2 >> =
        | Then of (^p1 * ^p2)

        member inline this.Parse(input: ^i ReadOnlySpan, ctx: ^c) : ParseResult< ^i, ^o1 * ^o2, ^e > =
            let (Then(p1, p2)) = this

            match p1.Parse(input, ctx) with
            | Success(value1, remaining) ->
                match p2.Parse(remaining, ctx) with
                | Success(value2, remaining) -> Success((value1, value2), remaining.Slice 0)
                | Failure error -> Failure error
            | Failure error -> Failure error

    let inline (.*>.)< ^i, ^o1, ^o2, ^e, ^c, ^p1, ^p2
        when Parser< ^i, ^o1, ^e, ^c, ^p1 > and Parser< ^i, ^o2, ^e, ^c, ^p2 >>
        (parser1: Parser< ^i, ^o1, ^e, ^c, ^p1 >)
        (parser2: Parser< ^i, ^o2, ^e, ^c, ^p2 >)
        : Then< ^i, ^o1, ^o2, ^e, ^c, ^p1, ^p2 > =
        Then(parser1, parser2)

    let inline pthen< ^i, ^o1, ^o2, ^e, ^c, ^p1, ^p2
        when Parser< ^i, ^o1, ^e, ^c, ^p1 > and Parser< ^i, ^o2, ^e, ^c, ^p2 >>
        (parser1: Parser< ^i, ^o1, ^e, ^c, ^p1 >)
        (parser2: Parser< ^i, ^o2, ^e, ^c, ^p2 >)
        : Then< ^i, ^o1, ^o2, ^e, ^c, ^p1, ^p2 > =
        parser1 .*>. parser2

[<AutoOpen>]
module ThenIgnore =
    [<Struct>]
    type ThenIgnore< ^i, ^o1, ^o2, ^e, ^c, ^p1, ^p2
        when Parser< ^i, ^o1, ^e, ^c, ^p1 > and Parser< ^i, ^o2, ^e, ^c, ^p2 >> =
        | ThenIgnore of (^p1 * ^p2)

        member inline this.Parse(input: ^i ReadOnlySpan, ctx: ^c) : ParseResult< ^i, ^o1, ^e > =
            let (ThenIgnore(p1, p2)) = this

            match p1.Parse(input, ctx) with
            | Success(value1, remaining) ->
                match p2.Parse(remaining, ctx) with
                | Success(_, remaining) -> Success(value1, remaining.Slice 0)
                | Failure error -> Failure error
            | Failure error -> Failure error

    let inline (.*>)< ^i, ^o1, ^o2, ^e, ^c, ^p1, ^p2
        when Parser< ^i, ^o1, ^e, ^c, ^p1 > and Parser< ^i, ^o2, ^e, ^c, ^p2 >>
        (parser1: Parser< ^i, ^o1, ^e, ^c, ^p1 >)
        (parser2: Parser< ^i, ^o2, ^e, ^c, ^p2 >)
        : ThenIgnore< ^i, ^o1, ^o2, ^e, ^c, ^p1, ^p2 > =
        ThenIgnore(parser1, parser2)

    let inline pthenIgnore< ^i, ^o1, ^o2, ^e, ^c, ^p1, ^p2
        when Parser< ^i, ^o1, ^e, ^c, ^p1 > and Parser< ^i, ^o2, ^e, ^c, ^p2 >>
        (parser1: Parser< ^i, ^o1, ^e, ^c, ^p1 >)
        (parser2: Parser< ^i, ^o2, ^e, ^c, ^p2 >)
        : ThenIgnore< ^i, ^o1, ^o2, ^e, ^c, ^p1, ^p2 > =
        parser1 .*> parser2

[<AutoOpen>]
module IgnoreThen =
    [<Struct>]
    type IgnoreThen< ^i, ^o1, ^o2, ^e, ^c, ^p1, ^p2
        when Parser< ^i, ^o1, ^e, ^c, ^p1 > and Parser< ^i, ^o2, ^e, ^c, ^p2 >> =
        | IgnoreThen of (^p1 * ^p2)

        member inline this.Parse(input: ^i ReadOnlySpan, ctx: ^c) : ParseResult< ^i, ^o2, ^e > =
            let (IgnoreThen(p1, p2)) = this

            match p1.Parse(input, ctx) with
            | Success(_, remaining) ->
                match p2.Parse(remaining, ctx) with
                | Success(value2, remaining) -> Success(value2, remaining.Slice 0)
                | Failure error -> Failure error
            | Failure error -> Failure error

    let inline ( *>. )< ^i, ^o1, ^o2, ^e, ^c, ^p1, ^p2
        when Parser< ^i, ^o1, ^e, ^c, ^p1 > and Parser< ^i, ^o2, ^e, ^c, ^p2 >>
        (parser1: Parser< ^i, ^o1, ^e, ^c, ^p1 >)
        (parser2: Parser< ^i, ^o2, ^e, ^c, ^p2 >)
        : IgnoreThen< ^i, ^o1, ^o2, ^e, ^c, ^p1, ^p2 > =
        IgnoreThen(parser1, parser2)

    let inline pignoreThen< ^i, ^o1, ^o2, ^e, ^c, ^p1, ^p2
        when Parser< ^i, ^o1, ^e, ^c, ^p1 > and Parser< ^i, ^o2, ^e, ^c, ^p2 >>
        (parser1: Parser< ^i, ^o1, ^e, ^c, ^p1 >)
        (parser2: Parser< ^i, ^o2, ^e, ^c, ^p2 >)
        : IgnoreThen< ^i, ^o1, ^o2, ^e, ^c, ^p1, ^p2 > =
        parser1 *>. parser2
