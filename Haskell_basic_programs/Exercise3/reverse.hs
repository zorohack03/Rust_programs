reverseList :: [a] -> [a]
reverseList = reverse

main :: IO ()
main = do
    print (reverseList [1, 2, 3]) 

