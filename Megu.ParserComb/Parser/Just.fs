namespace Megu.ParserComb

open System

[<AutoOpen>]
module Just =
    [<Struct>]
    type Just< ^i when ^i: equality> =
        | Just of ^i

        member inline this.Parse< ^e, ^c when Error< ^e > and PosCtx< ^i, ^c >>
            (input: ^i ReadOnlySpan, ctx: ^c)
            : ParseResult< ^i, ^i, ^e > =
            let (Just value) = this

            if input.Length > 0 && input[0] = value then
                Success(value, input.Slice 1)
            else
                let msg = sprintf "Expected '%A', but got '%A'" value (if input.Length > 0 then input[0] else Unchecked.defaultof< ^i>)
                let pos = ctx.GetPos input
                let error = { msg = Some msg; pos = Some pos }
                Failure(emitError error)

    let inline just< ^i, ^e, ^c when ^i: equality> (value: ^i) : Just< ^i > = Just value
