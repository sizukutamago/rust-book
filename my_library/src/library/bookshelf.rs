use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

use super::book::Book;

pub struct Bookshelf {
    books: Vec<Book>,
    matcher: SkimMatcherV2,
}

impl Bookshelf {
    pub fn new() -> Self {
        let matcher = SkimMatcherV2::default();
        Self {
            books: Vec::new(),
            matcher: matcher,
        }
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    pub fn search_books(&self, title_query: &str) -> Vec<&Book> {
        self.books
            .iter()
            .filter(|book| self.matcher.fuzzy_match(&book.title, title_query).is_some())
            .collect()
    }

    pub fn search_books_exact(&self, title_query: &str) -> Vec<&Book> {
        self.books
            .iter()
            .filter(|book| book.title == title_query)
            .collect()
    }

    pub fn search_books_partial(&self, title_query: &str) -> Vec<&Book> {
        self.books
            .iter()
            .filter(|book| book.title.contains(title_query))
            .collect()
    }

    pub fn remove_book(&mut self, book: &Book) -> Option<Book> {
        todo!("implement");
    }

    pub fn take_all_books(&mut self) -> Vec<Book> {
        todo!("implement");
    }
}

#[cfg(test)]
mod tests {
    use super::Book;
    use super::Bookshelf;

    #[test]
    fn test_bookshelf() {
        let mut shelf = Bookshelf::new();

        let book1 = Book::new("すごいぞChatGPT! AIを使って学ぼうRust!", "山田太郎");
        let book2 = Book::new("Pythonプログラミング", "山田花子");

        shelf.add_book(book1);
        shelf.add_book(book2);

        let found_books = shelf.search_books("chatgpt");
        println!("{:?}", found_books);
    }
}
