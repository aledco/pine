module Main where

import System.IO
import System.Environment
import Scan
import Parse
import Lambda
import Machine

dumpTokens :: [Token] -> IO ()
dumpTokens tokens = do
    putStrLn "BEGIN TOKEN DUMP"
    _ <- mapM print tokens
    putStrLn "END TOKEN DUMP\n"

main :: IO ()
main = do
    file:_ <- getArgs    
    handle <- openFile file ReadMode
    contents <- hGetContents handle
    
    let tokens = scan contents
    let scanError = checkForScanError tokens
    case scanError of
        Just (Token _ _ (ln, col)) -> error ("scan error at (" ++ show ln ++ ":" ++ show col ++ ")") 
        Nothing -> pure ()
    dumpTokens tokens
 
    parse
    lambda
    machine

