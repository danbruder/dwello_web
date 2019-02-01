module Request.Deal exposing (CreateDealInput, UpdateDealInput, createDeal, getDeals, updateDeal)

import Api exposing (ApiData, decodeApiResponse)
import Config exposing (Config)
import Data.Deal as Deal exposing (Deal, DealStatus, decodeDeal)
import Data.Session exposing (Token)
import Data.User as User exposing (User)
import Geocoding exposing (ComponentType(..), GeocodingResult, LocationType(..))
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
    , google_address : GeocodingResult
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
        , ( "address", JE.string v.address )
        , ( "google_address", encodeGoogleAddress v.google_address )
        ]


encodeUpdateDealInput : UpdateDealInput -> JE.Value
encodeUpdateDealInput v =
    JE.object [ ( "status", JE.string (v.status |> Deal.statusToString) ) ]


encodeGoogleAddress : GeocodingResult -> JE.Value
encodeGoogleAddress v =
    JE.object
        [ ( "address_components", JE.list encodeAddressComponent v.addressComponents )
        , ( "formatted_address", JE.string v.formattedAddress )
        , ( "geometry", encodeGeometry v.geometry )
        , ( "types", JE.list encodeComponentTypes v.types )
        , ( "place_id", JE.string v.placeId )
        ]


encodeComponentTypes : ComponentType -> JE.Value
encodeComponentTypes v =
    case v of
        StreetAddress ->
            JE.string "StreetAddress"

        Route ->
            JE.string "Route"

        Intersection ->
            JE.string "Intersection"

        Political ->
            JE.string "Political"

        Country ->
            JE.string "Country"

        AdministrativeAreaLevel1 ->
            JE.string "AdministrativeAreaLevel1"

        AdministrativeAreaLevel2 ->
            JE.string "AdministrativeAreaLevel2"

        AdministrativeAreaLevel3 ->
            JE.string "AdministrativeAreaLevel3"

        AdministrativeAreaLevel4 ->
            JE.string "AdministrativeAreaLevel4"

        AdministrativeAreaLevel5 ->
            JE.string "AdministrativeAreaLevel5"

        ColloquialArea ->
            JE.string "ColloquialArea"

        Locality ->
            JE.string "Locality"

        Sublocality ->
            JE.string "Sublocality"

        SublocalityLevel1 ->
            JE.string "SublocalityLevel1"

        SublocalityLevel2 ->
            JE.string "SublocalityLevel2"

        SublocalityLevel3 ->
            JE.string "SublocalityLevel3"

        SublocalityLevel4 ->
            JE.string "SublocalityLevel4"

        SublocalityLevel5 ->
            JE.string "SublocalityLevel5"

        Neighborhood ->
            JE.string "Neighborhood"

        Premise ->
            JE.string "Premise"

        Subpremise ->
            JE.string "Subpremise"

        PostalCode ->
            JE.string "PostalCode"

        NaturalFeature ->
            JE.string "NaturalFeature"

        Airport ->
            JE.string "Airport"

        Park ->
            JE.string "Park"

        PostBox ->
            JE.string "PostBox"

        StreetNumber ->
            JE.string "StreetNumber"

        Floor ->
            JE.string "Floor"

        Room ->
            JE.string "Room"

        Establishment ->
            JE.string "Establishment"

        PointOfInterest ->
            JE.string "PointOfInterest"

        Parking ->
            JE.string "Parking"

        PostalTown ->
            JE.string "PostalTown"

        BusStation ->
            JE.string "BusStation"

        TrainStation ->
            JE.string "TrainStation"

        TransitStation ->
            JE.string "TransitStation"

        PostalCodeSuffix ->
            JE.string "PostalCodeSuffix"

        OtherComponent ->
            JE.string "OtherComponent"


encodeGeometry : Geometry -> JE.Value
encodeGeometry v =
    JE.object
        [ ( "location"
          , JE.object
                [ ( "latitude", JE.float v.location.latitude )
                , ( "longitude", JE.float v.location.longitude )
                ]
          )
        , ( "location_type"
          , case v.locationType of
                Rooftop ->
                    JE.string "Rooftop"

                RangeInterpolated ->
                    JE.string "RangeInterpolated"

                GeometricCenter ->
                    JE.string "GeometricCenter"

                Approximate ->
                    JE.string "Approximate"

                OtherLocationType ->
                    JE.string "OtherLocationType"
          )
        , ( "viewport"
          , JE.object
                [ ( "northeast"
                  , JE.object
                        [ ( "latitude", JE.float v.viewport.northeast.latitude )
                        , ( "longitude", JE.float v.viewport.northeast.longitude )
                        ]
                  )
                , ( "southwest"
                  , JE.object
                        [ ( "latitude", JE.float v.viewport.southwest.latitude )
                        , ( "longitude", JE.float v.viewport.southwest.longitude )
                        ]
                  )
                ]
          )
        ]


encodeAddressComponent : AddressComponent -> JE.Value
encodeAddressComponent v =
    JE.object
        [ ( "long_name"
          , case v.longName of
                Just vp ->
                    JE.string vp

                Nothing ->
                    JE.null
          )
        , ( "short_name"
          , case v.shortName of
                Just vp ->
                    JE.string vp

                Nothing ->
                    JE.null
          )
        , ( "types", JE.list encodeComponentTypes v.types )
        ]



-- Google address stuff


type alias AddressComponent =
    { longName : Maybe String
    , shortName : Maybe String
    , types : List ComponentType
    }


type alias Geometry =
    { location : Location
    , locationType : LocationType
    , viewport : Viewport
    }


type alias Location =
    { latitude : Float
    , longitude : Float
    }


type alias Viewport =
    { northeast : Location
    , southwest : Location
    }
