namespace Megu.ParserComb

open System

[<AutoOpen>]
module Many1 =
    [<Struct>]
    type Many1<'i, 'o, 'e, 'c, 'p when Parser<'i, 'o, 'e, 'c, 'p>> =
        | Many of 'p

        member inline this.Parse(input: 'i ReadOnlySpan, ctx: 'c) : ParseResult<'i, 'o list, 'e> =
            let (Many(p)) = this
            let mutable i = input
            let mutable r = []
            let mutable fail = None
            let mutable flag = true

            while flag do
                match p.Parse(i, ctx) with
                | Success(value, remaining) ->
                    r <- value :: r
                    i <- remaining
                | Failure error ->
                    flag <- false
                    fail <- Some error

            match fail with
            | Some error when List.isEmpty r -> Failure error
            | _ -> Success(List.rev r, i.Slice 0)

    let inline many1< ^p, ^i, ^o, ^e, ^c when Parser< ^i, ^o, ^e, ^c, ^p >> (parser: ^p) : Many1<'i, 'o, 'e, 'c, 'p> =
        Many parser

[<AutoOpen>]
module Many0 =
    [<Struct>]
    type Many0<'i, 'o, 'e, 'c, 'p when Parser<'i, 'o, 'e, 'c, 'p>> =
        | Many of 'p

        member inline this.Parse(input: 'i ReadOnlySpan, ctx: 'c) : ParseResult<'i, 'o list, 'e> =
            let (Many(p)) = this
            let mutable i = input
            let mutable r = []
            let mutable flag = true

            while flag do
                match p.Parse(i, ctx) with
                | Success(value, remaining) ->
                    r <- value :: r
                    i <- remaining
                | Failure _ -> flag <- false

            Success(List.rev r, i.Slice 0)

    let inline many0< ^p, ^i, ^o, ^e, ^c when Parser< ^i, ^o, ^e, ^c, ^p >> (parser: ^p) : Many0<'i, 'o, 'e, 'c, 'p> =
        Many parser
