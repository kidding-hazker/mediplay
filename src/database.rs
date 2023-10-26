use sqlite;

pub struct Database {
    connection: sqlite::Connection,
}

impl Database {
    pub fn connect() -> Self {
        Self {
            connection: sqlite::open("storage/database.db").unwrap(),
        }
    }
    pub fn users<'a>() -> Vec<&'a str> {
        Vec::new()
    }
    pub fn videos<'a>(user: &str) -> Vec<&'a str> {
        Vec::new()
    }
    pub fn audios<'a>(user: &str) -> Vec<&'a str> {
        Vec::new()
    }
    pub fn avatar<'a>(user: &str) -> &'a str {
        ""
    }
}
