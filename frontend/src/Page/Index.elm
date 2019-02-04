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
import Data.Session exposing (Session)
import Data.User exposing (User)
import Global exposing (Global)
import Html exposing (..)
import Html.Attributes exposing (class, href, src, title, type_)
import RemoteData as RD exposing (RemoteData(..), WebData)
import Request.User
import Route exposing (Route)



-- COMMANDS


getUsers : Config -> Cmd Msg
getUsers config =
    Request.User.getUsers config
        |> RD.sendRequest
        |> Cmd.map GetUsersResponse



-- MODEL


type alias Model =
    { users : ApiResponse (List User)
    }


init : Global -> ( Model, Cmd Msg, Global.Msg )
init global =
    ( { users = Loading }
    , getUsers (Global.getConfig global)
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
        , viewContent model
        ]
    }


viewContent : Model -> Html Msg
viewContent model =
    case model.users of
        NotAsked ->
            div [] []

        Loading ->
            text "Loading..."

        Failure error ->
            text "Error"

        Success response ->
            case response of
                Data userList ->
                    div [] <| List.map viewUser userList

                _ ->
                    div [] []


viewUser : User -> Html Msg
viewUser user =
    article [ class "dt w-100 bb b--black-05 pb2 mt2" ]
        [ a [ class "link", Route.href <| Route.UserDetail { id = user.id |> String.fromInt } ]
            [ div [ class "dtc v-mid  " ]
                [ h1 [ class "f6 f5-ns fw6 lh-title black mv0" ]
                    [ text user.name
                    ]
                , h2 [ class "f6 fw4 mt0 mb0 black-60" ]
                    [ text user.email ]
                ]
            ]
        ]
