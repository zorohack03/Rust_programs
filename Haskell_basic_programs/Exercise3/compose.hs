addThenMultiply :: Int -> Int -> Int -> Int
addThenMultiply x y z = (z *) . (+ x) $ y

main :: IO ()
main = do
    print (addThenMultiply 2 3 4) 

