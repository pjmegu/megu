namespace Megu.ParserComb

open System

[<AutoOpen>]
module Recover =
    type Recover< ^i, ^o, ^e, ^c, ^p when Parser< ^i, ^o, ^e, ^c, ^p >> =
        | Recover of ^p * (^e -> ^o)

        member inline this.Parse(input: ^i ReadOnlySpan, ctx: ^c) : ParseResult< ^i, ^o, ^e > =
            let (Recover(parser, recover)) = this

            match parser.Parse(input, ctx) with
            | Success(output, remaining) -> Success(output, remaining.Slice 0)
            | Failure error -> Success(recover error, input.Slice 0)

    let inline recover< ^i, ^o, ^e, ^c, ^p when Parser< ^i, ^o, ^e, ^c, ^p >>
        (recover: ^e -> ^o)
        (parser: ^p)
        : Recover< ^i, ^o, ^e, ^c, ^p > =
        Recover(parser, recover)
