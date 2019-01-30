module Api exposing (ApiData(..), ApiResponse, ValidationError, decodeApiResponse)

import Json.Decode as JD
import Json.Decode.Pipeline as JDP
import RemoteData exposing (WebData)


type alias ApiResponse a =
    WebData (ApiData a)


type ApiData a
    = Data a
    | ValidationErrors (List ValidationError)
    | ServerError String


type alias ValidationError =
    { field : String
    , message : String
    }


decodeApiResponse : JD.Decoder a -> JD.Decoder (ApiData a)
decodeApiResponse decoder =
    let
        dec =
            JD.field "data" decoder
    in
    JD.field "success" JD.bool
        |> JD.andThen
            (\success ->
                case success of
                    True ->
                        JD.map Data dec

                    False ->
                        JD.map ValidationErrors decodeValidationErrors
            )


decodeValidationErrors =
    let
        dec =
            JD.succeed ValidationError
                |> JDP.required "field" JD.string
                |> JDP.required "message" JD.string
    in
    JD.field "validation_errors" (JD.list dec)
