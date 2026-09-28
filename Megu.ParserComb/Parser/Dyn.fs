namespace Megu.ParserComb

open System

type DynParseDelegate<'i, 'o, 'e, 'c> =
    delegate of
        ReadOnlySpan<'i> * 'c
        -> ParseResult<'i, 'o, 'e>

type DynParser<'i, 'o, 'e, 'c> =
    private
    | DynParser of DynParseDelegate<'i, 'o, 'e, 'c>

    member this.Parse(
        input: ReadOnlySpan<'i>,
        ctx: 'c
    ) : ParseResult<'i, 'o, 'e> =

        let (DynParser parser) = this
        parser.Invoke(input, ctx)

[<AutoOpen>]
module DynParse =
    let asDynImpl
        (factory: unit -> DynParseDelegate<'i, 'o, 'e, 'c>)
        : DynParser<'i, 'o, 'e, 'c> =

        DynParser(
            DynParseDelegate(fun input ctx ->
                let parser = factory()
                parser.Invoke(input, ctx)
            )
        )
    
    let inline asDyn<^i, ^o, ^e, ^c, ^p when Parser<^i, ^o, ^e, ^c, ^p>> ([<InlineIfLambda>] factory: unit -> ^p) =
        asDynImpl (fun () ->
            let parser = factory()
            DynParseDelegate(parse parser)
        )
