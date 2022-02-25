#![allow(dead_code)]

use std::fs;
use std::io::Write;
use std::net::TcpStream;
use std::collections::HashMap;
use regex::Regex;
use substring::Substring;

use crate::http::request::HttpRequest;
use crate::method::*;
use crate::logger;
use crate::session;
use crate::util::json;
use crate::Session;
use crate::SessionStorage::SessionStorage;
use crate::controller::{ MainController, UserController };

type Expr = Box<dyn Fn(HttpRequest) -> String>;

pub struct Router{
    pub get_route: HashMap<String, (String, Expr)>,
    pub post_route: HashMap<String, (String, Expr)>,
    pub ignore_url: Vec<String>
}

impl Router{
    pub fn add_router<H>(&mut self, method: Method, url: &str, name: &str, handler: H)
    where
        H: Fn(HttpRequest) -> String + 'static
    {
        let (url, name) = (url.to_string(), name.to_string());

        match method{
            Method::Get => {
                self.get_route.insert(url, (name, Box::new(handler)));
            },
            Method::Post => {
                self.post_route.insert(url, (name, Box::new(handler)));
            },
            _ => {

            }
        }
    }

    pub fn find_router(&mut self, method: &Method, url: &str) -> Option<&(String, Box<dyn Fn(HttpRequest) -> String>)> {
        let (mut logger, _) = logger::new();

        match method{
            Method::Get => {
                let mut get_route = self.get_route.iter();

                let find = get_route.find(|route| {
                    let (target_url, _) = route;
                    let regex = Regex::new(":").unwrap();

                    if target_url[0..] == url.to_string() {
                        return true;
                    }else {
                        match regex.is_match(url) {
                            true => {
                                let start_index = target_url.find(":").unwrap();
                                let end = target_url[start_index..].find("/");

                                if url.starts_with(&target_url[..start_index]) {
                                    let result = match end {
                                        Some(end_index) => {
                                            println!("{}, {}, {}", target_url, start_index, end_index);
    
                                            true
                                        },
                                        None => {
                                            logger.green().log(&format!("   Mapped Url: {}", target_url));
    
                                            let reg_url = target_url.replace(&target_url[start_index..], "[0-9a-zA-Z]");
                                            let regex: Regex = Regex::new(&reg_url).unwrap();
                                            
                                            println!("   \x1b[32mGenerated Regex: \x1b[0m\x1b[33m[\x1b[0m\x1b[34m0\x1b[0m\x1b[33m-\x1b[0m\x1b[34m9\x1b[0m\x1b[34ma\x1b[0m\x1b[33m-\x1b[0m\x1b[34mz\x1b[0m\x1b[34mA\x1b[0m\x1b[33m-\x1b[0m\x1b[34mZ\x1b[0m\x1b[33m]\x1b[0m");
                                            if regex.is_match(url) {
                                                logger.green().log(&format!("   Pattern \x1b[33m{}{} \x1b[32mis matched at\x1b[0m \x1b[33m{}\x1b[0m", &target_url[..6], "[\x1b[0m\x1b[34m0\x1b[0m\x1b[33m-\x1b[0m\x1b[34m9\x1b[0m\x1b[34ma\x1b[0m\x1b[33m-\x1b[0m\x1b[34mz\x1b[0m\x1b[34mA\x1b[0m\x1b[33m-\x1b[0m\x1b[34mZ\x1b[0m\x1b[33m]\x1b[0m", url));
                                            }else {
                                                logger.green().log(&format!("   Pattern  {}  is not matched at {}", target_url, regex));
                                            }

                                            regex.is_match(url)
                                        }
                                    };
    
                                    return result;   
                                };
                            },
                            false => {
                                
                            }
                        }

                        return false;
                    }
                });

                match find {
                    Some(route) => Some(route.1),
                    None => None
                }
            },
            Method::Post => {
                let mut _index = 0;
                let mut post_route = self.post_route.iter();
                let _limit: usize = self.post_route.len();

                let find = post_route.find(|route| {
                    let (target_url, _) = route;

                    if target_url[..] == url.to_string() {
                        return true;
                    }else {
                        return false;
                    }
                });

                match find {
                    Some(route) => Some(route.1),
                    None => None
                }
            },
            _ => {
                None
            }
        }
    }

