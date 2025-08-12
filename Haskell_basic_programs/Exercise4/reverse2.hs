reverseList :: [a] -> [a] 
reverseList [] = [] 
reverseList (x:xs) = reverseList xs ++ [x] 
main :: IO () 
main = do 
    let result = reverseList [1] 
    print result  

