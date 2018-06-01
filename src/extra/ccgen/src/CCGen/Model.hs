{-# LANGUAGE DeriveGeneric #-}
{-# LANGUAGE DuplicateRecordFields #-}

module CCGen.Model (
    Seed(..),
    Head(..),
    Size(..),
    Vec2(..),
    Cube(..),
    Move(..),
    Path(..)
) where

-----------------------------------------------------------------------------
-- import

import GHC.Generics (Generic)
import Data.Map (Map)
import Data.Text.Lazy (Text)
import Data.Aeson (
    ToJSON,
    Options,
    toJSON,
    genericToJSON,
    defaultOptions,
    fieldLabelModifier,
    omitNothingFields)

-----------------------------------------------------------------------------
-- Data

data Seed = Seed {
    version :: Int,

    head :: Head,
    size :: Size,
    cube :: [Cube],
    dest :: [Vec2]
} deriving (Show, Generic)

type Head = Map Text Text

type Vec2 = (Int, Int)

data Size = Size {
    width  :: Int,
    height :: Int
} deriving (Show, Generic)

data Cube = Cube {
    type' :: Char,
    body  :: [Vec2],
    move  :: Maybe Move
} deriving (Show, Generic)

data Move = Move {
    loop :: Bool,
    path :: Path
} deriving (Show, Generic)

type Path = [(Char, Int)]

-----------------------------------------------------------------------------
-- Aeson

instance ToJSON Seed where toJSON = genericToJSON option
instance ToJSON Size where toJSON = genericToJSON option
instance ToJSON Cube where toJSON = genericToJSON option
instance ToJSON Move where toJSON = genericToJSON option

-----------------------------------------------------------------------------
-- Helper

option :: Options
option = defaultOptions {
    -- modify "type'" to "type"
    fieldLabelModifier = \s -> if last s == '\'' then init s else s,
    -- ignore "Nothing"
    omitNothingFields = True
}
