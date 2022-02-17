use std::collections::HashMap;

use crate::router::Router;
use crate::logger;
use crate::method::*;
use crate::util::json;

pub struct MainController{}
pub struct UserController{}

impl MainController{
    pub fn new() -> Router {
        let mut route = Router{
            get_route: HashMap::new(),
            post_route: HashMap::new(),
            ignore_url: Vec::new()
        };

        route.add_router(Method::Get, "/", "main_handler", |_text| -> String { String::from("index") });
        route.add_router(Method::Get, "/test", "test_handler", |_text| -> String { String::from("test") });

        //rest
        route.add_router(Method::Post, "/", "main_post_handler", |text| -> String {
            let (mut logger, _) = logger::new();
            let data: HashMap<String, String> = json::parse(&text);
            logger.log(&format!("   Request Params: {:?}", data));

            let mut map: HashMap<&str, &str> = HashMap::new();
            map.insert("status", "success");
            map.insert("message", "test-success");

            json::stringify(map)
        });

        route
    }
}

impl UserController{
    pub fn new() -> Router {
        let mut route = Router{
            get_route: HashMap::new(),
            post_route: HashMap::new(),
            ignore_url: Vec::new()
        };

        route.add_router(Method::Get, "/user/:userId", "user_test_handler", |text| -> String {
            println!("   Request Parameter: {}", text);
            String::from("profile")
        });

        //rest
        route.add_router(Method::Post, "/user/register", "user_register_handler", |data| -> String {
            "asdf".into()
        });

        route
    }
}