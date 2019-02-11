module Page.Login exposing
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


login : Config -> Token -> LoginInput -> Cmd Msg
login config token input =
    Request.Login.login config token input
        |> RD.sendRequest
        |> Cmd.map GotLogin



-- MODEL


type alias Model =
    { email : String
    , password : String
    , registration : ApiResponse AuthPayload
    , toast : Toast
    }


init : Global -> ( Model, Cmd Msg, Global.Msg )
init global =
    ( Model "" "" NotAsked Empty
    , Cmd.none
    , Global.none
    )



-- UPDATE


type Msg
    = Email String
    | Password String
    | SubmitForm
    | GotLogin (ApiResponse AuthPayload)


update : Global -> Msg -> Model -> ( Model, Cmd Msg, Global.Msg )
update global msg model =
    case msg of
        SubmitForm ->
            let
                input =
                    LoginInput model.email model.password

                config =
                    Global.getConfig global

                token =
                    Global.getToken global
            in
            ( { model | registration = Loading }, login config token input, Global.none )

        GotLogin response ->
            case response of
                Success r ->
                    case r of
                        Data { token } ->
                            ( { model | registration = response, toast = Good "Success" }, BN.pushUrl (Global.getKey global) (Route.toString Route.Index), SetToken token )

                        ValidationErrors _ ->
                            ( { model | registration = response }, Cmd.none, Global.none )

                        ServerError err ->
                            ( { model | registration = response, toast = Bad err }, Cmd.none, Global.none )

                Failure err ->
                    case err of
                        Http.NetworkError ->
                            ( { model | registration = response, toast = Bad "Could not connect to server" }, Cmd.none, Global.none )

                        Http.Timeout ->
                            ( { model | registration = response, toast = Bad "Could not connect to server" }, Cmd.none, Global.none )

                        _ ->
                            ( { model | registration = response, toast = Bad "Server error" }, Cmd.none, Global.none )

                _ ->
                    ( model, Cmd.none, Global.none )

        Email v ->
            ( { model | email = v, toast = Empty }, Cmd.none, Global.none )

        Password v ->
            ( { model | password = v, toast = Empty }, Cmd.none, Global.none )



-- SUBSCRIPTIONS


subscriptions : Global -> Model -> Sub Msg
subscriptions _ _ =
    Sub.none



-- VIEW


view : Global -> Model -> Document Msg
view _ model =
    { title = "Login"
    , body =
        [ div [ class "bg-grey-lighter h-full" ]
            [ viewContent model
            , viewToast model.toast
            ]
        ]
    }


viewContent : Model -> Html Msg
viewContent model =
    viewLoginForm model


viewLoginForm : Model -> Html Msg
viewLoginForm model =
    let
        submit =
            case isLoading model of
                True ->
                    div [ class "spinner ml-8" ] []

                False ->
                    input [ class "bg-indigo hover:bg-indigo-dark text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline", type_ "submit", value "Submit" ] []
    in
    div [ class "flex justify-center items-center h-full" ]
        [ div [ class "w-full max-w-xs" ]
            [ Html.form [ class "bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4", onSubmit SubmitForm ]
                [ h1 [ class "text-grey-darkest text-lg pb-4" ] [ text "Login" ]
                , viewInput model "text" "Email" model.email "email" Email
                , viewInput model "password" "Password" model.password "password" Password
                , div [ class "pt-6 flex items-center justify-between" ]
                    [ submit
                    , a [ class "inline-block align-baseline font-bold text-sm text-indigo hover:text-indigo-darker", href "#" ]
                        [ text "Forgot Password?      " ]
                    ]
                ]
            , p [ class "text-center text-grey text-xs" ]
                [ text "©2019 Dwello. All rights reserved.  " ]
            ]
        ]


isLoading : Model -> Bool
isLoading model =
    case model.registration of
        Loading ->
            True

        _ ->
            False


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
    case model.registration of
        Success d ->
            case d of
                ValidationErrors validationErrors ->
                    validationErrors
                        |> List.filter (\{ field } -> field == f)
                        |> List.map .message
                        |> String.join ", "

                _ ->
                    ""

        _ ->
            ""


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
