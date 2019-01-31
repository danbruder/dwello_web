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
import Data.Deal as Deal exposing (DealStatus, DealWithHouse)
import Data.Session exposing (Token)
import Data.User exposing (User)
import Global exposing (Global)
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (onBlur, onClick, onInput, onSubmit)
import Json.Encode as JE
import RemoteData as RD exposing (RemoteData(..))
import Request.Deal exposing (CreateDealInput, UpdateDealInput)
import Request.User
import Route



-- COMMANDS


getUser : Config -> Token -> String -> Cmd Msg
getUser config token id =
    Request.User.getUser config token id
        |> RD.sendRequest
        |> Cmd.map GotUser


createDeal : Config -> Token -> CreateDealInput -> Cmd Msg
createDeal config token input =
    Request.Deal.createDealWithHouse config token input
        |> RD.sendRequest
        |> Cmd.map DealCreated


updateDeal : Config -> Token -> UpdateDealInput -> Int -> Cmd Msg
updateDeal config token input id =
    Request.Deal.updateDeal config token input id
        |> RD.sendRequest
        |> Cmd.map DealUpdated


getDealsWithHouses : Config -> Token -> String -> Cmd Msg
getDealsWithHouses config token id =
    Request.Deal.getDealsWithHouses config token id
        |> RD.sendRequest
        |> Cmd.map GotDealsWithHouses



-- MODEL


type alias Model =
    { id : String
    , response : ApiResponse User
    , dealResponse : ApiResponse DealWithHouse
    , dealsWithHousesResponse : ApiResponse (List DealWithHouse)
    , updateDealResponse : ApiResponse DealWithHouse
    , dealList : List DealWithHouse
    , address : String
    , lat : String
    , lon : String
    , editingDeal : Maybe DealWithHouse
    }


init : Global -> String -> ( Model, Cmd Msg, Global.Msg )
init global id =
    let
        config =
            Global.getConfig global

        token =
            Global.getToken global
    in
    ( { id = id
      , response = Loading
      , dealResponse = NotAsked
      , dealsWithHousesResponse = Loading
      , updateDealResponse = NotAsked
      , dealList = []
      , address = ""
      , lat = ""
      , lon = ""
      , editingDeal = Nothing
      }
    , Cmd.batch
        [ getUser config token id
        , getDealsWithHouses config token id
        ]
    , Global.none
    )



-- UPDATE


type Msg
    = GotUser (ApiResponse User)
    | DealCreated (ApiResponse DealWithHouse)
    | DealUpdated (ApiResponse DealWithHouse)
    | GotDealsWithHouses (ApiResponse (List DealWithHouse))
    | StartEditing DealWithHouse
    | StopEditing
    | Address String
    | Status String
    | SaveStatus
    | Lat String
    | Lon String
    | CreateDeal
    | NoOp


update : Global -> Msg -> Model -> ( Model, Cmd Msg, Global.Msg )
update global msg model =
    case msg of
        GotUser response ->
            ( { model | response = response }, Cmd.none, Global.none )

        GotDealsWithHouses response ->
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
            ( { model | dealsWithHousesResponse = response, dealList = dealList }, Cmd.none, Global.none )

        StartEditing deal ->
            ( { model | editingDeal = Just deal }, Cmd.none, Global.none )

        StopEditing ->
            ( { model | editingDeal = Nothing }, Cmd.none, Global.none )

        DealCreated response ->
            let
                config =
                    Global.getConfig global

                token =
                    Global.getToken global

                newModel =
                    case response of
                        Success d ->
                            case d of
                                Data data ->
                                    { model | dealResponse = response, address = "", lat = "", lon = "" }

                                _ ->
                                    { model | dealResponse = response }

                        a ->
                            { model | dealResponse = a }
            in
            ( newModel
            , getDealsWithHouses config token model.id
            , Global.none
            )

        DealUpdated response ->
            let
                config =
                    Global.getConfig global

                token =
                    Global.getToken global

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
                    CreateDealInput model.id model.address model.lat model.lon

                config =
                    Global.getConfig global

                token =
                    Global.getToken global
            in
            ( { model | dealResponse = Loading }, createDeal config token input, Global.none )

        SaveStatus ->
            let
                config =
                    Global.getConfig global

                token =
                    Global.getToken global

                cmd =
                    case model.editingDeal of
                        Just deal ->
                            let
                                input =
                                    UpdateDealInput deal.status
                            in
                            updateDeal config token input deal.id

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

        Lat a ->
            ( { model | lat = a }, Cmd.none, Global.none )

        Lon a ->
            ( { model | lon = a }, Cmd.none, Global.none )

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
    case ( model.response, model.dealsWithHousesResponse ) of
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


viewDeal : Model -> DealWithHouse -> Html Msg
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
                [ div [] [ deal.address |> text ]
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
        , viewInput model "text" "Lat" model.lat "lat" Lat
        , viewInput model "text" "Lon" model.lon "lon" Lon
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
