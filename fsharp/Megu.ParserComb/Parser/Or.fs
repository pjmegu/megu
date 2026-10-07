namespace Megu.ParserComb
#nowarn "1189" // fsharp compiler bug: https://github.com/dotnet/fsharp/issues/7797#issuecomment-3133850052

open System

[<AutoOpen>]
module Or =
    [<Struct>]
    type Or< ^i, ^o, ^e, ^c, ^p1, ^p2 when Parser< ^i, ^o, ^e, ^c, ^p1 > and Parser< ^i, ^o, ^e, ^c, ^p2 >> =
        | Or of (^p1 * ^p2)

        member inline this.Parse(input: ^i ReadOnlySpan, ctx: ^c) : ParseResult< ^i, ^o, ^e > =
            let (Or(p1, p2)) = this

            match p1.Parse(input, ctx) with
            | Success _ as result -> result
            | Failure(_) ->
                match p2.Parse(input, ctx) with
                | Success _ as result -> result
                | Failure(error) -> Failure(error)

    let inline (</>)< ^i, ^o, ^e, ^c, ^p1, ^p2 when Parser< ^i, ^o, ^e, ^c, ^p1 > and Parser< ^i, ^o, ^e, ^c, ^p2 >>
        (parser1: Parser< ^i, ^o, ^e, ^c, ^p1 >)
        (parser2: Parser< ^i, ^o, ^e, ^c, ^p2 >)
        : Or< ^i, ^o, ^e, ^c, ^p1, ^p2 > =
        Or(parser1, parser2)

    let inline por< ^i, ^o, ^e, ^c, ^p1, ^p2 when Parser< ^i, ^o, ^e, ^c, ^p1 > and Parser< ^i, ^o, ^e, ^c, ^p2 >>
        (parser1: Parser< ^i, ^o, ^e, ^c, ^p1 >)
        (parser2: Parser< ^i, ^o, ^e, ^c, ^p2 >)
        : Or< ^i, ^o, ^e, ^c, ^p1, ^p2 > =
        parser1 </> parser2
