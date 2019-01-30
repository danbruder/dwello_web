module Main.View exposing (view)

import Browser exposing (Document)
import Html exposing (..)
import Main.Model exposing (Model, Page(..))
import Main.Msg exposing (Msg(..))
import Page.Index
import Page.Login
import Page.Register
import Page.UserDetail


view : Model -> Document Msg
view model =
    let
        viewPage toMsg pageModel pageView =
            let
                { title, body } =
                    pageView model.global pageModel
            in
            { title = title
            , body = List.map (Html.map toMsg) body
            }
    in
    case model.page of
        Index indexModel ->
            viewPage IndexMsg indexModel Page.Index.view

        Login lmodel ->
            viewPage LoginMsg lmodel Page.Login.view

        Register lmodel ->
            viewPage RegisterMsg lmodel Page.Register.view

        UserDetail lmodel ->
            viewPage UserDetailMsg lmodel Page.UserDetail.view

        NotFound ->
            { title = "Not Found"
            , body = [ text ":(" ]
            }
