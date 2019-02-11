module Util exposing (updateInPlace)


updateInPlace : List { a | id : Int } -> { a | id : Int } -> List { a | id : Int }
updateInPlace aList a =
    let
        f =
            \b ->
                if b.id == a.id then
                    a

                else
                    b
    in
    List.map f aList
