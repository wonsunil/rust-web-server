#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::io::{ Read, };
use std::net::{ TcpListener, TcpStream };
use std::str::SplitWhitespace;
use std::collections::HashMap;
use regex::Regex;

mod http;
mod logger;
mod service;
mod controller;
mod util;
mod db;
mod method;
mod router;
mod session;

use method::*;
use router::Router;
use session::Session::Session;
use session::SessionStorage;

fn main() {
    const PORT: i32 = 3000;
    let mut session_storage = SessionStorage::new();
    let mut route: Router = Router::new();
    let (mut logger, _) = logger::new();
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    logger.yellow().log(&format!("Server is Established at port: {}", PORT));

    let mut test: HashMap<&str, String> = HashMap::new();
    let mut t_session: HashMap<&str, String> = HashMap::new();

    t_session.insert("as", "asd".into());
    t_session.insert("sa", "dsa".into());

    test.insert("test", "test".into());
    test.insert("session", util::json::stringify(t_session));

    let s = util::json::stringify(test.clone());
    let p = util::json::parse(&s);

    println!("{:?}", test);
    println!("{:?}", s);
    println!("{:?}", p);
    println!("{:?}", p.get("session"));

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 5120];

        stream.read(&mut buffer).unwrap();

        let request = String::from_utf8_lossy(&buffer[..]);
        let request_line = request.lines().next().unwrap();
        let mut request_type = "Request";
        let mut data = "";

        let cookie_regex = Regex::new("Cookie: [0-9a-zA-Z]*=[0-9a-zA-Z]*").unwrap();
        let mut session: Session = Session::empty();

        if cookie_regex.is_match(&request) {
            let cookie = cookie_regex.find(&request).unwrap();
            let cookie = &request[cookie.start()..cookie.end()];
            let session_id = cookie.split("=").clone().nth(1).unwrap();
            
            session = session_storage.get(session_id.into());
        };

        for header in request.split("\n") {
            let header = header.split(": ");
            let header_info: &str = header.clone().nth(0).unwrap();
            let header_data = header.clone().nth(1);

            if header_info == "Sec-Fetch-Dest" {
                match header_data.unwrap() {
                    "image\r" => {
                        request_type = "Image Request"
                    },
                    "style\r" => {
                        request_type = "Stylesheet Request"
                    },
                    "script\r" => {
                        request_type = "Javascript Request"
                    },
                    _ => {
                        
                    }
                };
            };
            if header_info == "Content-Type" && header_data.unwrap() == "application/json\r" {
                request_type = "JSON Request";
            };

            if header_info == "Content-Length" {
                let regex = Regex::new(r#"\{[a-zA-Z0-9":,]*\}*"#).unwrap();
                let result = regex.find(&request).unwrap();
                let start = result.start();
                let end = result.end();

                data = &request[start..end];
            };
        }

        let color_request_type = logger.bright_green().get_color_text(request_type);
        let color_text = logger.yellow().get_color_text("[");

        println!("");
        logger.log(&format!("{} {}", color_request_type, color_text));
        
        let parts = &mut request_line.split_whitespace();
        
        handle_connection(stream, &mut route, request_type.to_string(), parts, data, &mut session, &mut session_storage);
    }
}

fn handle_connection(stream: TcpStream, route: &mut Router, request_type: String, parts: &mut SplitWhitespace, data: &str, session: &mut Session, session_storage: &mut SessionStorage::SessionStorage) {
    let (mut logger, _) = logger::new();

    match parts.next() {
        Some(method) => {
            logger.log(&format!("   Method: {}", method));

            let method: Method = get_method(method);

            match parts.next() {
                Some(url) => {
                    logger.log(&format!("   Request Url: {}", url));

                    if route.is_ignore_url(&url) {
                        logger.log("   \x1b[33mError:\x1b[0m \x1b[31mNot Allowed Request Url\x1b[0m");
                        logger.log("]");

                        return;
                    };

                    route.call_router(stream, request_type, method, url, data, session, session_storage);
                },
                None => {
                    logger.log("   \x1b[33mError:\x1b[0m \x1b[31mRequest url is not allowed\x1b[0m");
                    logger.log("]");
                }
            }
        },
        None => {
            logger.log("   \x1b[33mError:\x1b[0m \x1b[31mHttp Method is not support\x1b[0m");
            logger.log("]");
        }
    }
}