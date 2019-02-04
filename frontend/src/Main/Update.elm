module Main.Update exposing (update)

import Browser
import Browser.Navigation as BN
import Global
import Main.Model exposing (Model, Page(..), initPage, updatePage)
import Main.Msg exposing (Msg(..))
import Page.Index
import Page.Login
import Page.Register
import Page.UserDetail
import Url


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case ( msg, model.page ) of
        ( UrlRequest urlRequest, _ ) ->
            case urlRequest of
                Browser.Internal url ->
                    ( model
                    , BN.pushUrl (Global.getKey model.global) (Url.toString url)
                    )

                Browser.External href ->
                    ( model
                    , BN.load href
                    )

        ( UrlChange url, _ ) ->
            initPage url model

        ( GlobalMsg gmsg, _ ) ->
            Global.update gmsg model.global
                |> Tuple.mapFirst (\global -> { model | global = global })
                |> Tuple.mapSecond (Cmd.map GlobalMsg)

        ( LoginMsg lmsg, Login lmodel ) ->
            Page.Login.update model.global lmsg lmodel
                |> updatePage Login LoginMsg model

        ( RegisterMsg lmsg, Register lmodel ) ->
            Page.Register.update model.global lmsg lmodel
                |> updatePage Register RegisterMsg model

        -- Protected routes
        ( IndexMsg imsg, Index indexModel ) ->
            Page.Index.update model.global imsg indexModel
                |> updatePage Index IndexMsg model

        ( UserDetailMsg lmsg, UserDetail lmodel ) ->
            Page.UserDetail.update model.global lmsg lmodel
                |> updatePage UserDetail UserDetailMsg model

        _ ->
            ( model, Cmd.none )
