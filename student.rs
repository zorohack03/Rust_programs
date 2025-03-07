#[derive(Debug)]
struct Student {
    name: String,
    age: u32,
    grade: f32,
}

impl Student {
    fn new(name: &str, age: u32, grade: f32) -> Self {
        Student {
            name: name.to_string(),
            age,
            grade,
        }
    }

    fn update_grade(&mut self, new_grade: f32) {
        self.grade = new_grade;
    }

    fn display_info(&self) {
        println!("Name: {}, Age: {}, Grade: {}", self.name, self.age, self.grade);
    }
}

fn main() {
    let mut students = Vec::new();

    let student1 = Student::new("Muthra", 20, 90.5);
    let student2 = Student::new("Ganesh", 22, 85.3);

    students.push(student1);
    students.push(student2);

    for student in &students {
        student.display_info();  // Borrowing student information immutably
    }

    // Modify a student's grade
    if let Some(student) = students.get_mut(0) {
        student.update_grade(95.0);  // Mutable borrow
    }

    println!("After grade update:");
    for student in &students {
        student.display_info();
    }
}

