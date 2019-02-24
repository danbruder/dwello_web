module Page.UserDetail exposing
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
import Data.Deal as Deal exposing (Deal, DealStatus)
import Data.User exposing (User)
import Geocoding exposing (GeocodingResult)
import Global exposing (Global)
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (onBlur, onClick, onInput, onSubmit)
import Http
import Json.Encode as JE
import RemoteData as RD exposing (RemoteData(..))
import Request.Deal exposing (CreateDealInput, UpdateDealInput)
import Request.User
import Route
import Svg exposing (svg)
import Svg.Attributes exposing (d, path, viewBox)
import Util exposing (updateInPlace)
import View exposing (Toast(..), toastFromApiResponse, viewToast)



-- COMMANDS


getUser : Config -> String -> Cmd Msg
getUser config id =
    Request.User.getUser config id
        |> RD.sendRequest
        |> Cmd.map GotUser


createDeal : Config -> CreateDealInput -> Cmd Msg
createDeal config input =
    Request.Deal.createDeal config input
        |> RD.sendRequest
        |> Cmd.map DealCreated


updateDeal : Config -> UpdateDealInput -> Int -> Cmd Msg
updateDeal config input id =
    Request.Deal.updateDeal config input id
        |> RD.sendRequest
        |> Cmd.map DealUpdated


getDeals : Config -> String -> Cmd Msg
getDeals config id =
    Request.Deal.getDeals config id
        |> RD.sendRequest
        |> Cmd.map GotDeals


searchForAddress : Config -> String -> Cmd Msg
searchForAddress config address =
    Geocoding.requestForAddress config.googleApiKey address
        |> Geocoding.send MyGeocoderResult



-- MODEL


type alias Model =
    { id : String
    , userResponse : ApiResponse User
    , dealResponse : ApiResponse Deal
    , dealsResponse : ApiResponse (List Deal)
    , updateDealResponse : ApiResponse Deal
    , dealList : List Deal
    , address : String
    , editingDeal : Maybe Deal
    , geocodeResponse : Maybe (Result Http.Error Geocoding.Response)
    , showAddressSearchControls : Bool
    , toast : Toast
    }


init : Global -> String -> ( Model, Cmd Msg, Global.Msg )
init global id =
    let
        config =
            Global.getConfig global
    in
    ( { id = id
      , userResponse = Loading
      , dealResponse = NotAsked
      , dealsResponse = Loading
      , updateDealResponse = NotAsked
      , dealList = []
      , address = ""
      , editingDeal = Nothing
      , geocodeResponse = Nothing
      , showAddressSearchControls = False
      , toast = Empty
      }
    , Cmd.batch
        [ getUser config id
        , getDeals config id
        ]
    , Global.none
    )



-- UPDATE


type
    Msg
    -- Got stuff
    = GotUser (ApiResponse User)
    | DealCreated (ApiResponse Deal)
    | DealUpdated (ApiResponse Deal)
    | GotDeals (ApiResponse (List Deal))
    | MyGeocoderResult (Result Http.Error Geocoding.Response)
      -- Do stuff to server
    | SaveStatus
    | CreateDeal GeocodingResult
    | SearchForAddress
      -- Do things locally
    | StartEditing Deal
    | StopEditing
    | Address String
    | Status String
    | ToggleAddressSearch
    | NoOp


