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
import Data.Session exposing (Token)
import Json.Encode as JE
import Ports exposing (setToken)
import Task
import Time exposing (Posix)



-- MODEL


type alias Model =
    { config : Config
    , time : Posix
    , key : Key
    }


type Global
    = Global Model


init : Config -> Key -> ( Global, Cmd Msg )
init config key =
    ( Global
        { config = config
        , time = Time.millisToPosix 0
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
    | SetToken Token
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

        SetToken token ->
            let
                config =
                    model.config

                c =
                    { config
                        | token = token
                    }

                m =
                    { model | config = c }
            in
            ( toGlobal m, saveToken token )

        NoOp ->
            set model



-- SUBSCRIPTIONS


subscriptions : Global -> Sub Msg
subscriptions global =
    Sub.none


saveToken : Token -> Cmd Msg
saveToken token =
    setToken (JE.string token)



-- PUBLIC GETTERS


getConfig : Global -> Config
getConfig =
    toModel >> .config


getToken : Global -> String
getToken =
    getConfig >> .token


getTime : Global -> Posix
getTime =
    toModel >> .time


getKey : Global -> Key
getKey =
    toModel >> .key
