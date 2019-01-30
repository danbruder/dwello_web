module Page.Register exposing
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
import Request.Register exposing (RegisterInput, encodeRegisterInput)
import Route



-- COMMANDS


login : Config -> Token -> Maybe JE.Value -> Cmd Msg
login config token input =
    Request.Register.register config token input
        |> RD.sendRequest
        |> Cmd.map GotRegister



-- MODEL


type alias Model =
    { email : String
    , password : String
    , name : String
    , registration : ApiResponse AuthPayload
    }


init : Global -> ( Model, Cmd Msg, Global.Msg )
init global =
    ( Model "" "" "" NotAsked
    , Cmd.none
    , Global.none
    )



-- UPDATE


type Msg
    = Email String
    | Password String
    | Name String
    | SubmitForm
    | GotRegister (ApiResponse AuthPayload)


update : Global -> Msg -> Model -> ( Model, Cmd Msg, Global.Msg )
update global msg model =
    case msg of
        SubmitForm ->
            let
                input =
                    RegisterInput model.email model.password model.name |> encodeRegisterInput

                config =
                    Global.getConfig global

                token =
                    Global.getToken global
            in
            ( { model | registration = Loading }, login config token (Just input), Global.none )

        GotRegister response ->
            case response of
                Success r ->
                    case r of
                        Data { token } ->
                            ( { model | registration = response }, Cmd.none, SetSession (Session token) )

                        _ ->
                            ( { model | registration = response }, Cmd.none, Global.none )

                _ ->
                    ( { model | registration = response }, Cmd.none, Global.none )

        Email v ->
            ( { model | email = v }, Cmd.none, Global.none )

        Password v ->
            ( { model | password = v }, Cmd.none, Global.none )

        Name v ->
            ( { model | name = v }, Cmd.none, Global.none )



-- SUBSCRIPTIONS


subscriptions : Global -> Model -> Sub Msg
subscriptions _ _ =
    Sub.none



-- VIEW


view : Global -> Model -> Document Msg
view _ model =
    { title = "Register"
    , body =
        [ h1 [] [ text "Register" ]
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
                        , viewRegisterForm model
                        ]

                _ ->
                    text "other"

        NotAsked ->
            viewRegisterForm model

        Loading ->
            text "loading"

        _ ->
            viewRegisterForm model


viewRegisterForm : Model -> Html Msg
viewRegisterForm model =
    Html.form [ onSubmit SubmitForm ]
        [ viewInput model "text" "Name" model.name "name" Name
        , viewInput model "text" "Email" model.email "email" Email
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
