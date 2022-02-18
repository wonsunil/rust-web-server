use std::collections::HashMap;

use crate::router::Router;
use crate::method::*;
use crate::util::{ map, json };
use crate::service::UserService;

pub fn new() -> Router {
    let mut route = Router{
        get_route: HashMap::new(),
        post_route: HashMap::new(),
        ignore_url: Vec::new()
    };

    route.add_router(Method::Get, "/user/:userId", "user_test_handler", |request| -> String {
        println!("   Request Parameter: {:?}", request);
        String::from("profile")
    });

    //rest
    route.add_router(Method::Post, "/user/register", "user_register", |request| -> String {
        let data = json::parse(&request);
        let mut user_service = UserService::new();
        
        let id = data.get("id");
        let password = data.get("password");

        let users = user_service.query("select id from user_info where id = :id", map!{ "id" => id });

        if users.len() > 0 {
            return json::stringify(map!{
                "status" => "fail",
                "message" => "이미 존재하는 아이디입니다."
            });
        }

        let id = data.get("id");
        let result: bool = user_service.insert_user(map!{
            "id" => id,
            "password" => password
        });

        if result {
            return json::stringify(map!{
                "status" => "success",
                "message" => "회원가입이 완료되었습니다."
            });
        };

        return json::stringify(map!{
            "status" => "fail",
            "message" => "회원가입에 실패했습니다."
        });
    });
    route.add_router(Method::Post, "/user/login", "login", |request| -> String {
        let _data = json::parse(&request);
        let mut _user_service = UserService::new();

        return json::stringify(map!{
            "status" => "",
            "message" => "로그인에 실패했습니다."
        });
    });

    route
}