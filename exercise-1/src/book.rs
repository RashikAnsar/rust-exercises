pub struct Book {
    pub name: String,
    pub author_name: String,
    pub is_available: bool,
    pub borrowed_by: Option<String>,
}

impl Book {
    pub fn new(name: &str, author_name: &str) -> Self {
        Book {
            name: name.to_string(),
            author_name: author_name.to_string(),
            is_available: true,
            borrowed_by: None,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn is_available(&self) -> bool {
        self.is_available
    }

    pub fn borrowed_person_name(&self) -> String {
        if self.borrowed_by.is_none() {
            return "".to_string();
        } else {
            return self.borrowed_by.clone().unwrap();
        }
    }

    pub fn borrow(&mut self, borrowed_by: &str) {
        self.borrowed_by = Some(borrowed_by.to_string());
        self.is_available = false;
    }

    pub fn return_book(&mut self) {
        self.is_available = true;
    }
}
