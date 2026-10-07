namespace Megu.ParserComb

open System

[<AutoOpen>]
module Action =
    [<Struct>]
    type Action< ^i, ^o1, ^o2, ^e, ^c, ^p when Parser< ^i, ^o1, ^e, ^c, ^p >> =
        | Action of (^p * (^o1 -> ^o2))

        member inline this.Parse(input: ^i ReadOnlySpan, ctx: ^c) : ParseResult< ^i, ^o2, ^e > =
            let (Action(parser, action)) = this

            match parser.Parse(input, ctx) with
            | Success(value, remaining) -> Success(action value, remaining.Slice 0)
            | Failure error -> Failure error

    let inline action< ^i, ^o1, ^o2, ^e, ^c, ^p when Parser< ^i, ^o1, ^e, ^c, ^p >>
        (action: (^o1 -> ^o2))
        (parser: Parser< ^i, ^o1, ^e, ^c, ^p >)
        : Action< ^i, ^o1, ^o2, ^e, ^c, ^p > =
        Action(parser, action)
