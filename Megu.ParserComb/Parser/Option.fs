namespace Megu.ParserComb

open System

[<AutoOpen>]
module Option =
    [<Struct>]
    type Option< ^i, ^o, ^e, ^c, ^p when Parser< ^i, ^o, ^e, ^c, ^p >> =
        | Option of ^p

        member inline this.Parse(input: ^i ReadOnlySpan, ctx: ^c) : ParseResult< ^i, ^o option, ^e > =
            let (Option p) = this

            match p.Parse(input, ctx) with
            | Success(value, rest) -> Success(Some value, rest.Slice 0)
            | Failure(_) -> Success(None, input.Slice 0)

    let inline opt< ^i, ^o, ^e, ^c, ^p when Parser< ^i, ^o, ^e, ^c, ^p >>
        (parser: Parser< ^i, ^o, ^e, ^c, ^p >)
        : Option< ^i, ^o, ^e, ^c, ^p > =
        Option parser
