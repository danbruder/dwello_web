module Page.Index exposing
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
import Data.Session exposing (Session, Token)
import Data.User exposing (User)
import Global exposing (Global)
import Html exposing (..)
import RemoteData as RD exposing (RemoteData(..), WebData)
import Request.User
import Route



-- COMMANDS


getUsers : Config -> Token -> Cmd Msg
getUsers config token =
    Request.User.getUsers config token
        |> RD.sendRequest
        |> Cmd.map GetUsersResponse



-- MODEL


type alias Model =
    { users : ApiResponse (List User)
    }


init : Global -> ( Model, Cmd Msg, Global.Msg )
init global =
    ( { users = Loading }
    , getUsers (Global.getConfig global) (Global.getToken global)
    , Global.none
    )



-- UPDATE


type Msg
    = GetUsersResponse (ApiResponse (List User))


update : Global -> Msg -> Model -> ( Model, Cmd Msg, Global.Msg )
update _ msg model =
    case msg of
        GetUsersResponse response ->
            ( { model | users = response }, Cmd.none, Global.none )



-- SUBSCRIPTIONS


subscriptions : Global -> Model -> Sub Msg
subscriptions _ _ =
    Sub.none



-- VIEW


view : Global -> Model -> Document Msg
view _ model =
    { title = "Home"
    , body =
        [ h1 [] [ text "Users" ]
        , a [ Route.href <| Route.Login ]
            [ text "Login" ]
        , a [ Route.href <| Route.Register ]
            [ text "Register" ]
        , viewUsers model
        ]
    }


viewUsers : Model -> Html Msg
viewUsers model =
    case model.users of
        NotAsked ->
            text "Not Asked."

        Loading ->
            text "Loading..."

        Failure error ->
            text "Error"

        Success response ->
            case response of
                Data userList ->
                    ul [] <| List.map viewUser userList

                _ ->
                    text "Other"


viewUser : User -> Html Msg
viewUser user =
    li []
        [ a [ Route.href <| Route.UserDetail { id = user.id |> String.fromInt } ]
            [ String.join " | " [ user.name, user.email ] |> text
            ]
        ]
