#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::io::{ Read, };
use std::net::{ TcpListener, TcpStream };
use std::str::SplitWhitespace;
use regex::Regex;

mod logger;
mod service;
mod controller;
mod util;
mod db;
mod method;
mod router;

use method::*;
use router::Router;

fn main() {
    const PORT: i32 = 3000;
    let mut route: Router = Router::new();
    let (mut logger, _) = logger::new();
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    logger.yellow().log(&format!("Server is Established at port: {}", PORT));

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 5120];

        stream.read(&mut buffer).unwrap();

        let request = String::from_utf8_lossy(&buffer[..]);
        let request_line = request.lines().next().unwrap();
        let mut request_type = "Request";
        let mut data = "";

        for header in request.split("\n") {
            let header = header.split(": ");
            let header_info: &str = header.clone().nth(0).unwrap();
            let header_data = header.clone().nth(1);

            if header_info == "Sec-Fetch-Dest" && header_data.unwrap() == "image\r" {
                request_type = "Image Request";
            };
            if header_info == "Content-Type" && header_data.unwrap() == "application/json\r" {
                request_type = "Json Request";
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
        
        handle_connection(stream, &mut route, request_type.to_string(), parts, data);
    }
}

fn handle_connection(stream: TcpStream, route: &mut Router, request_type: String, parts: &mut SplitWhitespace, data: &str) {
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

                    route.call_router(stream, request_type, method, url, data);
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