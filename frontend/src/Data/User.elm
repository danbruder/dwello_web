module Data.User exposing (Profile, User, profileDecoder, userDecoder)

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


type alias Profile =
    { id : Int
    , uid : UserId
    , title : String
    , intro : String
    , body : String
    }


profileDecoder : Decoder Profile
profileDecoder =
    JD.map5 Profile
        (JD.field "id" JD.int)
        (JD.field "uid" JD.int)
        (JD.field "title" JD.string)
        (JD.field "intro" JD.string)
        (JD.field "body" JD.string)