update : Global -> Msg -> Model -> ( Model, Cmd Msg, Global.Msg )
update global msg model =
    case msg of
        -- Responses
        MyGeocoderResult result ->
            ( { model | geocodeResponse = Just result }, Cmd.none, Global.none )

        GotUser response ->
            ( { model | userResponse = response, toast = toastFromApiResponse response }, Cmd.none, Global.none )

        GotDeals response ->
            let
                dealList =
                    case response of
                        Success (Data d) ->
                            d

                        _ ->
                            []
            in
            ( { model | dealsResponse = response, dealList = dealList, toast = toastFromApiResponse response }, Cmd.none, Global.none )

        DealCreated response ->
            let
                newModel =
                    case response of
                        Success (Data data) ->
                            { model | address = "", geocodeResponse = Nothing, showAddressSearchControls = False }

                        _ ->
                            model
            in
            ( { newModel | dealResponse = response, toast = toastFromApiResponse response }
            , getDeals (Global.getConfig global) model.id
            , Global.none
            )

        DealUpdated response ->
            let
                newModel =
                    case response of
                        Success (Data data) ->
                            { model | dealList = updateInPlace model.dealList data, editingDeal = Nothing }

                        Failure _ ->
                            { model | editingDeal = Nothing }

                        _ ->
                            model
            in
            ( { newModel | updateDealResponse = response, toast = toastFromApiResponse response }
            , Cmd.none
            , Global.none
            )

        -- Trigger requests
        CreateDeal address ->
            let
                input =
                    CreateDealInput model.id address.formattedAddress address
            in
            ( { model | dealResponse = Loading }, createDeal (Global.getConfig global) input, Global.none )

        SearchForAddress ->
            ( model, searchForAddress (Global.getConfig global) model.address, Global.none )

        SaveStatus ->
            let
                config =
                    Global.getConfig global

                cmd =
                    case model.editingDeal of
                        Just deal ->
                            let
                                input =
                                    UpdateDealInput deal.status
                            in
                            updateDeal config input deal.id

                        Nothing ->
                            Cmd.none
            in
            ( { model | updateDealResponse = Loading }
            , cmd
            , Global.none
            )

        StartEditing deal ->
            ( { model | editingDeal = Just deal }, Cmd.none, Global.none )

        StopEditing ->
            ( { model | editingDeal = Nothing }, Cmd.none, Global.none )

        -- New Deal form
        Address a ->
            ( { model | address = a, showAddressSearchControls = True, geocodeResponse = Nothing }, Cmd.none, Global.none )

        -- Edit deal values
        Status a ->
            let
                newDeal =
                    case model.editingDeal of
                        Just deal ->
                            Just { deal | status = Deal.stringToStatus a }

                        Nothing ->
                            Nothing
            in
            ( { model | editingDeal = newDeal }, Cmd.none, Global.none )

        ToggleAddressSearch ->
            ( { model | showAddressSearchControls = not model.showAddressSearchControls, address = "", geocodeResponse = Nothing, dealResponse = NotAsked }, Cmd.none, Global.none )

        NoOp ->
            ( model, Cmd.none, Global.none )



-- SUBSCRIPTIONS


subscriptions : Global -> Model -> Sub Msg
subscriptions global model =
    Sub.none



-- VIEW


view : Global -> Model -> Document Msg
view global model =
    { title = "User " ++ model.id
    , body =
        [ div [ class "container mx-auto" ]
            [ viewContent
                model
            ]
        , viewToast model.toast
        ]
    }


viewContent model =
    case model.userResponse of
        Success (Data d) ->
            div []
                [ viewUser d
                , viewDealList model
                ]

        Loading ->
            div [ class "spinner" ] []

        NotAsked ->
            div [] []

        _ ->
            div [] [ text "Could not get user" ]


viewGeocodeResults : Model -> Html Msg
viewGeocodeResults model =
    let
        formatHit =
            \a -> li [ class "cursor-pointer hover:bg-indigo-lightest  bg-white p-4 ", onClick (CreateDeal a) ] [ text a.formattedAddress ]

        viewHits =
            \list ->
                ul [ class "list-reset fixed mt-2 border border-grey-light rounded shadow" ] (List.map formatHit list)
    in
    case model.dealResponse of
        Loading ->
            viewLoading

        _ ->
            case model.geocodeResponse of
                Just (Ok r) ->
                    viewHits r.results

                _ ->
                    text ""


viewLoading =
    div [ class "spinner" ] []


viewUser user =
    div []
        [ h1 [ class "text-grey-darkest pb-2" ] [ text user.name ]
        , div [] [ text user.email ]
        , div []
            [ a [ class "no-underline text-indigo", Route.href <| Route.UserProfileForm { id = user.id |> String.fromInt } ]
                [ text "Profile"
                ]
            ]
        ]


viewDealList model =
    let
        addressRow =
            viewAddressForm model

        rows =
            List.map (viewDeal model) model.dealList

        allRows =
            addressRow :: rows
    in
    div [ class "bg-white shadow rounded mt-4 p-6 " ]
        [ div [ class " border-b border-grey-light overflow-hidden relative" ]
            [ div [ class " overflow-y-auto scrollbar-w-2 scrollbar-track-grey-lighter scrollbar-thumb-rounded scrollbar-thumb-grey scrolling-touch" ]
                [ table [ class "w-full text-left table-collapse" ]
                    [ thead []
                        [ tr []
                            [ th [ class "text-sm font-semibold text-grey-darker p-2" ]
                                [ text "Address" ]
                            , th [ class "text-sm font-semibold text-grey-darker p-2" ]
                                [ text "Code" ]
                            , th [ style "min-width" "100px", class "text-sm font-semibold text-grey-darker p-2" ]
                                [ text "Status" ]
                            , th [ style "min-width" "100px", class "text-sm font-semibold text-grey-darker p-2" ]
                                [ text "" ]
                            ]
                        ]
                    , tbody [ class "align-baseline" ] allRows
                    ]
                ]
            ]
        ]


