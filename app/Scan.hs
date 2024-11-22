module Scan where

import Data.Char

data TokenType 
    = Plus
    | Minus
    | Times
    | Divide
    | Keyword
    | Id
    | IntLit
    | Error

data Token = Token TokenType String (Int, Int)

-- TODO this is builing tokens backward, need to reverse list
scan :: String -> [Token]
scan input = reverse (scan' input 1 1)
    where
        scan' :: String -> Int -> Int -> [Token]
        scan' [] _ _ = []
        scan' (' ':cs) ln col = scan' cs ln (col+1)
        scan' ('\n':cs) ln col = scan' cs (ln+1) 1
 
        scan' ('+':cs) ln col = (Token Plus "+" (ln, col)):(scan' cs ln (col+1)) 
        scan' ('-':cs) ln col = (Token Minus "-" (ln, col)):(scan' cs ln (col+1))      
        scan' ('*':cs) ln col = (Token Times "*" (ln, col)):(scan' cs ln (col+1))      
        scan' ('/':cs) ln col = (Token Divide "/" (ln, col)):(scan' cs ln (col+1))
        
        scan' (c:cs) ln col 
            | isAlpha c = t1:(scan' s1 ln1 col1)
            | isDigit c = t2:(scan' s2 ln2 col2)
            | otherwise = [Token Error "" (ln, col)]
            where
                scanIdOrKey :: String -> Int -> Int -> (Token, String, Int, Int)
                scanIdOrKey (c:cs) ln col = ((Token Plus "Test" (ln, col)), cs, ln, col+1)

                (t1, s1, ln1, col1)  = scanIdOrKey (c:cs) ln col
                
                scanIntLit :: String -> Int -> Int -> (Token, String, Int, Int)
                scanIntLit (c:cs) ln col = ((Token Plus "Test" (ln, col)), cs, ln, col+1)

                (t2, s2, ln2, col2)  = scanIdOrKey (c:cs) ln col

checkForScanError :: [Token] -> Maybe Token
checkForScanError tokens = checkForScanError' t
    where
        t = last tokens
        checkForScanError' (Token Error s p) = Just t
        checkForScanError' _ = Nothing

