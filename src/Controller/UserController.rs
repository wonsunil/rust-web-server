use std::collections::HashMap;

use crate::router::Router;
use crate::method::*;
use crate::util::{ map, json };
use crate::service::UserService;
use crate::logger;
use crate::session::SessionStorage::SessionStorage;

pub fn new() -> Router {
    let mut route = Router{
        get_route: HashMap::new(),
        post_route: HashMap::new(),
        ignore_url: Vec::new()
    };

    route.add_router(Method::Get, "/user/register", "register_handler", |_| -> String { String::from("register") });
    route.add_router(Method::Get, "/user/login", "login_handler", |_| -> String { String::from("login") });
    route.add_router(Method::Get, "/user/:userId", "user_test_handler", |request| -> String {
        println!("   Request Parameter: {:?}", request.get_data());
        String::from("profile")
    });

    //rest
    route.add_router(Method::Post, "/user/register", "user_register", |request| -> String {
        let (mut logger, _) = logger::new();
        let data = json::parse(&request.get_data());
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
        let (mut logger, _) = logger::new();
        let data = json::parse(&request.get_data());
        let id = data.get("id");
        let password = data.get("password");
        let mut user_service = UserService::new();

        let mut session = json::parse(&data.get("session"));

        session.insert("test".into(), "test".into());
        session.insert("isLogin".into(), "true".into());

        logger.log("   UserService [");

        let user = user_service.selectUserById(&id);

        if user.len() <= 0 {
            logger.log("   ]");

            return json::stringify(map!{
                "status" => "",
                "message" => "등록되지 않은 아이디입니다."
            });
        };

        let user = user_service.selectUserByIdAndPassword(map!{ "id" => id, "password" => password });

        if user.len() <= 0 {
            logger.log("   ]");

            return json::stringify(map!{
                "status" => "",
                "message" => "잘못된 비밀번호입니다."
            });
        };

        let mut map = map!{
            "status" => "200",
            "message" => "로그인에 성공했습니다."
        };

        let session_data = session.get_data();
        let json_session_data = &json::stringify(session_data);
        
        map.insert("session", json_session_data);

        logger.log("   ]");

        return json::stringify(map);
    });

    route
}