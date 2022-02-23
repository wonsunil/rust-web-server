use std::collections::HashMap;

use crate::router::Router;
use crate::method::*;
use crate::util::{ map, json };
use crate::service::UserService;
use crate::logger;

pub fn new() -> Router {
    let mut route = Router{
        get_route: HashMap::new(),
        post_route: HashMap::new(),
        ignore_url: Vec::new()
    };

    route.add_router(Method::Get, "/user/register", "register_handler", |_| -> String { String::from("register") });
    route.add_router(Method::Get, "/user/login", "login_handler", |_| -> String { String::from("login") });
    route.add_router(Method::Get, "/user/:userId", "user_test_handler", |request| -> String {
        println!("   Request Parameter: {:?}", request);
        String::from("profile")
    });

    //rest
    route.add_router(Method::Post, "/user/register", "user_register", |request| -> String {
        let (mut logger, _) = logger::new();
        let data = json::parse(&request);
        let mut user_service = UserService::new();

        logger.log("   UserService[");
        
        let id = data.get("id");
        let password = data.get("password");
        let users = user_service.selectUserById(&id);

        if users.len() > 0 {
            logger.log("   ]");

            return json::stringify(map!{
                "status" => "fail",
                "message" => "이미 존재하는 아이디입니다."
            });
        }

        let id = data.get("id");
        let result: bool = user_service.insertUser(map!{
            "id" => id,
            "password" => password
        });

        logger.log("   ]");

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
        let data = json::parse(&request);
        let id = data.get("id");
        let password = data.get("password");
        let mut user_service = UserService::new();
        let user = user_service.selectUserById(&id);

        if user.len() <= 0 {
            return json::stringify(map!{
                "status" => "",
                "message" => "등록되지 않은 아이디입니다."
            });
        };

        let user = user_service.selectUserByIdAndPassword(map!{ "id" => id, "password" => password });

        if user.len() <= 0 {
            return json::stringify(map!{
                "status" => "",
                "message" => "잘못된 비밀번호입니다."
            });
        };

        return json::stringify(map!{
            "status" => "",
            "message" => "로그인에 성공했습니다.",
            "cookie" => "isLogin=true"
        });
    });

    route
}