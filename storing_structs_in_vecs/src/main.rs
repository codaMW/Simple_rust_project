//creating a Book struct and a library of books
// using a vector to store multiple books
// and iterating through the vector to print book details
#[derive(Debug)]    
struct Book {
  title: String,
  pages: u32,
}



fn main() {
  
  //creating an instance of Book struct
  // and initializing it with values
  let book1 = Book {
    title: String::from("Harry Poter"),
    pages: 1000,
  };
  
  let book2 = Book {
    title: String::from("Jim & Jack"),
    pages: 900,
  };
  
  let book3 = Book {
    title: String::from("Lord of the Rings"),
    pages: 1150,
  };
  
  //creating a vector to store multiple books

  let library: Vec<Book> = vec![book1, book2, book3]; 
  
  //iterating through the vector using a for loop
  // and printing the title and pages of each book
  
  for books in library {
    println!("{} book has got {} pages", books.title, books.pages);
  }
  
  
  
}