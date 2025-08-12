averageMarks :: [Int] -> Float
averageMarks marks = fromIntegral (sum marks) / fromIntegral (length marks)
displayStudentAverages :: [(String, [Int])] -> [(String, Float)]
displayStudentAverages students = [(name, averageMarks marks) | (name, marks) <- students]
main :: IO ()
main = do
    let students = [("Muthra", [80, 90, 85]), ("Mithun", [70, 75, 80])]
    let result = displayStudentAverages students
    print result  

