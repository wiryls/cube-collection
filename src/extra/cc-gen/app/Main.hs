module Main where

import Prelude hiding (writeFile, readFile)

import Control.Monad(forM)
import Control.Applicative (pure)
import System.IO (openFile, IOMode(..))
import System.Directory (doesFileExist)
import System.Environment (getArgs)
import System.FilePath ((-<.>))
import Data.Traversable(for)
import Data.Bool (bool)
import Data.Text.Lazy (Text)
import Data.Text.Lazy.IO (writeFile, readFile)


import CCGen.Generator
import CCGen.Parser


path :: IO ([Either String String])
path = getArgs >>= \xs -> for xs $ \x -> bool
        (Left  $ "File " ++ x ++ " not exists")
        (Right $ x)
        <$> (doesFileExist x)

main :: IO ()
main = path >>= mapM_ (either
    (putStrLn)
    (\x -> parse x <$> readFile x >>= either
        (putStrLn)
        (writeFile (x -<.> "json") . generate)))
