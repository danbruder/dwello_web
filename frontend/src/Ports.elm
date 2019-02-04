port module Ports exposing (logout, setToken)

import Json.Encode as JE


port setToken : JE.Value -> Cmd msg


port logout : () -> Cmd msg
