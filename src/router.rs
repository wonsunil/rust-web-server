use std::fs;
use std::io::Write;
use std::net::TcpStream;
use std::collections::HashMap;
use regex::Regex;
use substring::Substring;

use crate::method::*;
use crate::logger;
use crate::util::json;

type Expr = Box<dyn Fn(String) -> String>;

pub struct RouteStruct{
    get_route: HashMap<String, (String, Expr)>,
    post_route: HashMap<String, (String, Expr)>,
    ignore_url: Vec<String>
}

impl RouteStruct{
    pub fn add_router<H>(&mut self, method: Method, url: &str, name: &str, handler: H)
    where
        H: Fn(String) -> String + 'static
    {
        let find = self.find_router(&method, url);
        let (url, name) = (url.to_string(), name.to_string());

        match find{
            Some(route_tuple) => {
                let (name, _) = route_tuple;

                println!("Already Registered Handler: {}, Url: {}", name, url);
            },
            None => {
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
        }
    }

    pub fn find_router(&mut self, method: &Method, url: &str) -> Option<&(String, Box<dyn Fn(String) -> String>)> {
        match method{
            Method::Get => {
                let mut get_route = self.get_route.iter();

                let find = get_route.find(|route| {
                    let (target_url, _) = route;

                    if target_url[..] == url.to_string() {
                        return true;
                    }else {
                        match target_url.find(":") {
                            Some(index) => {
                                let start_index = index;
                                let end = target_url[6..].find("/");

                                if url.starts_with(&target_url[..6]) {
                                    let result = match end {
                                        Some(end_index) => {
                                            println!("{}, {}, {}", target_url, start_index, end_index);
    
                                            true
                                        },
                                        None => {
                                            println!("   Mapped Original Url {}", target_url);
    
                                            let reg_url = target_url.replace(&target_url[start_index..], "[0-9a-zA-Z]");
                                            let regex: Regex = Regex::new(&reg_url).unwrap();
                                            
                                            println!("   Generated Regex {}", regex);
    
                                            regex.is_match(url)
                                        }
                                    };
    
                                    return result;   
                                }
                            },
                            None => {

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

    pub fn call_router(&mut self, mut stream: TcpStream, request_type: String, method: Method, url: &str, data: &str) {
        let (mut logger, mut error_logger) = logger::new();
        let response = self.find_router(&method, url);

        match response{
            Some(route_tuple) => {
                let (name, handler) = route_tuple;
    
                logger.log(&format!("   Handler: {}", name));
    
                let data = match data.len() {
                    0 => url.to_string(),
                    _ => data.to_string()
                };

                
                if request_type == "Request" {
                    let content = get_content_format("public/view/".to_owned() + &handler(data) + ".html");

                    stream.write(content.as_bytes()).unwrap();
                }else {
                    let contents = handler(data);
                    let content = format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                        contents.len(),
                        contents
                    );

                    stream.write(content.as_bytes()).unwrap();
                };

                stream.flush().unwrap();

                logger.log("]");
                println!("");
            },
            None => {
                let css_request_regex = Regex::new("/css/[a-zA-Z]*").unwrap();

                if css_request_regex.is_match(url) {
                    match self.get_route.get("/css") {
                        Some(route_tuple) => {
                            let (name, handler) = route_tuple;

                            logger.log(&format!("   Handler: {}", name));

                            let content = get_content_format("public/".to_owned() + &handler(url.substring(1, url.len()).to_string() + ".css")); 

                            if content == "Error" {
                                logger.log("]");
                                println!("");

                                return;
                            };

                            stream.write(content.as_bytes()).unwrap();
                            stream.flush().unwrap();

                            logger.log("]");
                            println!("");
                        },
                        None => {
                            error_logger.log("Invalid css address");
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
            logger.log("   \x1b[33mError:\x1b[0m \x1b[31mInvalid File Name\x1b[0m");

            String::from("Error")
        }
    }
}

pub fn main() -> RouteStruct {
    let mut route = RouteStruct{
        get_route: HashMap::new(),
        post_route: HashMap::new(),
        ignore_url: Vec::new()
    };

    //default router
    route.add_router(Method::Get, "/", "main_handler", |_text| -> String { String::from("index") });
    route.add_router(Method::Get, "/test", "test_handler", |_text| -> String { String::from("test") });
    route.add_router(Method::Get, "/user/:userId", "user_test_handler", |text| -> String {
        println!("   Request Parameter: {}", text);
        String::from("profile")
    });

    //xhr, fetch router
    route.add_router(Method::Post, "/", "main_post_handler", |text| -> String {
        let (mut logger, _) = logger::new();
        let data: HashMap<String, String> = json::parse(&text);
        logger.log(&format!("   Request Params: {:?}", data));

        "{\"status\": \"success\", \"test\": \"test-success\"}".into()
    });

    //css, js router
    route.add_router(Method::Get, "/css", "css_handler", |url| url);

    route.ignore("/favicon.ico");

    route
}