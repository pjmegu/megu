namespace Megu.ParserComb

[<Struct>]
type ErrorInput = { msg: string option; pos: int option }

type Error< ^T when ^T: (static member emit: ErrorInput -> ^T)> = ^T

[<AutoOpen>]
module Error =
    let inline emitError< ^T when Error< ^T >> input : Error< ^T > =
        (^T: (static member emit: ErrorInput -> ^T) input)

[<Struct>]
type EmptyError =
    | EmptyError

    static member emit(_: ErrorInput) : EmptyError = EmptyError

[<Struct>]
type StdError =
    | StdError of string

    static member emit(input: ErrorInput) : StdError =
        match input.msg, input.pos with
        | Some msg, Some pos -> StdError(sprintf "%s at position %d" msg pos)
        | Some msg, None -> StdError msg
        | None, Some pos -> StdError(sprintf "Error at position %d" pos)
        | None, None -> StdError "Unknown error"
