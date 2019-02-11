module View exposing (Toast(..), toastFromApiResponse, toastFromHttpError, viewToast)

import Api exposing (ApiResponse)
import Html exposing (Html, div, text)
import Html.Attributes exposing (class)
import Http
import RemoteData exposing (RemoteData(..))


type Toast
    = Empty
    | Warn String
    | Bad String
    | Good String


viewToast : Toast -> Html msg
viewToast toast =
    case toast of
        Empty ->
            text ""

        Warn t ->
            div [ class "fixed pin-l pin-b w-full z-10 bg-yellow-lighter p-2 text-xs" ]
                [ text t
                ]

        Bad t ->
            div [ class "fixed pin-l pin-b w-full z-10 bg-red-lighter p-2 text-xs" ]
                [ text t
                ]

        Good t ->
            div [ class "fixed pin-l pin-b w-full z-10 bg-green-lighter p-2 text-xs" ]
                [ text t
                ]


toastFromHttpError : Http.Error -> Toast
toastFromHttpError err =
    case err of
        Http.NetworkError ->
            Bad "Could not connect to server"

        Http.Timeout ->
            Bad "Could not connect to server"

        _ ->
            Bad "Server error"


toastFromApiResponse : ApiResponse a -> Toast
toastFromApiResponse response =
    case response of
        Failure f ->
            toastFromHttpError f

        _ ->
            Empty