    pub fn call_router(&mut self, mut stream: TcpStream, request_type: String, method: Method, url: &str, data: &str, session: Session) {
        let (mut logger, mut error_logger) = logger::new();
        let response = self.find_router(&method, url);

        match response{
            Some(route_tuple) => {
                let (name, handler) = route_tuple;
    
                logger.log(&format!("   Method: {}", name));
    
                let data = match data.len() {
                    0 => url.to_string(),
                    _ => data.to_string()
                };
                
                if request_type == "Request" && url != "/favicon.ico" {
                    let content = get_content_format("public/view/".to_owned() + &handler(data) + ".html");

                    stream.write(content.as_bytes()).unwrap();
                } else if request_type == "Image Request" || url == "/favicon.ico" {
                    let content = get_image_content("public/image/".to_owned() + &handler(data));
                    let formatted_content = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: image/{}\r\nContent-Length: {}\r\n\r\n",
                        content.0,
                        content.1.len(),
                    );

                    stream.write(formatted_content.as_bytes()).unwrap();
                    stream.write_all(&content.1).unwrap();
                }else {
                    let mut t = json::parse(&data);
                    t.insert("asdf".into(), "asdf".into());
                    t.insert_map("session".into(), session.get_data());

                    let contents = handler(json::stringify(t.get_data()));
                    let parse_content = json::parse(&contents);

                    logger.log(&format!("   Response {:?}", parse_content));

                    let content = format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nSet-Cookie: sessionid={}\r\n\r\n{}",
                        contents.len(),
                        session.get_id(),
                        contents
                    );

                    stream.write(content.as_bytes()).unwrap();
                };

                stream.flush().unwrap();

                logger.log("]");
            },
            None => {
                let css_request_regex = Regex::new("/css/[a-zA-Z]*").unwrap();
                let js_request_regex = Regex::new("/js/[a-zA-Z]*").unwrap();

                if css_request_regex.is_match(url) {
                    match self.get_route.get("/css") {
                        Some(route_tuple) => {
                            let (name, handler) = route_tuple;

                            logger.log(&format!("   Handler: {}", name));

                            let content = get_resource_format("public/".to_owned() + &handler(url.substring(1, url.len()).to_string() + ".css"));

                            if content == "Error" {
                                logger.log("]");

                                return;
                            };

                            stream.write(content.as_bytes()).unwrap();
                            stream.flush().unwrap();

                            logger.log("]");
                        },
                        None => {
                            error_logger.log("Invalid css address");
                        }
                    };

                    return;
                };

                if js_request_regex.is_match(url) {
                    match self.get_route.get("/js") {
                        Some(route_tuple) => {
                            let (name, handler) = route_tuple;

                            logger.log(&format!("   Handler: {}", name));

                            let content = get_resource_format("public/".to_owned() + &handler(url.substring(1, url.len()).to_string() + ".js"));

                            if content == "Error" {
                                logger.log("]");

                                return;
                            };

                            stream.write(content.as_bytes()).unwrap();
                            stream.flush().unwrap();

                            logger.log("]");
                        },
                        None => {
                            error_logger.log("Invalid js address");
                        }
                    };

                    return;
                };

                logger.log("   \x1b[33mError:\x1b[0m \x1b[31mNot Mapped Request Url\x1b[0m");
                logger.log("]");
            }
        }
    }

    pub fn ignore(&mut self, url: &str) {
        self.ignore_url.push(url.into());
    }

    pub fn is_ignore_url(&mut self, url: &str) -> bool {
        self.ignore_url.contains(&url.to_string())
    }

    pub fn set(&mut self, controller: Router) {
        controller.get_route.into_iter().for_each(|(url, t)| {
            self.get_route.insert(url, t);
        });
        controller.post_route.into_iter().for_each(|(url, t)| {
            self.post_route.insert(url, t);
        });
    }

    pub fn new() -> Router {
        let mut route = Router{
            get_route: HashMap::new(),
            post_route: HashMap::new(),
            ignore_url: Vec::new()
        };

        route.set(MainController::new());
        route.set(UserController::new());
    
        //css, js router
        route.add_router(Method::Get, "/css", "css_handler", |request| request.get_request_url());
        route.add_router(Method::Get, "/js", "js_handler", |request| request.get_request_url());
    
        route
    }
}

fn get_content_format(view_name: String) -> String {
    let (mut logger, _) = logger::new();
    let contents = fs::read_to_string(&view_name);

    match contents {
        Ok(contents) => {
            logger.log(&format!("   Find File Name: {}", view_name));

            format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                contents.len(),
                contents
            )
        },
        Err(_) => {
            logger.log("   \x1b[33mError:\x1b[0m \x1b[31mFile not found\x1b[0m");

            String::from("Error")
        }
    }
}

fn get_resource_format(file_name: String) -> String {
    let (mut logger, _) = logger::new();
    let contents = fs::read_to_string(&file_name);

    match contents {
        Ok(contents) => {
            logger.log(&format!("   Find File Name: {}", file_name));

            format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                contents.len(),
                contents
            )
        },
        Err(_) => {
            logger.log("   \x1b[33mError:\x1b[0m \x1b[31mFile not found\x1b[0m");

            String::from("Error")
        }
    }
}

fn get_image_content(file_name: String) -> (String, Vec<u8>) {
    let (mut logger, _) = logger::new();
    let file = fs::read(&file_name).unwrap();
    let file_type = file_name.split(".").collect::<Vec<&str>>();
    let mime_type = match file_type.get(1) {
        Some(mime_type) => {
            match mime_type {
                &"ico" => "x-icon",
                _ => mime_type
            }
        },
        None => {
            ""
        }
    };

    logger.log(&format!("   Find File Name: {}", file_name));

    (mime_type.into(), file)
}