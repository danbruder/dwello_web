module Main.View exposing (view)

import Browser exposing (Document)
import Global
import Html exposing (..)
import Html.Attributes exposing (class, href, src, style, title, type_)
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

        viewPageWithNoWrap toMsg pageModel pageView =
            let
                { title, body } =
                    pageView model.global pageModel

                mappedBody =
                    List.map (Html.map toMsg) body
            in
            { title = title
            , body = mappedBody
            }
    in
    case model.page of
        Index indexModel ->
            viewPage IndexMsg indexModel Page.Index.view

        Login lmodel ->
            viewPageWithNoWrap LoginMsg lmodel Page.Login.view

        Register lmodel ->
            viewPageWithNoWrap RegisterMsg lmodel Page.Register.view

        UserDetail lmodel ->
            viewPage UserDetailMsg lmodel Page.UserDetail.view

        NotFound ->
            { title = "Not Found"
            , body = [ text ":(" ]
            }


viewPageWrap : Model -> List (Html Msg) -> Html Msg
viewPageWrap model mb =
    div [ class "w-100 bg-grey-lightest h-full" ]
        [ viewHeader model
        , div [ class "" ]
            [ div [ class "p-8 " ] mb
            ]
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

        wrappedLinks =
            div [ class "flex" ] links

        logo =
            span [ class "font-semibold text-xl tracking-tight" ]
                [ a [ class "text-white no-underline", Route.href <| Route.Index ]
                    [ text "Dwello" ]
                ]

        linksWithLogo =
            logo :: [ wrappedLinks ]
    in
    nav [ class "bg-indigo-dark p-4" ]
        [ div [ class "container mx-auto" ]
            [ div [ class "w-full flex items-center flex-grow lg:flex lg:items-center lg:w-auto justify-between" ] linksWithLogo
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
    a [ class "block  lg:inline-block lg:mt-0 text-indigo-lighter hover:text-white mr-4", Route.href <| route, title txt ]
        [ text txt ]
