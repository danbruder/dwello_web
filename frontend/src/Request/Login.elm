module Request.Login exposing (LoginInput, encodeLoginInput, login)

import Api exposing (ApiData, decodeApiResponse)
import Config exposing (Config)
import Data.AuthPayload exposing (AuthPayload, decodeAuthPayload)
import Data.Session exposing (Token)
import Data.User as User exposing (User)
import Http exposing (Request)
import HttpBuilder as HB
import Json.Decode as JD
import Json.Decode.Pipeline as JDP
import Json.Encode as JE
import Url.Builder as UB


login : Config -> Token -> Maybe JE.Value -> Request (ApiData AuthPayload)
login config token input =
    UB.crossOrigin config.api [ "login" ] []
        |> HB.post
        |> HB.withJsonBody (input |> Maybe.withDefault JE.null)
        |> HB.withExpect (Http.expectJson decode)
        |> HB.withHeader "X-API-KEY" token
        |> HB.toRequest


decode : JD.Decoder (ApiData AuthPayload)
decode =
    decodeApiResponse decodeAuthPayload


type alias LoginInput =
    { email : String
    , password : String
    }


encodeLoginInput : LoginInput -> JE.Value
encodeLoginInput v =
    JE.object
        [ ( "email", JE.string v.email )
        , ( "password", JE.string v.password )
        ]
