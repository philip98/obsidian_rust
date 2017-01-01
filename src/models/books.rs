#[derive(RustcEncodable, RustcDecodable)]
pub struct Book {
    id: Option<usize>,
    isbn: String,
    title: String,
    form: String
}

impl Book {
    pub fn new(id: Option<usize>, isbn: String, title: String, form: String) -> Book {
        Book {
            id: id,
            isbn: isbn,
            title: title,
            form: form
        }
    }
}
