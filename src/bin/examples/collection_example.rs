
use std::sync::atomic::AtomicUsize;
pub struct Student {
    name: String,
    number: f64,
    grade: char,
}

pub fn collection_example () {

    print!("I am a dummy block!!");

    let size: AtomicUsize = AtomicUsize::new(0);
    size.fetch_add(10, std::sync::atomic::Ordering::Relaxed);
    print!("current size: {:?}", size);
    print!("modified size: {}", size.load(std::sync::atomic::Ordering::Relaxed));

    let names =  vec![String::from("Buddha"), String::from("Shankaracharya"), String::from("Gouryapada"), String::from("Abhinav Gupta"), String::from("Ramakrishna"), String::from("Jesus")];
    let numbers = vec![98.75, 98.5, 97.0, 98.65, 99.0, 90.0];
    let grades = vec!['A', 'A', 'B', 'A', 'A', 'C'];

    let students = names.iter()
    .zip(numbers.iter())
    .zip(grades.iter())
    .map(|((name, &number), &grade)| Student { name: name.clone(), number, grade })
    .collect::<Vec<_>>();

    for student in &students {
        println!("Student {} scrored {} and got {}", student.name, student.number, student.grade);
    }
}