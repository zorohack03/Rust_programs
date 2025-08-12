filterEven :: [Int] -> [Int] 
filterEven xs = filter odd xs 
main :: IO () 
main = do 
    let result = filterEven [1, 2, 3, 4, 5, 6] 
    print result
