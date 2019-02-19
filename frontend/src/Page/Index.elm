module Page.Index exposing
    ( Model
    , Msg(..)
    , init
    , subscriptions
    , update
    , view
    )

import Api exposing (ApiData(..), ApiResponse)
import Browser exposing (Document)
import Config exposing (Config)
import Data.Session exposing (Session)
import Data.User exposing (User)
import Global exposing (Global)
import Html exposing (..)
import Html.Attributes exposing (attribute, class, classList, for, href, id, placeholder, src, style, title, type_, value)
import Html.Events exposing (onClick, onInput, onSubmit)
import RemoteData as RD exposing (RemoteData(..), WebData)
import Request.User exposing (CreateUserInput, Role(..))
import Route exposing (Route)
import Util exposing (updateInPlace)
import View exposing (Toast(..), toastFromHttpError, viewToast)



-- COMMANDS


getUsers : Config -> Cmd Msg
getUsers config =
    Request.User.getUsers config
        |> RD.sendRequest
        |> Cmd.map GetUsersResponse


createUser : Config -> CreateUserInput -> Cmd Msg
createUser config input =
    Request.User.createUser config input
        |> RD.sendRequest
        |> Cmd.map GotCreateUserResponse



-- MODEL


type alias Model =
    { usersResponse : ApiResponse (List User)
    , users : List User
    , newUserModalOpen : Bool
    , createUserResponse : ApiResponse User
    , name : String
    , lastName : String
    , email : String
    , password : String
    , toast : Toast
    }


init : Global -> ( Model, Cmd Msg, Global.Msg )
init global =
    ( { usersResponse = Loading
      , users = []
      , newUserModalOpen = False
      , createUserResponse = NotAsked
      , name = ""
      , lastName = ""
      , email = ""
      , password = ""
      , toast = Empty
      }
    , getUsers (Global.getConfig global)
    , Global.none
    )



-- UPDATE


type
    Msg
    -- Got stuff
    = GetUsersResponse (ApiResponse (List User))
    | GotCreateUserResponse (ApiResponse User)
      -- Send Stuff
    | CreateUser
      -- Local UI
    | OpenNewUserModal
    | CloseNewUserModal
      -- User form
    | FirstName String
    | LastName String
    | Email String
    | Password String


update : Global -> Msg -> Model -> ( Model, Cmd Msg, Global.Msg )
update global msg model =
    case msg of
        GetUsersResponse response ->
            let
                newModel =
                    case response of
                        Success (Data u) ->
                            { model | users = u }

                        Failure f ->
                            { model | toast = toastFromHttpError f }

                        _ ->
                            model
            in
            ( { newModel | usersResponse = response }, Cmd.none, Global.none )

        GotCreateUserResponse response ->
            let
                newModel =
                    case response of
                        Success (Data d) ->
                            { model | name = "", email = "", password = "", newUserModalOpen = False, users = List.append model.users [ d ] }

                        Failure f ->
                            { model | toast = toastFromHttpError f }

                        _ ->
                            model
            in
            ( { newModel | createUserResponse = response }, Cmd.none, Global.none )

        CreateUser ->
            ( { model | createUserResponse = Loading }
            , createUser (Global.getConfig global) (CreateUserInput model.name model.email model.password [ Authenticated ])
            , Global.none
            )

        -- Modal
        OpenNewUserModal ->
            ( { model | newUserModalOpen = True }, Cmd.none, Global.none )

        CloseNewUserModal ->
            -- Clear away validation errors
            let
                r =
                    case model.createUserResponse of
                        Success (ValidationErrors e) ->
                            NotAsked

                        _ ->
                            model.createUserResponse
            in
            ( { model | newUserModalOpen = False, name = "", email = "", password = "", createUserResponse = r }, Cmd.none, Global.none )

        -- Create user form
        FirstName s ->
            ( { model | name = s }, Cmd.none, Global.none )

        LastName s ->
            ( { model | lastName = s }, Cmd.none, Global.none )

        Email s ->
            ( { model | email = s }, Cmd.none, Global.none )

        Password s ->
            ( { model | password = s }, Cmd.none, Global.none )



-- SUBSCRIPTIONS


subscriptions : Global -> Model -> Sub Msg
subscriptions _ _ =
    Sub.none



-- VIEW


view : Global -> Model -> Document Msg
view _ model =
    { title = "Home"
    , body =
        [ div [ class "container mx-auto " ]
            [ h1 [] [ text "Users" ]
            , viewContent model
            ]
        , viewToast model.toast
        ]
    }


viewContent : Model -> Html Msg
viewContent model =
    case model.usersResponse of
        Loading ->
            div [ class "spinner" ] []

        _ ->
            let
                users =
                    viewUserTable model.users
            in
            div [] [ users, viewNewUserModal model ]


