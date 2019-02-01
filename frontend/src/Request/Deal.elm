module Request.Deal exposing (CreateDealInput, UpdateDealInput, createDeal, getDeals, updateDeal)

import Api exposing (ApiData, decodeApiResponse)
import Config exposing (Config)
import Data.Deal as Deal exposing (Deal, DealStatus, decodeDeal)
import Data.Session exposing (Token)
import Data.User as User exposing (User)
import Geocoding exposing (GeocodingResult)
import Http exposing (Request)
import HttpBuilder as HB
import Json.Decode as JD
import Json.Decode.Pipeline as JDP
import Json.Encode as JE
import Url.Builder as UB



-- Types


type alias CreateDealInput =
    { buyer_id : String
    , address : GeocodingResult
    }


type alias UpdateDealInput =
    { status : DealStatus
    }



-- Requests


createDeal : Config -> CreateDealInput -> Request (ApiData Deal)
createDeal config input =
    UB.crossOrigin config.api [ "deals" ] []
        |> HB.post
        |> HB.withJsonBody (input |> encodeCreateDealInput)
        |> HB.withExpect (Http.expectJson (decodeApiResponse decodeDeal))
        |> HB.withHeader "X-API-KEY" config.token
        |> HB.toRequest


updateDeal : Config -> UpdateDealInput -> Int -> Request (ApiData Deal)
updateDeal config input id =
    UB.crossOrigin config.api [ "deals", id |> String.fromInt, "update" ] []
        |> HB.post
        |> HB.withJsonBody (input |> encodeUpdateDealInput)
        |> HB.withExpect (Http.expectJson (decodeApiResponse decodeDeal))
        |> HB.withHeader "X-API-KEY" config.token
        |> HB.toRequest


getDeals : Config -> String -> Request (ApiData (List Deal))
getDeals config id =
    UB.crossOrigin config.api [ "deals" ] [ UB.string "buyer_id" id ]
        |> HB.get
        |> HB.withExpect (Http.expectJson (decodeApiResponse (JD.list decodeDeal)))
        |> HB.withHeader "X-API-KEY" config.token
        |> HB.toRequest



-- Encoders
-- TODO: FIX BUYER ID, add protections


encodeCreateDealInput : CreateDealInput -> JE.Value
encodeCreateDealInput v =
    JE.object
        [ ( "buyer_id", JE.int (String.toInt v.buyer_id |> Maybe.withDefault 0) )
        , ( "address", JE.object v.address )
        ]


encodeUpdateDealInput : UpdateDealInput -> JE.Value
encodeUpdateDealInput v =
    JE.object [ ( "status", JE.string (v.status |> Deal.statusToString) ) ]
