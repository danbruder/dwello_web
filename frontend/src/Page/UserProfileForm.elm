module Page.UserProfileForm exposing
    ( Model
    , Msg(..)
    , init
    , subscriptions
    , update
    , view
    )

import Api exposing (ApiData(..), ApiResponse)
import Browser exposing (Document)
import Browser.Navigation as BN
import Config exposing (Config)
import Data.AuthPayload exposing (AuthPayload)
import Data.Session exposing (Session, Token)
import Data.User exposing (Profile, User)
import Dict exposing (Dict)
import Global exposing (Global, Msg(..))
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (onClick, onInput, onSubmit)
import Http
import Json.Encode as JE
import RemoteData as RD exposing (RemoteData(..))
import Request.User exposing (ProfileInput, createProfile, getProfile, updateProfile)
import Route
import Task
import Url.Builder as UB
import View exposing (Toast(..), toastFromHttpError, viewToast)



-- COMMANDS
--


getProfile : Config -> String -> Cmd Msg
getProfile config id =
    Request.User.getProfile config id
        |> RD.sendRequest
        |> Cmd.map GotProfile


createProfile : Config -> String -> ProfileInput -> Cmd Msg
createProfile config id input =
    Request.User.createProfile config id input
        |> RD.sendRequest
        |> Cmd.map CreatedProfile


updateProfile : Config -> String -> ProfileInput -> Cmd Msg
updateProfile config id input =
    Request.User.updateProfile config id input
        |> RD.sendRequest
        |> Cmd.map UpdatedProfile



--
-- MODEL


type alias Model =
    { id : String
    , profileRequest : ApiResponse Profile
    , createProfileRequest : ApiResponse Profile
    , updateProfileRequest : ApiResponse Profile
    , toast : Toast
    , profileExists : ProfileExists
    , editing : Bool
    , title : String
    , intro : String
    , body : String
    }


type ProfileExists
    = NotSure
    | Yes
    | No


init : Global -> String -> ( Model, Cmd Msg, Global.Msg )
init global id =
    ( { id = id
      , profileRequest = Loading
      , createProfileRequest = NotAsked
      , updateProfileRequest = NotAsked
      , toast = Empty
      , profileExists = NotSure
      , editing = False
      , title = ""
      , intro = ""
      , body = ""
      }
    , getProfile (Global.getConfig global) id
    , Global.none
    )



-- UPDATE


type Msg
    = ToggleEdit
    | GotProfile (ApiResponse Profile)
    | CreatedProfile (ApiResponse Profile)
    | UpdatedProfile (ApiResponse Profile)
    | Save
      -- local edits
    | Title String
    | Intro String
    | Body String


update : Global -> Msg -> Model -> ( Model, Cmd Msg, Global.Msg )
update global msg model =
    case msg of
        -- Remote Data
        GotProfile response ->
            let
                newModel =
                    case response of
                        Success (Data d) ->
                            { model | title = d.title, intro = d.intro, body = d.body, profileExists = Yes }

                        Failure (Http.BadStatus res) ->
                            if res.status.code == 404 then
                                { model | profileExists = No, editing = True }

                            else
                                { model | toast = toastFromHttpError (Http.BadStatus res) }

                        Failure f ->
                            { model | toast = toastFromHttpError f }

                        _ ->
                            model
            in
            ( { newModel | profileRequest = response }, Cmd.none, Global.none )

        CreatedProfile response ->
            let
                newModel =
                    case response of
                        Success (Data d) ->
                            { model | title = d.title, intro = d.intro, body = d.body, editing = False, profileExists = Yes }

                        Failure f ->
                            { model | toast = toastFromHttpError f }

                        _ ->
                            model
            in
            ( { newModel | createProfileRequest = response }, Cmd.none, Global.none )

        UpdatedProfile response ->
            let
                newModel =
                    case response of
                        Success (Data d) ->
                            { model | title = d.title, intro = d.intro, body = d.body, editing = False, profileExists = Yes }

                        Failure f ->
                            { model | toast = toastFromHttpError f }

                        _ ->
                            model
            in
            ( { newModel | updateProfileRequest = response }, Cmd.none, Global.none )

        Save ->
            let
                input =
                    ProfileInput model.title model.intro model.body

                ( newModel, command ) =
                    case model.profileExists of
                        NotSure ->
                            ( { model | createProfileRequest = Loading }, createProfile (Global.getConfig global) model.id input )

                        Yes ->
                            ( { model | updateProfileRequest = Loading }, updateProfile (Global.getConfig global) model.id input )

                        No ->
                            ( { model | createProfileRequest = Loading }, createProfile (Global.getConfig global) model.id input )
            in
            ( newModel, command, Global.none )

        -- Local edits
        ToggleEdit ->
            ( { model | editing = not model.editing }, Cmd.none, Global.none )

        Title a ->
            ( { model | title = a }, Cmd.none, Global.none )

        Intro a ->
            ( { model | intro = a }, Cmd.none, Global.none )

        Body a ->
            ( { model | body = a }, Cmd.none, Global.none )



