multiplyElements :: Num a => [a] -> a -> [a] 
multiplyElements xs n = [x * n | x <- xs] 
main :: IO () 
main = do 
        let result = multiplyElements [1, 2, 3, 4] 0
        print result   

