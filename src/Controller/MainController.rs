// let connect = DataBaseAccess::new();
// let mut conn = connect.pool.get_conn().unwrap();

// let users: Vec<(String, String)> = conn.query("select id, user_role from user_info").unwrap()

use std::collections::HashMap;

use crate::router::Router;
use crate::logger;
use crate::method::*;
use crate::util::json;

pub struct MainController{

}

impl MainController{
    pub fn new() -> Router {
        let mut route = Router{
            get_route: HashMap::new(),
            post_route: HashMap::new(),
            ignore_url: Vec::new()
        };
    
        //default router
        route.add_router(Method::Get, "/", "main_handler", |_text| -> String { String::from("index") });
    
        //xhr, fetch router
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