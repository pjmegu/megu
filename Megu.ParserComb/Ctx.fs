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
    
    static member inline Create(input: System.ReadOnlySpan<^i>) : StdCtx =
        { length = input.Length }

// For PosCtx
type StdCtx with
    member inline this.GetPos(input: System.ReadOnlySpan<^i>) : int =
        this.length - input.Length

[<Struct>]
type StdRecursiveCtx<^e when Error<^e>> =
    {
        length: int
        mutable memoTable: (obj * int * RecursiveResult<^e>) list
    }

type StdRecursiveCtx<^e when Error<^e>> with
    static member inline Create(input: System.ReadOnlySpan<^i>) : StdRecursiveCtx<^e> =
        { length = input.Length; memoTable = [] }
    
    member inline this.GetPos(input: System.ReadOnlySpan<^i>) : int =
        this.length - input.Length

    member inline this.ParseRecursive<^i, ^o> (parser: RecursiveDelegate<^i, ^o, ^e, StdRecursiveCtx<^e>>, input: System.ReadOnlySpan<^i>) : ParseResult<^i, ^o, ^e> =
        let now = this.GetPos input
        let key = List.tryFind (fun (o, pos, _) -> obj.ReferenceEquals(o, parser) && pos = now) this.memoTable
        match key with
        | Some (_, pos,Evaluating) when pos = now ->
            Failure (emitError { msg = Some "Left recursion detected"; pos = Some (this.GetPos input) })
        | Some (_, pos, ResultSuccess (result, endPos)) when pos = now ->
            assert (endPos >= now)
            let remaining = input.Slice (endPos - now)
            Success (result :?> ^o, remaining.Slice 0)
        | Some (_, pos, ResultFailure e) when pos = now ->
            Failure e
        | _ ->
            this.memoTable <- (parser :> obj, now, Evaluating) :: this.memoTable
            let result = parser.Invoke(input, this)
            match result with
            | Success (value, remaining) ->
                let mutable value = value
                let mutable remaining = remaining
                let mutable endPos = this.GetPos remaining
                this.memoTable <- (parser :> obj, now, ResultSuccess (value :> obj, endPos)) :: this.memoTable
                let mutable flag = true

                while flag do
                    let nextResult = parser.Invoke(input, this)
                    match nextResult with
                    | Success (nextValue, nextRemaining) ->
                        let nextEndPos = this.GetPos nextRemaining
                        if nextEndPos > endPos then
                            this.memoTable <- (parser :> obj, now, ResultSuccess (nextValue :> obj, nextEndPos)) :: this.memoTable
                            endPos <- nextEndPos
                            value <- nextValue
                            remaining <- nextRemaining
                        else
                            flag <- false
                    | Failure _ ->
                        flag <- false
                
                Success (value, remaining.Slice 0)
            | Failure e ->
                this.memoTable <- (parser :> obj, now, ResultFailure e) :: this.memoTable
                Failure e