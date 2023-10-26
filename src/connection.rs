use crate::Request;
use crate::Response;

use std::io::Read;
use std::net::TcpStream;

pub fn connect(mut stream: &TcpStream) {
    let mut data: [u8; 16384] = [0; 16384];
    stream.read(&mut data).unwrap();

    let request: Request = Request::parse(std::str::from_utf8(&data).unwrap());
    let response: Response = if request.path == "/" {
        Response::file("text/html", "web/index.html")
    } else if request.path == "/favicon.ico" {
        Response::file("image/x-icon", "web/favicon.ico")
    } else if request.path == "/content" {
        Response::content()
    } else if request.path.starts_with("/stylesheet/") {
        Response::file("text/css", format!("web/{}", request.path).as_str())
    } else if request.path.starts_with("/images/") {
        let t = request.path[8..].split_once(".").unwrap().1;
        Response::file(
            format!("image/{}", t).as_str(),
            format!("storage/images/{}", &request.path[8..]).as_str(),
        )
    } else if request.path.starts_with("/scripts/") {
        if request.path.ends_with(".ts") {
            Response::file(
                "text/x-typescript",
                format!("web/{}", request.path).as_str(),
            )
        } else {
            Response::file("text/javascript", format!("web/{}", request.path).as_str())
        }
    } else {
        Response::err()
    };
    response.write(&mut stream);
}
