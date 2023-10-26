use std::fs;
use std::io::Write;
use std::net::TcpStream;

pub struct Response {
    code: u16,
    headers: Vec<String>,
    content: Vec<u8>,
}
impl Response {
    pub fn file(content_type: &str, path: &str) -> Self {
        let content = fs::read(path).unwrap();

        Self {
            code: 200,
            headers: Vec::from([
                format!("Content-Type: {}", content_type),
                format!("Content-Length: {}", content.len()),
            ]),
            content: content,
        }
    }
    pub fn ok() -> Self {
        Self {
            code: 200,
            headers: Vec::new(),
            content: Vec::new(),
        }
    }
    pub fn err() -> Self {
        let content = fs::read("web/error.html").unwrap();

        Self {
            code: 404,
            headers: Vec::from([
                "Content-Type: text/html".to_string(),
                format!("Content-Length: {}", content.len()),
            ]),
            content: content,
        }
    }
    pub fn content() -> Self {
        let videos = fs::read_dir("storage/videos")
            .unwrap()
            .map(|dir| {
                dir.unwrap()
                    .file_name()
                    .to_str()
                    .unwrap()
                    .to_string()
                    .split_once(".")
                    .unwrap()
                    .0
                    .to_string()
            })
            .collect::<Vec<String>>();
        let audios = fs::read_dir("storage/audios")
            .unwrap()
            .map(|dir| {
                dir.unwrap()
                    .file_name()
                    .to_str()
                    .unwrap()
                    .to_string()
                    .split_once(".")
                    .unwrap()
                    .0
                    .to_string()
            })
            .collect::<Vec<String>>();
        let images = fs::read_dir("storage/images")
            .unwrap()
            .map(|dir| {
                dir.unwrap()
                    .file_name()
                    .to_str()
                    .unwrap()
                    .to_string()
            })
            .collect::<Vec<String>>();
        let mut content = format!(
            "\"videos\": {:?}, \"audios\": {:?}, \"images\": {:?}",
            videos, audios, images
        );
        content.insert(0, '{');
        content.insert(content.len(), '}');
        Self {
            code: 200,
            headers: Vec::from([
                "Content-Type: application/json".to_string(),
                format!("Content-Length: {}", content.len()),
            ]),
            content: content.into_bytes(),
        }
    }
    pub fn write(&self, mut stream: &TcpStream) {
        stream
            .write(
                format!(
                    "HTTP/1.1 {}\r\n{}\r\n\r\n",
                    self.code,
                    self.headers.join("\r\n"),
                )
                .as_bytes(),
            )
            .unwrap();
        stream.write(&self.content).unwrap();
        stream.flush().unwrap()
    }
}
