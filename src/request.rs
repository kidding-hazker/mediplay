use std::collections::HashMap;

pub struct Request<'a> {
    pub method: &'a str,
    pub path: &'a str,
    pub headers: HashMap<&'a str, &'a str>,
    pub content: &'a str,
}
impl<'a> Request<'a> {
    pub fn parse(request: &'a str) -> Self {
        let (request, content): (&'a str, &'a str) = match request.split_once("\r\n\r\n") {
            Some((_key, _value)) => (_key, _value),
            None => ("", ""),
        };
        let string: Vec<&str> = request.split("\r\n").collect::<Vec<&str>>()[0]
            .split(" ")
            .collect();
        let method = string[0];
        let path = if string.len() > 1 { string[1] } else { "" };

        Self {
            method: method,
            path: path,
            headers: request.split("\r\n").collect::<Vec<&str>>()[1..]
                .iter()
                .map(|header| header.split_once(": ").unwrap())
                .collect::<HashMap<&'a str, &'a str>>(),
            content: content,
        }
    }
    pub fn to_string(&self) -> String {
        let headers: String = self
            .headers
            .iter()
            .map(|(key, value)| format!("{}: {}\n", key, value))
            .collect();
        format!(
            "{} {} HTTP/1.1\n{}\n\n{}",
            self.method, self.path, headers, self.content
        )
    }
}