-- SUBSCRIPTIONS


subscriptions : Global -> Model -> Sub Msg
subscriptions _ _ =
    Sub.none



-- VIEW


view : Global -> Model -> Document Msg
view global model =
    { title = "Edit User " ++ model.id ++ " Profile"
    , body =
        [ div [ class "container mx-auto max-w-lg" ]
            [ viewContent model
            , viewToast model.toast
            ]
        ]
    }


viewContent : Model -> Html Msg
viewContent model =
    let
        action =
            case ( model.editing, model.createProfileRequest, model.updateProfileRequest ) of
                ( _, Loading, _ ) ->
                    button [ class "disabled shadow-m bg-indigo hover:bg-indigo-dark text-white font-bold py-2 px-4 rounded absolute pin-r m-4" ]
                        [ text "Updating" ]

                ( _, _, Loading ) ->
                    button [ class "disabled shadow-m bg-indigo hover:bg-indigo-dark text-white font-bold py-2 px-4 rounded absolute pin-r m-4" ]
                        [ text "Saving" ]

                ( True, _, _ ) ->
                    button [ class "shadow-m bg-indigo hover:bg-indigo-dark text-white font-bold py-2 px-4 rounded absolute pin-r m-4", onClick Save ]
                        [ text "Save" ]

                ( False, _, _ ) ->
                    button [ class "shadow-m bg-indigo hover:bg-indigo-dark text-white font-bold py-2 px-4 rounded absolute pin-r m-4", onClick ToggleEdit ]
                        [ text "Edit" ]

        profile =
            div [ class "border bg-white border-grey rounded-sm relative" ]
                [ div
                    [ class "cursor-pointer opacity-75 absolute bg-grey h-48 w-full"
                    , classList
                        [ ( "hidden", not model.editing )
                        ]
                    ]
                    []
                , div
                    [ class "h-48"
                    , style "background-image" "url(https://media.licdn.com/dms/image/C4D16AQFZ6g-MPFewGQ/profile-displaybackgroundimage-shrink_350_1400/0?e=1556755200&v=beta&t=jDQXQBLmcl9PBvK4kAqmq4Wp8kaq8wi0_rkMVNPS6zA)"
                    ]
                    [ action
                    ]
                , div [ class "-mt-16 ml-8 flex relative" ]
                    [ div
                        [ class "cursor-pointer opacity-75 absolute border border-grey-light w-32 h-32 shadow-md p-1 bg-white rounded-full"
                        , classList
                            [ ( "hidden", not model.editing )
                            ]
                        ]
                        []
                    , img
                        [ class "border border-grey-light w-32 h-32 shadow-md p-1 bg-white rounded-full"
                        , src "https://media.licdn.com/dms/image/C4D03AQFXOuGuhG9IeQ/profile-displayphoto-shrink_200_200/0?e=1556755200&v=beta&t=8FzsaJ7TuPZzA20Xtmu3Ydf51Sy8zvFOENR58PxRBko"
                        ]
                        []
                    , div [ class "bg-white  w-full mt-16 mr-8 ml-4  p-3 " ]
                        [ h1 [ class "m-0" ]
                            [ case model.editing of
                                True ->
                                    input [ type_ "text", onInput Title, value model.title, class "-m-px border", autofocus True ] []

                                False ->
                                    text model.title
                            ]
                        ]
                    ]
                , div [ class "bg-white p-8 border-b w-full" ]
                    [ h2 []
                        [ case model.editing of
                            True ->
                                input [ type_ "text", onInput Intro, value model.intro, class "-m-px border w-full", autofocus True ] []

                            False ->
                                text model.intro
                        ]
                    ]
                , div [ class "bg-white p-4 px-8 border-b" ] [ text "👪 🐶 🐱" ]
                , div [ class "bg-white p-6 px-8" ]
                    [ case model.editing of
                        True ->
                            textarea
                                [ value model.body
                                , class "h-48 -m-px border w-full"
                                , autofocus True
                                , onInput Body
                                ]
                                []

                        False ->
                            text model.body
                    ]
                ]
    in
    if model.profileExists == No then
        div []
            [ h1 [ class "mb-8" ]
                [ text "Create profile" ]
            , profile
            ]

    else
        div [] [ profile ]


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
    ""



-- case model.registration of
--     Success d ->
--         case d of
--             ValidationErrors validationErrors ->
--                 validationErrors
--                     |> List.filter (\{ field } -> field == f)
--                     |> List.map .message
--                     |> String.join ", "
--
--             _ ->
--                 ""
--
--     _ ->
--         ""
--


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
