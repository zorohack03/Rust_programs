swapTuple :: (a, b) -> (b, a) 
swapTuple (x, y) = (y, x) 
main :: IO () 
main = do  
    let result = swapTuple ("Muthra", 100) 
    print result  
