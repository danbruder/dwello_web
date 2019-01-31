module Request.User exposing (getUser, getUsers)

import Api exposing (ApiData, decodeApiResponse)
import Config exposing (Config)
import Data.User as User exposing (User)
import Http exposing (Request)
import HttpBuilder as HB
import Json.Decode as JD
import Json.Decode.Pipeline as JDP
import Url.Builder as UB


getUser : Config -> String -> Request (ApiData User)
getUser config id =
    UB.crossOrigin config.api [ "users", id ] []
        |> HB.get
        |> HB.withExpect (Http.expectJson (decodeApiResponse User.userDecoder))
        |> HB.withHeader "X-API-KEY" config.token
        |> HB.toRequest


getUsers : Config -> Request (ApiData (List User))
getUsers config =
    UB.crossOrigin config.api [ "users" ] []
        |> HB.get
        |> HB.withExpect (Http.expectJson decodeAllUsersResponse)
        |> HB.withHeader "X-API-KEY" config.token
        |> HB.toRequest


decodeAllUsersResponse : JD.Decoder (ApiData (List User))
decodeAllUsersResponse =
    decodeApiResponse (JD.list User.userDecoder)
