module Data.Deal exposing (DealStatus, DealWithHouse, decodeDealWithHouse, statusToString, stringToStatus)

import Json.Decode as JD exposing (Decoder)
import Json.Decode.Pipeline as JDP


type alias DealWithHouse =
    { id : Int
    , buyer_id : Maybe Int
    , seller_id : Maybe Int
    , house_id : Maybe Int
    , access_code : String
    , status : DealStatus
    , address : String
    , lat : String
    , lon : String
    }


type DealStatus
    = Initialized
    | MailerSent


statusToString : DealStatus -> String
statusToString dealStatus =
    case dealStatus of
        Initialized ->
            "Initialized"

        MailerSent ->
            "MailerSent"


stringToStatus : String -> DealStatus
stringToStatus string =
    case string of
        "Initialized" ->
            Initialized

        "MailerSent" ->
            MailerSent

        _ ->
            Initialized


decodeDealWithHouse : JD.Decoder DealWithHouse
decodeDealWithHouse =
    JD.succeed DealWithHouse
        |> JDP.required "id" JD.int
        |> JDP.required "buyer_id" (JD.nullable JD.int)
        |> JDP.required "seller_id" (JD.nullable JD.int)
        |> JDP.required "house_id" (JD.nullable JD.int)
        |> JDP.required "access_code" JD.string
        |> JDP.required "status" decodeDealStatus
        |> JDP.required "address" JD.string
        |> JDP.required "lat" JD.string
        |> JDP.required "lon" JD.string


decodeDealStatus : JD.Decoder DealStatus
decodeDealStatus =
    JD.string
        |> JD.andThen
            (\string ->
                case string of
                    "Initialized" ->
                        JD.succeed Initialized

                    "MailerSent" ->
                        JD.succeed MailerSent

                    _ ->
                        JD.fail "Invalid DealStatus"
            )
