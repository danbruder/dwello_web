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
import Html.Attributes exposing (attribute, class, href, src, style, title, type_)
import Html.Events exposing (onClick)
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
    , newUserModalOpen : Bool
    }


init : Global -> ( Model, Cmd Msg, Global.Msg )
init global =
    ( { users = Loading, newUserModalOpen = False }
    , getUsers (Global.getConfig global)
    , Global.none
    )



-- UPDATE


type Msg
    = GetUsersResponse (ApiResponse (List User))
    | CreateUser
    | OpenNewUserModal
    | CloseNewUserModal


update : Global -> Msg -> Model -> ( Model, Cmd Msg, Global.Msg )
update _ msg model =
    case msg of
        GetUsersResponse response ->
            ( { model | users = response }, Cmd.none, Global.none )

        CreateUser ->
            ( { model | newUserModalOpen = False }, Cmd.none, Global.none )

        OpenNewUserModal ->
            ( { model | newUserModalOpen = True }, Cmd.none, Global.none )

        CloseNewUserModal ->
            ( { model | newUserModalOpen = False }, Cmd.none, Global.none )



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
    let
        users =
            case model.users of
                Loading ->
                    div [ class "spinner flex justify-center w-full h-16" ] []

                Failure error ->
                    text "Something went wrong..."

                Success (Data userList) ->
                    viewUserTable userList

                _ ->
                    div [] []
    in
    div [] [ users, viewNewUserModal model ]


viewUserTable : List User -> Html Msg
viewUserTable users =
    div [ class "bg-white shadow-md rounded my-8 p-6 " ]
        [ button [ onClick OpenNewUserModal, class "bg-indigo border-indigo text-white px-4 py-2 mb-2 rounded hover:bg-indigo-light" ] [ text "New User" ]
        , div [ class " border-b border-grey-light overflow-hidden relative" ]
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
        ]


viewNewUserModal : Model -> Html Msg
viewNewUserModal model =
    modal
        (ModalConfig
            "New User"
            (div [] [ text "body" ])
            CreateUser
            CloseNewUserModal
            "Save"
            model.newUserModalOpen
        )


type alias ModalConfig =
    { title : String
    , body : Html Msg
    , onSubmit : Msg
    , onClose : Msg
    , submitText : String
    , isOpen : Bool
    }


modal : ModalConfig -> Html Msg
modal config =
    let
        content =
            div [ class "fixed pin z-50 overflow-auto bg-smoke-light flex", style "background-color" "rgba(0, 0, 0, 0.4)" ]
                [ div [ class "relative p-8 bg-white w-full max-w-md m-auto flex-col flex" ]
                    [ h3 [] [ text config.title ]
                    , config.body
                    , div []
                        [ button [ onClick config.onSubmit ] [ text config.submitText ]
                        , button [ onClick config.onClose ] [ text "close" ]
                        ]
                    ]
                ]
    in
    case config.isOpen of
        True ->
            content

        False ->
            div [] []


viewUser : User -> Html Msg
viewUser user =
    tr []
        [ td [ class "p-2 border-t border-grey-light text-s  whitespace-no-wrap" ]
            [ a [ class "no-underline text-indigo", Route.href <| Route.UserDetail { id = user.id |> String.fromInt } ]
                [ text user.name
                ]
            ]
        , td [ class "p-2 border-t border-grey-light  text-s  whitespace-pre" ]
            [ text user.email ]
        ]
