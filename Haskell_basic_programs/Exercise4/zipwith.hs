listZipWith :: (a -> b -> c) -> [a] -> [b] -> [c] 
listZipWith _ [] [] = [] 
listZipWith f (x:xs) (y:ys) = f x y : listZipWith f xs ys  
main :: IO () 
main = do 
    let result = listZipWith (+) [1, 2, 3] [4, 5, 6] 
    print result  
 
