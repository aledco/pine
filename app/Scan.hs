module Scan where

import Data.Char

data TokenType 
    = Plus
    | Minus
    | Times
    | Divide
    | Equals
    | NotEquals
    | Keyword
    | Id
    | IntLit
    | Error
    deriving Show

data Token = Token TokenType String (Int, Int)

-- | Scans the input program, producing a list of tokens.
scan :: String -> [Token]
scan input = scan' input 1 1
    where
        scan' :: String -> Int -> Int -> [Token]
        scan' [] _ _ = []
        scan' (' ':cs) ln col = scan' cs ln (col+1)
        scan' ('\n':cs) ln _ = scan' cs (ln+1) 1
        
        -- scan for operators
        scan' ('+':cs) ln col = (Token Plus "+" (ln, col)):(scan' cs ln (col+1)) 
        scan' ('-':cs) ln col = (Token Minus "-" (ln, col)):(scan' cs ln (col+1))      
        scan' ('*':cs) ln col = (Token Times "*" (ln, col)):(scan' cs ln (col+1))      
        scan' ('/':cs) ln col = (Token Divide "/" (ln, col)):(scan' cs ln (col+1))
        scan' ('=':cs) ln col = (Token Equals "=" (ln, col)):(scan' cs ln (col+1))
        scan' ('!':'=':cs) ln col = (Token NotEquals "!=" (ln, col)):(scan' cs ln (col+2))
        
        -- scan for keywords
        scan' ('f':'n':cs) ln col = (Token Keyword "fn" (ln, col)):(scan' cs ln (col+2))

        -- scan for identifiers and literals
        scan' (c:cs) ln col 
            | isAlpha c = t1:(scan' s1 ln1 col1)
            | isDigit c = t2:(scan' s2 ln2 col2)
            | otherwise = [Token Error "" (ln, col)]
            where 
                (t1, s1, ln1, col1)  = scanId (c:cs) ln col
                (t2, s2, ln2, col2)  = scanIntLit (c:cs) ln col
       
        -- scan for an identifier 
        scanId :: String -> Int -> Int -> (Token, String, Int, Int)
        scanId s ln col = ((Token Id t (ln, col)), ns, nln, ncol)
            where
                scanId' :: String -> Int -> Int -> (String, String, Int, Int)
                scanId' [] ln' col' = ([], [], ln', col')
                scanId' (c:cs) ln' col'
                    | isAlphaNum c = (c:t', ns', nln', ncol')
                    | otherwise = ([], c:cs, ln', col')
                    where
                        (t', ns', nln', ncol') = scanId' cs ln' (col'+1)
                (t, ns, nln, ncol) = scanId' s ln col 
        
        -- scan for an integer literal
        scanIntLit :: String -> Int -> Int -> (Token, String, Int, Int)
        scanIntLit s ln col = ((Token IntLit t (ln, col)), ns, nln, ncol)
            where
                scanIntLit' :: String -> Int -> Int -> (String, String, Int, Int)
                scanIntLit' [] ln' col' = ([], [], ln', col')
                scanIntLit' (c:cs) ln' col' 
                    | isDigit c = (c:t', ns', nln', ncol')
                    | otherwise = ([], c:cs, ln', col')
                    where
                        (t', ns', nln', ncol') = scanIntLit' cs ln' (col'+1)
                (t, ns, nln, ncol) = scanIntLit' s ln col


-- | Checks for an error in the list of tokens, and returns the error if it exists.
checkForScanError :: [Token] -> Maybe Token
checkForScanError tokens = checkForScanError' t
    where
        t = last tokens
        checkForScanError' (Token Error _ _) = Just t
        checkForScanError' _ = Nothing

--                      --
--      DEBUGGING       --
--                      --
instance Show Token where
    show (Token tt s (ln, col)) = "Token " ++ show tt ++ " " ++ s ++ " (" ++ show ln ++ ", " ++ show col ++ ")"
 
