namespace Megu.ParserComb

open System

[<AutoOpen>]
module Just =
    [<Struct>]
    type Just< ^i when ^i: equality> =
        | Just of ^i

        member inline this.Parse< ^e, ^c when Error< ^e > and PosCtx< ^c >>
            (input: ^i ReadOnlySpan, ctx: ^c)
            : ParseResult< ^i, ^i, ^e > =
            let (Just value) = this

            if input.Length > 0 && input[0] = value then
                ctx.IncrementPos()
                Success(value, input.Slice(1))
            else
                Failure(^e: (static member emitWithPos: string -> int -> ^e) ("expected " + string value, ctx.GetPos()))

    let inline just< ^i, ^e, ^c when ^i: equality> (value: ^i) : Just< ^i > = Just value


