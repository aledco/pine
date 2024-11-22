module Scan where

import Data.Char


scan :: String -> [Char]
scan input = scan' input 1 1
    where
        scan' [] l c = []
        scan' (' ':cs) l c = scan' cs l c+1
        scan' ('\n':cs) l c = scan' cs l+1 1      
        scan' (c:cs) line col 
            | isAlpha c = scanIdOrKey c:cs l c
            | otherwise = [c]
            where
                scanIdOrKey s l c = []   

