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
import Html.Attributes exposing (attribute, class, href, src, title, type_)
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
        [ div [ class "container mx-auto " ]
            [ h1 [] [ text "Users" ]
            , viewContent model
            ]
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
                    viewUserTable userList

                _ ->
                    div [] []


viewUserTable : List User -> Html Msg
viewUserTable users =
    div [ class "pt-5 border-b border-grey-light overflow-hidden relative" ]
        [ div [ class " overflow-y-auto scrollbar-w-2 scrollbar-track-grey-lighter scrollbar-thumb-rounded scrollbar-thumb-grey scrolling-touch" ]
            [ table [ class "w-full text-left table-collapse" ]
                [ thead []
                    [ tr []
                        [ th [ class "text-sm font-semibold text-grey-darker p-2" ]
                            [ text "Name" ]
                        , th [ class "text-sm font-semibold text-grey-darker p-2" ]
                            [ text "Email" ]
                        ]
                    ]
                , tbody [ class "align-baseline" ]
                    (List.map
                        viewUser
                        users
                    )
                ]
            ]
        ]


viewUser : User -> Html Msg
viewUser user =
    tr []
        [ td [ class "p-2 border-t border-grey-light  text-xs text-purple-dark whitespace-no-wrap" ]
            [ a [ class "link", Route.href <| Route.UserDetail { id = user.id |> String.fromInt } ]
                [ text user.name
                ]
            ]
        , td [ class "p-2 border-t border-grey-light  text-xs text-blue-dark whitespace-pre" ]
            [ text user.email ]
        ]
