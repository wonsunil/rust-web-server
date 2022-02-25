use std::collections::HashMap;

use crate::router::Router;
use crate::logger;
use crate::method::*;
use crate::util::{ json, map };

pub fn new() -> Router {
    let mut route = Router{
        get_route: HashMap::new(),
        post_route: HashMap::new(),
        ignore_url: Vec::new()
    };

    route.add_router(Method::Get, "/", "main_handler", |_| -> String { String::from("index") });
    route.add_router(Method::Get, "/test", "test_handler", |_| -> String { String::from("test") });
    route.add_router(Method::Get, "/favicon.ico", "favicon", |_| -> String { String::from("favicon.ico") });

    //rest
    route.add_router(Method::Post, "/", "main_post_handler", |request| -> String {
        let (mut logger, _) = logger::new();
        let data = json::parse(&request.get_data());
        logger.log(&format!("   Request Params: {:?}", data));

        json::stringify(map!{
            "status" => "success",
            "message" => "test-success"
        })
    });

    route
}