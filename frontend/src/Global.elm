module Global exposing
    ( Global
    , Msg(..)
    , getConfig
    , getKey
    , getTime
    , getToken
    , init
    , none
    , update
    )

import Browser.Navigation exposing (Key)
import Config exposing (Config)
import Data.Session exposing (Session)
import Json.Encode as JE
import Ports exposing (setToken)
import Task
import Time exposing (Posix)



-- MODEL


type alias Model =
    { config : Config
    , time : Posix
    , session : Maybe Session
    , key : Key
    }


type Global
    = Global Model


init : Config -> Key -> ( Global, Cmd Msg )
init config key =
    ( Global
        { config = config
        , time = Time.millisToPosix 0
        , session = Maybe.map (\token -> Session token) config.token
        , key = key
        }
    , Task.perform SetTime Time.now
    )


toModel : Global -> Model
toModel (Global model) =
    model


toGlobal : Model -> Global
toGlobal model =
    Global model



-- UPDATE


type Msg
    = SetTime Posix
    | SetSession Session
    | NoOp


none : Msg
none =
    NoOp


update : Msg -> Global -> ( Global, Cmd Msg )
update msg (Global model) =
    let
        set nextModel =
            ( toGlobal nextModel, Cmd.none )
    in
    case msg of
        SetTime time ->
            set { model | time = time }

        SetSession session ->
            let
                m =
                    { model | session = Just session }
            in
            ( toGlobal m, saveToken session )

        NoOp ->
            set model



-- SUBSCRIPTIONS


subscriptions : Global -> Sub Msg
subscriptions global =
    Sub.none


saveToken : Session -> Cmd Msg
saveToken session =
    setToken (JE.string session.token)



-- PUBLIC GETTERS


getConfig : Global -> Config
getConfig =
    toModel >> .config


getSession : Global -> Maybe Session
getSession =
    toModel >> .session


getToken : Global -> String
getToken global =
    case getSession global of
        Just session ->
            session.token

        Nothing ->
            ""


getTime : Global -> Posix
getTime =
    toModel >> .time


getKey : Global -> Key
getKey =
    toModel >> .key
