namespace Megu.ParserComb

open System

[<AutoOpen>]
module Recursive =
    type RecursiveDelegate<'i, 'o, 'e, 'c> = delegate of ReadOnlySpan<'i> * 'c -> ParseResult<'i, 'o, 'e>

    type RecursiveCtx< ^i, ^o, ^e, ^T
        when ^T: (member ParseRecursive:
            RecursiveDelegate< ^i, ^o, ^e, ^T > -> ^i ReadOnlySpan -> ParseResult< ^i, ^o, ^e >)> = ^T

    and Recursive<'i, 'o, 'e, 'c when RecursiveCtx<'i, 'o, 'e, 'c>> =
        | Recursive of RecursiveDelegate<'i, 'o, 'e, 'c>

        member inline this.Parse(input: ReadOnlySpan<'i>, ctx: 'c) : ParseResult<'i, 'o, 'e> =

            let (Recursive parser) = this
            ctx.ParseRecursive(parser, input)

    type RecursiveResult< ^e> =
        | Evaluating
        | ResultSuccess of obj * int // result * end position
        | ResultFailure of ^e

    let inline recursiveImpl (factory: unit -> RecursiveDelegate<'i, 'o, 'e, 'c>) : Recursive<'i, 'o, 'e, 'c> =

        Recursive(
            RecursiveDelegate(fun input ctx ->
                let parser = factory ()
                parser.Invoke(input, ctx))
        )

    let inline recursive< ^i, ^o, ^e, ^c, ^p when RecursiveCtx< ^i, ^o, ^e, ^c > and Parser< ^i, ^o, ^e, ^c, ^p >>
        ([<InlineIfLambda>] factory: unit -> ^p)
        =
        let parser = ref ValueNone

        recursiveImpl (fun () ->
            if parser.Value.IsNone then
                parser.Value <- ValueSome(factory ())

            RecursiveDelegate(parse parser.Value.Value))
