module Parse where

import Scan

data Expr t
    = Var String
    | Lit t
    | Neg (Expr t) 
    | Add (Expr t) (Expr t)
    | Sub (Expr t) (Expr t)
    | Mul (Expr t) (Expr t)
    | Div (Expr t) (Expr t)
    | App (Expr t) (Expr t)

data Fn = ?? -- TODO
    
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


