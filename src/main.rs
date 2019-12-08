#[derive(Debug)]

struct Student {
    name: String,
    grade: String,
    age: i32,
    percentage: f32,
}

impl Student {
    fn new(name: String, grade: String, age: i32, percentage: f32) -> Student {
        Student {
            name,
            grade,
            age,
            percentage,
        }
    }
}

impl Student {
    fn new_1(&self) -> f32 {
        self.percentage
    }
}

fn main() {
    let student1 = Student { name: String::from("Qutaiba"), grade: String::from("A") , age: 33, percentage: 90.1 };
        
    println!("_________________________");
    println!("Using Associated Function");
    println!("_________________________\n");
    
    println!("{:#?}", student1);

    println!("\n_________________________");
    println!("\nUsing Methods Function");
    println!("_________________________\n");

    println!("{}\n", student1.new_1());
}