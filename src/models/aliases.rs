#[derive(RustcEncodable, RustcDecodable)]
pub struct Alias {
    id: Option<usize>,
    book_id: usize,
    name: String
}

impl Alias {
    pub fn new(id: Option<usize>, book_id: usize, name: String) -> Alias {
        Alias {
            id: id,
            book_id: book_id,
            name: name
        }
    }
}
