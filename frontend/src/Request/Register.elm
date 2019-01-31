module Request.Register exposing (RegisterInput, encodeRegisterInput, register)

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


register : Config -> Token -> RegisterInput -> Request (ApiData AuthPayload)
register config token input =
    UB.crossOrigin config.api [ "register" ] []
        |> HB.post
        |> HB.withJsonBody (input |> encodeRegisterInput)
        |> HB.withExpect (Http.expectJson decode)
        |> HB.withHeader "X-API-KEY" token
        |> HB.toRequest


decode : JD.Decoder (ApiData AuthPayload)
decode =
    decodeApiResponse decodeAuthPayload


type alias RegisterInput =
    { email : String
    , password : String
    , name : String
    }


encodeRegisterInput : RegisterInput -> JE.Value
encodeRegisterInput v =
    JE.object
        [ ( "email", JE.string v.email )
        , ( "password", JE.string v.password )
        , ( "name", JE.string v.name )
        ]
