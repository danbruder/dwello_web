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
    , response : ApiResponse User
    , dealResponse : ApiResponse Deal
    , dealsResponse : ApiResponse (List Deal)
    , updateDealResponse : ApiResponse Deal
    , dealList : List Deal
    , address : String
    , editingDeal : Maybe Deal
    , geocodeResponse : Maybe (Result Http.Error Geocoding.Response)
    , showAddressSearchControls : Bool
    }


init : Global -> String -> ( Model, Cmd Msg, Global.Msg )
init global id =
    let
        config =
            Global.getConfig global
    in
    ( { id = id
      , response = Loading
      , dealResponse = NotAsked
      , dealsResponse = Loading
      , updateDealResponse = NotAsked
      , dealList = []
      , address = ""
      , editingDeal = Nothing
      , geocodeResponse = Nothing
      , showAddressSearchControls = False
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
        MyGeocoderResult result ->
            ( { model | geocodeResponse = Just result }, Cmd.none, Global.none )

        GotUser response ->
            ( { model | response = response }, Cmd.none, Global.none )

        GotDeals response ->
            let
                dealList =
                    case response of
                        Success d ->
                            case d of
                                Data e ->
                                    e

                                _ ->
                                    []

                        _ ->
                            []
            in
            ( { model | dealsResponse = response, dealList = dealList }, Cmd.none, Global.none )

        StartEditing deal ->
            ( { model | editingDeal = Just deal }, Cmd.none, Global.none )

        StopEditing ->
            ( { model | editingDeal = Nothing }, Cmd.none, Global.none )

        DealCreated response ->
            let
                config =
                    Global.getConfig global

                newModel =
                    case response of
                        Success d ->
                            case d of
                                Data data ->
                                    { model | dealResponse = response, address = "", geocodeResponse = Nothing, showAddressSearchControls = False }

                                _ ->
                                    { model | dealResponse = response }

                        a ->
                            { model | dealResponse = a }
            in
            ( newModel
            , getDeals config model.id
            , Global.none
            )

        DealUpdated response ->
            let
                config =
                    Global.getConfig global

                newModel =
                    case response of
                        Success d ->
                            case d of
                                Data data ->
                                    let
                                        updateInPlace =
                                            \deal ->
                                                if deal.id == data.id then
                                                    data

                                                else
                                                    deal

                                        newDeals =
                                            List.map updateInPlace model.dealList
                                    in
                                    { model | updateDealResponse = response, dealList = newDeals, editingDeal = Nothing }

                                _ ->
                                    { model | updateDealResponse = response }

                        Failure _ ->
                            { model | updateDealResponse = response, editingDeal = Nothing }

                        a ->
                            { model | updateDealResponse = a }
            in
            ( newModel
            , Cmd.none
            , Global.none
            )

        CreateDeal address ->
            let
                input =
                    CreateDealInput model.id address.formattedAddress address

                config =
                    Global.getConfig global
            in
            ( { model | dealResponse = Loading }, createDeal config input, Global.none )

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
        ]
    }


viewContent model =
    case ( model.response, model.dealsResponse ) of
        ( Loading, _ ) ->
            viewLoading

        ( _, Loading ) ->
            viewLoading

        ( Success s, _ ) ->
            case s of
                Data d ->
                    div []
                        [ viewUser d
                        , viewDealList model
                        ]

                _ ->
                    div [] [ text "Something went wrong" ]

        ( Failure e, _ ) ->
            div [] [ text "Error" ]

        _ ->
            div [] [ text "Something else" ]


viewGeocodeResults : Model -> Html Msg
viewGeocodeResults model =
    let
        formatHit =
            \a -> li [ class "mt-2 cursor-pointer hover:bg-indigo-lightest absolute bg-white p-4 border border-grey shadow", onClick (CreateDeal a) ] [ text a.formattedAddress ]

        viewHits =
            \list ->
                div [ class "" ]
                    [ ul [ class "list-reset" ] (List.map formatHit list)
                    ]
    in
    case model.dealResponse of
        Loading ->
            viewLoading

        _ ->
            case model.geocodeResponse of
                Just (Ok r) ->
                    let
                        items =
                            r.results
                    in
                    viewHits items

                _ ->
                    text ""


viewLoading =
    div [ class "spinner" ] []


viewUser user =
    div []
        [ h1 [ class "text-grey-darkest pb-2" ] [ text user.name ]
        , div [] [ text user.email ]
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
                        [ input [ class "cursor-pointer  font-bold", type_ "submit", value "Search" ] []
                        , button [ onClick ToggleAddressSearch, class "cursor-pointer text-indigo  ", type_ "button" ] [ text "Cancel" ]
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
