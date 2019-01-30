module Data.User exposing (User, userDecoder)

import Json.Decode as JD exposing (Decoder)
import Json.Decode.Pipeline as JDP


type alias UserId =
    Int


type alias User =
    { id : UserId
    , name : String
    , email : String
    }


userDecoder : Decoder User
userDecoder =
    JD.succeed User
        |> JDP.required "id" JD.int
        |> JDP.required "name" JD.string
        |> JDP.required "email" JD.string
