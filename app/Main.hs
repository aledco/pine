module Main where

import System.IO
import System.Environment
import Scan
import Parse
import Lambda
import Machine

dumpTokens :: [Token] -> IO ()
dumpTokens [] = pure ()
dumpTokens ((Token c t (ln, col)):ts) = do
    putStrLn t
    dumpTokens ts

main :: IO ()
main = do
    file:_ <- getArgs    
    handle <- openFile file ReadMode
    contents <- hGetContents handle
    
    let tokens = scan contents
    let scanError = checkForScanError tokens
    case scanError of
        Just (Token _ _ (ln, col)) -> putStrLn "Scan Error" 
        Nothing -> pure ()
    dumpTokens tokens
 
    parse
    lambda
    machine

