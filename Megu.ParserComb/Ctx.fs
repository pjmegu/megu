namespace Megu.ParserComb

type PosCtx< ^i, ^T when ^T: (member GetPos: System.ReadOnlySpan<^i> -> int)> = ^T

[<Struct>]
type EmptyCtx =
    | EmptyCtx

// For PosCtx
type EmptyCtx with
    member inline _.GetPos(_: System.ReadOnlySpan<^i>) : int =
        raise (System.NotSupportedException "EmptyCtx does not support GetPos.")

[<Struct>]
type StdCtx =
    {
        length: int
    }

// For PosCtx
type StdCtx with
    member inline this.GetPos(input: System.ReadOnlySpan<^i>) : int =
        this.length - input.Length