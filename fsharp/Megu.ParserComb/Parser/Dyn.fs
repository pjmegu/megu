namespace Megu.ParserComb

open System

[<AutoOpen>]
module Dyn =
    type DynDelegate<'i, 'o, 'e, 'c> = delegate of ReadOnlySpan<'i> * 'c -> ParseResult<'i, 'o, 'e>

    type Dyn<'i, 'o, 'e, 'c> =
        private
        | Dyn of DynDelegate<'i, 'o, 'e, 'c>

        member this.Parse(input: ReadOnlySpan<'i>, ctx: 'c) : ParseResult<'i, 'o, 'e> =

            let (Dyn parser) = this
            parser.Invoke(input, ctx)

    let asDynImpl (factory: unit -> DynDelegate<'i, 'o, 'e, 'c>) : Dyn<'i, 'o, 'e, 'c> =

        Dyn(
            DynDelegate(fun input ctx ->
                let parser = factory ()
                parser.Invoke(input, ctx))
        )

    let inline asDyn< ^i, ^o, ^e, ^c, ^p when Parser< ^i, ^o, ^e, ^c, ^p >> ([<InlineIfLambda>] factory: unit -> ^p) =
        let parser = ref ValueNone

        asDynImpl (fun () ->
            if parser.Value.IsNone then
                parser.Value <- ValueSome(factory ())

            DynDelegate(parse parser.Value.Value))
