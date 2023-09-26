use crate::book::Book;

pub struct Library {
    books: Vec<Book>,
}

impl Library {
    pub fn new() -> Self {
        Library { books: Vec::new() }
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    pub fn checkout_book(&mut self, name: &str, borrowed_by: &str) {
        for book in self.books.iter_mut() {
            if book.name() == name && book.is_available() {
                book.borrow(borrowed_by);
                println!("\n{} borrowed '{}'", borrowed_by, name);
            }
        }
    }

    pub fn return_book(&mut self, name: &str, borrowed_by: &str) {
        for book in self.books.iter_mut() {
            if book.name() == name && !book.is_available() {
                book.return_book();
                println!("\n{} returned '{}'", borrowed_by, name);
            }
        }
    }

    pub fn list_checkout_books(&self) {
        println!("\n\nCheckout books");
        println!("{:<24} | {:<24} | {:<24}", "Title", "Author", "Borrowed By");
        self.books
            .iter()
            .filter(|book| !book.is_available)
            .for_each(|book| {
                println!(
                    "{:<24} | {:<24} | {:<24}",
                    book.name,
                    book.author_name,
                    book.borrowed_person_name()
                )
            });
    }

    pub fn list_available_books(&self) {
        println!("\n\nAvailable books");
        println!("{:<24} | {:<24}", "Title", "Author");
        self.books
            .iter()
            .filter(|book| book.is_available)
            .for_each(|book| println!("{:<24} | {:<24}", book.name, book.author_name));
    }
}
