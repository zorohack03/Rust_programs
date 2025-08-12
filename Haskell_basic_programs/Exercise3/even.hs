isEven :: Int -> Bool
isEven x = x `mod` 2 == 0

main :: IO ()
main = do
    print (isEven 4)  
    print (isEven 7)
