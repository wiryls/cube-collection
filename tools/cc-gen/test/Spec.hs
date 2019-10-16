{-# LANGUAGE DeriveGeneric #-}
{-# LANGUAGE DuplicateRecordFields #-}

import GHC.Generics (Generic)
import qualified Data.Aeson as Aeson
import qualified Data.Aeson.Encode.Pretty as Aeson
import qualified Data.ByteString.Lazy.Char8 as Char8

data Action = I | L | D | U | R
    deriving (Show, Generic)

instance Aeson.ToJSON Action

source :: [(Int, Int)]
source = [(1, 2), (3, 4)]

main :: IO ()
main = do
    putStrLn $ Char8.unpack $ Aeson.encodePretty source