viewDeal : Model -> Deal -> Html Msg
viewDeal model deal =
    let
        targetDeal =
            case model.editingDeal of
                Just dealWithHouse ->
                    dealWithHouse

                Nothing ->
                    deal

        editing =
            case model.editingDeal of
                Just dealWithHouse ->
                    dealWithHouse.id == deal.id

                Nothing ->
                    False

        status =
            case editing of
                True ->
                    select [ onInput Status, autofocus True ]
                        [ option [ value "Initialized" ] [ text "Initialized" ]
                        , option [ value "MailerSent" ] [ text "MailerSent" ]
                        ]

                False ->
                    deal.status |> Deal.statusToString |> text

        edit =
            case editing of
                False ->
                    div [ style "cursor" "pointer", onClick (StartEditing deal) ] [ text "edit" ]

                True ->
                    div [ class "flex" ]
                        [ div [ class "pr-3", style "cursor" "pointer", onClick SaveStatus ] [ text "save" ]
                        , div [ style "cursor" "pointer", onClick StopEditing ] [ text "cancel" ]
                        ]
    in
    tr []
        [ td [ class "p-2 border-t border-grey-light  text-s whitespace-no-wrap" ]
            [ text deal.title
            ]
        , td [ class "p-2 border-t border-grey-light  text-s  whitespace-pre" ]
            [ text deal.access_code ]
        , td [ class "p-2 border-t border-grey-light  text-s  whitespace-pre" ]
            [ status ]
        , td [ class "p-2 border-t border-grey-light  text-s text-indigo whitespace-pre" ]
            [ edit ]
        ]


viewAddressForm : Model -> Html Msg
viewAddressForm model =
    let
        searchControls =
            case model.showAddressSearchControls of
                True ->
                    span []
                        [ input [ class "cursor-pointer text-indigo ", type_ "submit", value "search" ] []
                        , button [ onClick ToggleAddressSearch, class "text-grey cursor-pointer text-grey-darker", type_ "button" ] [ text "cancel" ]
                        ]

                False ->
                    text ""

        f =
            div []
                [ Html.form [ class "flex justify-start items-start ", onSubmit SearchForAddress ]
                    [ div [ class "w-full max-w-s" ]
                        [ input
                            [ class " appearance-none   w-full text-grey-darkest leading-tight focus:outline-none focus:bg-white focus:border-indigo"
                            , classList [ ( "border border-red", hasValidationErrors model "address" ) ]
                            , id "address"
                            , type_ "text"
                            , placeholder "Add new pin address"
                            , onInput Address
                            , value model.address
                            ]
                            []
                        , formatValidationErrors model "address"
                        ]
                    , searchControls
                    ]
                , viewGeocodeResults model
                ]
    in
    tr []
        [ td [ class "p-2 border-t border-grey-light  text-s whitespace-no-wrap" ]
            [ f
            ]
        , td [ class "p-2 border-t border-grey-light  text-s  whitespace-pre" ]
            []
        , td [ class "p-2 border-t border-grey-light  text-s  whitespace-pre" ]
            []
        , td [ class "p-2 border-t border-grey-light  text-s text-indigo whitespace-pre" ]
            []
        ]


viewInput :
    Model
    -> String
    -> String
    -> String
    -> String
    -> (String -> msg)
    -> Html msg
viewInput model t p v field toMsg =
    div []
        [ label []
            [ text p ]
        , input
            [ type_ t, placeholder p, value v, onInput toMsg ]
            []
        , getValidationErrors model field |> text
        ]


getValidationErrors : Model -> String -> String
getValidationErrors model f =
    case model.dealResponse of
        Success d ->
            case d of
                ValidationErrors validationErrors ->
                    validationErrors
                        |> List.filter (\{ field } -> field == f)
                        |> List.map .message
                        |> String.join "\n"

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
