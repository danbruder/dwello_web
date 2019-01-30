port module Ports exposing (setToken)

import Json.Encode as JE


port setToken : JE.Value -> Cmd msg