viewUserTable : List User -> Html Msg
viewUserTable users =
    div [ class "bg-white shadow-md rounded my-8 p-6 " ]
        [ button [ onClick OpenNewUserModal, class "bg-indigo border-indigo text-white px-4 py-2 mb-2 rounded hover:bg-indigo-light" ] [ text "New User" ]
        , div [ class " border-b border-grey-light overflow-hidden relative" ]
            [ div [ class " overflow-y-auto scrollbar-w-2 scrollbar-track-grey-lighter scrollbar-thumb-rounded scrollbar-thumb-grey scrolling-touch" ]
                [ table [ class "w-full text-left table-collapse" ]
                    [ thead []
                        [ tr []
                            [ th [ class "text-sm font-semibold text-grey-darker p-2" ]
                                [ text "Name" ]
                            , th [ class "text-sm font-semibold text-grey-darker p-2" ]
                                [ text "Email" ]
                            ]
                        ]
                    , tbody [ class "align-baseline" ]
                        (List.map
                            viewUser
                            users
                        )
                    ]
                ]
            ]
        ]


viewNewUserModal : Model -> Html Msg
viewNewUserModal model =
    let
        body =
            div [ class "flex justify-center items-center h-full" ]
                [ div [ class "w-full" ]
                    [ Html.form [ class "", onSubmit CreateUser ]
                        [ viewInput model "text" "First name" model.name "name" FirstName
                        , viewInput model "text" "Email" model.email "email" Email
                        , viewInput model "password" "Password" model.password "password" Password
                        , div [ class "pt-6 flex items-center justify-between" ] []
                        , input [ type_ "submit", value "Submit", class "hidden" ] []
                        ]
                    ]
                ]
    in
    modal
        (ModalConfig
            "New User"
            body
            CreateUser
            CloseNewUserModal
            "Save"
            model.newUserModalOpen
            (model.createUserResponse
                == Loading
            )
        )


type alias ModalConfig =
    { title : String
    , body : Html Msg
    , onSubmit : Msg
    , onClose : Msg
    , submitText : String
    , isOpen : Bool
    , isLoading : Bool
    }


modal : ModalConfig -> Html Msg
modal config =
    let
        submit =
            if config.isLoading then
                div [ class "spinner ml-8" ] []

            else
                input [ class "cursor-pointer bg-indigo hover:bg-indigo-dark text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline", type_ "submit", onClick CreateUser, value config.submitText ] []

        content =
            div [ class "fixed pin z-50 overflow-auto bg-smoke-light flex", style "background-color" "rgba(0, 0, 0, 0.4)" ]
                [ div [ class "relative  bg-white w-full max-w-md m-auto flex-col flex" ]
                    [ h2 [ class "p-6 py-4 text-white text-bold bg-indigo" ] [ text config.title ]
                    , div [ class "p-6 pb-0" ] [ config.body ]
                    , div [ class "p-6 pt-0 flex justify-end" ]
                        [ button [ class "mr-4 text-indigo", onClick config.onClose ] [ text "close" ]
                        , submit
                        ]
                    ]
                ]
    in
    case config.isOpen of
        True ->
            content

        False ->
            div [] []


viewUser : User -> Html Msg
viewUser user =
    tr []
        [ td [ class "p-2 border-t border-grey-light text-s  whitespace-no-wrap" ]
            [ a [ class "no-underline text-indigo", Route.href <| Route.UserDetail { id = user.id |> String.fromInt } ]
                [ text user.name
                ]
            ]
        , td [ class "p-2 border-t border-grey-light  text-s  whitespace-pre" ]
            [ text user.email ]
        ]


viewInput :
    Model
    -> String
    -> String
    -> String
    -> String
    -> (String -> msg)
    -> Html msg
viewInput model t pl v field toMsg =
    div [ class "mb-4" ]
        [ label [ class "block text-grey-darker text-sm font-bold mb-2", for field ]
            [ text pl ]
        , input
            [ class "shadow appearance-none border  rounded w-full py-2 px-3 text-grey-darker  leading-tight focus:outline-none focus:shadow-outline"
            , classList [ ( "border-red", hasValidationErrors model field ) ]
            , id field
            , placeholder pl
            , type_ t
            , value v
            , onInput toMsg
            ]
            []
        , formatValidationErrors model field
        ]


getValidationErrors : Model -> String -> String
getValidationErrors model f =
    case model.createUserResponse of
        Success d ->
            case d of
                ValidationErrors validationErrors ->
                    validationErrors
                        |> List.filter (\{ field } -> field == f)
                        |> List.map .message
                        |> String.join ", "

                _ ->
                    ""

        _ ->
            ""


formatValidationErrors : Model -> String -> Html msg
formatValidationErrors model field =
    let
        errs =
            getValidationErrors model field
    in
    if errs /= "" then
        p [ class "text-red text-xs italic pt-2" ]
            [ text errs ]

    else
        text ""


hasValidationErrors : Model -> String -> Bool
hasValidationErrors model field =
    getValidationErrors model field /= ""
