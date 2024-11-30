module Parse where

import Scan

data Expr
    = Var String
    | Lit Int
    | App Expr Expr

type Symbol = String

data Fn = (Symbol, [Symbol], Expr) -- fn name, parameters, body

-- TODO need SymbolTable
    
parse :: [Token] -> IO ()
parse tokens = parseFns tokens
    where
        parseFns :: [Token] -> ??
        parseFns [] = []
        parseFns tokens = fn:(parseFns rest) 
            where
                parseFn :: [Token] -> (Fn, [Token])
                parseFn tokens = (??, ??) -- TODO

                (fn, rest) = parseFn tokens


