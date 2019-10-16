{-# LANGUAGE RecordWildCards #-}

module CCGen.Parser (
    parse,
) where

-----------------------------------------------------------------------------
-- import

import Prelude hiding (head)

import qualified System.FilePath as FilePath
import Control.Applicative (liftA2, liftA3)
import Text.Parsec hiding (parse)
import qualified Text.Parsec as Parsec
import Text.Parsec.Text ()
import Data.Bool (bool)
import Data.List (intercalate)
import Data.Maybe (isNothing)
import Data.Text.Lazy (Text, strip, pack)
import Data.Map (Map)
import qualified Data.Map as Map
import qualified CCGen.Model as Model

-----------------------------------------------------------------------------
-- export

parse :: String -> Text -> Either String Model.Seed
parse title = either (Left . show) (Right . id)
            . Parsec.parse (seed' . pack $ FilePath.takeBaseName title) title

-----------------------------------------------------------------------------
-- inner

seed' :: Text -> Parsec Text st Model.Seed
seed' title = liftA2 make (part') (spaces *> head' <* spaces <* eof)
    where
        make seed head =
            let k = pack "title"
                h = bool
                    (Map.insert k title head)
                    head
                    (Map.member k head)
            in seed {
                Model.version = 1,
                Model.head    = h
            }

part' :: Parsec Text st Model.Seed
part' = do
    g <- grid'
    m <- move'
    let w = foldr max 0 . map length $ g
        h = length g
        ms = concat [[(v, mv)| v <- vs]| (vs, mv) <- m]
        is = filter ((/= None) . snd) $ item g
        ds = filter ((== Destination) . snd) is
        cs = filter ((/= Destination) . snd) is
        es = diff ms cs
    if not $ null es then
        unexpected (
            "Path " ++
            (intercalate ", " (map (show . fst) es)) ++
            " does NOT affect any Cube"
        )
    else
        pure Model.Seed {
            version = 0,
            head = Map.empty,
            size = Model.Size {
                width  = w,
                height = h
            },
            cube = link [Model.Cube (toChar t) [p] Nothing | (p, t)    <- diff cs ms]
                     ++ [Model.Cube (toChar t) [p] $Just o | (p, t, o) <- conc cs ms],
            dest = map fst ds
        } where
            diff xs ys = [x | x <- xs, isNothing $ lookup (fst x) ys]
            conc xs ys = [(a, b, d) | (a, b) <- xs, (c, d) <- ys, a == c]

link :: [Model.Cube] -> [Model.Cube]
link cs =
    let w = toChar White
        body = Model.body
        white = (==) w . Model.type'
        fixed = isNothing . Model.move
    in (Model.Cube w (concat [body c| c <- cs, white c && fixed c]) Nothing)
    : [c| c <- cs, not(white c && fixed c)]

head':: Parsec Text st (Map Text Text)
head' = Map.fromList <$> many meta'

meta':: Parsec Text st (Text, Text)
meta' = liftA2 (\k v -> (pack k, strip $ pack v))
    (many alphaNum <* spaces <* char '=')
    (manyTill anyChar (endOfLine *> pure() <|> try eof))

item :: [[a]] -> [((Int, Int), a)]
item = concat . zipWith zip (map (zip [0..] . repeat) [0..])

grid' :: Parsec Text st [[Unit]]
grid' = try $ many1 (many1 unit' <* endOfLine)

unit' :: Parsec Text st Unit
unit' = fromChar <$> oneOf "WwRrBbGgXx "

move' :: Parsec Text st [([Model.Vec2], Model.Move)]
move' = try . many $ liftA3
    (\vs l p -> (vs, Model.Move l p))
    (many1 (vec2' <* spaces))
    (between spaces spaces loop')
    (path' <* manyTill space (endOfLine *> pure() <|> try eof))

vec2' :: Parsec Text st Model.Vec2
vec2' = between (char '[' >> spaces) (spaces >> char ']') $
    liftA3 (\x _ y -> (x, y))
    (integer)
    (spaces *> char ',' <* spaces)
    (integer)

loop' :: Parsec Text st Bool
loop' = f <$> oneOf "TtFf"
    where
        f 'T' = True
        f 't' = True
        f 'F' = False
        f 'f' = False

path' :: Parsec Text st Model.Path
path' = many $ liftA2 (\time action -> (action, time))
    (integer <|> pure 1)
    (oneOf "ILDUR")

-----------------------------------------------------------------------------
-- helper parser

integer :: Parsec Text st Int
integer = read <$> many1 digit

-----------------------------------------------------------------------------
-- type

data Unit = None | White | Red | Blue | Green | Destination
    deriving (Eq)

toChar :: Unit -> Char
toChar White = 'W'
toChar Red   = 'R'
toChar Blue  = 'B'
toChar Green = 'G'

fromChar :: Char -> Unit
fromChar ' ' = None
fromChar 'W' = White
fromChar 'w' = White
fromChar 'R' = Red
fromChar 'r' = Red
fromChar 'B' = Blue
fromChar 'b' = Blue
fromChar 'G' = Green
fromChar 'g' = Green
fromChar 'X' = Destination
fromChar 'x' = Destination

{- References:

[An introduction to parsing text in Haskell with Parsec]
(http://unbui.lt/#!/post/haskell-parsec-basics)
[An Introduction to the Parsec Library]
(https://kunigami.blog/2014/01/21/an-introduction-to-the-parsec-library/)
[Real World Haskell - Chapter 16. Using Parsec]
(http://book.realworldhaskell.org/read/using-parsec.html)
[In Haskell, how do you trim whitespace from the beginning and end of a string?]
(https://stackoverflow.com/a/6270382)
-}
