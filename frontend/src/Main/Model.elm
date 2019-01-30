module Main.Model exposing
    ( Model
    , Page(..)
    , init
    , initPage
    , updatePage
    )

import Browser.Navigation exposing (Key)
import Config exposing (Config)
import Global exposing (Global)
import Main.Msg exposing (Msg(..))
import Page.Index
import Page.Login
import Page.Register
import Page.UserDetail
import Route
import Url exposing (Url)


type Page
    = Index Page.Index.Model
    | Login Page.Login.Model
    | Register Page.Register.Model
    | UserDetail Page.UserDetail.Model
    | NotFound


type alias Model =
    { page : Page
    , global : Global
    }


init : Config -> Url -> Key -> ( Model, Cmd Msg )
init config url key =
    let
        ( global, globalCmd ) =
            Global.init config key

        ( model, cmd ) =
            initPage url
                { page = NotFound
                , global = global
                }
    in
    ( model
    , Cmd.batch
        [ cmd
        , Cmd.map GlobalMsg globalCmd
        ]
    )


initPage : Url -> Model -> ( Model, Cmd Msg )
initPage url model =
    case Route.fromUrl url of
        Just Route.Index ->
            Page.Index.init model.global
                |> updatePage Index IndexMsg model

        Just Route.Login ->
            Page.Login.init model.global
                |> updatePage Login LoginMsg model

        Just Route.Register ->
            Page.Register.init model.global
                |> updatePage Register RegisterMsg model

        Just (Route.UserDetail { id }) ->
            Page.UserDetail.init model.global id
                |> updatePage UserDetail UserDetailMsg model

        Nothing ->
            ( { model | page = NotFound }
            , Cmd.none
            )


updatePage :
    (pageModel -> Page)
    -> (pageMsg -> Msg)
    -> Model
    -> ( pageModel, Cmd pageMsg, Global.Msg )
    -> ( Model, Cmd Msg )
updatePage toPage toMsg model ( pageModel, pageCmd, globalMsg ) =
    let
        ( global, globalCmd ) =
            Global.update globalMsg model.global
    in
    ( { model | page = toPage pageModel, global = global }
    , Cmd.batch
        [ Cmd.map toMsg pageCmd
        , Cmd.map GlobalMsg globalCmd
        ]
    )
