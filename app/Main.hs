module Main where

import System.IO
import System.Environment
import Scan
import Parse
import Lambda
import Machine

main :: IO ()
main = do
    file:_ <- getArgs    
    handle <- openFile file ReadMode
    contents <- hGetContents handle
    
    scan contents
    parse
    lambda
    machine

