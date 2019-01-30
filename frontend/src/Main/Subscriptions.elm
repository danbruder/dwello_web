module Main.Subscriptions exposing (subscriptions)

import Main.Model exposing (Model, Page(..))
import Main.Msg exposing (Msg(..))
import Page.Index
import Page.Login
import Page.Register
import Page.UserDetail


subscriptions : Model -> Sub Msg
subscriptions model =
    case model.page of
        Index indexModel ->
            Page.Index.subscriptions model.global indexModel
                |> Sub.map IndexMsg

        Login m ->
            Page.Login.subscriptions model.global m
                |> Sub.map LoginMsg

        Register m ->
            Page.Register.subscriptions model.global m
                |> Sub.map RegisterMsg

        UserDetail m ->
            Page.UserDetail.subscriptions model.global m
                |> Sub.map UserDetailMsg

        NotFound ->
            Sub.none
