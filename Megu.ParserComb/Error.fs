namespace Megu.ParserComb

[<Struct>]
type StdError =
    | StdError of string

    static member emit(message: string) : StdError = StdError message
    static member emitWithPos(message: string, pos: int) : StdError =
        StdError (sprintf "%s at position %d" message pos)
