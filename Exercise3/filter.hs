filterEven :: [Int] -> [Int]
filterEven xs = filter even xs

main :: IO ()
main = do
    print (filterEven [1, 2, 3, 4])  

