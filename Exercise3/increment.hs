incrementEach :: [Int] -> [Int]
incrementEach = map (+1)

main :: IO ()
main = do
    print (incrementEach [1, 2, 3]) 

