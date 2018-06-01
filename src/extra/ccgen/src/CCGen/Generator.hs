module CCGen.Generator (
    generate,
    pretty
) where

import Data.Text.Lazy (Text)
import Data.Text.Lazy.Encoding (decodeUtf8)
import Data.Aeson (ToJSON)
import Data.Aeson.Text (encodeToLazyText)
import Data.Aeson.Encode.Pretty (encodePretty)

generate :: (ToJSON a) => a -> Text
generate = encodeToLazyText

pretty :: (ToJSON a) => a -> Text
pretty = decodeUtf8 . encodePretty
