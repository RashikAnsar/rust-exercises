use std::fmt::Display;

pub struct Person {
    pub name: String,
    pub age: usize,
}

// not required
impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}\t", self.name, self.age)
    }
}

impl Person {
    pub fn new(name: &str, age: usize) -> Person {
        Person {
            name: name.to_string(),
            age,
        }
    }

    pub fn display(&self) {
        println!("Name: {},\t age: {}.", self.name, self.age)
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}
