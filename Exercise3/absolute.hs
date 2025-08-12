absolute :: Float -> Float
absolute x
  | x < 0     = -x
  | otherwise = x

main :: IO ()
main = do
    print (absolute (-5.6)) 
    print (absolute 3.2)  

