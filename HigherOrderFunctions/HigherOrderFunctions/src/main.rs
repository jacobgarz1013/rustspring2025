// Create a struct Student (major)
struct Student {
    major:String,
}

// Higher order functions update majors
fn update_majors(mut collection: Vec<Student>,majors: Vec<String>,behavior: fn(&mut Student, String),) -> Vec<Student> {
    for (student, major) in collection.iter_mut().zip(majors.into_iter()) {
        behavior(student, major);
    }
    collection
}

// First Order functions, assign_major(student,major_declared)
fn assign_major(s: &mut Student, major: String) {
    s.major = major;
}

fn main() {
    let students = vec![
        Student { major: "".to_string() },
        Student { major: "".to_string() },
        Student { major: "".to_string() },
    ];

    let majors = vec![
        "Computer Science".to_string(),
        "Cybersecurity".to_string(),
        "Mathematics".to_string(),
    ];

    let update_student = update_majors(students, majors, assign_major);

    for (i, student) in update_student.iter().enumerate() {
        println!("Student {} Major: {}", i + 1, student.major);
    }
}