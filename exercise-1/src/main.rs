mod book;
mod library;
mod person;

use crate::person::Person;
use crate::{book::Book, library::Library};

fn main() {
    let person1 = Person::new("John", 25);
    person1.display();

    let book1 = Book::new("Thinking in Systems", "Donella Meadows");
    let book2 = Book::new("Database Internals", "Alex Petrov ");

    let mut library = Library::new();
    library.add_book(book1);
    library.add_book(book2);

    library.list_available_books();
    library.checkout_book("Thinking in Systems", &person1.name());
    library.list_checkout_books();
    library.list_available_books();
    library.return_book("Thinking in Systems", person1.name());
    library.list_available_books();
}
