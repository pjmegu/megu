namespace Megu.ParserComb

open System

type Error< ^T when ^T: (static member emit: string -> ^T) and ^T: (static member emitWithPos: string -> int -> ^T)> =
    ^T

[<Struct; Runtime.CompilerServices.IsByRefLike>]
type ParseResult< ^i, ^o, ^e> =
    | Success of 'o * 'i ReadOnlySpan
    | Failure of 'e

type Parser< ^i, ^o, ^e, ^c, ^T when ^T: (member Parse: ^i ReadOnlySpan * ^c -> ParseResult< ^i, ^o, ^e >)> = ^T

[<AutoOpen>]
module Parser =
    let inline (-->) v1 f = f v1
    let inline parse< ^p, ^i, ^o, ^e, ^c when Parser< ^i, ^o, ^e, ^c, ^p >>
        (parser: ^p)
        (input: ^i ReadOnlySpan)
        (ctx: ^c)
        : ParseResult< ^i, ^o, ^e > =
        parser.Parse(input, ctx)
