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
        , div [ class "min-h-screen md:flex" ]
            [ viewSidebar model
            , div [ class "flex-1 p-8 bg-grey-lightest" ] mb
            ]
        ]


viewHeader : Model -> Html Msg
viewHeader model =
    nav [ class "flex items-center justify-between flex-wrap bg-indigo-dark p-6" ]
        [ div [ class "flex items-center flex-no-shrink text-white mr-6" ]
            [ span [ class "font-semibold text-xl tracking-tight" ]
                [ text "Dwello" ]
            ]
        , div [ class "w-full block flex-grow lg:flex lg:items-center lg:w-auto" ]
            [ div [ class "text-sm lg:flex-grow" ] []
            ]
        ]


loggedInLinks =
    [ viewSidebarLink Route.Index "Users"
    , viewSidebarLink Route.Logout "Logout"
    ]


loggedOutLinks =
    [ viewSidebarLink Route.Login "Login"
    , viewSidebarLink Route.Register "Register"
    ]


viewHeaderLink : Route -> String -> Html Msg
viewHeaderLink route txt =
    a [ class "block mt-4 lg:inline-block lg:mt-0 text-indigo-lighter hover:text-white mr-4", Route.href <| route, title txt ]
        [ text txt ]


viewSidebarLink : Route -> String -> Html Msg
viewSidebarLink route txt =
    li []
        [ a [ class "no-underline text-black px-4 py-3 block", Route.href <| route, title txt ]
            [ text txt ]
        ]


viewSidebar model =
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
    div [ style "max-width" "200px", class "flex-none w-full p-6 bg-grey-lighter" ]
        [ ul [ class "list-reset" ] links
        ]
