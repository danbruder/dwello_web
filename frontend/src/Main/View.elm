module Main.View exposing (view)

import Browser exposing (Document)
import Global
import Html exposing (..)
import Html.Attributes exposing (class, href, src, title, type_)
import Main.Model exposing (Model, Page(..))
import Main.Msg exposing (Msg(..))
import Page.Index
import Page.Login
import Page.Register
import Page.UserDetail
import Route exposing (Route)


view : Model -> Document Msg
view model =
    let
        viewPage toMsg pageModel pageView =
            let
                { title, body } =
                    pageView model.global pageModel

                mappedBody =
                    List.map (Html.map toMsg) body
            in
            { title = title
            , body = [ viewPageWrap model mappedBody ]
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


viewPageWrap : Model -> List (Html Msg) -> Html Msg
viewPageWrap model mb =
    div [ class "w-100" ]
        [ viewHeader model
        , div [ class "mw7 center ph3 pv3" ] mb
        ]


viewHeader : Model -> Html Msg
viewHeader model =
    let
        loggedIn =
            Global.getToken model.global /= ""

        links =
            case loggedIn of
                True ->
                    loggedInLinks

                False ->
                    loggedOutLinks
    in
    header [ class "w-100 bg-white center shadow-4" ]
        [ div [ class "mw7 center pa3" ]
            [ div [ class "db dt-ns center w-100" ]
                [ div [ class "db dtc-ns v-mid tl w-50" ]
                    [ a [ class "dib f5 f4-ns fw6 mt0 mb1 link black-70", Route.href <| Route.Index, title "Home" ]
                        [ text "Dwello"
                        ]
                    ]
                , nav [ class "db dtc-ns v-mid w-100 tl tr-ns mt2 mt0-ns" ]
                    links
                ]
            ]
        ]


loggedInLinks =
    [ viewHeaderLink Route.Index "Users"
    , viewHeaderLink Route.Logout "Logout"
    ]


loggedOutLinks =
    [ viewHeaderLink Route.Login "Login"
    , viewHeaderLink Route.Register "Register"
    ]


viewHeaderLink : Route -> String -> Html Msg
viewHeaderLink route txt =
    a [ class "f6 fw6 hover-blue link black-70 ml2 ml3-m ml4-l dib", Route.href <| route, title txt ]
        [ text txt ]
