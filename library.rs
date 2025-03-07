#[derive(Debug, Clone)]
struct Book {
    title: String,
    author: String,
    isbn: String,
    is_issued: bool,
}

struct Library {
    books: Vec<Book>,
}

struct Borrower {
    name: String,
    borrowed_books: Vec<Book>,
}

impl Library {
    fn new() -> Self {
        Library { books: Vec::new() }
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn issue_book(&mut self, isbn: &str, borrower: &mut Borrower) -> Option<Book> {
        if let Some(index) = self.books.iter().position(|b| b.isbn == isbn && !b.is_issued) {
            let mut book = self.books[index].clone();  // Clone the book first
            book.is_issued = true;  // Mark it as issued

            self.books.remove(index);  // Remove the original book from library

            borrower.borrowed_books.push(book.clone());  // Add the cloned book to the borrower’s borrowed list

            Some(book)
        } else {
            None
        }
    }

    fn return_book(&mut self, isbn: &str, borrower: &mut Borrower) -> Option<Book> {
        if let Some(index) = borrower.borrowed_books.iter().position(|b| b.isbn == isbn) {
            let book = borrower.borrowed_books.remove(index);
            self.books.push(book.clone()); // Add the returned book back to the library
            Some(book)
        } else {
            None
        }
    }
}

impl Borrower {
    fn new(name: &str) -> Self {
        Borrower {
            name: name.to_string(),
            borrowed_books: Vec::new(),
        }
    }
}

fn main() {
    let mut library = Library::new();
    let mut borrower = Borrower::new("Alice");

    // Add a book to the library
    library.add_book(Book {
        title: "dark matter".to_string(),
        author: "muthra".to_string(),
        isbn: "123456789".to_string(),
        is_issued: false,
    });

    // Issue a book
    if let Some(book) = library.issue_book("123456789", &mut borrower) {
        println!("Book '{}' has been issued to {}.", book.title, borrower.name);
    } else {
        println!("Book with ISBN '123456789' is not available.");
    }

    // Print borrowed books
    println!("Borrowed books: {:?}", borrower.borrowed_books);

    // Return the book
    if let Some(book) = library.return_book("123456789", &mut borrower) {
        println!("Book '{}' has been returned to the library.", book.title);
    } else {
        println!("Book with ISBN '123456789' not found in borrowed books.");
    }

    // Print the library's collection after return
    println!("Library's books after return: {:?}", library.books);
}

