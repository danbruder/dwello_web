module Request.Deal exposing (CreateDealInput, UpdateDealInput, createDealWithHouse, getDealsWithHouses, updateDeal)

import Api exposing (ApiData, decodeApiResponse)
import Config exposing (Config)
import Data.Deal as Deal exposing (DealStatus, DealWithHouse, decodeDealWithHouse)
import Data.Session exposing (Token)
import Data.User as User exposing (User)
import Http exposing (Request)
import HttpBuilder as HB
import Json.Decode as JD
import Json.Decode.Pipeline as JDP
import Json.Encode as JE
import Url.Builder as UB



-- Types


type alias CreateDealInput =
    { buyer_id : String
    , address : String
    , lat : String
    , lon : String
    }


type alias UpdateDealInput =
    { status : DealStatus
    }



-- Requests


createDealWithHouse : Config -> CreateDealInput -> Request (ApiData DealWithHouse)
createDealWithHouse config input =
    UB.crossOrigin config.api [ "deals" ] []
        |> HB.post
        |> HB.withJsonBody (input |> encodeCreateDealInput)
        |> HB.withExpect (Http.expectJson (decodeApiResponse decodeDealWithHouse))
        |> HB.withHeader "X-API-KEY" config.token
        |> HB.toRequest


updateDeal : Config -> UpdateDealInput -> Int -> Request (ApiData DealWithHouse)
updateDeal config input id =
    UB.crossOrigin config.api [ "deals", id |> String.fromInt, "update" ] []
        |> HB.post
        |> HB.withJsonBody (input |> encodeUpdateDealInput)
        |> HB.withExpect (Http.expectJson (decodeApiResponse decodeDealWithHouse))
        |> HB.withHeader "X-API-KEY" config.token
        |> HB.toRequest


getDealsWithHouses : Config -> String -> Request (ApiData (List DealWithHouse))
getDealsWithHouses config id =
    UB.crossOrigin config.api [ "views", "deals-with-houses" ] [ UB.string "buyer_id" id ]
        |> HB.get
        |> HB.withExpect (Http.expectJson (decodeApiResponse (JD.list decodeDealWithHouse)))
        |> HB.withHeader "X-API-KEY" config.token
        |> HB.toRequest



-- Encoders
-- TODO: FIX BUYER ID, add protections


encodeCreateDealInput : CreateDealInput -> JE.Value
encodeCreateDealInput v =
    JE.object
        [ ( "buyer_id", JE.int (String.toInt v.buyer_id |> Maybe.withDefault 0) )
        , ( "address", JE.string v.address )
        , ( "lat", JE.string v.lat )
        , ( "lon", JE.string v.lon )
        ]


encodeUpdateDealInput : UpdateDealInput -> JE.Value
encodeUpdateDealInput v =
    JE.object [ ( "status", JE.string (v.status |> Deal.statusToString) ) ]
