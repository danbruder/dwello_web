module Main.Model exposing
    ( Model
    , Page(..)
    , init
    , initPage
    , updatePage
    )

import Browser.Navigation as BN exposing (Key)
import Config exposing (Config)
import Global exposing (Global)
import Main.Msg exposing (Msg(..))
import Page.Index
import Page.Login
import Page.Register
import Page.UserDetail
import Ports exposing (logout)
import Route
import Task
import Url exposing (Url)
import Url.Builder as UB


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
    let
        loggedIn =
            Global.getToken model.global /= ""
    in
    case ( Route.fromUrl url, loggedIn ) of
        -- Non authd routes
        ( Just Route.Login, False ) ->
            Page.Login.init model.global
                |> updatePage Login LoginMsg model

        ( Just Route.Register, False ) ->
            Page.Register.init model.global
                |> updatePage Register RegisterMsg model

        ( Just Route.Login, True ) ->
            ( model, BN.load (UB.absolute [] []) )

        ( Just Route.Register, True ) ->
            ( model, BN.load (UB.absolute [] []) )

        ( Just Route.Logout, _ ) ->
            ( model, logout () )

        -- Redirect to login
        ( _, False ) ->
            ( model, BN.load (UB.absolute [ "login" ] []) )

        -- Auth'd routes
        ( Just Route.Index, True ) ->
            Page.Index.init model.global
                |> updatePage Index IndexMsg model

        ( Just (Route.UserDetail { id }), True ) ->
            Page.UserDetail.init model.global id
                |> updatePage UserDetail UserDetailMsg model

        -- 404
        ( Nothing, _ ) ->
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
