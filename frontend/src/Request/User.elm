module Request.User exposing (CreateUserInput, ProfileInput, Role(..), createProfile, createUser, decodeAllUsersResponse, encodeCreateUserInput, encodeProfileInput, encodeRole, encodeRoles, getProfile, getUser, getUsers, updateProfile)

import Api exposing (ApiData, decodeApiResponse)
import Config exposing (Config)
import Data.User as User exposing (Profile, User)
import Http exposing (Request)
import HttpBuilder as HB
import Json.Decode as JD
import Json.Decode.Pipeline as JDP
import Json.Encode as JE
import Url.Builder as UB


getUser : Config -> String -> Request (ApiData User)
getUser config id =
    UB.crossOrigin config.api [ "users", id ] []
        |> HB.get
        |> HB.withExpect (Http.expectJson (decodeApiResponse User.userDecoder))
        |> HB.withHeader "X-API-KEY" config.token
        |> HB.toRequest


getProfile : Config -> String -> Request (ApiData Profile)
getProfile config id =
    UB.crossOrigin config.api [ "users", id, "profile" ] []
        |> HB.get
        |> HB.withExpect (Http.expectJson (decodeApiResponse User.profileDecoder))
        |> HB.withHeader "X-API-KEY" config.token
        |> HB.toRequest


createProfile : Config -> String -> ProfileInput -> Request (ApiData Profile)
createProfile config id input =
    UB.crossOrigin config.api [ "users", id, "profile" ] []
        |> HB.post
        |> HB.withExpect (Http.expectJson (decodeApiResponse User.profileDecoder))
        |> HB.withJsonBody (encodeProfileInput input)
        |> HB.withHeader "X-API-KEY" config.token
        |> HB.toRequest


updateProfile : Config -> String -> ProfileInput -> Request (ApiData Profile)
updateProfile config id input =
    UB.crossOrigin config.api [ "users", id, "profile" ] []
        |> HB.put
        |> HB.withExpect (Http.expectJson (decodeApiResponse User.profileDecoder))
        |> HB.withJsonBody (encodeProfileInput input)
        |> HB.withHeader "X-API-KEY" config.token
        |> HB.toRequest


createUser : Config -> CreateUserInput -> Request (ApiData User)
createUser config input =
    UB.crossOrigin config.api [ "users" ] []
        |> HB.post
        |> HB.withExpect (Http.expectJson (decodeApiResponse User.userDecoder))
        |> HB.withJsonBody (encodeCreateUserInput input)
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


type alias CreateUserInput =
    { name : String
    , email : String
    , password : String
    , roles : List Role
    }


type alias ProfileInput =
    { title : String
    , intro : String
    , body : String
    }


type Role
    = Anonymous
    | Authenticated
    | Admin


encodeCreateUserInput : CreateUserInput -> JE.Value
encodeCreateUserInput v =
    JE.object
        [ ( "name", JE.string v.name )
        , ( "email", JE.string v.email )
        , ( "password", JE.string v.password )
        , ( "roles", encodeRoles v.roles )
        ]


encodeRoles : List Role -> JE.Value
encodeRoles v =
    JE.list (\a -> encodeRole a) v


encodeRole : Role -> JE.Value
encodeRole v =
    let
        r =
            case v of
                Anonymous ->
                    "Anonymous"

                Authenticated ->
                    "Authenticated"

                Admin ->
                    "Admin"
    in
    JE.string r


encodeProfileInput : ProfileInput -> JE.Value
encodeProfileInput v =
    JE.object
        [ ( "title", JE.string v.title )
        , ( "intro", JE.string v.intro )
        , ( "body", JE.string v.body )
        ]
