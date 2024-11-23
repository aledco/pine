module Parse where

import Scan

data BinOp
    = Add
    | Sub
    | Mul
    | Div
    | Eq
    | Neq

data Expr t
    = Var String
    | Lit t
    | Un UnOp (Expr t)
    | Bin BinOp (Expr t) (Expr t)
    | App (Expr t) (Expr t)   
    
parse :: [Token] -> IO ()
parse tokens = putStrLn "Hello from Parse!"

