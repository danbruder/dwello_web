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
import Geocoding
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
      }
    , Cmd.batch
        [ getUser config id
        , getDeals config id
        ]
    , Global.none
    )



-- UPDATE


type Msg
    = GotUser (ApiResponse User)
    | DealCreated (ApiResponse Deal)
    | DealUpdated (ApiResponse Deal)
    | GotDeals (ApiResponse (List Deal))
    | StartEditing Deal
    | StopEditing
    | Address String
    | Status String
    | SaveStatus
    | CreateDeal
    | MyGeocoderResult (Result Http.Error Geocoding.Response)
    | NoOp


update : Global -> Msg -> Model -> ( Model, Cmd Msg, Global.Msg )
update global msg model =
    case msg of
        MyGeocoderResult result ->
            let
                _ =
                    Debug.log (result |> Debug.toString)
            in
            ( model, Cmd.none, Global.none )

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
                                    { model | dealResponse = response, address = "" }

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

        CreateDeal ->
            let
                input =
                    CreateDealInput model.id model.address

                config =
                    Global.getConfig global
            in
            ( { model | dealResponse = Loading }, createDeal config input, Global.none )

        -- CreateDeal ->
        --     ( model, searchForAddress (Global.getConfig global) "3557 Jamesfield Dr. Hudsonvile, MI 49426", Global.none )
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
            ( { model | address = a }, Cmd.none, Global.none )

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
        [ a [ Route.href Route.Index ] [ text "Home" ]
        , viewContent model
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
                        , viewDealForm model
                        , viewDealList model
                        ]

                _ ->
                    div [] [ text "Something went wrong" ]

        ( Failure e, _ ) ->
            div [] [ text "Error" ]

        _ ->
            div [] [ text "Something else" ]


viewLoading =
    div [] [ text "Loading" ]


viewUser user =
    div []
        [ h1 [] [ text user.email ]
        , div [] [ text user.name ]
        ]


viewDealList model =
    div []
        [ ul [] <| List.map (viewDeal model) model.dealList
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
                    select [ onInput Status, onBlur SaveStatus, autofocus True, value (targetDeal.status |> Deal.statusToString) ]
                        [ option [ value "Initialized" ] [ text "Initialized" ]
                        , option [ value "MailerSent" ] [ text "MailerSent" ]
                        ]

                False ->
                    deal.status |> Deal.statusToString |> text

        dealDetail =
            div []
                [ div [] [ deal.title |> text ]
                , div [] [ deal.access_code |> text ]
                , div [ onClick (StartEditing targetDeal) ] [ status ]
                ]
    in
    li []
        [ dealDetail ]


viewDealForm : Model -> Html Msg
viewDealForm model =
    Html.form [ onSubmit CreateDeal ]
        [ viewInput model "text" "Address" model.address "address" Address
        , input [ type_ "submit", value "Submit" ] []
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
