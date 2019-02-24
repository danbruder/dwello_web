module Page.UserProfileForm exposing
    ( Model
    , Msg(..)
    , init
    , subscriptions
    , update
    , view
    )

import Api exposing (ApiData(..), ApiResponse)
import Browser exposing (Document)
import Browser.Navigation as BN
import Config exposing (Config)
import Data.AuthPayload exposing (AuthPayload)
import Data.Session exposing (Session, Token)
import Data.User exposing (User)
import Dict exposing (Dict)
import Global exposing (Global, Msg(..))
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (onInput, onSubmit)
import Http
import Json.Encode as JE
import RemoteData as RD exposing (RemoteData(..))
import Request.Login exposing (LoginInput, encodeLoginInput)
import Route
import Task
import Url.Builder as UB
import View exposing (Toast(..), viewToast)



-- COMMANDS
--
-- login : Config -> Token -> LoginInput -> Cmd Msg
-- login config token input =
--     Request.Login.login config token input
--         |> RD.sendRequest
--         |> Cmd.map GotLogin
--
-- MODEL


type alias Model =
    { id : String
    , toast : Toast
    }


init : Global -> String -> ( Model, Cmd Msg, Global.Msg )
init global id =
    ( Model id Empty
    , Cmd.none
    , Global.none
    )



-- UPDATE


type Msg
    = NoOp


update : Global -> Msg -> Model -> ( Model, Cmd Msg, Global.Msg )
update global msg model =
    ( model, Cmd.none, Global.none )



-- SUBSCRIPTIONS


subscriptions : Global -> Model -> Sub Msg
subscriptions _ _ =
    Sub.none



-- VIEW


view : Global -> Model -> Document Msg
view global model =
    { title = "Edit User " ++ model.id ++ " Profile"
    , body =
        [ div [ class "container mx-auto" ]
            [ text "edit" ]
        , viewToast model.toast
        ]
    }


viewInput :
    Model
    -> String
    -> String
    -> String
    -> String
    -> (String -> msg)
    -> Html msg
viewInput model t pl v field toMsg =
    div [ class "mb-4" ]
        [ label [ class "block text-grey-darker text-sm font-bold mb-2", for field ]
            [ text pl ]
        , input
            [ class "shadow appearance-none border  rounded w-full py-2 px-3 text-grey-darker  leading-tight focus:outline-none focus:shadow-outline"
            , classList [ ( "border-red", hasValidationErrors model field ) ]
            , id field
            , placeholder pl
            , type_ t
            , value v
            , onInput toMsg
            ]
            []
        , formatValidationErrors model field
        ]


getValidationErrors : Model -> String -> String
getValidationErrors model f =
    ""



-- case model.registration of
--     Success d ->
--         case d of
--             ValidationErrors validationErrors ->
--                 validationErrors
--                     |> List.filter (\{ field } -> field == f)
--                     |> List.map .message
--                     |> String.join ", "
--
--             _ ->
--                 ""
--
--     _ ->
--         ""
--


formatValidationErrors : Model -> String -> Html msg
formatValidationErrors model field =
    let
        errs =
            getValidationErrors model field
    in
    if errs /= "" then
        p [ class "text-red text-xs italic pt-2" ]
            [ text errs ]

    else
        text ""


hasValidationErrors : Model -> String -> Bool
hasValidationErrors model field =
    getValidationErrors model field /= ""
