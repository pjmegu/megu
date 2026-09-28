namespace Megu.ParserComb

type PosCtx< ^T when ^T: (member IncrementPos: unit -> unit) and ^T: (member GetPos: unit -> int)> = ^T

[<Struct>]
type EmptyCtx =
    | EmptyCtx

// For PosCtx
type EmptyCtx with
    member inline _.IncrementPos() : unit = ()
    member inline _.GetPos() : int = -1
