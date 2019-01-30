module Main.Msg exposing (Msg(..))

import Browser exposing (UrlRequest)
import Global
import Page.Index
import Page.Login
import Page.Register
import Page.UserDetail
import Url exposing (Url)


type Msg
    = UrlRequest UrlRequest
    | UrlChange Url
    | GlobalMsg Global.Msg
    | IndexMsg Page.Index.Msg
    | LoginMsg Page.Login.Msg
    | RegisterMsg Page.Register.Msg
    | UserDetailMsg Page.UserDetail.Msg
