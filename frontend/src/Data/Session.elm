module Data.Session exposing (Session, Token)


type alias Session =
    { token : Token
    }


type alias Token =
    String
