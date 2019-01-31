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
import Config exposing (Config)
import Data.AuthPayload exposing (AuthPayload)
import Data.Session exposing (Session, Token)
import Data.User exposing (User)
import Dict exposing (Dict)
import Global exposing (Global, Msg(..))
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (onInput, onSubmit)
import Json.Encode as JE
import RemoteData as RD exposing (RemoteData(..))
import Request.Login exposing (LoginInput, encodeLoginInput)
import Route



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
    }


init : Global -> ( Model, Cmd Msg, Global.Msg )
init global =
    ( Model "" "" NotAsked
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
                            ( { model | registration = response }, Cmd.none, SetToken token )

                        _ ->
                            ( { model | registration = response }, Cmd.none, Global.none )

                _ ->
                    ( { model | registration = response }, Cmd.none, Global.none )

        Email v ->
            ( { model | email = v }, Cmd.none, Global.none )

        Password v ->
            ( { model | password = v }, Cmd.none, Global.none )



-- SUBSCRIPTIONS


subscriptions : Global -> Model -> Sub Msg
subscriptions _ _ =
    Sub.none



-- VIEW


view : Global -> Model -> Document Msg
view _ model =
    { title = "Login"
    , body =
        [ h1 [] [ text "Login" ]
        , a [ Route.href <| Route.Index ]
            [ text "Home" ]
        , viewContent model
        ]
    }


viewContent : Model -> Html Msg
viewContent model =
    case model.registration of
        Success v ->
            case v of
                Data { token } ->
                    div []
                        [ text "Data received!"
                        , text token
                        ]

                ValidationErrors errors ->
                    div
                        []
                        [ text "Data and an error received!"
                        , viewLoginForm model
                        ]

                _ ->
                    text "other"

        NotAsked ->
            viewLoginForm model

        Loading ->
            text "loading"

        _ ->
            viewLoginForm model


viewLoginForm : Model -> Html Msg
viewLoginForm model =
    Html.form [ onSubmit SubmitForm ]
        [ viewInput model "text" "Email" model.email "email" Email
        , viewInput model "password" "Password" model.password "password" Password
        , input [ type_ "submit", value "Submit" ] []
        ]


viewInput :
    Model
    -> String
    -> String
    -> String
    -> String
    -> (String -> msg)
    -> Html msg
viewInput model t p v field toMsg =
    div []
        [ label []
            [ text p ]
        , input
            [ type_ t, placeholder p, value v, onInput toMsg ]
            []
        , getValidationErrors model field |> text
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
                        |> String.join "\n"

                _ ->
                    ""

        _ ->
            ""
