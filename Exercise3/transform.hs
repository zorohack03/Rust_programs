square :: Int -> Int
square x = x * x
addTen :: Int -> Int
addTen x = x + 10
transformList :: [Int] -> [Int]
transformList = map (addTen . square)
main :: IO ()
main = do
    let numbers = [1, 2, 3, 4]
    print (transformList numbers) 
