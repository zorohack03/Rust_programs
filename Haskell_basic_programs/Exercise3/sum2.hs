sumList :: [Int] -> Int
sumList = foldr (+) 0

main :: IO ()
main = do
    print (sumList [1, 2, 3])

