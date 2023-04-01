struct Book {
    pages: i32,
    rating: i32,
}

fn display_pages(book: &Book) {
    println!("Pages: {}", book.pages);
}

fn display_rating(book: &Book) {
    println!("Rating: {}", book.rating);
}

fn main() {
    let book = Book {
        pages: 100,
        rating: 5,
    };

    display_pages(&book);
    display_rating(&book);
}