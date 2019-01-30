module Route exposing
    ( Route(..)
    , fromUrl
    , href
    , newUrl
    , parser
    , toString
    )

import Browser.Navigation as BN exposing (Key)
import Html exposing (Attribute)
import Html.Attributes
import Url exposing (Url)
import Url.Builder as UB
import Url.Parser as UP exposing ((</>), Parser)


type Route
    = Index
    | Login
    | Register
    | UserDetail { id : String }


parser : Parser (Route -> a) a
parser =
    UP.oneOf
        [ UP.map Index <| UP.top
        , UP.map Login <| UP.s "login"
        , UP.map Register <| UP.s "register"
        , UP.map (\id -> UserDetail { id = id }) <| UP.s "user" </> UP.string
        ]


toString : Route -> String
toString route =
    case route of
        Index ->
            UB.absolute [] []

        Login ->
            UB.absolute [ "login" ] []

        Register ->
            UB.absolute [ "register" ] []

        UserDetail { id } ->
            UB.absolute [ "user", id ] []


newUrl : Key -> Route -> Cmd msg
newUrl key =
    toString >> BN.pushUrl key


fromUrl : Url -> Maybe Route
fromUrl url =
    UP.parse parser url


href : Route -> Attribute msg
href route =
    Html.Attributes.href <| toString route
