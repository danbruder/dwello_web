module Data.AuthPayload exposing (AuthPayload, decodeAuthPayload)

import Data.Session exposing (Token)
import Data.User exposing (User, userDecoder)
import Json.Decode as JD exposing (Decoder)
import Json.Decode.Pipeline as JDP


type alias AuthPayload =
    { token : Token
    , user : User
    }


decodeAuthPayload : Decoder AuthPayload
decodeAuthPayload =
    JD.succeed AuthPayload
        |> JDP.required "token" JD.string
        |> JDP.required "user" userDecoder
